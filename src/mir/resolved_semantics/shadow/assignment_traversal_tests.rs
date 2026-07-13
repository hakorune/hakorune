use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, Span};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

use super::resolve_function_shadow_v0;

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: vec!["x".into()],
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

fn variable() -> ASTNode {
    ASTNode::Variable {
        name: "x".into(),
        span: Span::unknown(),
    }
}

fn site(role: SourcePathSegmentV1) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        role,
    ]))
}

#[test]
fn compound_variable_target_is_both_read_and_write_at_one_site() {
    let tree = function(vec![ASTNode::CompoundAssignment {
        target: Box::new(variable()),
        operator: BinaryOperator::Add,
        value: Box::new(variable()),
        span: Span::unknown(),
    }]);
    let product = resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree).unwrap();
    let target = site(SourcePathSegmentV1::Target);
    let value = site(SourcePathSegmentV1::Value);
    assert!(product.variable_uses.contains_key(&target));
    assert!(product.assignment_targets.contains_key(&target));
    assert!(product.variable_uses.contains_key(&value));
}

#[test]
fn grouped_assignment_keeps_rhs_and_string_target_distinct() {
    let tree = function(vec![ASTNode::GroupedAssignmentExpr {
        lhs: "x".into(),
        rhs: Box::new(variable()),
        span: Span::unknown(),
    }]);
    let product = resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree).unwrap();
    let target = site(SourcePathSegmentV1::Target);
    let value = site(SourcePathSegmentV1::Value);
    assert!(product.assignment_targets.contains_key(&target));
    assert!(!product.variable_uses.contains_key(&target));
    assert!(product.variable_uses.contains_key(&value));
}
