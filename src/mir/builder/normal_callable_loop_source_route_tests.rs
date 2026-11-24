use super::{
    CallableLoopRouteMatchV1, CallableLoopSourceItemBindingV1, CallableLoopSourceItemDispositionV1,
    CallableLoopSourceRouteRejectV1, CallableLoopSourceRouteTokenV1,
    CallableLoopSourceTargetProbeV1, CallableLoopSourceTargetRelationV1,
    CallableLoopSourceTargetRequirementV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::single_planner::{
    self, CallableLoopFactsPlannerInputV1,
};
use crate::mir::builder::control_flow::plan::{LoopFactsPolicyFrameV1, PlanBuildOutcome};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    VerifiedCallableResultRepresentationV1, VerifiedStaticCallResultPublicationDemandV1,
    VerifiedStaticCallResultPublicationHandoffV1,
};
use crate::mir::compiler::loop_cond_break_continue_projection::issue_loop_cond_break_continue_source_forest_projection_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1,
};

fn route_fixture() -> ASTNode {
    let variable = |name: &str| ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    };
    let integer = |value: i64| ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    };
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(variable("flag")),
        right: Box::new(integer(2)),
        span: Span::unknown(),
    };
    let branch_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(variable("flag")),
        right: Box::new(integer(1)),
        span: Span::unknown(),
    };
    let increment = ASTNode::Assignment {
        target: Box::new(variable("flag")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable("flag")),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "loop_cond_route_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["flag".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(condition),
                body: vec![ASTNode::If {
                    condition: Box::new(branch_condition),
                    then_body: vec![increment],
                    else_body: Some(vec![ASTNode::Break {
                        span: Span::unknown(),
                    }]),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

#[test]
fn source_loop_item_inventory_rejects_a_loop_without_resolver_method_rows() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(route_fixture())
        .expect("loop-cond fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");
    let error = CallableLoopSourceRouteTokenV1::source_items_for_loop(
        &ledger,
        input.owner(),
        root.site().node(),
    )
    .expect_err("item inventory must be explicit when no method row exists");
    assert_eq!(error, CallableLoopSourceRouteRejectV1::SourceItemsMissing);
}

fn requirement_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ]))
}

fn requirement_target() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_static_box_method("StringHelpers", "to_i64", 1)
}

#[test]
fn source_target_requirement_copies_selected_handoff_evidence() {
    let caller =
        CanonicalSameModuleCallableKeyV1::test_static_box_method("StringHelpers", "int_to_str", 1);
    let site = requirement_site();
    let target = requirement_target();
    let handoff = VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(
        7,
        VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
            caller,
            site.clone(),
            target.clone(),
        ),
        &[1],
    );
    let requirement = CallableLoopSourceTargetRequirementV1::from_handoff(&handoff);
    assert_eq!(
        requirement.representation(),
        &VerifiedCallableResultRepresentationV1::ExactI64
    );
    assert_eq!(requirement.required_callee_i64_arguments(), &[1]);
}

#[test]
fn source_target_relation_accepts_exact_i64_without_ordinal_policy() {
    let site = requirement_site();
    let target = requirement_target();
    let exact = |ordinals: &[u32]| CallableLoopSourceTargetRequirementV1 {
        representation: VerifiedCallableResultRepresentationV1::ExactI64,
        required_callee_i64_arguments: ordinals.to_vec().into_boxed_slice(),
    };

    let selected =
        CallableLoopSourceTargetRelationV1::new(site.clone(), target.clone(), Some(exact(&[1])));
    assert!(selected.has_exact_i64_result());

    let wrong_ordinals =
        CallableLoopSourceTargetRelationV1::new(site.clone(), target.clone(), Some(exact(&[0, 2])));
    assert!(wrong_ordinals.has_exact_i64_result());

    let wrong_representation = CallableLoopSourceTargetRelationV1::new(
        site.clone(),
        target.clone(),
        Some(CallableLoopSourceTargetRequirementV1 {
            representation: VerifiedCallableResultRepresentationV1::ExactNominalBox { nullable: false,
                box_name: "Text".into(),
            },
            required_callee_i64_arguments: vec![1].into_boxed_slice(),
        }),
    );
    assert!(!wrong_representation.has_exact_i64_result());

    let missing_evidence = CallableLoopSourceTargetRelationV1::new(site, target, None);
    assert!(!missing_evidence.has_exact_i64_result());
}

/// Everything `issue_with_source_relations` needs from one resolved armed
/// loop: the planner outcome, the exclusive route selection, the issued
/// forest projection, and the real resolver-bound item rows.
pub(super) struct ArmedLoopCondParts {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) parent_site: SourceNodeSiteV1,
    pub(super) function_origin: FunctionOriginV1,
    pub(super) source_kind: SemanticOwnerSourceKindV1,
    pub(super) outcome: PlanBuildOutcome,
    pub(super) selection: CallableLoopRouteMatchV1,
    pub(super) projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    pub(super) items: Box<[CallableLoopSourceItemBindingV1]>,
    pub(super) call_sites: Box<[SourceExprSiteV1]>,
}

pub(super) fn armed_loop_cond_parts(source: &str) -> ArmedLoopCondParts {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let program =
        crate::parser::NyashParser::parse_from_string(source).expect("loop-cond fixture parses");
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
    let loop_index = body
        .iter()
        .enumerate()
        .find(|(_, node)| matches!(node, ASTNode::Loop { .. }))
        .map(|(index, _)| index)
        .expect("loop statement");

    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
        .expect("loop-cond fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let root_body = input.source().root_body().expect("root body");
    let root = input
        .source()
        .body_stmt(&root_body, loop_index)
        .expect("root loop");
    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        panic!("fixture root must be a loop")
    };
    let policy = LoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true);
    let outcome = single_planner::try_build_source_outcome(CallableLoopFactsPlannerInputV1::new(
        condition,
        body,
        policy,
        "armed-loop-cond-fixture".into(),
        false,
    ))
    .expect("planner outcome");
    let selection = CallableLoopRouteMatchV1::issue(outcome.facts.as_ref().expect("facts"));
    assert_eq!(
        selection.matched_routes(),
        [crate::mir::loop_recipe_contract::route_id::LoopRouteId::LoopCondBreakContinue]
    );
    let projection = issue_loop_cond_break_continue_source_forest_projection_v1(input, &root)
        .expect("forest projection");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");
    let items = CallableLoopSourceRouteTokenV1::source_items_for_loop(
        &ledger,
        input.owner(),
        root.site().node(),
    )
    .expect("armed loop must catalog resolver method rows");
    let call_sites = items
        .iter()
        .map(|item| item.call_site().clone())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    ArmedLoopCondParts {
        owner: input.owner(),
        parent_site: root.site().node().clone(),
        function_origin: input.function().function_origin(),
        source_kind: input.function().source_kind(),
        outcome,
        selection,
        projection,
        items,
        call_sites,
    }
}

pub(super) const ARMED_LOOP_COND_SOURCE: &str = r#"
static function caller(flag: i64, text: i64): i64 {
    loop(flag < 2) {
        if text.starts_with("a", 0) == 1 {
            flag = flag + 1
        } else {
            break
        }
    }
    return flag
}
"#;

/// Two cataloged calls under one loop root: the selected publication
/// obligation may cover only one of them, so the unrelated item exercises
/// the static-plus-other mix.
const MIXED_LOOP_COND_SOURCE: &str = r#"
static function caller(flag: i64, text: i64): i64 {
    loop(flag < 2) {
        if text.starts_with("a", 0) == 1 {
            local other = text.starts_with("b", 1)
            flag = flag + 1
        } else {
            break
        }
    }
    return flag
}
"#;

pub(super) fn selected_relation(
    call_site: &SourceExprSiteV1,
) -> CallableLoopSourceTargetRelationV1 {
    let caller =
        CanonicalSameModuleCallableKeyV1::test_static_box_method("ParserProgramBox", "parse", 2);
    let target = CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserStringUtilsBox",
        "starts_with",
        3,
    );
    let handoff = VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(
        7,
        VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
            caller,
            call_site.clone(),
            target.clone(),
        ),
        &[1],
    );
    CallableLoopSourceTargetRelationV1::new(
        call_site.clone(),
        target,
        Some(CallableLoopSourceTargetRequirementV1::from_handoff(
            &handoff,
        )),
    )
}

#[test]
fn composite_target_probe_requires_complete_source_order() {
    let parts = armed_loop_cond_parts(MIXED_LOOP_COND_SOURCE);
    assert_eq!(parts.items.len(), 2);
    let relations = parts
        .call_sites
        .iter()
        .map(selected_relation)
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let selected = CallableLoopSourceTargetProbeV1::from_parts(relations, Box::new([]), false)
        .into_selected_relations(&parts.items)
        .expect("composite source items require one selected relation each");
    assert_eq!(selected.len(), parts.items.len());

    let mut reversed = selected.into_vec();
    reversed.reverse();
    let error = CallableLoopSourceTargetProbeV1::from_parts(
        reversed.into_boxed_slice(),
        Box::new([]),
        false,
    )
    .into_selected_relations(&parts.items)
    .expect_err("composite handoffs must remain source ordered");
    assert!(matches!(
        error,
        CallableLoopSourceRouteRejectV1::SourceTargetOrderMismatch { .. }
    ));
}

#[test]
fn composite_target_probe_rejects_missing_selected_relation() {
    let parts = armed_loop_cond_parts(MIXED_LOOP_COND_SOURCE);
    let error = CallableLoopSourceTargetProbeV1::from_parts(
        vec![selected_relation(&parts.call_sites[0])].into_boxed_slice(),
        Box::new([]),
        false,
    )
    .into_selected_relations(&parts.items)
    .expect_err("composite source handoffs must cover every item");
    assert!(matches!(
        error,
        CallableLoopSourceRouteRejectV1::SourceTargetCardinality {
            expected: 2,
            actual: 1
        }
    ));
}

#[test]
fn composite_item_probe_accepts_mixed_static_and_core_method_order() {
    let parts = armed_loop_cond_parts(MIXED_LOOP_COND_SOURCE);
    let dispositions = CallableLoopSourceTargetProbeV1::from_parts_with_core_methods(
        vec![selected_relation(&parts.call_sites[0])].into_boxed_slice(),
        Box::new([]),
        false,
        vec![parts.items[1].clone()].into_boxed_slice(),
    )
    .into_item_dispositions(&parts.items)
    .expect("mixed source items must retain source order");
    assert_eq!(dispositions.len(), 2);
    assert!(matches!(
        &dispositions[0],
        CallableLoopSourceItemDispositionV1::SelectedStatic(relation)
            if relation.call_site() == &parts.call_sites[0]
    ));
    assert!(matches!(
        &dispositions[1],
        CallableLoopSourceItemDispositionV1::CoreMethod(item)
            if item.call_site() == &parts.call_sites[1]
    ));
}

#[test]
fn composite_item_probe_rejects_duplicate_source_item() {
    let parts = armed_loop_cond_parts(MIXED_LOOP_COND_SOURCE);
    let error = CallableLoopSourceTargetProbeV1::from_parts_with_core_methods(
        vec![selected_relation(&parts.call_sites[0])].into_boxed_slice(),
        Box::new([]),
        false,
        Box::new([]),
    )
    .into_item_dispositions(&[parts.items[0].clone(), parts.items[0].clone()])
    .expect_err("duplicate resolver item must not be silently coalesced");
    assert!(matches!(
        error,
        CallableLoopSourceRouteRejectV1::SourceItemDuplicate { .. }
    ));
}

#[test]
fn composite_item_probe_rejects_residual_core_method() {
    let parts = armed_loop_cond_parts(MIXED_LOOP_COND_SOURCE);
    let error = CallableLoopSourceTargetProbeV1::from_parts_with_core_methods(
        vec![selected_relation(&parts.call_sites[0])].into_boxed_slice(),
        Box::new([]),
        false,
        vec![parts.items[1].clone()].into_boxed_slice(),
    )
    .into_item_dispositions(&[parts.items[0].clone()])
    .expect_err("unconsumed core method must remain a named residual");
    assert!(matches!(
        error,
        CallableLoopSourceRouteRejectV1::SourceItemDispositionResidual { .. }
    ));
}
