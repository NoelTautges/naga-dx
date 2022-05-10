use dxbc::dr::shex::{Immediate, OperandType};
use dxbc::dr::{ComponentMask, ComponentSelectMode, OperandToken0};
use naga::proc::ResolveContext;
use naga::{
    Constant, ConstantInner, Expression, Handle, ScalarKind, Span, Statement, SwizzleComponent,
    Type, TypeInner, VectorSize,
};

use crate::utils::{
    get_component_name_index, get_first_immediate, get_immediate_value, get_immediate_width,
    get_scalar_value, get_swizzle_component_index, get_swizzle_components, get_vector_size,
};
use crate::NagaConsumer;

/// Broad type of a type - scalar, vector, or pointer.
///
/// Only vectors can be swizzled directly. Pointers need to be
/// [`Load`][Expression::Load]ed before swizzling. Scalars can't be swizzled
/// at all.
enum BroadType {
    /// Any type that can't be swizzled.
    Scalar,
    /// Any type that can be swizzled.
    Vector,
    /// A pointer whose value might be able to be swizzled if
    /// [`Load`][Expression::Load]ed.
    Pointer,
}

impl NagaConsumer {
    /// Resolve [BroadType] corresponding to given [Expression].
    fn get_broad_type(&mut self, expr: Handle<Expression>) -> BroadType {
        let ctx = ResolveContext {
            constants: &self.module.constants,
            types: &self.module.types,
            global_vars: &self.module.global_variables,
            local_vars: &self.function.local_variables,
            functions: &self.module.functions,
            arguments: &self.function.arguments,
        };
        // Panic safety: if there's a resolution error, that's a bug on my end
        // and it deserves to crash
        self.typifier
            .grow(expr, &self.function.expressions, &ctx)
            .unwrap();
        let ty = self.typifier.get(expr, &self.module.types);

        if let TypeInner::Pointer { .. } = ty {
            BroadType::Pointer
        } else if ty.indexable_length(&self.module).is_ok() {
            BroadType::Vector
        } else {
            BroadType::Scalar
        }
    }

    /// [`Expression::Swizzle`] or [`AccessIndex`][Expression::AccessIndex] the
    /// given expression, [`Load`][Expression::Load]ing beforehand as
    /// necessary.
    // TODO: pare down swizzles to human-like forms
    fn get_swizzle(
        &mut self,
        expr: Handle<Expression>,
        op: &OperandToken0,
        span: Span,
    ) -> Handle<Expression> {
        // If swizzle target is a scalar, just return the target
        // If it's a vector, it's fine
        // If it's a pointer, load it and check again
        // TODO: this is bad and doesn't deal with matrices
        let broad_ty = self.get_broad_type(expr);
        let vector = match broad_ty {
            BroadType::Scalar => return expr,
            BroadType::Vector => expr,
            BroadType::Pointer => {
                let load_expr = Expression::Load { pointer: expr };
                let load_expr = self.function.expressions.append(load_expr, span);
                let loaded_ty = self.get_broad_type(load_expr);
                match loaded_ty {
                    BroadType::Scalar => return load_expr,
                    BroadType::Vector => load_expr,
                    // I'm not dealing with pointers to pointers
                    BroadType::Pointer => unimplemented!(),
                }
            }
        };

        // 1-component swizzles are just accesses: https://github.com/gfx-rs/naga/blob/cf32c2b7f38c985e1c770eeff05a91e0cd15ee04/src/front/glsl/variables.rs#L343
        let mode = op.get_component_select_mode();
        let swizzle = if let ComponentSelectMode::Select1 = mode {
            Expression::AccessIndex {
                base: vector,
                index: get_component_name_index(op.get_component_swizzle().0),
            }
        } else {
            let (size, pattern): (VectorSize, [SwizzleComponent; 4]) =
                match op.get_component_select_mode() {
                    ComponentSelectMode::Mask => {
                        let mut components = Vec::with_capacity(4);
                        let mask = op.get_component_mask();

                        if mask.bits().count_ones() == 1 {}

                        if mask.contains(ComponentMask::COMPONENT_MASK_R) {
                            components.push(SwizzleComponent::X);
                        }
                        if mask.contains(ComponentMask::COMPONENT_MASK_G) {
                            components.push(SwizzleComponent::Y);
                        }
                        if mask.contains(ComponentMask::COMPONENT_MASK_B) {
                            components.push(SwizzleComponent::Z);
                        }
                        if mask.contains(ComponentMask::COMPONENT_MASK_A) {
                            components.push(SwizzleComponent::W);
                        }

                        if components.len() == 1 {
                            let expr = Expression::AccessIndex {
                                base: vector,
                                // Panic safety: components.len() == 1
                                index: get_swizzle_component_index(components.first().unwrap()),
                            };
                            return self.function.expressions.append(expr, span);
                        }

                        let size = get_vector_size(components.len());
                        for _ in 0..4 - components.len() {
                            components.push(SwizzleComponent::X);
                        }

                        // Panic safety: we explicitly add dummy elements until the
                        // array is of length 4 so this won't fail
                        (size, components.try_into().unwrap())
                    }
                    ComponentSelectMode::Swizzle => {
                        let swizzle = op.get_component_swizzle();
                        (VectorSize::Quad, get_swizzle_components(&swizzle))
                    }
                    _ => unreachable!(),
                };

            Expression::Swizzle {
                size,
                vector,
                pattern,
            }
        };

        self.function.expressions.append(swizzle, span)
    }

    /// Create an [Expression] corresponding to an operand and return its
    /// handle.
    fn get_variable_expression(&mut self, op: &OperandToken0, span: Span) -> Handle<Expression> {
        let handle = match op.get_operand_type() {
            OperandType::ConstantBuffer => {
                let imms = op.get_immediates();
                let cb_index = get_immediate_value(&imms[0]) as usize;
                let var_index = get_immediate_value(&imms[1]) as usize;
                let expr = self.constant_buffers[cb_index][var_index];
                Some(expr)
            }
            OperandType::Temp => {
                let i = get_first_immediate(*op);
                Some(self.temps[i as usize])
            }
            OperandType::Output => {
                let i = get_first_immediate(*op);
                Some(self.outs[i as usize])
            }
            _ => {
                let expr = match op.get_operand_type() {
                    // TODO: collect inputs into ins
                    OperandType::Input => {
                        let index = get_first_immediate(*op);
                        let base = Expression::FunctionArgument(0);
                        let base = self.function.expressions.append(base, span);
                        let member = Expression::AccessIndex { base, index };
                        Some(member)
                    }
                    OperandType::Immediate32 => {
                        let imms = op.get_immediates();
                        let first = imms.first().unwrap();
                        if imms.len() == 1 {
                            let c = Constant {
                                name: None,
                                // TODO: find out what this is
                                specialization: None,
                                inner: ConstantInner::Scalar {
                                    width: get_immediate_width(first),
                                    value: get_scalar_value(first),
                                },
                            };
                            let const_handle = self.module.constants.fetch_or_append(c, span);
                            Some(Expression::Constant(const_handle))
                        } else {
                            let width = match first {
                                Immediate::U32(_) => 4,
                                Immediate::U64(_) => 8,
                                Immediate::Relative(_) => todo!(),
                                Immediate::U32Relative(_, _) => todo!(),
                                Immediate::U64Relative(_, _) => todo!(),
                            };
                            let ty = Type {
                                name: None,
                                inner: TypeInner::Vector {
                                    size: get_vector_size(imms.len()),
                                    kind: match first {
                                        Immediate::U32(_) | Immediate::U64(_) => ScalarKind::Uint,
                                        Immediate::Relative(_) => todo!(),
                                        Immediate::U32Relative(_, _) => todo!(),
                                        Immediate::U64Relative(_, _) => todo!(),
                                    },
                                    width,
                                },
                            };
                            let ty = self.module.types.insert(ty, span);
                            let components: Vec<Handle<Constant>> = imms
                                .into_iter()
                                .map(|imm| {
                                    let c = Constant {
                                        name: None,
                                        specialization: None,
                                        inner: ConstantInner::Scalar {
                                            width,
                                            value: get_scalar_value(&imm),
                                        },
                                    };
                                    self.module.constants.fetch_or_append(c, span)
                                })
                                .collect();
                            let c = Constant {
                                name: None,
                                specialization: None,
                                inner: ConstantInner::Composite { ty, components },
                            };
                            let handle = self.module.constants.fetch_or_append(c, span);
                            Some(Expression::Constant(handle))
                        }
                    }
                    _ => None,
                };

                if let Some(e) = expr {
                    Some(self.function.expressions.append(e, span))
                } else {
                    None
                }
            }
        };

        if let Some(h) = handle {
            if let Expression::Constant(_) = self.function.expressions[h] {
                h
            } else {
                self.get_swizzle(h, op, span)
            }
        } else {
            todo!()
        }
    }

    pub(crate) fn get_dst_variable_statement(
        &mut self,
        op: &OperandToken0,
        span: Span,
        value: Handle<Expression>,
    ) -> Statement {
        let pointer = self.get_variable_expression(op, span);
        Statement::Store { pointer, value }
    }

    pub(crate) fn get_src_variable_expression(
        &mut self,
        op: &OperandToken0,
        span: Span,
    ) -> Handle<Expression> {
        let var_expr = self.get_variable_expression(op, span);
        if let BroadType::Pointer = self.get_broad_type(var_expr) {
            let load_expr = Expression::Load { pointer: var_expr };
            self.function.expressions.append(load_expr, span)
        } else {
            var_expr
        }
    }
}
