use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};

use super::{
    resolve_function_shadow_v0, ShadowBindingKindV0, ShadowControlExitV0, ShadowRegionKindV0,
    ShadowResolveErrorV0, ShadowScopeKindV0,
};

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
    assert_eq!(
        product.variable_uses[&after],
        super::ShadowLexicalRefV0::Local(parameter)
    );
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

#[test]
fn standalone_program_has_exact_lexical_lifetime_and_source_coverage() {
    let container = ASTNode::Program {
        statements: vec![local_x()],
        span: Span::unknown(),
    };
    assert_container_scope(container.clone(), SourcePathSegmentV1::ProgramBody(0));

    let product = resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &function(container))
        .expect("standalone Program block");
    let scope_origin = SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::ProgramBodyRoot,
    ]);
    assert!(product.scopes.values().any(|scope| {
        scope.kind == ShadowScopeKindV0::LexicalBlock
            && scope.origin.as_ref() == Some(&scope_origin)
    }));
    assert!(product.regions.values().any(|region| {
        region.kind == ShadowRegionKindV0::LexicalScope
            && region.origin.as_ref() == Some(&scope_origin)
    }));
    for segments in [
        vec![SourcePathSegmentV1::Body(0)],
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::ProgramBody(0),
        ],
    ] {
        assert!(product
            .statement_sites
            .contains(&SourceStmtSiteV1::from_node(
                SourceNodeSiteV1::from_segments(segments)
            )));
    }
}

#[test]
fn duplicate_local_inside_one_program_block_remains_a_typed_redeclaration() {
    let tree = function(ASTNode::Program {
        statements: vec![local_x(), local_x()],
        span: Span::unknown(),
    });
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::SameScopeRedeclaration { name }) if &*name == "x"
    ));
}

#[test]
fn program_block_inside_loop_keeps_the_enclosing_break_target() {
    let tree = function(ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        body: vec![ASTNode::Program {
            statements: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    });
    let product = resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree)
        .expect("nested Program break");
    let (loop_region, _) = product
        .regions
        .iter()
        .find(|(_, region)| region.kind == ShadowRegionKindV0::Loop)
        .expect("enclosing Loop region");
    let exit_site = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::ProgramBody(0),
    ]));
    assert!(matches!(
        product.resolved_exits[&exit_site].transfer,
        ShadowControlExitV0::Break { target_loop } if target_loop == *loop_region
    ));
}
