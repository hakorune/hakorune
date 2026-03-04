//! Phase 29ai P0: Facts normalization skeleton (pure transforms)

pub(in crate::mir::builder) mod canonicalize;

pub(in crate::mir::builder) use canonicalize::{canonicalize_loop_facts, CanonicalLoopFacts};
