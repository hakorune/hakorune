//! Atomic parser-owned callable source for the final parser Program.
//!
//! Declaration and generator transactions issue opaque anchors.  This module
//! only co-seals those anchors with their exact final AST placements; it never
//! recreates identity from names, spans, ordinals, or pointers.

mod issue;
mod model;
mod syntax_loan;

pub(in crate::parser) use issue::expected_callable_slots;
pub(super) use issue::{
    compatibility_program_can_enter_initial_callable_lane_v1,
    issue_initial_callable_program_source_v1, InitialCallableProgramSourceRejectV1,
};
pub(crate) use model::InitialCallableFinalSlotV1;
pub(crate) use model::VerifiedInitialCallableProgramSourceV1;
pub(in crate::parser) use syntax_loan::declaration_at;

#[cfg(test)]
mod tests;
