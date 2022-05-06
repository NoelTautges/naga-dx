use dxbc::binary::{Action, Consumer, Parser};
use dxbc::dr::{Operands, SparseInstruction};
use include_dir::{include_dir, Dir, DirEntry, File};
use naga::FastHashMap;
use once_cell::sync::Lazy;

static SHADER_DIR: Dir = include_dir!("shaders/compiled");

/// Smaller representation of [`dxbc`]'s [`Operands`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    // Boolean
    And,
    Eq,
    Ge,
    Ige,
    Lt,
    Ne,
    Or,
    // Math
    Add,
    Div,
    Dp2,
    Dp3,
    Dp4,
    Exp,
    Frc,
    IAdd,
    Log,
    Mad,
    Max,
    Min,
    Mul,
    RoundNe,
    RoundNi,
    RoundPi,
    RoundZ,
    Rsq,
    SinCos,
    Sqrt,
    // Memory
    Mov,
    MovC,
    // Conversions
    Itof,
    Utof,
    Ftou,
    // Control flow
    If,
    Else,
    EndIf,
    Loop,
    EndLoop,
    Break,
    BreakC,
    // Textures
    Sample,
    SampleL,
    // All others
    Unknown,
}

impl Instruction {
    /// Get an [`Instruction`] from [`dxbc`]'s [`SparseInstruction`] if it
    /// should be included in an instruction chain.
    fn try_from(instruction: SparseInstruction) -> Option<Self> {
        match instruction.operands {
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
            Operands::DclTemps(_) => None,
            Operands::DclIndexableTemp(_) => None,
            // Boolean
            Operands::And(_) => Some(Instruction::And),
            Operands::Eq(_) => Some(Instruction::Eq),
            Operands::Ge(_) => Some(Instruction::Ge),
            Operands::Ige(_) => Some(Instruction::Ige),
            Operands::Lt(_) => Some(Instruction::Lt),
            Operands::Ne(_) => Some(Instruction::Ne),
            Operands::Or(_) => Some(Instruction::Or),
            // Math
            Operands::Add(_) => Some(Instruction::Add),
            Operands::Div(_) => Some(Instruction::Div),
            Operands::Dp2(_) => Some(Instruction::Dp2),
            Operands::Dp3(_) => Some(Instruction::Dp3),
            Operands::Dp4(_) => Some(Instruction::Dp4),
            Operands::Exp(_) => Some(Instruction::Exp),
            Operands::Frc(_) => Some(Instruction::Frc),
            Operands::IAdd(_) => Some(Instruction::IAdd),
            Operands::Log(_) => Some(Instruction::Log),
            Operands::Mad(_) => Some(Instruction::Mad),
            Operands::Max(_) => Some(Instruction::Max),
            Operands::Min(_) => Some(Instruction::Min),
            Operands::MovC(_) => Some(Instruction::MovC),
            Operands::Mul(_) => Some(Instruction::Mul),
            Operands::RoundNe(_) => Some(Instruction::RoundNe),
            Operands::RoundNi(_) => Some(Instruction::RoundNi),
            Operands::RoundPi(_) => Some(Instruction::RoundPi),
            Operands::RoundZ(_) => Some(Instruction::RoundZ),
            Operands::Rsq(_) => Some(Instruction::Rsq),
            Operands::SinCos(_) => Some(Instruction::SinCos),
            Operands::Sqrt(_) => Some(Instruction::Sqrt),
            // Memory
            Operands::Mov(_) => Some(Instruction::Mov),
            // Conversions
            Operands::Itof(_) => Some(Instruction::Itof),
            Operands::Utof(_) => Some(Instruction::Utof),
            Operands::Ftou(_) => Some(Instruction::Ftou),
            // Control flow
            Operands::If(_) => Some(Instruction::If),
            Operands::Else => Some(Instruction::Else),
            Operands::EndIf => Some(Instruction::EndIf),
            Operands::Loop => Some(Instruction::Loop),
            Operands::EndLoop => Some(Instruction::EndLoop),
            Operands::Break => Some(Instruction::Break),
            Operands::BreakC(_) => Some(Instruction::BreakC),
            Operands::Ret => None,
            // Textures
            Operands::Sample(_) => Some(Instruction::Sample),
            Operands::SampleL(_) => Some(Instruction::SampleL),
            // All others
            Operands::Unknown(_) => Some(Instruction::Unknown),
        }
    }
}

pub type MacroMap = FastHashMap<Vec<Instruction>, Vec<String>>;

/// [`dxbc`] [`Consumer`] to create chains of instructions from macros compiled as shaders.
pub struct CollectMacrosConsumer {
    macros: Vec<String>,
    current_macro: usize,
    map: MacroMap,
    instructions: Vec<Instruction>,
}

impl CollectMacrosConsumer {
    /// Create new [`CollectMacros`] from list of macros.
    pub fn new(macros: Vec<String>) -> Self {
        Self {
            macros,
            current_macro: 0,
            map: FastHashMap::default(),
            instructions: Vec::new(),
        }
    }
}

impl Consumer for CollectMacrosConsumer {
    fn initialize(&mut self) -> Action {
        self.instructions = Vec::new();

        Action::Continue
    }

    fn consume_instruction(&mut self, _offset: u32, instruction: SparseInstruction) -> Action {
        if let Some(i) = Instruction::try_from(instruction) {
            self.instructions.push(i);
        }

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        self.map
            .entry(self.instructions.clone())
            .or_default()
            .push(self.macros[self.current_macro].clone());
        self.current_macro += 1;

        Action::Continue
    }
}

/// Create a map of instructions to macros.
fn get_instruction_chains() -> MacroMap {
    let macros: Vec<&File> = SHADER_DIR
        .find("macros/*.dxbc")
        .unwrap()
        .filter_map(|entry| match entry {
            DirEntry::File(file) => Some(file),
            _ => None,
        })
        .collect();
    let macro_names = macros
        .iter()
        .map(|file| {
            file.path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    let mut consumer = CollectMacrosConsumer::new(macro_names);

    for file in macros {
        let mut parser = Parser::new(file.contents(), &mut consumer);
        parser.parse().unwrap();
    }

    consumer.map
}

static MACROS: Lazy<MacroMap> = Lazy::new(get_instruction_chains);

/// Collect all instructions from a shader to match macros.
#[derive(Default)]
pub struct MatchMacrosConsumer {
    instructions: Vec<Instruction>,
}

impl MatchMacrosConsumer {
    /// Create new [`MatchMacrosConsumer`].
    pub fn new() -> Self {
        Default::default()
    }
}

impl Consumer for MatchMacrosConsumer {
    fn initialize(&mut self) -> Action {
        self.instructions = Vec::new();

        Action::Continue
    }

    fn consume_instruction(&mut self, _offset: u32, instruction: SparseInstruction) -> Action {
        if let Some(i) = Instruction::try_from(instruction) {
            self.instructions.push(i);
        }

        Action::Continue
    }

    fn finalize(&mut self) -> Action {
        for j in 1..self.instructions.len() + 1 {
            for i in 0..j {
                let instruction_sample = &self.instructions[i..j];
                if let Some(macros) = MACROS.get(instruction_sample) {
                    dbg!(instruction_sample, macros);
                }
            }
        }

        Action::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chains() {
        println!("{:#?}", get_instruction_chains());
    }
}
