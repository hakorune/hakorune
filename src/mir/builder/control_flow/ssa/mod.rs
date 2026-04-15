//! Top-level owner surface for control-flow SSA and exit-binding repair.
//!
//! During folderization, migrated SSA repair modules live here first.
//! Non-`plan/` consumers should depend on this module first.

pub(in crate::mir::builder) mod exit_binding;
pub(in crate::mir::builder) mod exit_binding_applicator;
pub(in crate::mir::builder) mod exit_binding_constructor;
pub(in crate::mir::builder) mod exit_binding_validator;

#[allow(unused_imports)]
pub(in crate::mir::builder) use exit_binding::*;
