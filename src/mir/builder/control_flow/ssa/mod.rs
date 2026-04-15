//! Top-level owner surface for control-flow SSA and exit-binding repair.
//!
//! During folderization, implementations still live under `plan/`.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod exit_binding;
pub(in crate::mir::builder) mod exit_binding_applicator;
pub(in crate::mir::builder) mod exit_binding_constructor;
pub(in crate::mir::builder) mod exit_binding_validator;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::exit_binding::*;
