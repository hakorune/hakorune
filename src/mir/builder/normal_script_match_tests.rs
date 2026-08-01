use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1,
    ScriptRootMatchControlAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn root_match(scrutinee: ASTNode, arms: Vec<(LiteralValue, ASTNode)>, else_expr: ASTNode) -> ASTNode {
    ASTNode::MatchExpr {
        scrutinee: Box::new(scrutinee),
        arms,
        else_expr: Box::new(else_expr),
        span: Span::unknown(),
    }
}

fn root_match_entry() -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::MatchControl(
            ScriptRootMatchControlAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn root_match_seals_all_child_sites_and_matches_legacy() {
    let program = ASTNode::Program {
        statements: vec![root_match(
            integer(1),
            vec![(LiteralValue::Integer(1), integer(2))],
            integer(3),
        )],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![root_match_entry()], 1).expect("Match window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(product) = FunctionSemanticResolverSessionV1::new(0)
        .expect("resolver")
        .resolve_script(view, &window)
        .expect("Match resolve")
    else {
        panic!("root Match must complete");
    };
    let sealed = super::VerifiedScriptSemanticSourceV1::seal(&source, product, &window)
        .expect("Match source seal");
    let receipts = sealed.match_controls().collect::<Vec<_>>();
    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].1, 1);
    assert_eq!(
        receipts[0].0.node().segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(0),
        ]
    );

    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some("script-root-match.hako"))
        .expect("legacy Match");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-root-match.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected Match");
    let printer = MirPrinter::new();
    assert_eq!(printer.print_module(&normal.module), printer.print_module(&legacy.module));
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn root_match_does_not_activate_nested_match() {
    let program = ASTNode::Program {
        statements: vec![root_match(
            root_match(integer(1), Vec::new(), integer(2)),
            Vec::new(),
            integer(3),
        )],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![root_match_entry()], 1).expect("Match window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("nested Match resolution"),
        ResolveScriptOutcomeV1::Deferred
    ));
}
