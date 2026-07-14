use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::function_control::{
    verify_function_completion_v1, FunctionCompletionVerificationErrorV1,
    VerifiedFunctionCompletionV1,
};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn return_stmt(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "completion_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn verify(
    body: Vec<ASTNode>,
) -> Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1> {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(body)).unwrap();
    verify_function_completion_v1(unit.root_function_input().unwrap())
}

#[test]
fn explicit_value_return_seals_exact_site_target_and_empty_cleanup() {
    let completion = verify(vec![return_stmt(Some(literal(7)))]).unwrap();
    assert!(completion.returns_value());
    assert!(!completion.is_implicit_void());
    assert_eq!(completion.unreachable_suffix_count(), 0);
    assert!(completion.cleanup().crossed_scopes().is_empty());
    assert_eq!(
        completion.explicit_site().unwrap().node().segments(),
        &[SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(completion.target_function().owner(), completion.owner());
}

#[test]
fn explicit_void_return_is_not_implicit_fallthrough() {
    let completion = verify(vec![return_stmt(None)]).unwrap();
    assert!(!completion.returns_value());
    assert!(!completion.is_implicit_void());
    assert!(completion.explicit_site().is_some());
    assert!(completion.cleanup().crossed_scopes().is_empty());
}

#[test]
fn implicit_void_is_a_separate_exact_completion_form() {
    let completion = verify(vec![literal(1)]).unwrap();
    assert!(!completion.returns_value());
    assert!(completion.is_implicit_void());
    assert!(completion.explicit_site().is_none());
    assert!(completion.cleanup().crossed_scopes().is_empty());
    assert_eq!(completion.target_function().owner(), completion.owner());
    let (body, end) = completion.implicit_body_end().unwrap();
    assert_eq!(body.owner(), completion.owner());
    assert_eq!(end, 1);
}

#[test]
fn nonterminal_root_return_cannot_seal() {
    let error = verify(vec![return_stmt(Some(literal(1))), literal(2)]).unwrap_err();
    assert!(matches!(
        error,
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}

#[test]
fn nested_return_cannot_impersonate_the_root_terminal_site() {
    let nested = ASTNode::If {
        condition: Box::new(literal(1)),
        then_body: vec![return_stmt(Some(literal(1)))],
        else_body: None,
        span: Span::unknown(),
    };
    let error = verify(vec![nested]).unwrap_err();
    assert!(matches!(
        error,
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}
