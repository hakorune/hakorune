use crate::ast::{ASTNode, LiteralValue, Span};

use super::vocabulary::{classify_shadow_ast_disposition_v0, ShadowAstDispositionV0};

fn span() -> Span {
    Span::unknown()
}

#[test]
fn classifies_representative_current_statement_and_expression_variants() {
    let statement = ASTNode::Break { span: span() };
    let expression = ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: span(),
    };

    assert_eq!(
        classify_shadow_ast_disposition_v0(&statement),
        ShadowAstDispositionV0::CurrentResolvedStatement
    );
    assert_eq!(
        classify_shadow_ast_disposition_v0(&expression),
        ShadowAstDispositionV0::CurrentResolvedExpression
    );
}

#[test]
fn compile_time_only_directive_is_only_a_transparent_candidate() {
    let directive = ASTNode::UsingStatement {
        namespace_name: "fixture".to_owned(),
        span: span(),
    };

    assert_eq!(
        classify_shadow_ast_disposition_v0(&directive),
        ShadowAstDispositionV0::SemanticallyTransparentCandidate
    );
}

#[test]
fn this_is_explicitly_unsupported_inventory_pending_resolver_correction() {
    let this = ASTNode::This { span: span() };

    assert_eq!(
        classify_shadow_ast_disposition_v0(&this),
        ShadowAstDispositionV0::ExplicitUnsupported
    );
}

#[test]
fn unconnected_program_container_is_only_a_transparent_candidate() {
    let program = ASTNode::Program {
        statements: Vec::new(),
        span: span(),
    };

    assert_eq!(
        classify_shadow_ast_disposition_v0(&program),
        ShadowAstDispositionV0::SemanticallyTransparentCandidate
    );
}

#[test]
fn qmark_and_throw_remain_explicitly_unsupported() {
    let qmark = ASTNode::QMarkPropagate {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: span(),
        }),
        span: span(),
    };
    let throw = ASTNode::Throw {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: span(),
        }),
        span: span(),
    };

    assert_eq!(
        classify_shadow_ast_disposition_v0(&qmark),
        ShadowAstDispositionV0::ExplicitUnsupported
    );
    assert_eq!(
        classify_shadow_ast_disposition_v0(&throw),
        ShadowAstDispositionV0::ExplicitUnsupported
    );
}
