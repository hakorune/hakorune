use std::collections::BTreeSet;

use super::{CallableGenericLoopSourceFactsIssuerV1, CallableLoopStructuralLeaseIssuerV1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingProjectionDispositionV1, CallableLoopBindingReceiptV1,
    CallableLoopBindingRoleV1, VerifiedCallableSemanticLoopBindingScheduleV1,
};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
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
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }))],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(variable("i")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
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

fn policy() -> GenericLoopFactsPolicyFrameV1 {
    GenericLoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true)
}

fn with_receipt<R>(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    use_receipt: impl for<'source> FnOnce(super::CallableGenericLoopSourceFactsReceiptV1<'source>) -> R,
) -> R {
    let parent = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(0).node(),
        body_kind: Some(SourceBodyKindV1::Function),
    };
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &parent,
        generic_loop(),
        Some(CallableLoopBindingProjectionDispositionV1::Ready(schedule(
            owner,
        ))),
    )
    .expect("located prepared Loop");
    let payload = prepared
        .into_callable_generic_loop_source_facts_payload(
            owner,
            "structural-lease",
            false,
            false,
            policy(),
        )
        .expect("source payload");
    let super::CallableGenericLoopSourceFactsDispositionV1::Ready(ready) =
        CallableGenericLoopSourceFactsIssuerV1::issue_once(payload)
    else {
        panic!("expected source-aware GenericLoop Ready outcome")
    };
    use_receipt(ready.claim_all().expect("claim receipt"))
}

#[test]
fn source_bound_lease_keeps_receipt_and_port_in_one_callback() {
    let source_owner = owner();
    let observed = with_receipt(source_owner, |claimed| {
        let handoff = CallableLoopStructuralLeaseIssuerV1::prepare(claimed)
            .expect("source-bound structural lease");
        handoff.with_view(|view| {
            assert_eq!(view.owner(), source_owner);
            assert_eq!(view.pre_effect().owner(), source_owner);
            assert!(view.outcome().facts.is_some());
            assert_eq!(
                view.selection().raw_execution_routes(),
                [crate::mir::loop_recipe_contract::route_id::LoopRouteId::GenericLoopV1]
            );
            assert_eq!(view.structural_port().owner(), source_owner);
            assert_eq!(view.structural_port().loop_site(), view.loop_site());
            (view.owner(), view.loop_site().clone())
        })
    });
    assert_eq!(observed.0, source_owner);
    assert_eq!(observed.1, SourcePathV1::root_body(0).node());
}

#[test]
fn source_bound_lease_rejects_foreign_root_before_callback() {
    let source_owner = owner();
    with_receipt(source_owner, |mut claimed| {
        claimed.condition_source = RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::NestedBoxMethod {
                parent_site: SourcePathV1::root_body(9).node(),
                method_key: "foreign".into(),
            },
            site: SourcePathV1::root_body(0)
                .child(SourcePathSegmentV1::LoopCondition)
                .node(),
            body_kind: None,
        };
        assert!(matches!(
            CallableLoopStructuralLeaseIssuerV1::prepare(claimed),
            Err(super::CallableLoopStructuralLeaseRejectV1::ForeignRootLineage)
        ));
    });
}

#[test]
fn source_bound_lease_rejects_pre_effect_owner_mismatch() {
    let source_owner = owner();
    let foreign_owner = owner();
    with_receipt(source_owner, |mut claimed| {
        claimed.owner = foreign_owner;
        assert!(matches!(
            CallableLoopStructuralLeaseIssuerV1::prepare(claimed),
            Err(super::CallableLoopStructuralLeaseRejectV1::PreEffectOwnerMismatch)
        ));
    });
}
