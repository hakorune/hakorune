use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1,
    ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use crate::parser::NyashParser;

fn assert_selected_parity(source: &str, hint: &str) {
    let program = NyashParser::parse_from_string(source).expect("assignment source");
    let legacy = MirCompiler::with_options(false)
        .compile_with_source(program.clone(), Some(hint))
        .expect("legacy assignment");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some(hint),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected assignment");
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module),
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

fn resolved_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn rebind_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(
            ScriptRootBindingRebindAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn variable_target_rebind_is_part_of_the_complete_script_owner() {
    let program = NyashParser::parse_from_string("local x = 1\nx = 2\nprint(x)")
        .expect("binding-rebind source");
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![resolved_entry(0), rebind_entry(1), resolved_entry(2)],
        3,
    )
    .expect("binding-rebind demand window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("binding-rebind resolve"),
        ResolveScriptOutcomeV1::Complete(_)
    ));
}

#[test]
fn prior_local_variable_assignment_rebinds_the_script_ledger() {
    assert_selected_parity(
        "local x = 1\nx = 2\nprint(x)",
        "script-binding-rebind-assignment.hako",
    );
}

#[test]
fn prior_local_variable_compound_assignment_rebinds_the_script_ledger() {
    assert_selected_parity(
        "local x = 1\nx += 2\nprint(x)",
        "script-binding-rebind-compound.hako",
    );
}

#[test]
fn failed_rebind_request_discards_its_ledger_before_fresh_reuse() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("local x = 1\nx = missing").expect("failing source"),
                Some("script-binding-rebind-failure.hako"),
                std::collections::HashMap::new(),
            )
            .expect("failing request"),
        )
        .expect_err("missing RHS must retain existing RootLower diagnostic");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("local x = 1\nx = 2\nprint(x)")
                    .expect("fresh source"),
                Some("script-binding-rebind-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh request"),
        )
        .expect("fresh request must not reuse a failed ledger");
}
