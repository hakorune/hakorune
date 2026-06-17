/*!
 * MIR-owned Map representation plans.
 *
 * This module records proof-bearing map representation families without
 * changing lowering behavior. Keep this file as a thin facade: plan vocabulary,
 * candidate detection, and refresh orchestration live in separate modules.
 * Positive `LocalFastPathFact` production is aggregated by
 * `crate::mir::local_fastpath_fact` so map-specific evidence does not own the
 * final backend-consumable fact assignment.
 */

mod candidates;
mod plans;
mod refresh;

pub use plans::{
    LocalI64MapDirectStoragePlan, LocalI64MapEntryValueTrackingPlan,
    LocalMapStorageRealizationPlan, MapReprKind, MapReprPlan,
};
pub use refresh::{refresh_function_map_repr_plans, refresh_module_map_repr_plans};

#[cfg(test)]
mod tests;
