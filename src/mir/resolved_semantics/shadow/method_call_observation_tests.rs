use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_semantics::{
    observe_method_calls_shadow_view_v0, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ShadowMethodCallReceiverV0, ShadowQualifiedReceiverDispositionV0, ShadowResolveErrorV0,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
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

fn return_value(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn me() -> ASTNode {
    ASTNode::Me {
        span: Span::unknown(),
    }
}

#[test]
fn static_current_owner_observes_only_method_call_receiver_me() {
    let body = vec![method(me(), "target", vec![integer(1)])];
    let rows =
        observe_method_calls_shadow_view_v0(FunctionSyntaxViewV1::from_borrowed_function_parts(
            &[],
            &body,
            ReceiverPolicyV1::StaticCurrentOwner,
        ))
        .unwrap();
    let row = &rows[&site(vec![SourcePathSegmentV1::Body(0)])];
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
fn static_current_owner_keeps_bare_argument_and_field_me_rejected() {
    let cases = [
        (
            return_value(me()),
            site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
            ]),
        ),
        (
            return_value(ASTNode::FunctionCall {
                name: "consume".into(),
                arguments: vec![me()],
                span: Span::unknown(),
            }),
            site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Argument(0),
            ]),
        ),
        (
            return_value(ASTNode::FieldAccess {
                object: Box::new(me()),
                field: "value".into(),
                span: Span::unknown(),
            }),
            site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Receiver,
            ]),
        ),
    ];

    for (expression, expected_site) in cases {
        let body = vec![expression];
        assert!(matches!(
            observe_method_calls_shadow_view_v0(
                FunctionSyntaxViewV1::from_borrowed_function_parts(
                    &[],
                    &body,
                    ReceiverPolicyV1::StaticCurrentOwner,
                ),
            ),
            Err(ShadowResolveErrorV0::UnsupportedExpression { kind: "Me", site })
                if site == expected_site
        ));
    }
}

#[test]
fn ordinary_static_function_and_lambda_do_not_gain_current_owner() {
    let call = return_value(method(me(), "target", Vec::new()));
    let static_function = function(&[], vec![call.clone()], true);
    assert!(matches!(
        observe_method_calls_shadow_view_v0(
            FunctionSyntaxViewV1::from_ast(&static_function).unwrap(),
        ),
        Err(ShadowResolveErrorV0::UnsupportedExpression {
            kind: "Me",
            site: error_site,
        })
            if error_site == site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Receiver,
            ])
    ));

    let lambda = ASTNode::Lambda {
        params: Vec::new(),
        body: vec![call],
        span: Span::unknown(),
    };
    assert!(matches!(
        observe_method_calls_shadow_view_v0(
            FunctionSyntaxViewV1::from_lambda_ast(&lambda).unwrap(),
        ),
        Err(ShadowResolveErrorV0::UnsupportedExpression {
            kind: "Me",
            site: error_site,
        })
            if error_site == site(vec![
                SourcePathSegmentV1::LambdaBody(0),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Receiver,
            ])
    ));
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

    let instance_function = function(&[], vec![method(me(), "run", Vec::new())], false);
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
