use std::mem::take;

use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::{shex::*, Operands, *};
use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};
use naga::{
    Binding, BuiltIn, Constant, ConstantInner, EntryPoint, Expression, Function, FunctionArgument,
    FunctionResult, GlobalVariable, Handle, LocalVariable, Module, ResourceBinding, ScalarKind,
    ScalarValue, ShaderStage, Span, Statement, StorageClass, StructMember, Type, TypeInner,
    VectorSize,
};

fn get_vector_size(size: usize) -> VectorSize {
    match size {
        2 => VectorSize::Bi,
        3 => VectorSize::Tri,
        4 => VectorSize::Quad,
        // TODO: figure out better solution for this
        _ => VectorSize::Quad,
    }
}

fn get_scalar_value(imm: &Immediate) -> ScalarValue {
    match imm {
        Immediate::U32(n) => ScalarValue::Uint(*n as u64),
        Immediate::U64(n) => ScalarValue::Uint(*n),
        // TODO: find out what these are
        Immediate::Relative(_) => todo!(),
        Immediate::U32Relative(_, _) => todo!(),
        Immediate::U64Relative(_, _) => todo!(),
    }
}

fn get_scalar_width(imm: &Immediate) -> u8 {
    match imm {
        Immediate::U32(_) => 4,
        Immediate::U64(_) => 8,
        Immediate::Relative(_) => todo!(),
        Immediate::U32Relative(_, _) => todo!(),
        Immediate::U64Relative(_, _) => todo!(),
    }
}

// TODO: better fails for bad bytecode
fn get_first_immediate(op: &OperandToken0) -> u32 {
    match op.get_immediate(0) {
        Immediate::U32(n) => n,
        _ => unreachable!(),
    }
}

pub struct NagaConsumer {
    /// Module populated in [`finalize`].
    module: Module,
    /// Entry point function.
    function: Function,
    /// Program type. Vertex, pixel, etc.
    program_ty: ProgramType,
    /// Temporary registers as [`Expression::LocalVariable`]s.
    temps: Vec<Handle<Expression>>,
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
        }
    }

    fn get_elements(&mut self, sgn: &IOsgnChunk) -> TypeInner {
        let mut members = Vec::with_capacity(sgn.elements.len());
        let mut span = 0;

        for elem in &sgn.elements {
            dbg!(&elem);
            let kind = match elem.component_type {
                RegisterComponentType::Float32 => ScalarKind::Float,
                RegisterComponentType::Int32 => ScalarKind::Sint,
                RegisterComponentType::Uint32 => ScalarKind::Uint,
                RegisterComponentType::Unknown => todo!(),
            };

            let zeros = 8 - elem.component_mask.leading_zeros();
            let width = zeros * 4;
            // TODO: matrices? https://docs.rs/naga/latest/naga/enum.Binding.html#method.apply_default_interpolation
            let inner = if zeros == 1 {
                TypeInner::Scalar { kind, width: 4 }
            } else {
                TypeInner::Vector {
                    size: get_vector_size(zeros as usize),
                    kind,
                    width: 4,
                }
            };

            // TODO: find binding from fake input name
            let binding = if let SemanticName::Undefined = elem.semantic_type {
                let mut binding = Binding::Location {
                    location: elem.register,
                    interpolation: None,
                    sampling: None,
                };
                binding.apply_default_interpolation(&inner);
                binding
            } else {
                let semantic = match elem.semantic_type {
                    SemanticName::Undefined => unreachable!(),
                    SemanticName::Position => BuiltIn::Position,
                    SemanticName::ClipDistance => BuiltIn::ClipDistance,
                    SemanticName::CullDistance => BuiltIn::CullDistance,
                    SemanticName::RenderTargetArrayIndex => todo!(),
                    SemanticName::ViewportArrayIndex => BuiltIn::ViewIndex,
                    SemanticName::VertexId => BuiltIn::VertexIndex,
                    SemanticName::PrimitiveId => BuiltIn::PrimitiveIndex,
                    SemanticName::InstanceId => BuiltIn::InstanceIndex,
                    SemanticName::IsFrontFace => BuiltIn::FrontFacing,
                    SemanticName::SampleIndex => BuiltIn::SampleIndex,
                    SemanticName::FinalQuadEdgeTessfactor => todo!(),
                    SemanticName::FinalQuadInsideTessfactor => todo!(),
                    SemanticName::FinalTriEdgeTessfactor => todo!(),
                    SemanticName::FinalTriInsideTessfactor => todo!(),
                    SemanticName::FinalLineDetailTessfactor => todo!(),
                    SemanticName::FinalLineDensityTessfactor => todo!(),
                    SemanticName::Target => todo!(),
                    SemanticName::Depth => BuiltIn::FragDepth,
                    SemanticName::Coverage => todo!(),
                    SemanticName::DepthGreaterEqual => todo!(),
                    SemanticName::DepthLessEqual => todo!(),
                };
                Binding::BuiltIn(semantic)
            };

            // Type construction is delayed because we need &inner for interpolation and sampling
            let ty = Type {
                // TODO: struct name
                name: None,
                inner,
            };

            members.push(StructMember {
                // TODO: create more sensible type name from fake semantic name
                name: Some(elem.name.clone()),
                // TODO: spans
                ty: self.module.types.insert(ty, Span::UNDEFINED),
                binding: Some(binding),
                offset: span,
            });
            span += width;
        }

        TypeInner::Struct { members, span }
    }

    fn get_variable_expression(&mut self, op: OperandToken0, span: Span) -> Handle<Expression> {
        let expr = match op.get_operand_type() {
            OperandType::Input => {
                let first = get_first_immediate(&op);
                Some(Expression::FunctionArgument(first))
            }
            OperandType::Output => {
                let first = get_first_immediate(&op);
                // TODO: remove hardcoding
                let global = GlobalVariable {
                    name: Some("Position".to_owned()),
                    class: StorageClass::Private,
                    binding: Some(ResourceBinding {
                        group: 0,
                        binding: 0,
                    }),
                    ty: self.module.types.insert(
                        Type {
                            name: None,
                            inner: TypeInner::Vector {
                                size: VectorSize::Quad,
                                kind: ScalarKind::Uint,
                                width: 4,
                            },
                        },
                        span,
                    ),
                    init: None,
                };
                Some(Expression::GlobalVariable(
                    self.module.global_variables.append(global, span),
                ))
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
                            width: get_scalar_width(first),
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
                let i = match op.get_immediate(0) {
                    Immediate::U32(n) => n,
                    _ => unreachable!(),
                };
                Some(self.temps[i as usize])
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
        self.program_ty = rdef.program_ty;
        match self.program_ty {
            // TODO: fail better
            ProgramType::Geometry | ProgramType::Hull | ProgramType::Domain => unimplemented!(),
            _ => Action::Continue,
        }
    }

    fn consume_isgn(&mut self, isgn: &IOsgnChunk) -> Action {
        let s = self.get_elements(isgn);
        if let TypeInner::Struct { members, .. } = s {
            for member in members {
                let arg = FunctionArgument {
                    name: member.name,
                    ty: member.ty,
                    binding: member.binding,
                };
                self.function.arguments.push(arg);
            }
        }
        println!("done with inputs");
        Action::Continue
    }

    fn consume_osgn(&mut self, osgn: &IOsgnChunk) -> Action {
        let s = self.get_elements(osgn);

        // Skip adding output struct if it's empty
        if let TypeInner::Struct { members, .. } = &s {
            if members.is_empty() {
                return Action::Continue;
            }
        }

        let ty = Type {
            name: None,
            inner: s,
        };
        let ty = self.module.types.insert(ty, Span::UNDEFINED);
        let result = FunctionResult { ty, binding: None };
        self.function.result = Some(result);

        Action::Continue
    }

    fn consume_instruction(&mut self, offset: u32, instruction: SparseInstruction) -> Action {
        dbg!(&instruction);
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
                let dst = self.get_variable_expression(dst, span);
                let src = self.get_variable_expression(src, span);
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
                    if let TypeInner::Struct { members, .. } = &self.module.types[r.ty].inner {
                        None
                    } else {
                        None
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
    let info = validator.validate(&consumer.module).unwrap();
    Ok((consumer.module, info))
}
