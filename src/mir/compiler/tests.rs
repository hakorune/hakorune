use super::{
    finish_schedule_for_normal_module,
    LegacyRcInsertionScheduleV1, MirCompiler, MirFinishScheduleV1,
};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::exact_numeric_value_facts::{ExactNumericReturnFact, ExactNumericValueFactSource};
use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::string_corridor_placement::StringCorridorCandidateKind;
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
    MirPrinter, MirType,
};
use crate::parser::NyashParser;

#[path = "tests/await_lowering.rs"]
mod await_lowering;
#[path = "tests/basic_lowering.rs"]
mod basic_lowering;
#[path = "tests/finish_schedule.rs"]
mod finish_schedule;
#[path = "tests/numeric_contracts.rs"]
mod numeric_contracts;
#[path = "tests/string_corridor.rs"]
mod string_corridor;
