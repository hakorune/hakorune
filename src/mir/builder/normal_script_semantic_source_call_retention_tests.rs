//! FunctionCall remains an R4-retained Script operation boundary.

use super::PreparedNormalDefaultProgramRootV1;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};

#[test]
fn script_function_call_remains_r4_retained() {
    let source = PreparedNormalDefaultProgramRootV1::seal(ASTNode::Program {
        statements: vec![ASTNode::FunctionCall {
            name: "helper".to_owned(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    })
    .expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .expect("complete root window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");

    assert!(matches!(
        resolver
            .resolve_script(view, &window)
            .expect("FunctionCall profile gate"),
        ResolveScriptOutcomeV1::Deferred
    ));
}
