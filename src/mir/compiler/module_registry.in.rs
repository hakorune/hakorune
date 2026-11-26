// Compiler module registry.
//
// This file contains only Cargo/module topology. Semantic authorities and
// compiler entry points remain in the sibling modules and in `mod.rs`.

pub(in crate::mir) mod callable_graph_inventory;
pub(in crate::mir) mod callable_scc_partition;
#[cfg(test)]
pub(crate) mod loop_source_expression_shapes;
pub(crate) mod canonical_source_identity;
pub(in crate::mir) mod capability;
#[cfg(test)]
pub(crate) mod direct_accum_observation;
pub(in crate::mir) mod direct_accum_projection;
pub(in crate::mir) mod dynamic_full_body_recipe;
pub(in crate::mir) mod dynamic_full_body_source;
#[cfg(test)]
mod dynamic_full_body_source_tests;
pub(in crate::mir) mod function_input;
#[cfg(test)]
pub(crate) mod generic_g0_observation;
#[cfg(test)]
pub(in crate::mir) mod generic_g0_projection;
pub(in crate::mir) mod located;
#[cfg(test)]
pub(crate) mod loop_cond_break_continue_observation;
#[cfg(test)]
mod loop_family_window_probe_tests;
pub(in crate::mir) mod loop_break_source_projection;
pub(in crate::mir) mod loop_cond_break_continue_projection;
#[cfg(test)]
pub(in crate::mir) mod loop_cond_break_continue_typed_map;
#[cfg(test)]
pub(in crate::mir) mod loop_cond_break_continue_typed_map_issue;
// Caller-zero Main0 Continue co-seal issuer; sole owner is this row's source
// front. Recheck at the Main0 canonical-root handoff / production caller
// switch slice (active card generic-loop-mainline-source-front-d0).
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice. Sole sealer of the source map product.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Second bounded Main0 profile (in-body-step); same canonical-root handoff
// and production caller switch as the continue profile.
// Third bounded Main0 profile (derived predicate); same canonical-root
// handoff and production caller switch as the landed profiles.
#[cfg(test)]
pub(crate) mod loop_true_break_continue_observation;
pub(in crate::mir) mod loop_true_break_continue_projection;
mod lowering_input;
mod module_session;
#[cfg(test)]
pub(in crate::mir) mod nested_predicate_effect_plan;
#[cfg(test)]
pub(crate) mod nested_predicate_observation;
#[cfg(test)]
pub(in crate::mir) mod nested_predicate_producer;
#[cfg(test)]
mod nested_predicate_producer_tests;
pub(in crate::mir) mod nested_predicate_projection;
#[cfg(test)]
pub(in crate::mir) mod nested_predicate_source_handoff;
pub(in crate::mir) mod pinned_text_backend_frame;
pub(in crate::mir) mod pinned_text_residence_backend_carrier;
pub(crate) mod pinned_text_residence_backend_projection;
#[cfg(test)]
pub(crate) use nested_predicate_producer_tests::{
    nested_function as nested_function_for_p3_test, projection_for as nested_projection_for_test,
};
#[cfg(test)]
mod loop_true_break_continue_projection_tests;
#[cfg(test)]
pub(crate) use loop_true_break_continue_projection_tests::positive_function as loop_true_function_for_test;
#[cfg(test)]
mod loop_cond_break_continue_projection_tests;
#[cfg(test)]
pub(crate) use loop_cond_break_continue_projection_tests::positive_function as loop_cond_function_for_test;
#[cfg(test)]
mod nested_predicate_effect_plan_tests;
#[cfg(test)]
mod nested_predicate_projection_tests;
mod normal_default_pipeline;
pub(crate) mod normal_source_plan;
pub(in crate::mir) mod recursive_callable_module_plan;
pub(in crate::mir) mod resolved_callable_module;
mod resolved_callable_module_input;
pub(in crate::mir) mod selected_dynamic_w6_activation;
pub(in crate::mir) mod source_projection;
pub(in crate::mir) mod source_view;
pub(crate) mod target_capability;
