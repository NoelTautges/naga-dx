#[forbid(missing_docs)]

mod expressions;
mod instructions;
mod io;
mod macros;
mod utils;

pub use macros::MatchMacrosConsumer;
use naga::front::Typifier;

use std::mem::take;

use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::*;
use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};
use naga::*;

pub(crate) struct NagaConsumer {
    /// Module populated in [`finalize`].
    pub module: Module,
    /// Entry point function.
    pub function: Function,
    /// Helper to get expression types.
    typifier: Typifier,
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
            typifier: Typifier::new(),
            program_ty: ProgramType::Vertex,
            temps: vec![],
            outs: vec![],
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
        self.register_constant_buffers(rdef);

        self.program_ty = rdef.program_ty;
        match self.program_ty {
            // TODO: fail better
            ProgramType::Geometry | ProgramType::Hull | ProgramType::Domain => unimplemented!(),
            _ => Action::Continue,
        }
    }

    fn consume_isgn(&mut self, isgn: &IOsgnChunk) -> Action {
        self.consume_isgn(isgn)
    }

    fn consume_osgn(&mut self, osgn: &IOsgnChunk) -> Action {
        self.consume_osgn(osgn)
    }

    fn consume_instruction(&mut self, offset: u32, instruction: SparseInstruction) -> Action {
        let span = Span::new(offset, offset + instruction.opcode.get_instruction_length());

        let statement = match instruction.operands {
            // Declarations
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
            Operands::DclTemps(dcl) => Some(self.handle_decl_temps(span, &dcl)),
            Operands::DclIndexableTemp(_) => None,
            // Boolean
            Operands::And(_) => None,
            Operands::Eq(_) => None,
            Operands::Ge(_) => None,
            Operands::Ige(_) => None,
            Operands::Lt(_) => None,
            Operands::Ne(_) => None,
            Operands::Or(_) => None,
            // Math
            Operands::Add(_) => None,
            Operands::Div(_) => None,
            Operands::Dp2(_) => None,
            Operands::Dp3(_) => None,
            Operands::Dp4(_) => None,
            Operands::Exp(_) => None,
            Operands::Frc(_) => None,
            Operands::IAdd(_) => None,
            Operands::Log(_) => None,
            Operands::Mad(_) => None,
            Operands::Max(_) => None,
            Operands::Min(_) => None,
            Operands::Mul(_) => None,
            Operands::RoundNe(_) => None,
            Operands::RoundNi(_) => None,
            Operands::RoundPi(_) => None,
            Operands::RoundZ(_) => None,
            Operands::Rsq(_) => None,
            Operands::SinCos(_) => None,
            Operands::Sqrt(_) => None,
            // Memory
            Operands::Mov(mov) => Some(self.handle_mov(span, &mov)),
            Operands::MovC(_) => None,
            // Conversions
            Operands::Itof(_) => None,
            Operands::Utof(_) => None,
            Operands::Ftou(_) => None,
            // Control flow
            Operands::If(_) => None,
            Operands::Else => None,
            Operands::EndIf => None,
            Operands::Loop => None,
            Operands::EndLoop => None,
            Operands::Break => None,
            Operands::BreakC(_) => None,
            Operands::Ret => Some(self.handle_ret(span)),
            // Textures
            Operands::Sample(_) => None,
            Operands::SampleL(_) => None,
            // All others
            Operands::Unknown(opcode) => {
                dbg!(opcode);
                todo!();
            }
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
