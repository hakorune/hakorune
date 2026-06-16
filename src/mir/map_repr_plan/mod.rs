/*!
 * MIR-owned Map representation plans.
 *
 * This module records proof-bearing map representation families without
 * changing lowering behavior. Keep this file as a thin facade: plan vocabulary,
 * candidate detection, fast-path fact production, and refresh orchestration
 * live in separate modules so new Map fast paths do not pile into one owner.
 */

mod candidates;
mod fastpath;
mod plans;
mod refresh;

pub use plans::{
    LocalI64MapDirectStoragePlan, LocalI64MapEntryValueTrackingPlan,
    LocalMapStorageRealizationPlan, MapReprKind, MapReprPlan,
};
pub use refresh::{refresh_function_map_repr_plans, refresh_module_map_repr_plans};

#[cfg(test)]
mod tests;
