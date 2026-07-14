//! Function-owned Binding SSA lowering for the closed trivial profile.

mod identity;
mod lowerer;
mod operation;

pub(super) use lowerer::CanonicalTrivialSsaLowererV1;
