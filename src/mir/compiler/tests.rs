use super::{
    finish_schedule_for_normal_module, module_session::CanonicalModuleLoweringSessionV1,
    require_canonical_verification, CanonicalFinishScheduleV1, CanonicalLoweringErrorV1,
    LegacyRcInsertionScheduleV1, MirCompiler, MirFinishScheduleV1,
};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::exact_numeric_value_facts::{ExactNumericReturnFact, ExactNumericValueFactSource};
use crate::mir::function::ExactNumericRuntimeCheckContractKind;
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
#[path = "tests/exception_control.rs"]
mod exception_control;
#[path = "tests/finish_schedule.rs"]
mod finish_schedule;
#[path = "tests/method_id.rs"]
mod method_id;
#[path = "tests/numeric_contracts.rs"]
mod numeric_contracts;
#[path = "tests/string_corridor.rs"]
mod string_corridor;
