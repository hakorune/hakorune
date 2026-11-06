// Compiler module registry.
//
// This file contains only Cargo/module topology. Semantic authorities and
// compiler entry points remain in the sibling modules and in `mod.rs`.

pub(in crate::mir) mod a_prime_i64_physical_capability;
pub(in crate::mir) mod acyclic_callable_graph;
pub(in crate::mir) mod acyclic_callable_module_plan;
pub(in crate::mir) mod callable_graph_inventory;
#[cfg(test)]
pub(crate) mod callable_loop_physical_canary;
pub(in crate::mir) mod callable_scc_partition;
pub(in crate::mir) mod callable_single_loop_operation_effect;
pub(in crate::mir) mod callable_semantic_program;
pub(crate) mod callable_single_loop_prelude_arguments;
pub(crate) mod callable_single_loop_recipe;
pub(crate) mod callable_single_loop_recipe_coseal;
#[cfg(test)]
#[path = "callable_single_loop_recipe_coseal_tests.rs"]
mod callable_single_loop_recipe_coseal_tests;
pub(crate) mod callable_single_loop_source_map;
pub(crate) mod callable_single_loop_source_shapes;
#[cfg(test)]
#[path = "callable_single_loop_static_fixture_tests.rs"]
pub(crate) mod callable_single_loop_static_fixture_tests;
pub(crate) mod callable_single_loop_syntax_facts;
pub(crate) mod canonical_core_dispatch;
pub(crate) mod canonical_script_source_a_input;
pub(crate) mod canonical_script_source_plan_envelope;
pub(crate) mod canonical_source_identity;
mod canonical_drain_manifest;
pub(in crate::mir) mod canonical_finalization;
pub(in crate::mir) mod canonical_physical_completion;
pub(in crate::mir) mod capability;
pub(in crate::mir) mod common_v2_session_admission;
pub(in crate::mir) mod common_v2_physical_function_entry_input;
pub(in crate::mir) mod common_v2_physical_function_skeleton;
pub(in crate::mir) mod direct_accum_capability;
pub(crate) mod direct_accum_observation;
pub(in crate::mir) mod direct_accum_prefix;
pub(in crate::mir) mod direct_accum_profile;
pub(in crate::mir) mod direct_accum_projection;
pub(in crate::mir) mod dynamic_full_body_recipe;
pub(in crate::mir) mod dynamic_full_body_source;
#[cfg(test)]
mod dynamic_full_body_source_tests;
pub(in crate::mir) mod external_commit;
pub(in crate::mir) mod function_input;
pub(crate) mod generic_g0_observation;
pub(in crate::mir) mod generic_g0_capability;
#[cfg(test)]
pub(crate) mod generic_g0_physical_prepare;
pub(in crate::mir) mod generic_g0_projection;
pub(in crate::mir) mod generic_g0_source_parent;
pub(in crate::mir) mod generic_g0_storage_lane_source;
pub(in crate::mir) mod generic_g0_physical_function_entry_input;
pub(in crate::mir) mod generic_g0_physical_function_effect;
pub(in crate::mir) mod generic_g0_physical_entry_facts;
pub(in crate::mir) mod generic_g0_physical_shell_facts;
pub(in crate::mir) mod generic_g0_physical_operation_cohort;
pub(in crate::mir) mod generic_g0_physical_operation_mapping;
pub(in crate::mir) mod generic_g0_function_effect;
pub(in crate::mir) mod generic_g0_completion;
pub(in crate::mir) mod generic_g0_result_abi;
pub(in crate::mir) mod generic_g0_top_level_declaration_header;
pub(in crate::mir) mod located;
pub(crate) mod loop_cond_break_continue_observation;
#[cfg(test)]
mod loop_family_window_probe_tests;
pub(crate) mod loop_node_winner_spine;
#[cfg(test)]
mod loop_node_winner_spine_tests;
// Caller-zero until the R0 route switch wires the physicalize edge;
// recheck at the production-switch slice.
pub(crate) mod loop_node_physical_admission;
#[cfg(test)]
mod loop_node_physical_admission_tests;
pub(in crate::mir) mod loop_cond_break_continue_typed_map;
pub(in crate::mir) mod loop_cond_break_continue_typed_map_issue;
pub(in crate::mir) mod loop_cond_break_continue_projection;
pub(in crate::mir) mod loop_break_source_projection;
pub(in crate::mir) mod loop_break_composite_source_projection;
pub(in crate::mir) mod loop_break_composite_body_role;
pub(crate) mod loop_physical_prepare;
// Caller-zero Main0 Continue co-seal issuer; sole owner is this row's source
// front. Recheck at the Main0 canonical-root handoff / production caller
// switch slice (active card generic-loop-mainline-source-front-d0).
pub(crate) mod main0_continue_recipe_coseal;
#[cfg(test)]
#[path = "main0_continue_recipe_coseal_tests.rs"]
mod main0_continue_recipe_coseal_tests;
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
pub(crate) mod main0_continue_root_selection;
#[cfg(test)]
#[path = "main0_continue_root_selection_tests.rs"]
mod main0_continue_root_selection_tests;
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
pub(in crate::mir) mod main0_continue_semantic_program;
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
pub(crate) mod main0_continue_source_map;
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice. Sole sealer of the source map product.
pub(crate) mod main0_continue_source_map_issue;
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
pub(crate) mod main0_continue_syntax_facts;
// Second bounded Main0 profile (in-body-step); same canonical-root handoff
// and production caller switch as the continue profile.
pub(crate) mod main0_in_body_step_recipe_coseal;
#[cfg(test)]
#[path = "main0_in_body_step_recipe_coseal_tests.rs"]
mod main0_in_body_step_recipe_coseal_tests;
pub(crate) mod main0_in_body_step_root_selection;
#[cfg(test)]
#[path = "main0_in_body_step_root_selection_tests.rs"]
mod main0_in_body_step_root_selection_tests;
pub(in crate::mir) mod main0_in_body_step_semantic_program;
pub(crate) mod main0_in_body_step_source_map;
pub(crate) mod main0_in_body_step_source_map_issue;
pub(crate) mod main0_in_body_step_syntax_facts;
// Third bounded Main0 profile (derived predicate); same canonical-root
// handoff and production caller switch as the landed profiles.
pub(crate) mod main0_derived_predicate_recipe_coseal;
#[cfg(test)]
#[path = "main0_derived_predicate_recipe_coseal_tests.rs"]
mod main0_derived_predicate_recipe_coseal_tests;
pub(crate) mod main0_derived_predicate_root_selection;
#[cfg(test)]
#[path = "main0_derived_predicate_root_selection_tests.rs"]
mod main0_derived_predicate_root_selection_tests;
pub(in crate::mir) mod main0_derived_predicate_semantic_program;
pub(crate) mod main0_derived_predicate_source_map;
pub(crate) mod main0_derived_predicate_source_map_issue;
pub(crate) mod main0_derived_predicate_syntax_facts;
pub(crate) mod loop_true_break_continue_observation;
pub(in crate::mir) mod loop_true_break_continue_projection;
mod lowering_input;
pub(in crate::mir) mod module_postprocess;
pub(in crate::mir) mod module_postprocess_stages;
pub(in crate::mir) mod pinned_text_backend_frame;
pub(in crate::mir) mod pinned_text_residence_backend_carrier;
pub(crate) mod pinned_text_residence_backend_projection;
mod module_session;
pub(in crate::mir) mod nested_predicate_effect_plan;
pub(crate) mod nested_predicate_observation;
pub(in crate::mir) mod nested_predicate_physical_input;
pub(in crate::mir) mod nested_predicate_producer;
#[cfg(test)]
mod nested_predicate_producer_tests;
pub(in crate::mir) mod nested_predicate_projection;
pub(in crate::mir) mod nested_predicate_source_handoff;
pub(in crate::mir) mod nested_predicate_topology;
pub(in crate::mir) mod variable_accum_recurrence_projection;
// Owner: selected package Loop handoff. Remove this temporary allowance when
// the typed Home/Recipe consumer lands; review by 2026-10-17.
pub(in crate::mir) mod variable_bound_mul_recurrence_source;
#[cfg(test)]
mod variable_accum_recurrence_projection_tests;
#[cfg(test)]
pub(crate) use nested_predicate_producer_tests::{
    nested_function as nested_function_for_p3_test,
    projection_for as nested_projection_for_test,
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
mod nested_predicate_physical_input_tests;
pub(in crate::mir) mod nested_predicate_profile;
#[cfg(test)]
mod nested_predicate_profile_tests;
#[cfg(test)]
mod nested_predicate_projection_tests;
#[cfg(test)]
mod nested_predicate_topology_tests;
mod normal_default_pipeline;
pub(crate) mod target_capability;
pub(crate) mod normal_source_plan;
pub(in crate::mir) mod publication_kernel;
pub(in crate::mir) mod raw_public_ingress;
pub(in crate::mir) mod raw_published_compile;
pub(in crate::mir) mod raw_root_callable_main;
pub(in crate::mir) mod raw_root_children;
pub(in crate::mir) mod raw_root_decl_access;
pub(in crate::mir) mod raw_root_drain;
pub(in crate::mir) mod raw_root_eligibility;
pub(in crate::mir) mod raw_root_eligibility_classifier;
pub(in crate::mir) mod raw_root_eligibility_prepare;
pub(in crate::mir) mod raw_root_external_commit;
pub(in crate::mir) mod raw_root_finalization;
pub(in crate::mir) mod raw_root_helper_coverage;
pub(in crate::mir) mod raw_root_package;
pub(in crate::mir) mod raw_root_postprocess;
pub(in crate::mir) mod raw_root_publication;
pub(in crate::mir) mod raw_root_publication_adapter;
pub(in crate::mir) mod raw_runtime_inputs;
pub(in crate::mir) mod raw_source_binding;
pub(in crate::mir) mod recursive_callable_module_plan;
pub(in crate::mir) mod resolved_callable_module;
mod resolved_callable_module_input;
pub(in crate::mir) mod resolved_callable_module_preflight;
mod resolved_direct_accum_cutover;
mod resolved_nested_predicate_cutover;
mod resolved_generic_g0_cutover;
pub(in crate::mir) mod selected_dynamic_w6_activation;
pub(in crate::mir) mod source_bound_package;
pub(in crate::mir) mod source_bound_package_generic_g0;
pub(in crate::mir) mod source_bound_plan;
pub(in crate::mir) mod source_entry_ny_main;
pub(in crate::mir) mod source_entry_physical;
pub(in crate::mir) mod source_entry_projection;
pub(in crate::mir) mod source_entry_published_invocation;
pub(in crate::mir) mod source_entry_reference;
pub(in crate::mir) mod source_entry_result;
pub(in crate::mir) mod source_entry_selection;
pub(in crate::mir) mod source_entry_thunk;
pub(in crate::mir) mod source_entry_vm_diagnostic;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_execution;
pub(in crate::mir) mod source_entry_vm_invocation;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_raw_adapter;
pub(in crate::mir) mod source_entry_vm_reference;
#[cfg(test)]
mod source_entry_vm_reference_p0;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_runner_adapter;
pub(in crate::mir) mod source_projection;
pub(in crate::mir) mod source_view;
