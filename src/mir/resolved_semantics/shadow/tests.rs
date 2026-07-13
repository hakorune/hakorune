use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_semantics::source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};

use super::{
    resolve_function_shadow_v0, ShadowAssignmentTargetV0, ShadowBindingKindV0, ShadowControlExitV0,
    ShadowRegionKindV0, ShadowResolveErrorV0,
};

fn span() -> Span {
    Span::unknown()
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: span(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn local(name: &str, initial: Option<ASTNode>) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![initial.map(Box::new)],
        declared_type_names: vec![None],
        span: span(),
    }
}

fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: span(),
    }
}

fn function(params: &[&str], body: Vec<ASTNode>) -> ASTNode {
    function_with_static(params, body, false)
}

fn function_with_static(params: &[&str], body: Vec<ASTNode>, is_static: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".to_owned(),
        params: params.iter().map(|name| (*name).to_owned()).collect(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: span(),
    }
}

fn expr_site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn stmt_site(segments: Vec<SourcePathSegmentV1>) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn resolve(function: &ASTNode) -> super::ShadowResolvedFunctionV0 {
    resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), function).expect("shadow resolution")
}

#[test]
fn initializer_observes_outer_binding_before_shadow_is_inserted() {
    let tree = function(
        &[],
        vec![
            local("x", Some(int(1))),
            ASTNode::ScopeBox {
                body: vec![local("x", Some(var("x")))],
                span: span(),
            },
        ],
    );
    let product = resolve(&tree);
    let outer = product.declarations[&crate::mir::resolved_semantics::SourceBindingSiteV1::Local {
        statement: stmt_site(vec![SourcePathSegmentV1::Body(0)]),
        ordinal: 0,
    }];
    let use_site = expr_site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::ScopeBody(0),
        SourcePathSegmentV1::Initializer(0),
    ]);
    assert_eq!(product.variable_uses[&use_site], outer);
    assert_eq!(product.bindings.len(), 3, "receiver plus two x bindings");
}

#[test]
fn nowait_expression_resolves_before_its_binding_is_declared() {
    let tree = function(
        &["x"],
        vec![ASTNode::Nowait {
            variable: "x".into(),
            expression: Box::new(var("x")),
            span: span(),
        }],
    );
    let product = resolve(&tree);
    let declaration = SourceBindingSiteV1::Nowait {
        statement: stmt_site(vec![SourcePathSegmentV1::Body(0)]),
    };
    let nowait = product.declarations[&declaration];
    let parameter = product
        .bindings
        .iter()
        .find(|(_, record)| record.kind == ShadowBindingKindV0::Parameter { index: 0 })
        .map(|(binding, _)| *binding)
        .unwrap();
    let initializer = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]);
    assert_eq!(product.bindings[&nowait].kind, ShadowBindingKindV0::Nowait);
    assert_eq!(product.variable_uses[&initializer], parameter);
    assert_ne!(nowait, parameter);
}

#[test]
fn same_scope_redeclaration_is_rejected() {
    let tree = function(&[], vec![local("x", None), local("x", None)]);
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::SameScopeRedeclaration { name }) if &*name == "x"
    ));
}

#[test]
fn parameter_receiver_and_shadowed_local_have_distinct_ordinals() {
    let tree = function(
        &["arg"],
        vec![ASTNode::ScopeBox {
            body: vec![local("arg", Some(ASTNode::Me { span: span() }))],
            span: span(),
        }],
    );
    let product = resolve(&tree);
    assert_eq!(product.bindings.len(), 3);
    let mut kinds = product.bindings.values().map(|record| record.kind);
    assert_eq!(kinds.next(), Some(ShadowBindingKindV0::Receiver));
    assert_eq!(
        kinds.next(),
        Some(ShadowBindingKindV0::Parameter { index: 0 })
    );
    assert_eq!(
        kinds.next(),
        Some(ShadowBindingKindV0::Local { ordinal: 0 })
    );
}

#[test]
fn variable_field_and_index_assignment_kinds_are_distinct() {
    let tree = function(
        &["obj", "arr", "i"],
        vec![
            local("x", Some(int(0))),
            assign(var("x"), int(1)),
            assign(
                ASTNode::FieldAccess {
                    object: Box::new(var("obj")),
                    field: "field".to_owned(),
                    span: span(),
                },
                int(2),
            ),
            assign(
                ASTNode::Index {
                    target: Box::new(var("arr")),
                    index: Box::new(var("i")),
                    span: span(),
                },
                int(3),
            ),
        ],
    );
    let product = resolve(&tree);
    let variable = expr_site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Target,
    ]);
    let field = expr_site(vec![
        SourcePathSegmentV1::Body(2),
        SourcePathSegmentV1::Target,
    ]);
    let index = expr_site(vec![
        SourcePathSegmentV1::Body(3),
        SourcePathSegmentV1::Target,
    ]);
    assert!(matches!(
        product.assignment_targets[&variable],
        ShadowAssignmentTargetV0::BindingRebind(_)
    ));
    assert!(matches!(
        product.assignment_targets[&field],
        ShadowAssignmentTargetV0::FieldWrite { .. }
    ));
    assert!(matches!(
        product.assignment_targets[&index],
        ShadowAssignmentTargetV0::IndexWrite { .. }
    ));
}

#[test]
fn nested_loop_exits_resolve_to_the_nearest_exact_region() {
    let tree = function(
        &[],
        vec![ASTNode::Loop {
            condition: Box::new(int(1)),
            body: vec![ASTNode::Loop {
                condition: Box::new(int(1)),
                body: vec![
                    ASTNode::Continue { span: span() },
                    ASTNode::Break { span: span() },
                ],
                span: span(),
            }],
            span: span(),
        }],
    );
    let product = resolve(&tree);
    let continue_site = stmt_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(0),
    ]);
    let break_site = stmt_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(1),
    ]);
    let ShadowControlExitV0::Continue { target_loop } = product.control_exits[&continue_site]
    else {
        panic!("continue target")
    };
    assert_eq!(
        product.control_exits[&break_site],
        ShadowControlExitV0::Break { target_loop }
    );
    assert_ne!(target_loop, product.function_region);
}

#[test]
fn return_targets_function_and_resolves_value_first() {
    let tree = function(
        &["value"],
        vec![ASTNode::Return {
            value: Some(Box::new(var("value"))),
            span: span(),
        }],
    );
    let product = resolve(&tree);
    let site = stmt_site(vec![SourcePathSegmentV1::Body(0)]);
    assert_eq!(
        product.control_exits[&site],
        ShadowControlExitV0::Return {
            target_function: product.function_region
        }
    );
    assert!(product.variable_uses.contains_key(&expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value
    ])));
}

#[test]
fn break_outside_loop_rejects_without_fallback() {
    let tree = function(&[], vec![ASTNode::Break { span: span() }]);
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::ExitOutsideLoop { kind: "Break", .. })
    ));
}

#[test]
fn unsupported_statement_rejects_without_partial_publication() {
    let tree = function(
        &[],
        vec![ASTNode::GlobalVar {
            name: "global".into(),
            value: Box::new(int(1)),
            span: span(),
        }],
    );
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::UnsupportedStatement {
            kind: "GlobalVar",
            ..
        })
    ));
}

#[test]
fn body_local_may_shadow_parameter_but_not_a_body_sibling() {
    let tree = function(&["x"], vec![local("x", Some(int(1))), local("x", None)]);
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::SameScopeRedeclaration { name }) if &*name == "x"
    ));

    let accepted = function(&["x"], vec![local("x", Some(var("x")))]);
    let product = resolve(&accepted);
    let parameter = product
        .bindings
        .iter()
        .find(|(_, record)| record.kind == ShadowBindingKindV0::Parameter { index: 0 })
        .map(|(binding, _)| *binding)
        .expect("parameter");
    let initializer = expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ]);
    assert_eq!(product.variable_uses[&initializer], parameter);
}

#[test]
fn all_initializers_precede_all_bindings_in_multi_local() {
    let tree = function(
        &[],
        vec![ASTNode::Local {
            variables: vec!["a".to_owned(), "b".to_owned()],
            initial_values: vec![Some(Box::new(int(1))), Some(Box::new(var("a")))],
            declared_type_names: vec![None, None],
            span: span(),
        }],
    );
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::UnresolvedName { name, .. }) if &*name == "a"
    ));
}

#[test]
fn scope_local_does_not_leak_but_outer_rebind_resolves() {
    let leaking = function(
        &[],
        vec![
            ASTNode::ScopeBox {
                body: vec![local("inner", Some(int(1)))],
                span: span(),
            },
            assign(var("inner"), int(2)),
        ],
    );
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &leaking),
        Err(ShadowResolveErrorV0::UnresolvedName { name, .. }) if &*name == "inner"
    ));

    let outer = function(
        &[],
        vec![
            local("outer", Some(int(0))),
            ASTNode::ScopeBox {
                body: vec![assign(var("outer"), int(1))],
                span: span(),
            },
        ],
    );
    let product = resolve(&outer);
    let target = expr_site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::ScopeBody(0),
        SourcePathSegmentV1::Target,
    ]);
    assert!(matches!(
        product.assignment_targets[&target],
        ShadowAssignmentTargetV0::BindingRebind(_)
    ));
}

#[test]
fn if_branches_have_independent_lexical_scopes() {
    let tree = function(
        &["flag"],
        vec![ASTNode::If {
            condition: Box::new(var("flag")),
            then_body: vec![local("x", Some(int(1)))],
            else_body: Some(vec![local("x", Some(int(2)))]),
            span: span(),
        }],
    );
    let product = resolve(&tree);
    let x_bindings = product
        .bindings
        .values()
        .filter(|record| &*record.diagnostic_name == "x")
        .collect::<Vec<_>>();
    assert_eq!(x_bindings.len(), 2);
    assert_ne!(x_bindings[0].owner_scope, x_bindings[1].owner_scope);
}

#[test]
fn outbox_has_its_own_binding_kind_and_ignores_compat_initializer() {
    let tree = function(
        &["seed"],
        vec![ASTNode::Outbox {
            variables: vec!["result".to_owned()],
            initial_values: vec![Some(Box::new(var("seed")))],
            span: span(),
        }],
    );
    let product = resolve(&tree);
    assert!(product
        .bindings
        .values()
        .any(|record| record.kind == ShadowBindingKindV0::Outbox { ordinal: 0 }));
    assert!(!product.variable_uses.contains_key(&expr_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0)
    ])));
}

#[test]
fn missing_local_metadata_and_initializer_entries_are_valid() {
    let tree = function(
        &[],
        vec![ASTNode::Local {
            variables: vec!["x".to_owned()],
            initial_values: Vec::new(),
            declared_type_names: vec![None],
            span: span(),
        }],
    );
    let product = resolve(&tree);
    assert!(product
        .bindings
        .values()
        .any(|record| &*record.diagnostic_name == "x"));
}

#[test]
fn static_function_has_no_receiver_binding() {
    let tree = function_with_static(
        &[],
        vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Me { span: span() })),
            span: span(),
        }],
        true,
    );
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::UnsupportedExpression { kind: "Me", .. })
    ));
}

#[test]
fn unsupported_expression_inside_return_rejects_at_exact_site() {
    let tree = function(
        &[],
        vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::This { span: span() })),
            span: span(),
        }],
    );
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::UnsupportedExpression { kind: "This", .. })
    ));
}

#[test]
fn duplicate_parameters_and_receiver_collision_reject() {
    let duplicate = function(&["arg", "arg"], Vec::new());
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &duplicate),
        Err(ShadowResolveErrorV0::SameScopeRedeclaration { name }) if &*name == "arg"
    ));

    let receiver_collision = function(&["me"], Vec::new());
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &receiver_collision),
        Err(ShadowResolveErrorV0::SameScopeRedeclaration { name }) if &*name == "me"
    ));
}

#[test]
fn outer_exit_after_nested_loop_targets_outer_region() {
    let tree = function(
        &[],
        vec![ASTNode::Loop {
            condition: Box::new(int(1)),
            body: vec![
                ASTNode::Loop {
                    condition: Box::new(int(1)),
                    body: vec![ASTNode::Break { span: span() }],
                    span: span(),
                },
                ASTNode::Continue { span: span() },
            ],
            span: span(),
        }],
    );
    let product = resolve(&tree);
    let inner_site = stmt_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(0),
    ]);
    let outer_site = stmt_site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(1),
    ]);
    let ShadowControlExitV0::Break { target_loop: inner } = product.control_exits[&inner_site]
    else {
        panic!("inner break")
    };
    let ShadowControlExitV0::Continue { target_loop: outer } = product.control_exits[&outer_site]
    else {
        panic!("outer continue")
    };
    assert_ne!(inner, outer);
    assert_eq!(product.regions[&inner].kind, ShadowRegionKindV0::Loop);
    assert_eq!(product.regions[&outer].kind, ShadowRegionKindV0::Loop);
    assert_eq!(product.regions[&inner].parent, Some(outer));
}

#[test]
fn assignment_to_undefined_binding_rejects_before_rhs() {
    let tree = function(&[], vec![assign(var("missing"), var("also_missing"))]);
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::UnresolvedName { name, site })
            if &*name == "missing"
                && site == expr_site(vec![SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Target])
    ));
}

#[test]
fn unsupported_assignment_target_rejects_without_shape_rewrite() {
    let tree = function(
        &[],
        vec![assign(
            ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: span(),
            },
            int(1),
        )],
    );
    assert!(matches!(
        resolve_function_shadow_v0(FunctionOriginV1::new(0, 0), &tree),
        Err(ShadowResolveErrorV0::UnsupportedAssignmentTarget { .. })
    ));
}

#[test]
fn region_origins_distinguish_function_and_branch_body_roles() {
    let tree = function(
        &["flag"],
        vec![ASTNode::If {
            condition: Box::new(var("flag")),
            then_body: Vec::new(),
            else_body: Some(Vec::new()),
            span: span(),
        }],
    );
    let product = resolve(&tree);
    let origin_for = |kind| {
        product
            .regions
            .values()
            .find(|record| record.kind == kind)
            .and_then(|record| record.origin.as_ref())
            .expect("region origin")
            .segments()
            .to_vec()
    };
    assert_eq!(
        origin_for(ShadowRegionKindV0::Sequence),
        vec![SourcePathSegmentV1::FunctionBody]
    );
    assert_eq!(
        origin_for(ShadowRegionKindV0::If),
        vec![SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(
        origin_for(ShadowRegionKindV0::IfThen),
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThenBody
        ]
    );
    assert_eq!(
        origin_for(ShadowRegionKindV0::IfElse),
        vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfElseBody
        ]
    );
}

#[test]
fn accepted_vocabulary_is_closed_and_reviewable() {
    assert_eq!(
        super::vocabulary::SHADOW_ACCEPTED_STATEMENTS_V0,
        [
            "Local",
            "Outbox",
            "Nowait",
            "Assignment",
            "CompoundAssignment",
            "ScopeBox",
            "If",
            "Loop",
            "Break",
            "Continue",
            "Return",
            "Print",
            "ClosedExpressionStatement",
        ]
    );
    assert_eq!(
        super::vocabulary::SHADOW_ACCEPTED_EXPRESSIONS_V0,
        [
            "Literal",
            "Variable",
            "Me",
            "UnaryOp",
            "BinaryOp",
            "MethodCall",
            "FieldAccess",
            "Index",
            "FunctionCall",
            "New",
            "AwaitExpression",
            "ArrayLiteral",
            "MapLiteral",
            "RecordLiteral",
            "RecordUpdate",
            "CheckExpr",
            "FromCall",
            "Call",
            "GroupedAssignmentExpr",
        ]
    );
    assert_eq!(
        super::vocabulary::SHADOW_ACCEPTED_ASSIGNMENT_TARGETS_V0,
        ["Variable", "FieldAccess", "Index"]
    );
}
