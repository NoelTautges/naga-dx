use dxbc::{
    binary::Action,
    dr::{IOsgnChunk, RegisterComponentType, SemanticName},
};
use naga::{
    Binding, BuiltIn, Expression, FunctionArgument, FunctionResult, GlobalVariable, Handle,
    ScalarKind, Span, Statement, StorageClass, StructMember, Type, TypeInner,
};

use crate::utils::get_vector_size;
use crate::NagaConsumer;

/// Where [`NagaConsumer::get_io_elements`] is called from.
enum IoCaller {
    Input,
    Output,
}

fn get_io_elements(
    consumer: &mut NagaConsumer,
    sgn: &IOsgnChunk,
    caller: &IoCaller,
) -> Option<Handle<Type>> {
    let mut members = Vec::with_capacity(sgn.elements.len());
    let mut span = 0;

    for elem in &sgn.elements {
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
            ty: consumer.module.types.insert(ty, Span::UNDEFINED),
            binding: Some(binding),
            offset: span,
        });
        span += width;
    }

    let len = consumer.function.expressions.len();
    for member in members.iter() {
        let global = GlobalVariable {
            name: member.name.clone(),
            class: StorageClass::Private,
            // TODO: find out if we need ResourceBindings on global variables
            binding: None,
            ty: member.ty,
            init: None,
        };
        let global = consumer
            .module
            .global_variables
            .append(global, Span::UNDEFINED);
        let expr = Expression::GlobalVariable(global);
        let handle = consumer.function.expressions.append(expr, Span::UNDEFINED);
        if let IoCaller::Output = caller {
            consumer.outs.push(handle);
        }
    }

    // Skip adding struct if it's empty
    if consumer.function.expressions.len() - len > 0 {
        let emit = Statement::Emit(consumer.function.expressions.range_from(len));
        consumer.function.body.push(emit, Span::UNDEFINED);

        let ty = TypeInner::Struct { members, span };
        let ty = Type {
            name: None,
            inner: ty,
        };
        let ty = consumer.module.types.insert(ty, Span::UNDEFINED);
        Some(ty)
    } else {
        None
    }
}

/// Add function arguments from the [input chunk](IOsgnChunk).
pub(crate) fn consume_isgn(consumer: &mut NagaConsumer, isgn: &IOsgnChunk) -> Action {
    let s = get_io_elements(consumer, isgn, &IoCaller::Input);
    if let Some(ty) = s {
        let arg = FunctionArgument {
            name: consumer.module.types[ty].name.clone(),
            ty,
            binding: None,
        };
        consumer.function.arguments.push(arg);
    }

    Action::Continue
}

/// Add function result from the [output chunk](IOsgnChunk).
pub(crate) fn consume_osgn(consumer: &mut NagaConsumer, osgn: &IOsgnChunk) -> Action {
    let s = get_io_elements(consumer, osgn, &IoCaller::Output);
    if let Some(ty) = s {
        let result = FunctionResult { ty, binding: None };
        consumer.function.result = Some(result);
    }

    Action::Continue
}
