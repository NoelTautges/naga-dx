use std::collections::HashMap;
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

enum ElementChunk {
    Input,
    Output,
}

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

pub struct NagaConsumer {
    /// Module populated in [`finalize`].
    module: Module,
    /// Entry point function.
    function: Function,
    /// Program type. Vertex, pixel, etc.
    program_ty: ProgramType,
    /// Output struct as a [`LocalVariable`](Expression::LocalVariable).
    out: Option<Handle<Expression>>,
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
            out: None,
        }
    }

    fn get_elements(
        &mut self,
        sgn: &IOsgnChunk,
        chunk: ElementChunk,
    ) -> Vec<(Handle<Type>, Option<Binding>)> {
        let mut map = HashMap::<u32, TypeInner>::new();

        for elem in &sgn.elements {
            dbg!(&elem);
            match map.entry(elem.semantic_index).or_insert(TypeInner::Struct {
                members: vec![],
                span: 0,
            }) {
                TypeInner::Struct { members, span } => {
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

                    // Delay constructing type because we need &inner for interpolation and sampling
                    let ty = Type {
                        // TODO: struct name
                        name: None,
                        inner,
                    };

                    members.push(StructMember {
                        // TODO: create more sensible type name from input semantic fake name
                        name: Some(elem.name.clone()),
                        // TODO: spans
                        ty: self.module.types.insert(ty, Span::UNDEFINED),
                        binding: Some(binding),
                        offset: *span,
                    });
                    *span += width;
                }
                _ => unreachable!(),
            }
        }

        map.into_iter()
            .map(|(i, ty)| {
                let struct_ = Type {
                    name: Some(format!(
                        "{}_{}",
                        match chunk {
                            ElementChunk::Input => "input",
                            ElementChunk::Output => "output",
                        },
                        i,
                    )),
                    inner: ty,
                };
                (self.module.types.insert(struct_, Span::UNDEFINED), None)
            })
            .collect()
    }

    fn get_variable_expression(&mut self, op: OperandToken0, span: Span) -> Handle<Expression> {
        let expr = match op.get_operand_type() {
            OperandType::Output => {
                let first = match op.get_immediate(0) {
                    Immediate::U32(n) => n,
                    _ => unreachable!(),
                };
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
                Expression::GlobalVariable(self.module.global_variables.append(global, span))
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
                    Expression::Constant(const_handle)
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
                    Expression::Constant(handle)
                }
            }
            _ => todo!(),
        };
        self.function.expressions.append(expr, span)
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
        self.function.arguments = self
            .get_elements(isgn, ElementChunk::Input)
            .into_iter()
            .map(|(ty, binding)| FunctionArgument {
                name: None,
                ty,
                binding,
            })
            .collect();
        println!("done with inputs");
        Action::Continue
    }

    fn consume_osgn(&mut self, osgn: &IOsgnChunk) -> Action {
        let mut elems = self.get_elements(osgn, ElementChunk::Output);
        if let Some(elem) = elems.pop() {
            self.function.result = Some(FunctionResult {
                ty: elem.0,
                binding: elem.1,
            });
            let out = LocalVariable {
                name: Some(format!("{:?}Out", self.program_ty)),
                ty: elem.0,
                init: None,
            };
            let out = self.function.local_variables.append(out, Span::UNDEFINED);
            let out = self
                .function
                .expressions
                .append(Expression::LocalVariable(out), Span::UNDEFINED);
            self.out = Some(out);
        }

        Action::Continue
    }

    fn consume_instruction(&mut self, offset: u32, instruction: SparseInstruction) -> Action {
        dbg!(&instruction);
        return Action::Continue;
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
            Operands::DclTemps(_) => None,
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
                Some(r) => None,
                None => None,
            },
            Operands::Unknown => None,
        };

        if let Some(s) = statement {
            self.function.body.push(s, span);
        }

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        self.module.entry_points.push(EntryPoint {
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
        });
        dbg!(&self.module.entry_points.first().unwrap().function.body);
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
