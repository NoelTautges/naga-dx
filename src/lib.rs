mod io;
mod utils;

use std::mem::take;

use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::shex::{Immediate, OperandType};
use dxbc::dr::*;
use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};
use naga::*;

use utils::{get_first_immediate, get_immediate_width, get_scalar_value, get_vector_size};

use crate::utils::{get_scalar_kind, get_scalar_width};

pub(crate) struct NagaConsumer {
    /// Module populated in [`finalize`].
    pub module: Module,
    /// Entry point function.
    pub function: Function,
    /// Program type. Vertex, pixel, etc.
    program_ty: ProgramType,
    /// Temporary registers as [`Expression::LocalVariable`]s.
    temps: Vec<Handle<Expression>>,
    /// Output struct members as [`Expression`]s.
    outs: Vec<Handle<Expression>>,
}

impl NagaConsumer {
    fn new() -> Self {
        let module = Module::default();
        let function = Function {
            name: Some("main".to_string()),
            ..Function::default()
        };
        NagaConsumer {
            module,
            function,
            program_ty: ProgramType::Vertex,
            temps: vec![],
            outs: vec![],
        }
    }

    /*fn get_swizzle(&mut self, op: &OperandToken0, span: &Span) -> Handle<Expression> {
        match op.get_component_select_mode() {
            ComponentSelectMode::Mask => todo!(),
            ComponentSelectMode::Swizzle => todo!(),
            ComponentSelectMode::Select1 => {
                let name = get_swizzle_component(&op.get_component_swizzle().0);
                [name, name, name, name]
            }
        }
    }*/

    fn get_variable_expression(&mut self, op: &OperandToken0, span: &Span) -> Handle<Expression> {
        let expr = match op.get_operand_type() {
            OperandType::Input => {
                let index = get_first_immediate(*op);
                let base = Expression::FunctionArgument(0);
                let base = self.function.expressions.append(base, *span);
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
                    let const_handle = self.module.constants.fetch_or_append(c, *span);
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
                    let ty = self.module.types.insert(ty, *span);
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
                            self.module.constants.fetch_or_append(c, *span)
                        })
                        .collect();
                    let c = Constant {
                        name: None,
                        specialization: None,
                        inner: ConstantInner::Composite { ty, components },
                    };
                    let handle = self.module.constants.fetch_or_append(c, *span);
                    Some(Expression::Constant(handle))
                }
            }
            _ => None,
        };

        if let Some(e) = expr {
            return self.function.expressions.append(e, *span);
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

    /*fn register_shader_type(&mut self, ty: &ShaderType) -> Handle<Type> {
        let inner = match ty.class {
            ShaderVariableClass::Scalar => TypeInner::Scalar {
                kind: get_scalar_kind(&ty.ty),
                width: get_scalar,
            },
            ShaderVariableClass::Vector => todo!(),
            ShaderVariableClass::MatrixRows => todo!(),
            ShaderVariableClass::MatrixColumns => todo!(),
            ShaderVariableClass::Object => todo!(),
            ShaderVariableClass::Struct => todo!(),
            ShaderVariableClass::InterfaceClass => todo!(),
            ShaderVariableClass::InterfacePointer => todo!(),
        };
    }*/
}

impl Default for NagaConsumer {
    fn default() -> Self {
        Self::new()
    }
}

impl Consumer for NagaConsumer {
    fn initialize(&mut self) -> Action {
        Action::Continue
    }

    fn consume_rdef(&mut self, rdef: &RdefChunk) -> Action {
        dbg!(rdef);
        self.program_ty = rdef.program_ty;

        for cb in &rdef.constant_buffers {
            let mut inner = TypeInner::Struct {
                members: Vec::new(),
                span: 0,
            };

            // I Can't Believe It's Not Struct
            if let TypeInner::Struct { members, span } = &mut inner {
                for var in &cb.variables {
                    let kind = match var.ty.class {
                        ShaderVariableClass::Scalar
                        | ShaderVariableClass::Vector
                        | ShaderVariableClass::MatrixColumns => {
                            println!("{:#?}", &var);
                            get_scalar_kind(var.ty.ty)
                        }
                        _ => todo!(),
                    };
                    let width = get_scalar_width(kind);
                    let inner = match var.ty.class {
                        ShaderVariableClass::Scalar => TypeInner::Scalar { kind, width },
                        ShaderVariableClass::Vector => TypeInner::Vector {
                            size: get_vector_size(var.ty.columns.into()),
                            kind,
                            width,
                        },
                        ShaderVariableClass::MatrixColumns => TypeInner::Matrix {
                            columns: get_vector_size(var.ty.columns.into()),
                            rows: get_vector_size(var.ty.rows.into()),
                            width,
                        },
                        _ => unreachable!(),
                    };
                    let ty = Type {
                        name: Some(var.name.to_owned()),
                        inner,
                    };
                    let ty = self.module.types.insert(ty, Span::UNDEFINED);

                    let member = StructMember {
                        name: Some(var.name.to_owned()),
                        ty,
                        binding: None,
                        offset: var.offset,
                    };
                    members.push(member);
                    *span += var.size;
                }
            }

            let name = cb.name.to_owned();
            let ty = Type {
                name: Some(name.clone()),
                inner,
            };
            let ty = self.module.types.insert(ty, Span::UNDEFINED);

            let global = GlobalVariable {
                name: Some(name),
                class: StorageClass::Uniform,
                binding: None,
                ty,
                init: None,
            };
            self.module.global_variables.append(global, Span::UNDEFINED);
        }

        match self.program_ty {
            // TODO: fail better
            ProgramType::Geometry | ProgramType::Hull | ProgramType::Domain => unimplemented!(),
            _ => Action::Continue,
        }
    }

    fn consume_isgn(&mut self, isgn: &IOsgnChunk) -> Action {
        io::consume_isgn(self, isgn)
    }

    fn consume_osgn(&mut self, osgn: &IOsgnChunk) -> Action {
        io::consume_osgn(self, osgn)
    }

    fn consume_instruction(&mut self, offset: u32, instruction: SparseInstruction) -> Action {
        let span = Span::new(offset, offset + instruction.opcode.get_instruction_length());

        let statement = match instruction.operands {
            Operands::DclGlobalFlags(_) => None,
            Operands::DclInput(_) => None,
            Operands::DclInputPs(_) => None,
            Operands::DclOutput(_) => None,
            Operands::DclConstantBuffer(_) => None,
            Operands::DclResource(_) => None,
            Operands::DclSampler(_) => None,
            Operands::DclOutputSiv(_) => None,
            Operands::DclOutputSgv(_) => None,
            Operands::DclInputPsSiv(_) => None,
            Operands::DclInputPsSgv(_) => None,
            Operands::DclTemps(dcl) => {
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
                Some(Statement::Emit(self.function.expressions.range_from(len)))
            }
            Operands::DclIndexableTemp(_) => None,
            Operands::Add(_) => None,
            Operands::And(_) => None,
            Operands::Mul(_) => None,
            Operands::Mad(_) => None,
            Operands::Mov(Mov { dst, src }) => {
                let dst = self.get_variable_expression(&dst, &span);
                let src = self.get_variable_expression(&src, &span);
                let store = Statement::Store {
                    pointer: dst,
                    value: src,
                };
                Some(store)
            }
            Operands::Itof(_) => None,
            Operands::Utof(_) => None,
            Operands::Ftou(_) => None,
            Operands::If(_) => None,
            Operands::Else => None,
            Operands::EndIf => None,
            Operands::Loop => None,
            Operands::EndLoop => None,
            Operands::Break => None,
            Operands::BreakC(_) => None,
            Operands::Sample(_) => None,
            Operands::SampleL(_) => None,
            Operands::Ret => match &self.function.result {
                Some(r) => {
                    if let TypeInner::Struct { .. } = &self.module.types[r.ty].inner {
                        let compose = Expression::Compose {
                            ty: r.ty,
                            components: self.outs.clone(),
                        };
                        let compose = self.function.expressions.append(compose, span);
                        Some(Statement::Return {
                            value: Some(compose),
                        })
                    } else {
                        Some(Statement::Return { value: None })
                    }
                }
                None => Some(Statement::Return { value: None }),
            },
            Operands::Unknown => None,
        };

        if let Some(s) = statement {
            self.function.body.push(s, span);
        }

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        let entry_point = EntryPoint {
            name: "main".to_owned(),
            stage: match self.program_ty {
                ProgramType::Pixel => ShaderStage::Fragment,
                ProgramType::Vertex => ShaderStage::Vertex,
                ProgramType::Compute => ShaderStage::Compute,
                _ => unreachable!(),
            },
            early_depth_test: None,
            workgroup_size: [0, 0, 0],
            function: take(&mut self.function),
        };
        self.module.entry_points.push(entry_point);
        Action::Continue
    }
}

pub fn parse<T: AsRef<[u8]>>(shader_bytes: T) -> Result<(Module, ModuleInfo), State> {
    let mut consumer = NagaConsumer::new();
    let mut parser = Parser::new(shader_bytes.as_ref(), &mut consumer);
    if let Err(e) = parser.parse() {
        return Err(e);
    }

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
    let info = validator.validate(&consumer.module);
    // TODO: better error handling
    let info = info.unwrap();

    Ok((consumer.module, info))
}
