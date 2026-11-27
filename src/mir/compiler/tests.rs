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

fn verify_callable_source(source: &str, path: &str) {
    use crate::mir::NormalCompileRequestV1;
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_with_identity_v1,
        NormalCallableMaterializationOutcomeV1,
    };

    crate::runtime::ring0::ensure_global_ring0_initialized();
    let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
        materialize_normal_callable_program_with_identity_v1(
            source,
            crate::parser::ParserBuildConfig::default(),
            path,
        )
        .expect("materialize callable source")
    else {
        panic!("fixture must use the source-backed entry");
    };
    MirCompiler::with_options(false)
        .compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            Some(path),
            Default::default(),
        ))
        .expect("compile callable source")
        .into_verified_module()
        .expect("verify callable source");
}
