//! Function-owned Binding SSA lowering for the closed trivial profile.

mod identity;
mod lowerer;
mod operation;
mod parameter_entry;

pub(super) use lowerer::CanonicalTrivialSsaLowererV1;
