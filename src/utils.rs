use dxbc::dr::shex::Immediate;
use dxbc::dr::{ComponentName, ComponentSwizzle, OperandToken0, ShaderVariableType};
use naga::{ScalarKind, ScalarValue, SwizzleComponent, VectorSize};

/// Get `naga`'s [VectorSize] from scalar.
pub(crate) fn get_vector_size(size: usize) -> VectorSize {
    match size {
        2 => VectorSize::Bi,
        3 => VectorSize::Tri,
        4 => VectorSize::Quad,
        // TODO: figure out better solution for this
        _ => VectorSize::Quad,
    }
}

/// Get `naga`'s [ScalarValue] from `dxbc`'s [Immediate].
pub(crate) fn get_scalar_value(imm: &Immediate) -> ScalarValue {
    match imm {
        Immediate::U32(n) => ScalarValue::Uint(*n as u64),
        Immediate::U64(n) => ScalarValue::Uint(*n),
        // TODO: find out what these are
        Immediate::Relative(_) => todo!(),
        Immediate::U32Relative(_, _) => todo!(),
        Immediate::U64Relative(_, _) => todo!(),
    }
}

/// Get scalar byte width of `dxbc`'s [Immediate].
pub(crate) fn get_scalar_width(imm: &Immediate) -> u8 {
    match imm {
        Immediate::U32(_) => 4,
        Immediate::U64(_) => 8,
        Immediate::Relative(_) => todo!(),
        Immediate::U32Relative(_, _) => todo!(),
        Immediate::U64Relative(_, _) => todo!(),
    }
}

/// Get first immediate for operand tokens where there's guaranteed to be a first immediate.
///
/// Panics if there is no first immediate.
// TODO: better fails for bad bytecode
pub(crate) fn get_first_immediate(op: &OperandToken0) -> u32 {
    match op.get_immediate(0) {
        Immediate::U32(n) => n,
        _ => unreachable!(),
    }
}

/// Get `naga`'s [SwizzleComponent] from `dxbc`'s [ComponentName].
pub(crate) fn get_swizzle_component(c: &ComponentName) -> SwizzleComponent {
    match c {
        ComponentName::X => SwizzleComponent::X,
        ComponentName::Y => SwizzleComponent::Y,
        ComponentName::Z => SwizzleComponent::Z,
        ComponentName::W => SwizzleComponent::W,
    }
}

/// Get `naga`'s swizzle representation from `dxbc`'s [ComponentSwizzle].
fn get_swizzle_components(c: &ComponentSwizzle) -> [SwizzleComponent; 4] {
    [
        get_swizzle_component(&c.0),
        get_swizzle_component(&c.1),
        get_swizzle_component(&c.2),
        get_swizzle_component(&c.3),
    ]
}

/// Get `naga`'s [ScalarKind] from `dxbc`'s [ShaderVariableType].
fn get_scalar_kind(ty: &ShaderVariableType) -> ScalarKind {
    match ty {
        ShaderVariableType::Int_ => ScalarKind::Sint,
        ShaderVariableType::UInt => ScalarKind::Uint,
        ShaderVariableType::UInt8 => ScalarKind::Uint,
        ShaderVariableType::Float => ScalarKind::Float,
        ShaderVariableType::Bool => ScalarKind::Bool,
        _ => todo!(),
    }
}