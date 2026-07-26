use std::collections::HashMap;

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};

use super::super::{
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalSourcePlanV1,
};
use super::*;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_owned(),
        arguments: vec![argument],
        span: Span::unknown(),
    }
}

fn helper(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: vec!["n".to_owned()],
        param_decls: vec![ParamDecl {
            name: "n".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        }],
        return_type_name: Some("i64".to_owned()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: "n".to_owned(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_box(result: Option<ASTNode>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        ASTNode::FunctionDeclaration {
            name: "main".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: result
                .map(|value| {
                    vec![ASTNode::Return {
                        value: Some(Box::new(value)),
                        span: Span::unknown(),
                    }]
                })
                .unwrap_or_default(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        },
    );
    ASTNode::BoxDeclaration {
        name: "Main".to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods,
        constructors: HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_sync: false,
        is_record: false,
        type_parameters: Vec::new(),
        extends: Vec::new(),
        implements: Vec::new(),
        is_static: true,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn source(statements: Vec<ASTNode>) -> super::super::VerifiedNormalMainDirectCallSourceUnitV1 {
    let plan = NormalSourcePlanClassifierV1::seal(PreparedNormalSourcePlanInputV1::new(
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        },
        "normal-main-direct-call-plan-test",
    ))
    .unwrap();
    let SealedNormalSourcePlanV1::CallableModule(source) = plan else {
        panic!("expected CallableModule")
    };
    source
        .prepare_callable_source()
        .unwrap()
        .prepare_helper_catalog(31)
        .unwrap()
        .prepare_main_with_helper_catalog()
        .unwrap()
}

#[test]
fn finite_main_plan_seals_exact_helper_call() {
    let plan = NormalMainDirectCallPreflightV1::seal(source(vec![
        main_box(Some(call("helper", literal(1)))),
        helper("helper"),
    ]))
    .unwrap();

    assert_eq!(plan.source_identity(), "normal-main-direct-call-plan-test");
    assert_eq!(plan.direct_call_count(), 1);
    assert_eq!(
        plan.direct_calls()[0].target().symbol().as_mir_name(),
        "helper/1"
    );
    assert!(plan.completion().returns_value());
}

#[test]
fn multiple_nested_main_calls_preserve_child_before_parent_rows() {
    let nested = call("left", call("right", literal(1)));
    let result = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(nested),
        right: Box::new(call("right", literal(2))),
        span: Span::unknown(),
    };
    let plan = NormalMainDirectCallPreflightV1::seal(source(vec![
        main_box(Some(result)),
        helper("left"),
        helper("right"),
    ]))
    .unwrap();
    let symbols = plan
        .direct_calls()
        .iter()
        .map(|row| row.target().symbol().as_mir_name())
        .collect::<Vec<_>>();

    assert_eq!(symbols, ["right/1", "left/1", "right/1"]);
}

#[test]
fn helper_declaration_order_does_not_change_main_call_meaning() {
    for statements in [
        vec![main_box(Some(call("helper", literal(1)))), helper("helper")],
        vec![helper("helper"), main_box(Some(call("helper", literal(1))))],
    ] {
        let plan = NormalMainDirectCallPreflightV1::seal(source(statements)).unwrap();
        assert_eq!(plan.direct_call_count(), 1);
        assert_eq!(
            plan.direct_calls()[0].target().symbol().as_mir_name(),
            "helper/1"
        );
    }
}

#[test]
fn call_free_main_uses_the_same_combined_plan_without_dummy_calls() {
    let plan =
        NormalMainDirectCallPreflightV1::seal(source(vec![main_box(None), helper("helper")]))
            .unwrap();

    assert_eq!(plan.direct_call_count(), 0);
}

#[test]
fn one_call_free_helper_forms_a_zero_edge_normal_dag() {
    let main = NormalMainDirectCallPreflightV1::seal(source(vec![
        main_box(Some(call("helper", literal(1)))),
        helper("helper"),
    ]))
    .unwrap();
    let completed = main.prepare_helper_resolution().resolve().unwrap();
    let plan = completed.prepare_acyclic_plan().unwrap();

    assert_eq!(plan.helper_count(), 1);
    assert_eq!(plan.helper_edge_count(), 0);
    assert_eq!(plan.main_direct_call_count(), 1);
}

#[test]
fn independent_helpers_keep_one_zero_edge_graph() {
    let main = NormalMainDirectCallPreflightV1::seal(source(vec![
        main_box(None),
        helper("left"),
        helper("right"),
    ]))
    .unwrap();
    let completed = main.prepare_helper_resolution().resolve().unwrap();
    let plan = completed.prepare_acyclic_plan().unwrap();

    assert_eq!(plan.helper_count(), 2);
    assert_eq!(plan.helper_edge_count(), 0);
    assert_eq!(plan.main_direct_call_count(), 0);
}
