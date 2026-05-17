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
    typed_object_plan::refresh_module_typed_object_field_value_types,
    user_box_method_route_plan::refresh_module_user_box_method_routes,
};

const ROUTE_FIXPOINT_ITERATIONS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteFixpointStopReason {
    BoundedRefreshComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteFixpointReport {
    pub generic_passes: usize,
    pub global_passes: usize,
    pub user_box_passes: usize,
    pub map_lookup_passes: usize,
    pub typed_object_field_value_type_passes: usize,
    pub stop_reason: RouteFixpointStopReason,
}

/// Refresh route families that can publish facts consumed by each other.
///
/// The sequence is intentionally behavior-preserving relative to the old
/// `semantic_refresh.rs` ordering. A future row may make this convergence
/// report change-aware, but this row only moves the owner.
pub fn refresh_module_route_fixpoint(module: &mut MirModule) -> RouteFixpointReport {
    let mut report = RouteFixpointReport {
        generic_passes: 0,
        global_passes: 0,
        user_box_passes: 0,
        map_lookup_passes: 0,
        typed_object_field_value_type_passes: 0,
        stop_reason: RouteFixpointStopReason::BoundedRefreshComplete,
    };

    refresh_module_generic_method_routes(module);
    report.generic_passes += 1;
    refresh_module_global_call_routes(module);
    report.global_passes += 1;
    refresh_module_user_box_method_routes(module);
    report.user_box_passes += 1;

    for function in module.functions.values_mut() {
        // Some generic method routes depend on global-call target shapes
        // discovered only at module scope.
        refresh_function_map_lookup_fusion_routes(function);
    }
    report.map_lookup_passes += 1;

    refresh_module_typed_object_field_value_types(module);
    report.typed_object_field_value_type_passes += 1;

    refresh_module_generic_method_routes(module);
    report.generic_passes += 1;

    for _ in 0..ROUTE_FIXPOINT_ITERATIONS {
        refresh_module_global_call_routes(module);
        report.global_passes += 1;
        refresh_module_user_box_method_routes(module);
        report.user_box_passes += 1;
    }

    refresh_module_global_call_routes(module);
    report.global_passes += 1;

    report
}
