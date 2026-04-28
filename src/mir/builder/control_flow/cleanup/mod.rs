//! Top-level owner surface for control-flow cleanup and policy helpers.
//!
//! During folderization, migrated cleanup helpers can move here first.
//! Consumers should import the concrete `policies` owner modules.

pub(in crate::mir::builder) mod policies;
