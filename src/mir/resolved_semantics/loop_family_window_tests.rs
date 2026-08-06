use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use std::sync::Arc;

use super::source_site::{SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1};
use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, LoopFamilyWindowLeaseIssueV1,
    SemanticOwnerSourceKindV1, VerifiedResolvedFunctionV1,
};

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "window_fixture".into(),
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

fn loop_stmt() -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        body: Vec::new(),
        span: Span::unknown(),
    }
}

fn root_site(index: u32) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(index),
    ]))
}

fn resolved(body: Vec<ASTNode>) -> Arc<VerifiedResolvedFunctionV1> {
    FunctionSemanticResolverSessionV1::new(0)
        .expect("resolver session")
        .resolve(FunctionSyntaxViewV1::from_ast(&function(body)).expect("function view"))
        .expect("resolved function")
}

#[test]
fn resolver_issues_one_identity_lease_from_exact_loop_site() {
    let product = resolved(vec![loop_stmt()]);
    let lease = product
        .issue_loop_family_window_lease_v1(&root_site(0))
        .expect("window lease");

    assert_eq!(lease.owner(), product.owner());
    assert_eq!(lease.function_origin(), product.function_origin());
    assert_eq!(
        lease.source_kind(),
        SemanticOwnerSourceKindV1::DeclaredFunction
    );
    assert_eq!(lease.site(), &root_site(0));
    assert!(lease.frame().matches(&lease.frame()));
}

#[test]
fn missing_loop_site_does_not_publish_a_lease() {
    let product = resolved(vec![loop_stmt()]);
    assert!(matches!(
        product.issue_loop_family_window_lease_v1(&root_site(1)),
        Err(LoopFamilyWindowLeaseIssueV1::Source(_))
    ));
}

#[test]
fn distinct_resolver_products_keep_distinct_owner_brands() {
    let first = resolved(vec![loop_stmt()]);
    let second = resolved(vec![loop_stmt()]);
    let first_lease = first
        .issue_loop_family_window_lease_v1(&root_site(0))
        .expect("first lease");
    let second_lease = second
        .issue_loop_family_window_lease_v1(&root_site(0))
        .expect("second lease");

    assert_ne!(first_lease.owner(), second_lease.owner());
}
