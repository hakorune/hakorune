//! Top-level owner surface for control-flow verification and observability.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod diagnostics;
pub(in crate::mir::builder) mod observability;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::verifier::*;
