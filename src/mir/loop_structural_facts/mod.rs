//! Sealed resolved-Loop source to portable recipe-source boundary.
//!
//! See `README.md` for the authority and caller-zero contract.

mod resolved_source_adapter;

#[allow(unused_imports)]
pub(crate) use resolved_source_adapter::{
    bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1, VerifiedLoopRootSourceV1,
};

#[cfg(test)]
mod tests;
