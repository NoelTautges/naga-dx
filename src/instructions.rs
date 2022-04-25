use dxbc::dr::*;
use naga::{Statement, TypeInner, VectorSize, ScalarKind, Span, Type, Expression, LocalVariable};

use crate::NagaConsumer;

impl NagaConsumer {
    pub(crate) fn handle_decl_temps(&mut self, span: Span, dcl: &DclTemps) -> Statement {
        let four_floats = Type {
            name: None,
            inner: TypeInner::Vector {
                size: VectorSize::Quad,
                // TODO: determine specific type of temporary - analyze fields written to?
                kind: ScalarKind::Float,
                width: 4,
            },
        };
        let four_floats = self.module.types.insert(four_floats, span);
        let len = self.function.expressions.len();
        for i in 0..dcl.register_count {
            let var = LocalVariable {
                name: Some(format!("temp_{}", i)),
                ty: four_floats,
                init: None,
            };
            let var = self.function.local_variables.append(var, span);
            let var = Expression::LocalVariable(var);
            let var = self.function.expressions.append(var, span);
            self.temps.push(var);
        }
    
        Statement::Emit(self.function.expressions.range_from(len))
    }
    
    pub(crate) fn handle_mov(&mut self, span: Span, mov: &Mov) -> Statement {
        let dst = self.get_variable_expression(&mov.dst, span);
        let src = self.get_variable_expression(&mov.src, span);
        Statement::Store {
            pointer: dst,
            value: src,
        }
    }
    
    pub(crate) fn handle_ret(&mut self, span: Span) -> Statement {
        match &self.function.result {
            Some(r) => {
                if let TypeInner::Struct { .. } = &self.module.types[r.ty].inner {
                    let compose = Expression::Compose {
                        ty: r.ty,
                        components: self.outs.clone(),
                    };
                    let compose = self.function.expressions.append(compose, span);
                    Statement::Return {
                        value: Some(compose),
                    }
                } else {
                    Statement::Return { value: None }
                }
            }
            None => Statement::Return { value: None },
        }
    }    
}
