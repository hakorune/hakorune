use std::collections::{BTreeMap, HashMap};

use super::{
    classify_raw_loop_child_entry_v1, CallableLoopBindingProjectionDispositionV1,
    PreparedLocatedRawLoopChildEntryV1, RawLoopChildEntryDispositionV1,
};
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingReceiptV1, CallableLoopBindingRoleV1, CallableLoopSourceProjectionV1,
    VerifiedCallableSemanticLoopBindingScheduleV1,
};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawUnlocatedPortalV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    FunctionOwnerIssuerV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::source_call_target::issue_source_bound_core_method_calls_v1;
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
) -> VerifiedCallableSemanticLoopBindingScheduleV1 {
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

/// Build one resolved LoopCond fixture plus its armed callable ledger: the
/// loop carries a resolver-cataloged `starts_with` call under the loop site,
/// so the source bridge arms the forest projection and catalogs the item row
/// exactly like the production `parse/2` shape.
struct ArmedLoopCondEdge {
    ledger: std::rc::Rc<
        std::cell::RefCell<
            crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
        >,
    >,
    loop_site: crate::mir::resolved_semantics::SourceNodeSiteV1,
    call_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    loop_node: ASTNode,
    loop_index: usize,
}

fn armed_loop_cond_edge() -> ArmedLoopCondEdge {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let program = crate::parser::NyashParser::parse_from_string(
        r#"
static function caller(flag: i64, text: String): i64 {
    loop(flag < text.length()) {
        local piece = text.substring(0, 1)
        if text.starts_with("a", 0) == 1 {
            flag = flag + 1
        } else {
            break
        }
    }
    return flag
}
"#,
    )
    .expect("loop-cond fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("fixture is a program")
    };
    let function = statements
        .iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("loop-cond function");
    let ASTNode::FunctionDeclaration { body, .. } = function else {
        panic!("fixture is a function")
    };
    let (loop_index, loop_node) = body
        .iter()
        .enumerate()
        .find(|(_, node)| matches!(node, ASTNode::Loop { .. }))
        .map(|(index, node)| (index, node.clone()))
        .expect("loop statement");

    let unit =
        crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
            .expect("loop-cond fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let loop_site = input
        .function()
        .loop_sites()
        .find(|site| site.node().segments().len() == 1)
        .expect("root loop site")
        .node()
        .clone();
    let source_ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("loop-cond source ledger");
    let core_method_rows = issue_source_bound_core_method_calls_v1(&source_ledger)
        .expect("loop-cond core method rows");
    assert_eq!(
        core_method_rows.len(),
        2,
        "fixture must issue length and substring rows"
    );
    let state = crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods(
        input,
        None,
        crate::mir::normal_callable_semantic_package::unconditional_test_rows(core_method_rows),
    )
    .expect("loop-cond callable state");
    let ledger = std::rc::Rc::new(std::cell::RefCell::new(state));
    let items = ledger
        .borrow()
        .source_loop_items(&loop_site)
        .expect("armed loop must catalog resolver method rows");
    let call_site = items
        .iter()
        .find(|item| item.selector() == "starts_with")
        .expect("loop-cond fixture must contain starts_with publication call")
        .call_site()
        .clone();
    ArmedLoopCondEdge {
        ledger,
        loop_site,
        call_site,
        loop_node,
        loop_index,
    }
}

/// Mint the exact `ExactI64`/`[1]` target relation the module port installs
/// for the selected static publication row.
fn armed_source_target(
    call_site: crate::mir::resolved_semantics::SourceExprSiteV1,
) -> crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetRelationV1 {
    let caller_key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserProgramBox",
        "parse",
        2,
    );
    let target_key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserStringUtilsBox",
        "starts_with",
        3,
    );
    let handoff =
        crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(
            7,
            crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
                caller_key,
                call_site.clone(),
                target_key.clone(),
            ),
            &[1],
        );
    let requirement =
        crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetRequirementV1::from_handoff(
            &handoff,
        );
    crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetRelationV1::new(
        call_site,
        target_key,
        Some(requirement),
    )
}

fn armed_loop_cond_builder(
    ledger: &std::cell::RefCell<
        crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
    >,
) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("caller/2".to_owned());
    for (name, ty) in [
        ("flag", crate::mir::MirType::Integer),
        ("text", crate::mir::MirType::String),
    ] {
        let parameter = builder.alloc_typed(ty);
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("test function")
            .params
            .push(parameter);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_owned(), parameter);
    }
    let entry = crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1::static_function(&builder, 2)
        .expect("static entry values");
    ledger
        .borrow_mut()
        .install_entry_values(&entry)
        .expect("entry install");
    builder
}

/// Drive the armed LoopCond production edge end to end: the located source
/// handoff, issued source relations, and route token must produce a
/// `LoopCondReady` facts product, and the named physical consumer must lower
/// it to a `CorePlan::Loop` plus a real `ValueId`. The `source_target`
/// relation is minted through the same `from_handoff` shape the module port
/// installs for the selected static publication; the residual caller-side
/// publication installation stays a later bounded slice.
#[test]
fn armed_loop_cond_edge_lowers_through_the_source_port() {
    let edge = armed_loop_cond_edge();
    let disposition = edge
        .ledger
        .borrow()
        .loop_binding_source_projection()
        .project_disposition(edge.loop_site.clone())
        .expect("loop binding disposition");
    let (_, root) = RawInvocationSourceContextV1::from_transport(
        crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceTransportV1::root(
            (),
            RawInvocationRootLineageV1::ScriptRoot,
        ),
    );
    let (_, parent_source) = RawInvocationSourceContextV1::from_transport(
        root.body_statement(edge.loop_node.clone(), edge.loop_index),
    );
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &parent_source,
        edge.loop_node,
        Some(disposition),
    )
    .expect("located loop-cond entry");
    let mut builder = armed_loop_cond_builder(&edge.ledger);
    let mut scope =
        crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1::for_test(
        );
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_STRICT_PLANNER_MODE, || {
        prepared
            .lower_v1_with_root_scope_and_callable_ledger(
                &mut builder,
                "caller",
                false,
                false,
                GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true),
                &mut scope,
                &edge.ledger,
                crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetProbeV1::from_parts(
                    vec![armed_source_target(edge.call_site)].into_boxed_slice(),
                    Box::new([]),
                    false,
                ),
            )
            .expect("armed loop-cond edge must lower through the source port");
    });
}

/// The same armed edge must still stop at the named `SourceTargetUnselected`
/// boundary when the site carries an exact same-module static target but its
/// selected publication row is absent (dropped, target-only, or consumed).
#[test]
fn armed_loop_cond_edge_rejects_missing_source_target() {
    let edge = armed_loop_cond_edge();
    let disposition = edge
        .ledger
        .borrow()
        .loop_binding_source_projection()
        .project_disposition(edge.loop_site.clone())
        .expect("loop binding disposition");
    let (_, root) = RawInvocationSourceContextV1::from_transport(
        crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceTransportV1::root(
            (),
            RawInvocationRootLineageV1::ScriptRoot,
        ),
    );
    let (_, parent_source) = RawInvocationSourceContextV1::from_transport(
        root.body_statement(edge.loop_node.clone(), edge.loop_index),
    );
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &parent_source,
        edge.loop_node,
        Some(disposition),
    )
    .expect("located loop-cond entry");
    let mut builder = armed_loop_cond_builder(&edge.ledger);
    let mut scope =
        crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1::for_test(
        );
    let error = crate::test_support::with_env_vars(
        &crate::test_support::JOINIR_STRICT_PLANNER_MODE,
        || {
            prepared.lower_v1_with_root_scope_and_callable_ledger(
                &mut builder,
                "caller",
                false,
                false,
                GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true),
                &mut scope,
                &edge.ledger,
                crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetProbeV1::from_parts(
                    Box::new([]),
                    vec![edge.call_site.clone()].into_boxed_slice(),
                    false,
                ),
            )
            .expect_err("missing source target must stay a named terminal")
        },
    );
    assert!(
        error.contains("LoopCondRouteRejected(SourceTargetUnselected"),
        "unexpected terminal: {error}"
    );
}

/// An armed edge whose items carry no same-module static publication
/// obligation at all stops at the distinct `SourceCallOutsideSelectedFamily`
/// terminal instead of disguising itself as missing evidence.
#[test]
fn armed_loop_cond_edge_rejects_items_outside_the_selected_family() {
    let edge = armed_loop_cond_edge();
    let disposition = edge
        .ledger
        .borrow()
        .loop_binding_source_projection()
        .project_disposition(edge.loop_site.clone())
        .expect("loop binding disposition");
    let (_, root) = RawInvocationSourceContextV1::from_transport(
        crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceTransportV1::root(
            (),
            RawInvocationRootLineageV1::ScriptRoot,
        ),
    );
    let (_, parent_source) = RawInvocationSourceContextV1::from_transport(
        root.body_statement(edge.loop_node.clone(), edge.loop_index),
    );
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &parent_source,
        edge.loop_node,
        Some(disposition),
    )
    .expect("located loop-cond entry");
    let mut builder = armed_loop_cond_builder(&edge.ledger);
    let mut scope =
        crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1::for_test(
        );
    let error = crate::test_support::with_env_vars(
        &crate::test_support::JOINIR_STRICT_PLANNER_MODE,
        || {
            prepared.lower_v1_with_root_scope_and_callable_ledger(
                &mut builder,
                "caller",
                false,
                false,
                GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true),
                &mut scope,
                &edge.ledger,
                crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetProbeV1::empty(),
            )
            .expect_err("no selected obligation must stay a named terminal")
        },
    );
    assert!(
        error.contains("LoopCondRouteRejected(SourceCallOutsideSelectedFamily"),
        "unexpected terminal: {error}"
    );
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
