// Compiler module registry.
//
// This file contains only Cargo/module topology. Semantic authorities and
// compiler entry points remain in the sibling modules and in `mod.rs`.

#[allow(dead_code)]
pub(in crate::mir) mod a_prime_i64_physical_capability;
#[allow(dead_code)]
pub(in crate::mir) mod acyclic_callable_graph;
#[allow(dead_code)]
pub(in crate::mir) mod acyclic_callable_module_plan;
#[allow(dead_code)]
pub(in crate::mir) mod callable_graph_inventory;
#[cfg(test)]
pub(crate) mod callable_loop_physical_canary;
#[allow(dead_code)]
pub(in crate::mir) mod callable_scc_partition;
#[allow(dead_code)]
pub(in crate::mir) mod callable_single_loop_operation_effect;
#[cfg(test)]
pub(crate) mod callable_single_loop_prelude_arguments;
#[allow(dead_code)]
pub(crate) mod callable_single_loop_recipe;
#[allow(dead_code)]
pub(crate) mod callable_single_loop_recipe_coseal;
#[cfg(test)]
#[path = "callable_single_loop_recipe_coseal_tests.rs"]
mod callable_single_loop_recipe_coseal_tests;
#[allow(dead_code)]
pub(crate) mod callable_single_loop_source_map;
#[allow(dead_code)]
pub(crate) mod callable_single_loop_source_shapes;
#[cfg(test)]
#[path = "callable_single_loop_static_fixture_tests.rs"]
pub(crate) mod callable_single_loop_static_fixture_tests;
#[allow(dead_code)]
pub(crate) mod callable_single_loop_syntax_facts;
#[allow(dead_code)]
pub(crate) mod canonical_core_dispatch;
#[allow(dead_code)]
mod canonical_drain_manifest;
#[allow(dead_code)]
pub(in crate::mir) mod canonical_finalization;
#[allow(dead_code)]
pub(in crate::mir) mod canonical_physical_completion;
pub(in crate::mir) mod capability;
#[allow(dead_code)]
pub(in crate::mir) mod common_v2_session_admission;
#[allow(dead_code)]
pub(in crate::mir) mod common_v2_physical_function_entry_input;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_capability;
#[cfg(test)]
pub(crate) mod direct_accum_observation;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_prefix;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_profile;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_projection;
#[allow(dead_code)]
pub(in crate::mir) mod dynamic_full_body_recipe;
#[allow(dead_code)]
pub(in crate::mir) mod dynamic_full_body_source;
#[cfg(test)]
mod dynamic_full_body_source_tests;
#[allow(dead_code)]
pub(in crate::mir) mod external_commit;
pub(in crate::mir) mod function_input;
#[cfg(test)]
pub(crate) mod generic_g0_observation;
#[cfg(test)]
pub(crate) mod generic_g0_physical_prepare;
#[allow(dead_code)]
pub(in crate::mir) mod generic_g0_projection;
#[allow(dead_code)]
pub(in crate::mir) mod located;
#[cfg(test)]
pub(crate) mod loop_cond_break_continue_observation;
#[allow(dead_code)]
pub(in crate::mir) mod loop_cond_break_continue_projection;
#[cfg(test)]
pub(crate) mod loop_physical_prepare;
#[cfg(test)]
pub(crate) mod loop_true_break_continue_observation;
#[allow(dead_code)]
pub(in crate::mir) mod loop_true_break_continue_projection;
mod lowering_input;
#[allow(dead_code)]
pub(in crate::mir) mod module_postprocess;
pub(in crate::mir) mod module_postprocess_stages;
pub(in crate::mir) mod pinned_text_backend_frame;
mod module_session;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_effect_plan;
#[cfg(test)]
pub(crate) mod nested_predicate_observation;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_physical_input;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_producer;
#[cfg(test)]
mod nested_predicate_producer_tests;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_projection;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_source_handoff;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_topology;
#[allow(dead_code)]
pub(in crate::mir) mod variable_accum_break_projection;
#[allow(dead_code)]
pub(in crate::mir) mod variable_accum_recurrence_projection;
#[cfg(test)]
mod variable_accum_recurrence_projection_tests;
#[cfg(test)]
pub(crate) use nested_predicate_producer_tests::nested_function as nested_function_for_p3_test;
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
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_profile;
#[cfg(test)]
mod nested_predicate_profile_tests;
#[cfg(test)]
mod nested_predicate_projection_tests;
#[cfg(test)]
mod nested_predicate_topology_tests;
mod normal_default_pipeline;
pub(crate) mod target_capability;
#[allow(dead_code)]
pub(crate) mod normal_source_plan;
pub(in crate::mir) mod publication_kernel;
pub(in crate::mir) mod raw_public_ingress;
pub(in crate::mir) mod raw_published_compile;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_callable_main;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_children;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_decl_access;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_drain;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility_classifier;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility_prepare;
pub(in crate::mir) mod raw_root_external_commit;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_finalization;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_helper_coverage;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_package;
pub(in crate::mir) mod raw_root_postprocess;
pub(in crate::mir) mod raw_root_publication;
pub(in crate::mir) mod raw_root_publication_adapter;
#[allow(dead_code)]
pub(in crate::mir) mod raw_runtime_inputs;
#[allow(dead_code)]
pub(in crate::mir) mod raw_source_binding;
#[allow(dead_code)]
pub(in crate::mir) mod recursive_callable_module_plan;
#[allow(dead_code)]
pub(in crate::mir) mod resolved_callable_module;
mod resolved_callable_module_input;
#[allow(dead_code)]
pub(in crate::mir) mod resolved_callable_module_preflight;
#[allow(dead_code)]
mod resolved_direct_accum_cutover;
#[allow(dead_code)]
mod resolved_nested_predicate_cutover;
pub(in crate::mir) mod selected_dynamic_w6_activation;
#[allow(dead_code)]
pub(in crate::mir) mod source_bound_package;
#[allow(dead_code)]
pub(in crate::mir) mod source_bound_plan;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_ny_main;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_physical;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_projection;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_published_invocation;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_reference;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_result;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_selection;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_thunk;
pub(in crate::mir) mod source_entry_vm_diagnostic;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_execution;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_vm_invocation;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_raw_adapter;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_vm_reference;
#[cfg(test)]
mod source_entry_vm_reference_p0;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_runner_adapter;
#[allow(dead_code)]
pub(in crate::mir) mod source_projection;
#[allow(dead_code)]
pub(in crate::mir) mod source_view;
