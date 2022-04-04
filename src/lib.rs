use std::collections::HashMap;

use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::{shex::*, Operands, *};
use naga::{
    Binding, BuiltIn, Constant, ConstantInner, Expression, Function, FunctionArgument,
    FunctionResult, Handle, Interpolation, Module, ScalarKind, ScalarValue, Span, Statement,
    StructMember, Type, TypeInner, VectorSize,
};

enum ElementChunk {
    Input,
    Output,
}

pub struct NagaConsumer {
    module: Module,
    function: Function,
    program_ty: ProgramType,
}

impl NagaConsumer {
    fn new() -> Self {
        NagaConsumer {
            module: Module::default(),
            function: Function {
                name: Some("main".to_string()),
                ..Function::default()
            },
            program_ty: ProgramType::Vertex,
        }
    }

    fn get_vector_size(&self, size: usize) -> VectorSize {
        match size {
            2 => VectorSize::Bi,
            3 => VectorSize::Tri,
            4 => VectorSize::Quad,
            // TODO: figure out better solution for this
            _ => VectorSize::Quad,
        }
    }

    fn get_elements(
        &mut self,
        sgn: &IOsgnChunk,
        chunk: ElementChunk,
    ) -> Vec<(Handle<Type>, Option<Binding>)> {
        let mut map = HashMap::<u32, TypeInner>::new();
        for elem in &sgn.elements {
            match map.entry(elem.semantic_index).or_insert(TypeInner::Struct {
                members: vec![],
                span: 0,
            }) {
                // TODO: not using structs with only 1 entry
                TypeInner::Struct { members, span } => {
                    let binding = if let SemanticName::Undefined = elem.semantic_type {
                        Binding::Location {
                            location: elem.register,
                            interpolation: Some(Interpolation::Flat),
                            sampling: None,
                        }
                    } else {
                        Binding::BuiltIn(match elem.semantic_type {
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
                        })
                    };
                    let kind = match elem.component_type {
                        RegisterComponentType::Float32 => ScalarKind::Float,
                        RegisterComponentType::Int32 => ScalarKind::Sint,
                        RegisterComponentType::Uint32 => ScalarKind::Uint,
                        RegisterComponentType::Unknown => todo!(),
                    };
                    let ty = Type {
                        // TODO: type name
                        name: None,
                        inner: {
                            let zeros = 8 - elem.component_mask.leading_zeros();
                            *span += zeros * 4;
                            match zeros {
                                1 => TypeInner::Scalar { kind, width: 4 },
                                _ => TypeInner::Vector {
                                    size: self.get_vector_size(zeros as usize),
                                    kind,
                                    width: 4,
                                },
                            }
                        },
                    };
                    members.push(StructMember {
                        name: Some(elem.name.clone()),
                        // TODO: spans
                        ty: self.module.types.insert(ty, Span::UNDEFINED),
                        binding: Some(binding),
                        offset: (members.len() * 4) as u32,
                    });
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

    fn get_scalar_value(&self, imm: &Immediate) -> ScalarValue {
        match imm {
            Immediate::U32(n) => ScalarValue::Uint(*n as u64),
            Immediate::U64(n) => ScalarValue::Uint(*n),
            // TODO: find out what these are
            Immediate::Relative(_) => todo!(),
            Immediate::U32Relative(_, _) => todo!(),
            Immediate::U64Relative(_, _) => todo!(),
        }
    }

    fn get_scalar_width(&self, imm: &Immediate) -> u8 {
        match imm {
            Immediate::U32(_) => 4,
            Immediate::U64(_) => 8,
            Immediate::Relative(_) => todo!(),
            Immediate::U32Relative(_, _) => todo!(),
            Immediate::U64Relative(_, _) => todo!(),
        }
    }

    fn get_variable_expression(&mut self, op: OperandToken0, span: Span) -> Expression {
        match op.get_operand_type() {
            OperandType::Immediate32 => {
                let imms = op.get_immediates();
                let first = imms.first().unwrap();
                if imms.len() == 1 {
                    let c = Constant {
                        name: None,
                        // TODO: find out what this is
                        specialization: None,
                        inner: ConstantInner::Scalar {
                            width: self.get_scalar_width(first),
                            value: self.get_scalar_value(first),
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
                            size: self.get_vector_size(imms.len()),
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
                                    value: self.get_scalar_value(&imm),
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
        println!("{:#?}", rdef);
        self.program_ty = rdef.program_ty;
        Action::Continue
    }

    fn consume_isgn(&mut self, isgn: &IOsgnChunk) -> Action {
        println!("inputs");
        for ine in &isgn.elements {
            println!("{:#?}", ine);
        }
        self.function.arguments = self
            .get_elements(isgn, ElementChunk::Input)
            .into_iter()
            .map(|(ty, binding)| FunctionArgument {
                name: None,
                ty,
                binding,
            })
            .collect();
        println!("{:#?}", self.function.arguments);
        Action::Continue
    }

    fn consume_osgn(&mut self, osgn: &IOsgnChunk) -> Action {
        println!("outputs");
        for out in &osgn.elements {
            println!("{:#?}", out);
        }
        let mut elems = self.get_elements(osgn, ElementChunk::Output);
        if let Some(elem) = elems.pop() {
            self.function.result = Some(FunctionResult {
                ty: elem.0,
                binding: elem.1,
            });
            println!("{:#?}", self.function.result);
        }
        Action::Continue
    }

    fn consume_instruction(&mut self, offset: u32, instruction: SparseInstruction) -> Action {
        println!("{:#?}", instruction);
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
                None
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
            Operands::Ret => Some(Statement::Kill),
            Operands::Unknown => None,
        };

        if let Some(s) = statement {
            self.function.body.push(s, span);
        }

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        Action::Continue
    }
}

pub fn parse<T: AsRef<[u8]>>(shader_bytes: T) -> Result<NagaConsumer, State> {
    let mut consumer = NagaConsumer::new();
    let mut parser = Parser::new(shader_bytes.as_ref(), &mut consumer);
    match parser.parse() {
        Ok(_) => Ok(consumer),
        Err(e) => Err(e),
    }
}
