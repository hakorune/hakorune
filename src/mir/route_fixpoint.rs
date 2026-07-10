/*!
 * Module route refresh fixpoint owner.
 *
 * This module owns the module-level route convergence sequence. It does not
 * add route acceptance shapes; family-specific planners still own their
 * materialization rules.
 *
 * SSOT:
 * docs/development/current/main/design/route-fixpoint-owner-ssot.md
 */

use super::MirModule;
use super::{
    generic_method_route_plan::refresh_module_generic_method_routes,
    global_call_route_plan::refresh_module_global_call_routes,
    map_lookup_fusion_plan::refresh_function_map_lookup_fusion_routes,
    map_repr_plan::refresh_function_map_repr_plans,
    ordered_map_origin_plan::refresh_module_ordered_map_get_result_origins,
    typed_object_plan::{
        refresh_module_typed_object_collection_field_element_value_types,
        refresh_module_typed_object_field_value_types,
    },
    user_box_method_route_plan::refresh_module_user_box_method_routes,
};
use std::time::{Duration, Instant};

const ROUTE_FIXPOINT_ITERATIONS: usize = 4;

/// Refresh route families that can publish facts consumed by each other.
///
/// The sequence is intentionally behavior-preserving relative to the old
/// `semantic_refresh.rs` ordering. All work is in-place mutation of `module`.
pub fn refresh_module_route_fixpoint(module: &mut MirModule) {
    let mut generic_elapsed = Duration::ZERO;
    let mut global_elapsed = Duration::ZERO;
    let mut user_box_elapsed = Duration::ZERO;
    let mut typed_elapsed = Duration::ZERO;

    let stage_start = Instant::now();
    refresh_module_generic_method_routes(module);
    generic_elapsed += stage_start.elapsed();
    let stage_start = Instant::now();
    let _ = refresh_module_global_call_routes(module);
    global_elapsed += stage_start.elapsed();
    let stage_start = Instant::now();
    let _ = refresh_module_user_box_method_routes(module);
    user_box_elapsed += stage_start.elapsed();

    for function in module.functions.values_mut() {
        // Some generic method routes depend on global-call target shapes
        // discovered only at module scope.
        refresh_function_map_lookup_fusion_routes(function);
        refresh_function_map_repr_plans(function);
    }

    let stage_start = Instant::now();
    refresh_module_typed_object_field_value_types(module);
    refresh_module_typed_object_collection_field_element_value_types(module);
    typed_elapsed += stage_start.elapsed();

    // Seed focused carrier-data map result origins before generic/user-box
    // refresh so downstream ArrayBox reads can inherit the published type.
    refresh_module_ordered_map_get_result_origins(module);

    let stage_start = Instant::now();
    refresh_module_generic_method_routes(module);
    generic_elapsed += stage_start.elapsed();

    let mut outer_iterations = 0usize;
    for _ in 0..ROUTE_FIXPOINT_ITERATIONS {
        outer_iterations += 1;
        let stage_start = Instant::now();
        let global_changed = refresh_module_global_call_routes(module);
        global_elapsed += stage_start.elapsed();
        let stage_start = Instant::now();
        let user_box_changed = refresh_module_user_box_method_routes(module);
        user_box_elapsed += stage_start.elapsed();
        if !global_changed && !user_box_changed {
            break;
        }
    }

    let stage_start = Instant::now();
    let _ = refresh_module_global_call_routes(module);
    global_elapsed += stage_start.elapsed();

    // Re-run the focused origin publication after user-box routes settle so
    // route result-box overrides and nested ArrayBox reads stay aligned.
    refresh_module_ordered_map_get_result_origins(module);
    let stage_start = Instant::now();
    refresh_module_typed_object_collection_field_element_value_types(module);
    typed_elapsed += stage_start.elapsed();
    let stage_start = Instant::now();
    refresh_module_generic_method_routes(module);
    generic_elapsed += stage_start.elapsed();
    let stage_start = Instant::now();
    let _ = refresh_module_user_box_method_routes(module);
    user_box_elapsed += stage_start.elapsed();
    refresh_module_ordered_map_get_result_origins(module);

    super::compile_timing::trace_stage("semantic.route.generic", generic_elapsed);
    super::compile_timing::trace_stage("semantic.route.global", global_elapsed);
    super::compile_timing::trace_stage("semantic.route.user_box", user_box_elapsed);
    super::compile_timing::trace_stage("semantic.route.typed", typed_elapsed);
    super::compile_timing::trace_count("semantic.route.outer_iterations", outer_iterations);
}
