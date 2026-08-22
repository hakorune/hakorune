//! GenericLoop body lowering helpers (apply-only).

mod carrier_orchestration;
mod carriers;
mod cleanup;
mod direct_associated;
#[cfg(test)]
mod direct_associated_tests;
mod direct_port;
mod helpers;
#[cfg(test)]
mod nested_depth_observer_tests;
mod nested_loop_depth1_handoff;
mod nested_loop_recipe_adoption;
mod nested_loop_reject_tail;
mod terminality;
mod v0;
mod v1;

pub(in crate::mir::builder) use carrier_orchestration::{
    orchestrate_generic_loop_v1_carriers, orchestrate_generic_loop_v1_carriers_from_targets,
    GenericLoopV1CarrierOrchestration,
};
pub(in crate::mir::builder) use cleanup::{
    apply_generic_loop_v1_fallthrough_cleanup, apply_generic_loop_v1_fallthrough_cleanup_input,
};
pub(in crate::mir::builder) use direct_associated::{
    lower_direct_body_input_with_policy, lower_direct_statement_inputs,
};
pub(in crate::mir::builder) use direct_port::lower_generic_loop_v1_direct_inputs;
pub(in crate::mir::builder) use terminality::{
    body_plans_exit_on_all_paths, plans_require_continue_edge,
};
pub(in crate::mir::builder) use v0::lower_generic_loop_v0_body;
pub(in crate::mir::builder) use v1::body_has_blockexpr_prelude_loop;
pub(in crate::mir::builder) use v1::lower_generic_loop_v1_body;

#[cfg(test)]
pub(in crate::mir::builder) use nested_depth_observer_tests::{
    observe_nested_depth1, NestedBuilderSnapshotV1, NestedDepthObservationV1, NestedStageResultV1,
};

const GENERIC_LOOP_ERR: &str = "[normalizer] generic loop v0";
