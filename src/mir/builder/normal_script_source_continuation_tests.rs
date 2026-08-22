use super::normal_script_source_continuation::{
    ScriptSourceContinuationIssueV1, VerifiedScriptSourceContinuationV1,
};
use crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptForestOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};

fn return_entry() -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(
            ScriptRootReturnExitAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn fixture() -> (ASTNode, VerifiedScriptRootDemandWindowV1) {
    let program = ASTNode::Program {
        statements: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![return_entry()], 1).unwrap();
    (program, window)
}

#[test]
fn resolver_retains_one_call_to_final_return_continuation() {
    let (program, window) = fixture();
    let catalog = issue_brand_program_declaration_catalog_v1(&program).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(990).unwrap();
    let outcome = resolver
        .resolve_script_forest_with_declaration_views(
            ScriptSyntaxViewV1::from_program(&program).unwrap(),
            &window,
            &(),
            &(),
            &(),
            &catalog,
        )
        .unwrap();
    let ResolveScriptForestOutcomeV1::Complete(forest) = outcome else {
        panic!("fixture must be a Complete Script")
    };
    let continuation = VerifiedScriptSourceContinuationV1::issue(&forest, &window).unwrap();
    assert_eq!(continuation.rows().count(), 0);
    assert_eq!(continuation.owner(), forest.roots()[0]);
}

#[test]
fn continuation_rejects_a_non_return_window_for_return_shape() {
    let (program, valid_window) = fixture();
    let sequence_window = VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(991).unwrap();
    let catalog = issue_brand_program_declaration_catalog_v1(&program).unwrap();
    let outcome = resolver
        .resolve_script_forest_with_declaration_views(
            ScriptSyntaxViewV1::from_program(&program).unwrap(),
            &valid_window,
            &(),
            &(),
            &(),
            &catalog,
        )
        .unwrap();
    let ResolveScriptForestOutcomeV1::Complete(forest) = outcome else {
        panic!("fixture must be a Complete Script")
    };
    assert!(matches!(
        VerifiedScriptSourceContinuationV1::issue(&forest, &sequence_window),
        Err(ScriptSourceContinuationIssueV1::ReturnAdmissionMismatch(_))
    ));
}
