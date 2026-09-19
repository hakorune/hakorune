use std::collections::{BTreeMap, HashMap};

use super::{
    classify_raw_loop_child_entry_v1, CallableLoopBindingProjectionDispositionV1,
    PreparedLocatedRawLoopChildEntryV1, RawLoopChildEntryDispositionV1,
    VerifiedCallableSemanticLoopBindingScheduleV1,
};
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingReceiptV1, CallableLoopBindingRoleV1, CallableLoopSourceProjectionV1,
};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawUnlocatedPortalV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    FunctionOwnerIssuerV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
};
use hakorune_mir_core::BindingId;

fn span() -> Span {
    Span::unknown()
}

fn literal_bool(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: crate::ast::LiteralValue::Bool(value),
        span: span(),
    }
}

fn box_declaration() -> ASTNode {
    ASTNode::BoxDeclaration {
        name: "Nested".to_string(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: crate::ast::BoxMethodInventoryV1::empty(),
        constructors: HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_record: false,
        extends: Vec::new(),
        implements: Vec::new(),
        type_parameters: Vec::new(),
        is_sync: false,
        is_static: true,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: span(),
    }
}

fn loop_node(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(literal_bool(true)),
        body,
        span: span(),
    }
}

fn located_loop_source() -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(3).node(),
        body_kind: None,
    }
}

fn callable_handoff(
    loop_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
) -> super::VerifiedCallableSemanticLoopBindingScheduleV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let owner = issuer.issue().unwrap();
    let binding = crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0));
    let condition = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let body_read = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let target = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Target)
        .node();
    VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner,
        loop_site.clone(),
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
        std::collections::BTreeSet::new(),
    )
    .unwrap()
}

fn outside_handoff(
    loop_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
) -> CallableLoopBindingProjectionDispositionV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let owner = issuer.issue().unwrap();
    let carrier = crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0));
    let outside = crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(1));
    let condition = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let carrier_read = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let carrier_rebind = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Target)
        .node();
    let outside_read = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopBody(1))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let outside_rebind = SourcePathV1::from_node(loop_site)
        .child(SourcePathSegmentV1::LoopBody(1))
        .child(SourcePathSegmentV1::Target)
        .node();
    let mut variables = BTreeMap::new();
    variables.insert(condition, carrier);
    variables.insert(carrier_read, carrier);
    variables.insert(outside_read, outside);
    let mut assignments = BTreeMap::new();
    assignments.insert(carrier_rebind, carrier);
    assignments.insert(outside_rebind, outside);
    let locals = BTreeMap::new();
    CallableLoopSourceProjectionV1::new(owner, &locals, &variables, &assignments)
        .project_disposition(loop_site.clone())
        .unwrap()
}

#[test]
fn accepts_plain_loop_syntax_without_box_declaration() {
    let body = vec![ASTNode::If {
        condition: Box::new(literal_bool(true)),
        then_body: vec![ASTNode::Print {
            expression: Box::new(literal_bool(false)),
            span: span(),
        }],
        else_body: None,
        span: span(),
    }];

    assert_eq!(
        classify_raw_loop_child_entry_v1(&literal_bool(true), &body),
        RawLoopChildEntryDispositionV1::NoChildFunctionEntry,
    );
}

#[test]
fn rejects_box_declaration_in_loop_body_or_expression() {
    let direct = vec![box_declaration()];
    let nested_expression = vec![ASTNode::FunctionCall {
        name: "consume".to_string(),
        arguments: vec![box_declaration()],
        span: span(),
    }];

    assert_eq!(
        classify_raw_loop_child_entry_v1(&literal_bool(true), &direct),
        RawLoopChildEntryDispositionV1::ReachableBoxDeclaration,
    );
    assert_eq!(
        classify_raw_loop_child_entry_v1(&literal_bool(true), &nested_expression),
        RawLoopChildEntryDispositionV1::ReachableBoxDeclaration,
    );
}

#[test]
fn rejects_box_declaration_in_nested_executable_loop() {
    let body = vec![ASTNode::Loop {
        condition: Box::new(literal_bool(true)),
        body: vec![box_declaration()],
        span: span(),
    }];

    assert_eq!(
        classify_raw_loop_child_entry_v1(&literal_bool(true), &body),
        RawLoopChildEntryDispositionV1::ReachableBoxDeclaration,
    );
}

#[test]
fn ignores_box_declaration_inside_deferred_lambda_body() {
    let body = vec![ASTNode::Lambda {
        params: Vec::new(),
        body: vec![box_declaration()],
        span: span(),
    }];

    assert_eq!(
        classify_raw_loop_child_entry_v1(&literal_bool(true), &body),
        RawLoopChildEntryDispositionV1::NoChildFunctionEntry,
    );
}

#[test]
fn located_entry_co_seals_exact_condition_and_body_root_receipts() {
    let source = located_loop_source();
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &source,
        loop_node(vec![ASTNode::Break { span: span() }]),
        None,
    )
    .expect("located Loop entry");

    assert_eq!(
        prepared.condition_source.site().unwrap().segments(),
        &[
            SourcePathSegmentV1::Body(3),
            SourcePathSegmentV1::LoopCondition,
        ]
    );
    assert!(matches!(
        prepared.body_source,
        RawInvocationSourceContextV1::Located {
            body_kind: Some(SourceBodyKindV1::Loop),
            ref site,
            ..
        } if site.segments() == [
            SourcePathSegmentV1::Body(3),
            SourcePathSegmentV1::LoopBodyRoot,
        ]
    ));
    assert_eq!(
        prepared.disposition,
        RawLoopChildEntryDispositionV1::NoChildFunctionEntry
    );
}

#[test]
fn located_entry_carries_callable_handoff_before_route_effects() {
    let source = located_loop_source();
    let loop_site = source.site().unwrap().clone();
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &source,
        loop_node(vec![ASTNode::Break { span: span() }]),
        Some(CallableLoopBindingProjectionDispositionV1::Ready(
            callable_handoff(&loop_site),
        )),
    )
    .expect("located Loop entry with callable handoff");

    assert!(prepared.callable_handoff.is_some());
}

#[test]
fn body_only_product_rejects_before_builder_effect_when_facts_are_absent() {
    let source = located_loop_source();
    let loop_site = source.site().unwrap().clone();
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &source,
        loop_node(Vec::new()),
        Some(outside_handoff(&loop_site)),
    )
    .expect("located Loop entry with Outside handoff");

    let mut builder = MirBuilder::new();
    let error = prepared
        .lower_v1(
            &mut builder,
            "outside-terminal/0",
            false,
            false,
            GenericLoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true),
        )
        .expect_err("body-only product must reject absent Facts before effects");

    assert!(error.contains("callable-loop/facts-absent"));
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}

#[test]
fn unlocated_entry_is_rejected_before_child_classification() {
    let error = PreparedLocatedRawLoopChildEntryV1::prepare(
        &RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: RawUnlocatedPortalV1::CallObject,
            expected_lineage: None,
        },
        loop_node(vec![box_declaration()]),
        None,
    )
    .err()
    .expect("unlocated Loop must fail");

    assert!(error.contains("requires-located-loop-source"));
}
