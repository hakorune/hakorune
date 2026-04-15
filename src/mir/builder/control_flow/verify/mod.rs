//! Top-level owner surface for control-flow verification and observability.
//!
//! During folderization, migrated verification modules live here first.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod coreloop_body_contract;
pub(in crate::mir::builder) mod diagnostics;
pub(in crate::mir::builder) mod observability;
pub(in crate::mir::builder) mod verifier;

#[allow(unused_imports)]
pub(in crate::mir::builder) use coreloop_body_contract::is_leaf_effect_plan;
#[allow(unused_imports)]
pub(in crate::mir::builder) use verifier::*;
