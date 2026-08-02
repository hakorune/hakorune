//! Top-level owner surface for control-flow cleanup and policy helpers.
//!
//! During folderization, migrated cleanup helpers can move here first.
//! Consumers should import the concrete `policies` owner modules.

mod exit_admission;
pub(in crate::mir::builder) mod policies;

pub(in crate::mir::builder) use exit_admission::{
    ensure_cleanup_exit_allowed_v1, CleanupExitKindV1, CleanupExitPolicyV1,
};
