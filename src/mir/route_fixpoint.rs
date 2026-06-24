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

const ROUTE_FIXPOINT_ITERATIONS: usize = 4;

/// Refresh route families that can publish facts consumed by each other.
///
/// The sequence is intentionally behavior-preserving relative to the old
/// `semantic_refresh.rs` ordering. All work is in-place mutation of `module`.
pub fn refresh_module_route_fixpoint(module: &mut MirModule) {
    refresh_module_generic_method_routes(module);
    refresh_module_global_call_routes(module);
    refresh_module_user_box_method_routes(module);

    for function in module.functions.values_mut() {
        // Some generic method routes depend on global-call target shapes
        // discovered only at module scope.
        refresh_function_map_lookup_fusion_routes(function);
        refresh_function_map_repr_plans(function);
    }

    refresh_module_typed_object_field_value_types(module);
    refresh_module_typed_object_collection_field_element_value_types(module);

    // Seed focused carrier-data map result origins before generic/user-box
    // refresh so downstream ArrayBox reads can inherit the published type.
    refresh_module_ordered_map_get_result_origins(module);

    refresh_module_generic_method_routes(module);

    for _ in 0..ROUTE_FIXPOINT_ITERATIONS {
        refresh_module_global_call_routes(module);
        refresh_module_user_box_method_routes(module);
    }

    refresh_module_global_call_routes(module);

    // Re-run the focused origin publication after user-box routes settle so
    // route result-box overrides and nested ArrayBox reads stay aligned.
    refresh_module_ordered_map_get_result_origins(module);
    refresh_module_typed_object_collection_field_element_value_types(module);
    refresh_module_generic_method_routes(module);
    refresh_module_user_box_method_routes(module);
    refresh_module_ordered_map_get_result_origins(module);
}
