// Phase 287 P2: Contract checks modularization
//
// This module provides contract verification functions organized by responsibility.
// Each contract is implemented in its own module for clear separation of concerns.
//
// Design principle: 1 module = 1 contract
// - terminator_targets: Branch/Jump target existence
// - exit_bindings: exit_bindings ↔ exit_phis consistency
// - carrier_inputs: carrier_inputs completeness
// - boundary_creation: B1/C2 invariants at boundary creation
// - entry_params: Entry function parameter consistency

mod terminator_targets;
mod exit_bindings;
mod carrier_inputs;
mod boundary_creation;
mod boundary_hygiene;
mod header_phi_layout;
mod entry_params;
mod entry_like_policy;

// Re-export public functions
pub(super) use terminator_targets::verify_all_terminator_targets_exist;
pub(super) use exit_bindings::verify_exit_bindings_have_exit_phis;
pub(super) use carrier_inputs::verify_carrier_inputs_complete;
pub(super) use boundary_hygiene::verify_boundary_hygiene;
pub(super) use header_phi_layout::verify_header_phi_layout;
pub(super) use entry_like_policy::is_entry_like_source;
pub(in crate::mir::builder::control_flow::joinir) use boundary_creation::verify_boundary_contract_at_creation;
pub(in crate::mir::builder) use entry_params::run_all_pipeline_checks;

// Note: get_entry_function is kept internal to entry_params module
// Patterns use the version from patterns/common/joinir_helpers.rs instead
