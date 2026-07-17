use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    observe_method_calls_shadow_view_v0, FunctionSyntaxViewV1, ShadowMethodCallReceiverV0,
    ShadowQualifiedReceiverDispositionV0, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn method(object: ASTNode, name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: name.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn function(params: &[&str], body: Vec<ASTNode>, is_static: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: params.iter().map(|name| (*name).into()).collect(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

#[test]
fn inventories_bound_unbound_and_current_owner_receivers() {
    let static_function = function(
        &["text"],
        vec![
            method(variable("text"), "length", Vec::new()),
            method(variable("Helpers"), "skip", vec![integer(1)]),
        ],
        true,
    );
    let static_rows = observe_method_calls_shadow_view_v0(
        FunctionSyntaxViewV1::from_ast(&static_function).unwrap(),
    )
    .unwrap();
    assert_eq!(
        static_rows[&site(vec![SourcePathSegmentV1::Body(0)])].receiver(),
        ShadowMethodCallReceiverV0::Qualified(ShadowQualifiedReceiverDispositionV0::Bound)
    );
    assert_eq!(
        static_rows[&site(vec![SourcePathSegmentV1::Body(1)])].receiver(),
        ShadowMethodCallReceiverV0::Qualified(ShadowQualifiedReceiverDispositionV0::ProvenUnbound)
    );

    let instance_function = function(
        &[],
        vec![method(
            ASTNode::Me {
                span: Span::unknown(),
            },
            "run",
            Vec::new(),
        )],
        false,
    );
    let instance_rows = observe_method_calls_shadow_view_v0(
        FunctionSyntaxViewV1::from_ast(&instance_function).unwrap(),
    )
    .unwrap();
    let row = &instance_rows[&site(vec![SourcePathSegmentV1::Body(0)])];
    assert_eq!(row.receiver(), ShadowMethodCallReceiverV0::CurrentOwner);
    assert_eq!(
        row.receiver_site(),
        &site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Receiver,
        ])
    );
}

#[test]
fn nested_and_repeated_calls_keep_distinct_exact_sites() {
    let nested = method(
        variable("Helpers"),
        "outer",
        vec![method(variable("Helpers"), "inner", Vec::new())],
    );
    let tree = function(
        &[],
        vec![nested, method(variable("Helpers"), "outer", Vec::new())],
        true,
    );
    let rows = observe_method_calls_shadow_view_v0(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let expected = [
        site(vec![SourcePathSegmentV1::Body(0)]),
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Argument(0),
        ]),
        site(vec![SourcePathSegmentV1::Body(1)]),
    ];
    assert_eq!(rows.len(), expected.len());
    for call_site in expected {
        assert!(rows.contains_key(&call_site));
    }
}

#[test]
fn non_name_receiver_is_inventory_only_dynamic() {
    let tree = function(&[], vec![method(integer(1), "run", Vec::new())], true);
    let rows = observe_method_calls_shadow_view_v0(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    assert_eq!(
        rows[&site(vec![SourcePathSegmentV1::Body(0)])].receiver(),
        ShadowMethodCallReceiverV0::Dynamic
    );
}
