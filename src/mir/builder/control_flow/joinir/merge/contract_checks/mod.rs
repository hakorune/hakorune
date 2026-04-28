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

mod boundary_creation;
mod boundary_hygiene;
mod carrier_inputs;
mod entry_like_policy;
mod exit_bindings;
mod header_phi_layout;
mod terminator_targets;

// Re-export public functions
pub(in crate::mir::builder::control_flow::joinir) use boundary_creation::verify_boundary_contract_at_creation;
pub(super) use boundary_hygiene::verify_boundary_hygiene;
pub(super) use carrier_inputs::verify_carrier_inputs_complete;
pub(super) use entry_like_policy::is_entry_like_source;
pub(super) use exit_bindings::verify_exit_bindings_have_exit_phis;
pub(super) use header_phi_layout::verify_header_phi_layout;
pub(super) use terminator_targets::verify_all_terminator_targets_exist;
