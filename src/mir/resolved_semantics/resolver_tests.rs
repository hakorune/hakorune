use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, RegionKindV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

fn fixture() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: vec!["arg".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: false,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn local_site() -> SourceBindingSiteV1 {
    SourceBindingSiteV1::Local {
        statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ])),
        ordinal: 0,
    }
}

#[test]
fn canonical_resolver_seals_receiver_parameter_and_local() {
    let tree = fixture();
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let product = session.resolve(view).unwrap();

    assert_eq!(product.binding_count(), 3);
    let local = product.declaration_binding(&local_site()).unwrap();
    assert_eq!(product.binding(local).unwrap().diagnostic_name(), "x");
}

#[test]
fn canonical_resolver_normalizes_across_compilation_sessions() {
    let tree = fixture();
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let first = FunctionSemanticResolverSessionV1::new(7)
        .unwrap()
        .resolve(view)
        .unwrap();
    let second = FunctionSemanticResolverSessionV1::new(7)
        .unwrap()
        .resolve(view)
        .unwrap();

    assert_ne!(first.owner(), second.owner());
    assert_eq!(first.normalized_graph(), second.normalized_graph());
}

#[test]
fn canonical_resolver_seals_atomic_top_level_return_record() {
    let tree = ASTNode::FunctionDeclaration {
        name: "returns".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap();
    let comparison = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap();
    let site = ResolvedExitSiteV1::Statement(SourceStmtSiteV1::from_node(
        SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::Body(0)]),
    ));
    let exit = product.resolved_exit(&site).unwrap();

    assert_eq!(exit.origin(), ResolvedExitOriginV1::ExplicitReturn);
    assert_eq!(
        exit.transfer(),
        ResolvedControlTransferV1::Return {
            target_function: product.function_region(),
        }
    );
    assert_eq!(
        product.region(exit.source_region()).unwrap().kind(),
        RegionKindV1::Sequence
    );
    assert_eq!(product.normalized_graph(), comparison.normalized_graph());
}

#[test]
fn canonical_resolver_seals_loop_exit_source_and_target_regions() {
    let tree = ASTNode::FunctionDeclaration {
        name: "loop_exits".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![
                ASTNode::Continue {
                    span: Span::unknown(),
                },
                ASTNode::Break {
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let continue_site = ResolvedExitSiteV1::Statement(SourceStmtSiteV1::from_node(
        SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
        ]),
    ));
    let break_site = ResolvedExitSiteV1::Statement(SourceStmtSiteV1::from_node(
        SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(1),
        ]),
    ));
    let continue_exit = product.resolved_exit(&continue_site).unwrap();
    let break_exit = product.resolved_exit(&break_site).unwrap();

    assert_eq!(continue_exit.source_region(), break_exit.source_region());
    assert_eq!(
        continue_exit.transfer(),
        ResolvedControlTransferV1::Continue {
            target_loop: continue_exit.source_region(),
        }
    );
    assert_eq!(
        break_exit.transfer(),
        ResolvedControlTransferV1::Break {
            target_loop: break_exit.source_region(),
        }
    );
}
