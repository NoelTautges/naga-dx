use dxbc::{
    binary::Action,
    dr::{IOsgnChunk, RdefChunk, RegisterComponentType, SemanticName, ShaderVariableClass},
};
use naga::{
    Binding, BuiltIn, Expression, FunctionArgument, FunctionResult, GlobalVariable, Handle,
    ScalarKind, Span, Statement, StorageClass, StructMember, Type, TypeInner,
};

use crate::utils::{get_scalar_kind, get_scalar_width, get_vector_size};
use crate::NagaConsumer;

/// Where [`NagaConsumer::get_io_elements`] is called from.
enum IoCaller {
    Input,
    Output,
}

impl NagaConsumer {
    /// Register all constant buffers found in an [RdefChunk].
    pub(crate) fn register_constant_buffers(&mut self, chunk: &RdefChunk) {
        for cb in &chunk.constant_buffers {
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
                        | ShaderVariableClass::MatrixColumns => get_scalar_kind(var.ty.ty),
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
                    *span = var.offset + var.size;
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
            let global = self.module.global_variables.append(global, Span::UNDEFINED);
            let global = Expression::GlobalVariable(global);
            let global = self.function.expressions.append(global, Span::UNDEFINED);

            let members = cb
                .variables
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let expr = Expression::AccessIndex {
                        base: global,
                        index: i as u32,
                    };
                    self.function.expressions.append(expr, Span::UNDEFINED)
                })
                .collect();
            self.constant_buffers.push(members);
        }
    }

    /// Get a struct filled with inputs/outputs, if there are any.
    fn get_io_elements(&mut self, chunk: &IOsgnChunk, caller: &IoCaller) -> Option<Handle<Type>> {
        let mut members = Vec::with_capacity(chunk.elements.len());
        let mut span = 0;
        let mut register = 0;

        for elem in &chunk.elements {
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
                // TODO: figure out what I should do with input names
                let mut binding = Binding::Location {
                    location: register,
                    interpolation: None,
                    sampling: None,
                };
                // TODO: figure out if autoincrementing registers screws things up anywhere
                // We don't use the DXBC register because struct packing means several
                // elements fit into the same register and the Naga IR doesn't allow that:
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-packing-rules
                register += 1;
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
            let ty = self.module.types.insert(ty, Span::UNDEFINED);

            members.push(StructMember {
                // TODO: create more sensible type name from fake semantic name
                name: Some(elem.name.clone()),
                // TODO: spans
                ty,
                binding: Some(binding),
                offset: span,
            });
            span += width;
        }

        let len = self.function.expressions.len();
        for member in members.iter() {
            let global = GlobalVariable {
                name: member.name.clone(),
                class: StorageClass::Private,
                // TODO: find out if we need ResourceBindings on global variables
                binding: None,
                ty: member.ty,
                init: None,
            };
            let global = self.module.global_variables.append(global, Span::UNDEFINED);
            let expr = Expression::GlobalVariable(global);
            let handle = self.function.expressions.append(expr, Span::UNDEFINED);
            if let IoCaller::Output = caller {
                self.outs.push(handle);
            }
        }

        // Skip adding struct if it's empty
        if self.function.expressions.len() - len > 0 {
            let emit = Statement::Emit(self.function.expressions.range_from(len));
            self.function.body.push(emit, Span::UNDEFINED);

            let ty = TypeInner::Struct { members, span };
            let ty = Type {
                name: None,
                inner: ty,
            };
            let ty = self.module.types.insert(ty, Span::UNDEFINED);
            Some(ty)
        } else {
            None
        }
    }

    /// Add function arguments from the [input chunk](IOsgnChunk).
    pub(crate) fn consume_isgn(&mut self, isgn: &IOsgnChunk) -> Action {
        let s = self.get_io_elements(isgn, &IoCaller::Input);
        if let Some(ty) = s {
            let arg = FunctionArgument {
                name: self.module.types[ty].name.clone(),
                ty,
                binding: None,
            };
            self.function.arguments.push(arg);
        }

        Action::Continue
    }

    /// Add function result from the [output chunk](IOsgnChunk).
    pub(crate) fn consume_osgn(&mut self, osgn: &IOsgnChunk) -> Action {
        let s = self.get_io_elements(osgn, &IoCaller::Output);
        if let Some(ty) = s {
            let result = FunctionResult { ty, binding: None };
            self.function.result = Some(result);
        }

        Action::Continue
    }
}
