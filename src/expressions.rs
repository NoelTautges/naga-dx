use dxbc::dr::{
    shex::{Immediate, OperandType},
    ComponentMask, ComponentName, ComponentSelectMode, OperandToken0,
};
use naga::{
    proc::ResolveContext, Constant, ConstantInner, Expression, Handle, ScalarKind, ScalarValue,
    Span, SwizzleComponent, Type, TypeInner, VectorSize,
};

use crate::utils::{
    get_first_immediate, get_immediate_width, get_scalar_value, get_swizzle_components,
    get_vector_size,
};
use crate::NagaConsumer;

/// Broad type of a type - scalar, vector, or pointer.
///
/// Only vectors can be swizzled directly. Pointers need to be
/// [`Load`][Expression::Load]ed before swizzling. Scalars can't be swizzled
/// at all.
enum BroadType {
    Scalar,
    Vector,
    Pointer,
}

impl NagaConsumer {
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

    /// [`Expression::Swizzle`] or [`Access`][Expression::Access] the given
    /// expression, [`Load`][Expression::Load]ing beforehand as necessary.
    fn get_swizzle(
        &mut self,
        expr: Handle<Expression>,
        op: &OperandToken0,
        span: Span,
    ) -> Handle<Expression> {
        // If swizzle target is not actually a vector, just return the target
        let broad_ty = self.get_broad_type(expr);
        let vector = match broad_ty {
            BroadType::Scalar => return expr,
            BroadType::Vector => expr,
            BroadType::Pointer => {
                let load_expr = Expression::Load { pointer: expr };
                self.function.expressions.append(load_expr, span)
            }
        };

        // 1-component swizzles are just accesses: https://github.com/gfx-rs/naga/blob/cf32c2b7f38c985e1c770eeff05a91e0cd15ee04/src/front/glsl/variables.rs#L343
        let mode = op.get_component_select_mode();
        let swizzle = if let ComponentSelectMode::Select1 = mode {
            Expression::Access {
                base: vector,
                index: {
                    let num = match op.get_component_swizzle().0 {
                        ComponentName::X => 0,
                        ComponentName::Y => 1,
                        ComponentName::Z => 2,
                        ComponentName::W => 3,
                    };
                    let constant = Constant {
                        name: None,
                        specialization: None,
                        inner: ConstantInner::Scalar {
                            width: 1,
                            value: ScalarValue::Uint(num),
                        },
                    };
                    let expr =
                        Expression::Constant(self.module.constants.fetch_or_append(constant, span));
                    self.function.expressions.append(expr, span)
                },
            }
        } else {
            let (size, pattern): (VectorSize, [SwizzleComponent; 4]) =
                match op.get_component_select_mode() {
                    ComponentSelectMode::Mask => {
                        let mut components = Vec::with_capacity(4);
                        let mask = op.get_component_mask();

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
    pub(crate) fn get_variable_expression(
        &mut self,
        op: &OperandToken0,
        span: Span,
    ) -> Handle<Expression> {
        let handle = match op.get_operand_type() {
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
            self.get_swizzle(h, op, span)
        } else {
            todo!()
        }
    }
}
