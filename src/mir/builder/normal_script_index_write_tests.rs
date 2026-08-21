use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootIndexWriteAdmissionV1,
    ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1,
    ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use crate::parser::NyashParser;

fn resolved_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn index_write_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IndexWrite(
            ScriptRootIndexWriteAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn resolve_script(
    source: &str,
    entries: Vec<VerifiedScriptRootDemandEntryV1>,
) -> ResolveScriptOutcomeV1 {
    let program = NyashParser::parse_from_string(source).expect("IndexWrite source");
    let statement_count = match &program {
        crate::ast::ASTNode::Program { statements, .. } => statements.len(),
        _ => unreachable!("parser returns Program"),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(entries, statement_count)
        .expect("IndexWrite demand window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    FunctionSemanticResolverSessionV1::new(0)
        .expect("resolver")
        .resolve_script(view, &window)
        .expect("IndexWrite resolve")
}

#[test]
fn prior_local_array_index_write_is_complete_and_matches_legacy() {
    let source = "local xs = [1]\nxs[0] = 2";
    assert!(matches!(
        resolve_script(source, vec![resolved_entry(0), index_write_entry(1)]),
        ResolveScriptOutcomeV1::Complete(_)
    ));

    let program = NyashParser::parse_from_string(source).expect("IndexWrite source");
    let legacy = MirCompiler::with_options(false)
        .compile_with_source(program.clone(), Some("script-index-write.hako"))
        .expect("legacy compile");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-index-write.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("normal compile");
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn non_array_or_rebound_index_write_stays_deferred() {
    assert!(matches!(
        resolve_script(
            "local xs = 1\nxs[0] = 2",
            vec![resolved_entry(0), index_write_entry(1)]
        ),
        ResolveScriptOutcomeV1::Deferred(_)
    ));
    assert!(matches!(
        resolve_script(
            "local xs = [1]\nxs = 2\nxs[0] = 3",
            vec![resolved_entry(0), resolved_entry(1), index_write_entry(2)],
        ),
        ResolveScriptOutcomeV1::Deferred(_)
    ));
}
