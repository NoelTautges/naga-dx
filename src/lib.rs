use std::collections::HashMap;

use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::{Operands, *};
use naga::{
    Binding, BuiltIn, Function, FunctionArgument, FunctionResult, Handle, Interpolation, Module,
    ScalarKind, Span, StructMember, Type, TypeInner, VectorSize,
};

enum ElementChunk {
    Input,
    Output,
}

pub struct NagaConsumer {
    module: Module,
    function: Function,
}

impl NagaConsumer {
    fn new() -> Self {
        NagaConsumer {
            module: Module::default(),
            function: Function::default(),
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
                                    size: match zeros {
                                        2 => VectorSize::Bi,
                                        3 => VectorSize::Tri,
                                        4 => VectorSize::Quad,
                                        _ => unreachable!(),
                                    },
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
}

impl Consumer for NagaConsumer {
    fn initialize(&mut self) -> Action {
        Action::Continue
    }

    fn finalize(&mut self) -> Action {
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

    fn consume_instruction(&mut self, _offset: u32, instruction: SparseInstruction) -> Action {
        //println!("{:#?}", instruction);
        match instruction.operands {
            Operands::DclGlobalFlags(_) => (),
            Operands::DclInput(_) => (),
            Operands::DclInputPs(_) => (),
            Operands::DclOutput(_) => (),
            Operands::DclConstantBuffer(_) => (),
            Operands::DclResource(_) => (),
            Operands::DclSampler(_) => (),
            Operands::DclOutputSiv(_) => (),
            Operands::DclOutputSgv(_) => (),
            Operands::DclInputPsSiv(_) => (),
            Operands::DclInputPsSgv(_) => (),
            Operands::DclTemps(_) => (),
            Operands::DclIndexableTemp(_) => (),
            Operands::Add(_) => (),
            Operands::And(_) => (),
            Operands::Mul(_) => (),
            Operands::Mad(_) => (),
            Operands::Mov(_) => (),
            Operands::Itof(_) => (),
            Operands::Utof(_) => (),
            Operands::Ftou(_) => (),
            Operands::If(_) => (),
            Operands::Else => (),
            Operands::EndIf => (),
            Operands::Loop => (),
            Operands::EndLoop => (),
            Operands::Break => (),
            Operands::BreakC(_) => (),
            Operands::Sample(_) => (),
            Operands::SampleL(_) => (),
            Operands::Ret => (),
            Operands::Unknown => (),
        }
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