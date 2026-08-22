use std::collections::BTreeSet;

use super::{CallableGenericLoopSourceFactsDispositionV1, CallableGenericLoopSourceFactsIssuerV1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingProjectionDispositionV1, CallableLoopBindingReceiptV1,
    CallableLoopBindingRoleV1, VerifiedCallableSemanticLoopBindingScheduleV1,
};
use crate::mir::builder::normal_callable_loop_physical_adapter::CallableGenericLoopV1PhysicalAdapterV1;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    FunctionOwnerIssuerV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
};
use hakorune_mir_core::BindingId;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn generic_loop() -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(variable("limit")),
            span: Span::unknown(),
        }),
        body: vec![
            ASTNode::Local {
                variables: vec!["tmp".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(variable("i")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    }
}

fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    issuer.issue().expect("owner")
}

fn schedule(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> VerifiedCallableSemanticLoopBindingScheduleV1 {
    let loop_site = SourcePathV1::root_body(0).node();
    let binding = crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0));
    let condition = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let body_read = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(1))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let target = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(1))
        .child(SourcePathSegmentV1::Target)
        .node();
    VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner,
        loop_site,
        vec![
            CallableLoopBindingReceiptV1::new(
                condition,
                binding,
                CallableLoopBindingRoleV1::ConditionRead,
            ),
            CallableLoopBindingReceiptV1::new(
                body_read,
                binding,
                CallableLoopBindingRoleV1::BodyRead,
            ),
            CallableLoopBindingReceiptV1::new(
                target,
                binding,
                CallableLoopBindingRoleV1::BodyRebind,
            ),
        ],
        BTreeSet::new(),
    )
    .expect("complete carrier schedule")
}

fn with_prepared<R>(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    node: ASTNode,
    f: impl for<'source> FnOnce(
        &'source RawInvocationSourceContextV1,
        PreparedLocatedRawLoopChildEntryV1<'source>,
    ) -> R,
) -> R {
    let parent = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(0).node(),
        body_kind: Some(SourceBodyKindV1::Function),
    };
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &parent,
        node,
        Some(CallableLoopBindingProjectionDispositionV1::Ready(schedule(
            owner,
        ))),
    )
    .expect("located prepared Loop");
    // The callback may borrow the exact parent root while consuming the
    // prepared entry; no test fixture needs a leaked source allocation.
    f(&parent, prepared)
}

fn policy() -> GenericLoopFactsPolicyFrameV1 {
    GenericLoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true)
}

#[test]
fn issuer_co_seals_one_generic_facts_outcome() {
    let source_owner = owner();
    with_prepared(source_owner, generic_loop(), |_, prepared| {
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload(
                source_owner,
                "caller-zero",
                false,
                false,
                policy(),
            )
            .expect("source payload");

        let outcome = CallableGenericLoopSourceFactsIssuerV1::issue_once(payload);
        let CallableGenericLoopSourceFactsDispositionV1::Ready(ready) = outcome else {
            panic!("expected source-aware GenericLoop Ready outcome")
        };

        assert_eq!(ready.owner(), source_owner);
        assert_eq!(ready.policy(), policy());
        assert_eq!(
            ready.selection().raw_execution_routes(),
            [crate::mir::loop_recipe_contract::route_id::LoopRouteId::GenericLoopV1]
        );
        assert!(ready.outcome().facts.is_some());
    });
}

#[test]
fn claim_all_retains_pre_effect_receipt_without_physical_effect() {
    let source_owner = owner();
    with_prepared(source_owner, generic_loop(), |_, prepared| {
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload(
                source_owner,
                "claim-all",
                false,
                false,
                policy(),
            )
            .expect("source payload");
        let outcome = CallableGenericLoopSourceFactsIssuerV1::issue_once(payload);
        let super::CallableGenericLoopSourceFactsDispositionV1::Ready(source_facts) = outcome
        else {
            panic!("expected source-aware GenericLoop Ready outcome")
        };

        let receipt = source_facts.claim_all().expect("claim receipt");
        assert_eq!(receipt.owner(), source_owner);
        assert_eq!(receipt.policy(), policy());
        assert_eq!(receipt.pre_effect().owner(), source_owner);
        assert!(!receipt.pre_effect().rows().is_empty());
    });
}

#[test]
fn claimed_facts_move_into_one_higher_ranked_semantic_recipe_view() {
    let source_owner = owner();
    with_prepared(source_owner, generic_loop(), |_, prepared| {
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload(
                source_owner,
                "semantic-recipe",
                true,
                true,
                policy(),
            )
            .expect("source payload");
        let super::CallableGenericLoopSourceFactsDispositionV1::Ready(source_facts) =
            CallableGenericLoopSourceFactsIssuerV1::issue_once(payload)
        else {
            panic!("expected source-aware GenericLoop Ready outcome")
        };

        let recipe = source_facts
            .claim_all()
            .expect("claim receipt")
            .into_semantic_recipe()
            .expect("semantic recipe");
        let observed = recipe
            .with_view(|view| {
                assert_eq!(view.owner(), source_owner);
                assert!(view.facts().facts.generic_loop_v1().is_some());
                assert_eq!(view.debug(), true);
                assert_eq!(view.in_static_box(), true);
                assert!(!view.pre_effect().rows().is_empty());
                true
            })
            .expect("semantic view");
        assert!(observed);
    });
}

#[test]
fn semantic_recipe_reaches_the_single_named_physical_adapter() {
    let source_owner = owner();
    with_prepared(source_owner, generic_loop(), |_, prepared| {
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload(
                source_owner,
                "semantic-physical",
                false,
                false,
                policy(),
            )
            .expect("source payload");
        let super::CallableGenericLoopSourceFactsDispositionV1::Ready(source_facts) =
            CallableGenericLoopSourceFactsIssuerV1::issue_once(payload)
        else {
            panic!("expected source-aware GenericLoop Ready outcome")
        };
        let recipe = source_facts
            .claim_all()
            .expect("claim receipt")
            .into_semantic_recipe()
            .expect("semantic recipe");

        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("semantic-physical/0".to_owned());
        let i = builder.alloc_value_for_test();
        builder.bind_variable_for_test("i", i);
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(i, crate::mir::MirType::Integer);
        let limit = builder.alloc_value_for_test();
        builder.bind_variable_for_test("limit", limit);
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(limit, crate::mir::MirType::Integer);

        let value = CallableGenericLoopV1PhysicalAdapterV1::lower_for_test(&mut builder, recipe)
            .expect("source Recipe must reach the named physical adapter");
        assert!(builder
            .function_state
            .type_ctx
            .value_types
            .contains_key(&value));
    });
}

#[test]
fn issuer_rejects_foreign_owner_before_facts() {
    let schedule_owner = owner();
    let foreign_owner = owner();
    with_prepared(schedule_owner, generic_loop(), |_, prepared| {
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload(
                foreign_owner,
                "foreign-owner",
                false,
                false,
                policy(),
            )
            .expect("source payload");

        assert!(matches!(
            CallableGenericLoopSourceFactsIssuerV1::issue_once(payload),
            CallableGenericLoopSourceFactsDispositionV1::SourceUnavailable(
                super::CallableGenericLoopSourceFactsSourceErrorV1::OwnerMismatch
            )
        ));
    });
}

#[test]
fn non_generic_loop_is_facts_absent_or_typed_rejected_without_ready() {
    let source_owner = owner();
    let node = ASTNode::Loop {
        condition: Box::new(boolean(true)),
        body: vec![],
        span: Span::unknown(),
    };
    with_prepared(source_owner, node, |_, prepared| {
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload(
                source_owner,
                "facts-absent",
                false,
                false,
                policy(),
            )
            .expect("source payload");

        assert!(!matches!(
            CallableGenericLoopSourceFactsIssuerV1::issue_once(payload),
            CallableGenericLoopSourceFactsDispositionV1::Ready(_)
        ));
    });
}

#[test]
fn ready_rejection_stops_before_builder_effect_and_never_uses_legacy_route() {
    let source_owner = owner();
    with_prepared(
        source_owner,
        ASTNode::Loop {
            condition: Box::new(boolean(true)),
            body: vec![],
            span: Span::unknown(),
        },
        |_, prepared| {
            let mut builder = MirBuilder::new();
            let result = prepared.lower_v1(&mut builder, "ready-reject", false, false, policy());

            let error = result.expect_err("non-Generic Ready must reject");
            assert!(error.contains("callable-loop/facts-absent"));
            assert!(builder.function_state.current_function.is_none());
        },
    );
}

#[test]
fn ready_source_facts_requires_the_unpublished_root_scope_before_physical_lowering() {
    let source_owner = owner();
    with_prepared(source_owner, generic_loop(), |_, prepared| {
        let mut builder = MirBuilder::new();
        let error = prepared
            .lower_v1(&mut builder, "missing-root-scope", false, false, policy())
            .expect_err("Ready physical lowering must require the root scope");

        assert!(error.contains("callable-loop/root-scope/missing"));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    });
}
