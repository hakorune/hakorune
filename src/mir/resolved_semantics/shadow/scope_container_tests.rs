use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};

use super::{resolve_function_shadow_v0, ShadowBindingKindV0};

fn local_x() -> ASTNode {
    ASTNode::Local {
        variables: vec!["x".into()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn function(container: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: vec!["x".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            container,
            ASTNode::Return {
                value: Some(Box::new(ASTNode::Variable {
                    name: "x".into(),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn assert_container_scope(container: ASTNode, child_segment: SourcePathSegmentV1) {
    let tree = function(container);
    let product = resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree).unwrap();
    let parameter = product
        .bindings
        .iter()
        .find(|(_, record)| record.kind == ShadowBindingKindV0::Parameter { index: 0 })
        .map(|(binding, _)| *binding)
        .unwrap();
    let inner_site = SourceBindingSiteV1::Local {
        statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            child_segment,
        ])),
        ordinal: 0,
    };
    let inner = product.declarations[&inner_site];
    let after = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Value,
    ]));
    assert_ne!(inner, parameter);
    assert_eq!(product.variable_uses[&after], parameter);
}

#[test]
fn task_scope_is_inline_execution_with_real_lexical_lifetime() {
    assert_container_scope(
        ASTNode::TaskScope {
            body: vec![local_x()],
            source_keyword: "co".into(),
            span: Span::unknown(),
        },
        SourcePathSegmentV1::TaskScopeBody(0),
    );
}

#[test]
fn fastmem_region_is_inline_execution_with_real_lexical_lifetime() {
    assert_container_scope(
        ASTNode::FastMemRegion {
            contract: "FixtureV0".into(),
            body: vec![local_x()],
            span: Span::unknown(),
        },
        SourcePathSegmentV1::FastMemBody(0),
    );
}
