//! MIRBUILDER-EXE-ACCEPTANCE-TARGET-ONLY-EMISSION-S0 pin: a
//! `StaticResultPublicationIngressV1::TargetOnly` row consumed at the
//! member_route `StaticReceiver` arm emits
//! `emit_static_global_target_value_terminal_v1` for the row's exact
//! target — no result-publication commit, no re-derived callee.
use std::collections::HashMap;

use crate::mir::{Callee, MirCompiler, MirInstruction, MirType, NormalCompileRequestV1};
use crate::parser::{NyashParser, ParserBuildConfig};

const TARGET_ONLY_SOURCE: &str = r#"
static box TextOwner {
    caller() {
        local x = TextOwner.text()
        return 0
    }
    text() { return 1.5 }
}
static box Main {
    main() { return 0 }
}
"#;

fn compile(source: &str) -> crate::mir::MirCompileResult {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("fixture parses");
    let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed =
            crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    });
    MirCompiler::with_options(false)
        .compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            Some("target_only.hako"),
            HashMap::new(),
        ))
        .expect("target-only static call lowers through the physical bridge")
}

fn global_calls(function: &crate::mir::MirFunction) -> Vec<(crate::mir::ValueId, String)> {
    function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter_map(|instruction| match instruction {
            MirInstruction::Call(call) => match &call.callee {
                Callee::Global(target) => Some((
                    call.dst.expect("target-only call binds a result"),
                    target.display_name(),
                )),
                _ => None,
            },
            MirInstruction::LegacyCallV0 {
                dst: Some(dst),
                callee: Some(Callee::Global(target)),
                ..
            } => Some((*dst, target.display_name())),
            _ => None,
        })
        .collect()
}

#[test]
fn target_only_static_row_emits_exact_global_call_without_result_publication() {
    let result = compile(TARGET_ONLY_SOURCE);
    let caller = result
        .module
        .functions
        .get("TextOwner.caller/0")
        .expect("caller function is emitted");

    let calls = global_calls(caller);
    assert_eq!(
        calls.len(),
        1,
        "exactly one global call is emitted for the target-only row"
    );
    let (dst, target) = &calls[0];
    assert_eq!(target, "TextOwner.text/0", "the row's exact target is used");

    assert_ne!(
        caller.metadata.value_types.get(dst),
        Some(&MirType::Integer),
        "target-only emission commits no i64 result publication"
    );
}

#[test]
fn target_only_callee_keeps_its_own_function_unchanged() {
    let result = compile(TARGET_ONLY_SOURCE);
    let callee = result
        .module
        .functions
        .get("TextOwner.text/0")
        .expect("callee function is emitted");
    assert!(
        global_calls(callee).is_empty(),
        "the callee body emits no extra global call"
    );
}
