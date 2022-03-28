use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::{Operands::*, SparseInstruction};
use naga::Module;

pub struct NagaConsumer {
    pub module: Module,
}

impl NagaConsumer {
    fn new() -> Self {
        NagaConsumer {
            module: Module::default(),
        }
    }
}

impl Consumer for NagaConsumer {
    fn initialize(&mut self) -> Action { Action::Continue }
    fn finalize(&mut self) -> Action { Action::Continue }

    fn consume_instruction(&mut self, offset: u32, instruction: SparseInstruction) -> Action {
        println!("{:#?}", instruction);
        match instruction.operands {
            DclGlobalFlags(_) => (),
            DclInput(_) => (),
            DclInputPs(_) => (),
            DclOutput(_) => (),
            DclConstantBuffer(_) => (),
            DclResource(_) => (),
            DclSampler(_) => (),
            DclOutputSiv(_) => (),
            DclOutputSgv(_) => (),
            DclInputPsSiv(_) => (),
            DclInputPsSgv(_) => (),
            DclTemps(_) => (),
            DclIndexableTemp(_) => (),
            Add(_) => (),
            And(_) => (),
            Mul(_) => (),
            Mad(_) => (),
            Mov(_) => (),
            Itof(_) => (),
            Utof(_) => (),
            Ftou(_) => (),
            If(_) => (),
            Else => (),
            EndIf => (),
            Loop => (),
            EndLoop => (),
            Break => (),
            BreakC(_) => (),
            Sample(_) => (),
            SampleL(_) => (),
            Ret => (),
            Unknown => (),
        }
        Action::Continue
    }
}

pub fn parse<'a, T: AsRef<[u8]>>(shader_bytes: T) -> Result<NagaConsumer, State> {
    let mut consumer = NagaConsumer::new();
    let mut parser = Parser::new(shader_bytes.as_ref(), &mut consumer);
    match parser.parse() {
        Ok(_) => Ok(consumer),
        Err(e) => Err(e),
    }
}
