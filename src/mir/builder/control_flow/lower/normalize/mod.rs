//! Lowering-side normalization helpers.

pub(in crate::mir::builder) mod canonicalize;

pub(in crate::mir::builder) use canonicalize::{canonicalize_loop_facts, CanonicalLoopFacts};
