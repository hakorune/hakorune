//! PHI input materialization facade.
//!
//! Per-edge rematerialization and active whole-function PHI repair are kept in
//! separate owners. The clone-only legacy candidate is retired; callers use
//! the live function-repair path below.

mod edge_rematerialization;
pub(in crate::mir::builder) mod edge_verifier;
mod function_repair;
pub(in crate::mir::builder) mod remat_fact;

#[cfg(test)]
mod edge_rematerialization_tests;
#[cfg(test)]
mod edge_verifier_p0_tests;
#[cfg(test)]
mod function_repair_tests;
#[cfg(test)]
mod remat_fact_tests;
#[cfg(test)]
mod test_support;

pub(in crate::mir::builder) use edge_rematerialization::for_pred;
pub(in crate::mir::builder) use function_repair::materialize_all_phi_inputs;
