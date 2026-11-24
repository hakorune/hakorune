//! Top-level owner surface for control-flow verification and observability.
//!
//! During folderization, migrated verification modules live here first.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod diagnostics;
pub(in crate::mir::builder) mod verifier;
