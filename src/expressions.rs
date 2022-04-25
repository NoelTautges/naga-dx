use dxbc::dr::{OperandToken0, shex::{OperandType, Immediate}};
use naga::{Span, Handle, Expression, ConstantInner, Constant, Type, TypeInner, ScalarKind};

use crate::{NagaConsumer, utils::{get_first_immediate, get_scalar_value, get_vector_size, get_immediate_width}};

impl NagaConsumer {
    pub(crate) fn get_variable_expression(&mut self, op: &OperandToken0, span: Span) -> Handle<Expression> {
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
            return self.function.expressions.append(e, span);
        }
    
        let handle = match op.get_operand_type() {
            OperandType::Temp => {
                let i = get_first_immediate(*op);
                Some(self.temps[i as usize])
            }
            OperandType::Output => {
                let i = get_first_immediate(*op);
                Some(self.outs[i as usize])
            }
            _ => todo!(),
        };
    
        if let Some(h) = handle {
            h
        } else {
            todo!()
        }
    }
}
