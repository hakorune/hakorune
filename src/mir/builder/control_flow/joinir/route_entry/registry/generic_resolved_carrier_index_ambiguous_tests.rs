//! D2-S2 source-backed IndexWrite/Ambiguous matrix evidence, test-only.
//!
//! This row closes one missing negative cell before any eligibility or
//! production handoff is considered. It consumes the existing resolver and
//! facts authorities, then stops before route execution.

use super::generic_resolved_carrier_projector_tests::{input_and_root, unit};
use super::route_id::LoopRouteId;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    test_issue_live_preflight_frame, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_source_forest_v1, VerifiedLoopSourceForestBindingV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, LoopExecutionFrameKeyV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
};

const INDEX_SOURCE: &str = r#"
function generic_both_nested_index(i, j, items) {
    loop(i < 3) {
        loop(j < 3) {
            if i < 2 {
                items[j] = i
            }
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexModeV1 {
    Release,
    Strict,
}

impl IndexModeV1 {
    fn config(self) -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[
            (
                "HAKO_JOINIR_STRICT",
                match self {
                    Self::Release => None,
                    Self::Strict => Some("1"),
                },
            ),
            ("HAKO_JOINIR_PLANNER_REQUIRED", None),
            ("NYASH_JOINIR_STRICT", None),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexAmbiguousDispositionV1 {
    UnresolvedStopIndexWriteAmbiguousCarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexObservationRejectV1 {
    Owner,
    Forest,
    Navigation,
    Facts,
    Target,
    Receiver,
    Subscript,
    Identity,
    Carrier,
}

#[derive(Debug, PartialEq, Eq)]
struct IndexAmbiguousObservationV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    frame_key: LoopExecutionFrameKeyV1,
    receiver_binding: BindingRefV1,
    subscript_binding: BindingRefV1,
    post_read_binding: BindingRefV1,
    source_identity_stable: bool,
    mode: IndexModeV1,
    raw_schedule: Box<[LoopRouteId]>,
    carrier_reason: Box<str>,
}

fn observe_index_ambiguous(
    mode: IndexModeV1,
) -> Result<IndexAmbiguousObservationV1, IndexObservationRejectV1> {
    let source_unit = unit(INDEX_SOURCE);
    let (input, root) = input_and_root(&source_unit);
    if input.owner() != root.owner() {
        return Err(IndexObservationRejectV1::Owner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| IndexObservationRejectV1::Forest)?;
    if forest.members().len() != 2
        || forest.members()[0].parent_index().is_some()
        || forest.members()[1].parent_index() != Some(0)
        || forest.members()[0].source().site() != root.site()
    {
        return Err(IndexObservationRejectV1::Forest);
    }
    let outer_site = forest.members()[0].source().site().clone();
    let outer_frame_key = forest.members()[0].source().frame_key();
    let inner_site = forest.members()[1].source().site().clone();
    let frame_key = outer_frame_key.clone();
    let forest_binding = bind_resolved_loop_source_forest_v1(forest)
        .map_err(|_| IndexObservationRejectV1::Forest)?;

    let source = input.source();
    let outer_body = source
        .child_body_from_stmt(&root, BodyChildRoleV1::LoopBody)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let inner = source
        .body_stmt(&outer_body, 0)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let inner_body = source
        .child_body_from_stmt(&inner, BodyChildRoleV1::LoopBody)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let if_stmt = source
        .body_stmt(
            &source
                .child_body_from_stmt(
                    &source
                        .body_stmt(&inner_body, 0)
                        .map_err(|_| IndexObservationRejectV1::Navigation)?,
                    BodyChildRoleV1::IfThen,
                )
                .map_err(|_| IndexObservationRejectV1::Navigation)?,
            0,
        )
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let target = source
        .child_expr_from_stmt(&if_stmt, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let subscript = source
        .child_expr_from_expr(&target, ExprChildRoleV1::IndexSubscript)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let function_body = source
        .root_body()
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let return_stmt = source
        .body_stmt(&function_body, 1)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;
    let return_value = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| IndexObservationRejectV1::Navigation)?;

    let receiver_site = match function.assignment_target(target.site()) {
        Some(ResolvedAssignmentTargetV1::IndexWrite { receiver }) => receiver,
        _ => return Err(IndexObservationRejectV1::Target),
    };
    let receiver_binding = match function.variable_ref(&receiver_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(IndexObservationRejectV1::Receiver),
    };
    let subscript_binding = match function.variable_ref(subscript.site()) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(IndexObservationRejectV1::Subscript),
    };
    let post_read_binding = match function.variable_ref(return_value.site()) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(IndexObservationRejectV1::Subscript),
    };
    let source_identity_stable = outer_frame_key.matches(&frame_key)
        && outer_site == *root.site()
        && inner_site != *root.site()
        && receiver_binding.owner() == function.owner()
        && subscript_binding == post_read_binding
        && post_read_binding.owner() == function.owner();
    if !source_identity_stable {
        return Err(IndexObservationRejectV1::Identity);
    }

    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(IndexObservationRejectV1::Navigation);
    };
    let ctx = LoopRouteContext::new(condition, body, "generic_index_ambiguous/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| IndexObservationRejectV1::Facts)?;
    let facts: &CanonicalLoopFacts = outcome
        .facts
        .as_ref()
        .ok_or(IndexObservationRejectV1::Facts)?;
    let generic = facts
        .facts
        .generic_loop_v1()
        .ok_or(IndexObservationRejectV1::Facts)?;
    let carrier_reason = match &generic.carrier_observation {
        GenericLoopCarrierObservationV1::Ambiguous(reason) if reason == "assignment target" => {
            reason.clone().into_boxed_str()
        }
        _ => return Err(IndexObservationRejectV1::Carrier),
    };
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    if planner_required {
        return Err(IndexObservationRejectV1::Facts);
    }
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict, planner_required);
    Ok(IndexAmbiguousObservationV1 {
        forest_binding,
        frame_key,
        receiver_binding,
        subscript_binding,
        post_read_binding,
        source_identity_stable,
        mode,
        raw_schedule: frame.test_raw_schedule().to_vec().into_boxed_slice(),
        carrier_reason,
    })
}

fn classify(observation: &IndexAmbiguousObservationV1) -> IndexAmbiguousDispositionV1 {
    assert_eq!(observation.forest_binding.members().len(), 2);
    assert!(observation.source_identity_stable);
    assert_eq!(observation.subscript_binding, observation.post_read_binding);
    assert_eq!(observation.carrier_reason.as_ref(), "assignment target");
    assert_eq!(
        observation.raw_schedule.as_ref(),
        [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert!(observation.frame_key.matches(&observation.frame_key));
    IndexAmbiguousDispositionV1::UnresolvedStopIndexWriteAmbiguousCarrier
}

fn observe_in_mode(mode: IndexModeV1) -> IndexAmbiguousObservationV1 {
    let _config = mode.config();
    observe_index_ambiguous(mode).expect("Index/Ambiguous source witness")
}

#[test]
fn index_write_ambiguous_source_row_is_typed_and_pre_effect() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [IndexModeV1::Release, IndexModeV1::Strict] {
        let observation = observe_in_mode(mode);
        assert_eq!(observation.mode, mode);
        assert_eq!(
            classify(&observation),
            IndexAmbiguousDispositionV1::UnresolvedStopIndexWriteAmbiguousCarrier
        );
    }
}

#[test]
fn index_write_ambiguous_source_repeat_is_stable() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let first = observe_in_mode(IndexModeV1::Release);
    let second = observe_in_mode(IndexModeV1::Release);
    // Compilation-unit owner ordinals are fresh for each parsed witness, so
    // compare the semantic source/facts projection rather than ephemeral
    // owner identities.
    assert_eq!(
        first.forest_binding.members(),
        second.forest_binding.members()
    );
    assert_eq!(first.frame_key, second.frame_key);
    assert_eq!(
        first.receiver_binding.binding(),
        second.receiver_binding.binding()
    );
    assert_eq!(
        first.subscript_binding.binding(),
        second.subscript_binding.binding()
    );
    assert_eq!(
        first.post_read_binding.binding(),
        second.post_read_binding.binding()
    );
    assert_eq!(first.source_identity_stable, second.source_identity_stable);
    assert_eq!(first.mode, second.mode);
    assert_eq!(first.raw_schedule, second.raw_schedule);
    assert_eq!(first.carrier_reason, second.carrier_reason);
    assert_eq!(classify(&first), classify(&second));
}

#[test]
fn index_write_ambiguous_mode_pair_rejects_before_selection() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let release = observe_in_mode(IndexModeV1::Release);
    let strict = observe_in_mode(IndexModeV1::Strict);
    assert_ne!(release.mode, strict.mode);
    assert_eq!(classify(&release), classify(&strict));
}
