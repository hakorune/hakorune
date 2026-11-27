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

fn callable_source_request(source: &str, path: &str) -> crate::mir::NormalCompileRequestV1 {
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
    NormalCompileRequestV1::for_mir_mode_callable_source(source, Some(path), Default::default())
}

fn verify_callable_source(source: &str, path: &str) {
    MirCompiler::with_options(false)
        .compile_normal(callable_source_request(source, path))
        .expect("compile callable source")
        .into_verified_module()
        .expect("verify callable source");
}

fn run_callable_source_exe(
    source: &str,
    fixture: &str,
    executable: &std::path::Path,
) -> std::process::Output {
    let runtime = std::env::var("NYASH_EMIT_EXE_NYRT").expect("native runtime archive");
    let path = executable.to_str().expect("UTF-8 executable path");
    let request = callable_source_request(source, fixture).for_closed_app_executable();
    let result = MirCompiler::with_options(true)
        .compile_normal_with_published(request, |view, verification| {
            verification.as_ref().expect("verified MIR");
            assert!(crate::host_providers::llvm_codegen::emit_published_view_exe(
                view, path, Some(&runtime), None,
            )?);
            Ok(())
        })
        .unwrap_or_else(|error| panic!("{fixture}: {error}"));
    assert!(matches!(result, crate::mir::NormalPublishedCompileOutcome::Consumed(())));
    std::process::Command::new(executable)
        .env("NYASH_NYRT_SILENT_RESULT", "1")
        .output()
        .expect("run executable")
}
