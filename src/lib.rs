mod expressions;
mod instructions;
mod io;
mod macros;
mod utils;

pub use macros::MatchMacrosConsumer;

use std::mem::take;

use dxbc::binary::{Action, Consumer, Parser, State};
use dxbc::dr::*;
use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};
use naga::*;

use crate::utils::*;

pub(crate) struct NagaConsumer {
    /// Module populated in [`finalize`].
    pub module: Module,
    /// Entry point function.
    pub function: Function,
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
            program_ty: ProgramType::Vertex,
            temps: vec![],
            outs: vec![],
        }
    }

    /*fn get_swizzle(&mut self, op: &OperandToken0, span: &Span) -> Handle<Expression> {
        match op.get_component_select_mode() {
            ComponentSelectMode::Mask => todo!(),
            ComponentSelectMode::Swizzle => todo!(),
            ComponentSelectMode::Select1 => {
                let name = get_swizzle_component(&op.get_component_swizzle().0);
                [name, name, name, name]
            }
        }
    }*/
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

        for cb in &rdef.constant_buffers {
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
                    *span += var.size;
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
            self.module.global_variables.append(global, Span::UNDEFINED);
        }

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
            Operands::Add(_) => None,
            Operands::And(_) => None,
            Operands::Mul(_) => None,
            Operands::Mad(_) => None,
            Operands::Mov(mov) => Some(self.handle_mov(span, &mov)),
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
            Operands::Ret => Some(self.handle_ret(span)),
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
    let info = validator.validate(&consumer.module);
    // TODO: better error handling
    let info = info.unwrap();

    Ok((consumer.module, info))
}
