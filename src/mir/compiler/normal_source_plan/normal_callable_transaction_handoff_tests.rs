use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};

use super::super::{
    NormalMainDirectCallPreflightV1, NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1,
    SealedNormalSourcePlanV1,
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

fn helper_with_result(name: &str, result: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: vec!["n".to_owned()],
        param_decls: vec![ParamDecl {
            name: "n".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        }],
        return_type_name: Some("i64".to_owned()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(result)),
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

fn helper(name: &str) -> ASTNode {
    helper_with_result(
        name,
        ASTNode::Variable {
            name: "n".to_owned(),
            span: Span::unknown(),
        },
    )
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
        methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
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

fn completed(statements: Vec<ASTNode>) -> CompletedNormalMainHelperResolutionV1 {
    let plan = NormalSourcePlanClassifierV1::seal(PreparedNormalSourcePlanInputV1::new(
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        },
        "normal-callable-transaction-handoff-test",
    ))
    .unwrap();
    let SealedNormalSourcePlanV1::CallableModule(source) = plan else {
        panic!("expected callable module")
    };
    let source = source
        .prepare_callable_source()
        .unwrap()
        .prepare_helper_catalog(41)
        .unwrap()
        .prepare_main_with_helper_catalog()
        .unwrap();
    NormalMainDirectCallPreflightV1::seal(source)
        .unwrap()
        .prepare_helper_resolution()
        .resolve()
        .unwrap()
}

fn schedule_keys(open: OpenNormalCallableModuleTransactionV1) -> Vec<String> {
    let (open, keys) = open
        .with_helper_plans(|source, schedule| {
            assert_eq!(
                source.source_identity(),
                "normal-callable-transaction-handoff-test"
            );
            assert_eq!(source.helper_count(), schedule.topology().helper_count());
            schedule
                .helper_keys()
                .map(|key| key.name().to_owned())
                .collect()
        })
        .unwrap();
    assert!(open.has_main_lowering_proof());
    keys
}

#[test]
fn handoff_consumes_completed_resolution_and_seals_acyclic_schedule() {
    let keys = schedule_keys(
        completed(vec![
            main_box(Some(call("helper", literal(1)))),
            helper("helper"),
        ])
        .into_tx0_handoff(),
    );

    assert_eq!(keys, ["helper"]);
}

#[test]
fn handoff_schedule_uses_canonical_key_order_not_declaration_order() {
    let first = schedule_keys(
        completed(vec![main_box(None), helper("beta"), helper("alpha")]).into_tx0_handoff(),
    );
    let second = schedule_keys(
        completed(vec![helper("alpha"), main_box(None), helper("beta")]).into_tx0_handoff(),
    );

    assert_eq!(first, ["alpha", "beta"]);
    assert_eq!(first, second);
}

#[test]
fn handoff_keeps_recursive_scc_receipt_with_independent_leaf() {
    let open = completed(vec![
        main_box(Some(call("looping", literal(1)))),
        helper_with_result(
            "looping",
            call(
                "looping",
                ASTNode::Variable {
                    name: "n".to_owned(),
                    span: Span::unknown(),
                },
            ),
        ),
        helper("leaf"),
    ])
    .into_tx0_handoff();
    let (open, receipt) = open
        .with_helper_plans(|_, schedule| {
            assert_eq!(schedule.helper_keys().count(), 2);
            match schedule.topology() {
                PreparedNormalHelperTopologyReceiptV1::Recursive(partition) => (
                    partition.components().len(),
                    partition.recursive_component_count(),
                ),
                PreparedNormalHelperTopologyReceiptV1::Acyclic(_) => {
                    panic!("recursive helper must not retry as acyclic")
                }
            }
        })
        .unwrap();
    assert!(open.has_main_lowering_proof());
    assert_eq!(receipt, (2, 1));
}

#[test]
fn schedule_rejection_retains_authority_without_running_callback() {
    let mut callback_ran = false;
    let rejected = completed(vec![
        main_box(None),
        helper_with_result(
            "broken",
            ASTNode::Literal {
                value: LiteralValue::String("not-i64".to_owned()),
                span: Span::unknown(),
            },
        ),
    ])
    .into_tx0_handoff()
    .with_helper_plans(|_, _| callback_ran = true)
    .unwrap_err();

    assert!(!callback_ran);
    assert_eq!(
        rejected.stage(),
        NormalCallableHandoffStageV1::HelperSchedule
    );
    assert!(matches!(
        rejected.error(),
        NormalAcyclicCallableModuleErrorV1::FunctionPreflight { .. }
    ));
    assert_eq!(rejected.source.helper_count(), 1);
    assert_eq!(rejected.main_lowering.owner(), rejected.source.main_owner());
    rejected.discard();
}

#[test]
fn success_rejection_then_success_preserves_one_shot_handoff_boundary() {
    let first = schedule_keys(completed(vec![main_box(None), helper("before")]).into_tx0_handoff());
    assert_eq!(first, ["before"]);

    let rejected = completed(vec![
        main_box(None),
        helper_with_result(
            "broken",
            ASTNode::Literal {
                value: LiteralValue::String("not-i64".to_owned()),
                span: Span::unknown(),
            },
        ),
    ])
    .into_tx0_handoff()
    .with_helper_plans(|_, _| ());
    assert!(rejected.is_err());

    let last = schedule_keys(completed(vec![main_box(None), helper("after")]).into_tx0_handoff());
    assert_eq!(last, ["after"]);
}
