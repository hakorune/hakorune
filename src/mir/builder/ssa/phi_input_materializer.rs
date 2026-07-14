//! PHI input materialization facade.
//!
//! Per-edge value rematerialization and legacy whole-function PHI repair are
//! intentionally separate responsibilities. Callers keep the existing API
//! while SSA-L0 changes only the physical ownership boundary.

mod edge_rematerialization;
mod function_repair;

#[cfg(test)]
mod edge_rematerialization_tests;
#[cfg(test)]
mod function_repair_tests;
#[cfg(test)]
mod test_support;

pub(in crate::mir::builder) use edge_rematerialization::for_pred;
pub(in crate::mir::builder) use function_repair::materialize_all_phi_inputs;
