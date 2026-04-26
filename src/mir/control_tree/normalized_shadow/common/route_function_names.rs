//! Normalized-shadow route-local JoinFunction names.
//!
//! These are not global JoinIR canonical names. They are local continuation
//! roles shared by normalized-shadow route builders.

pub(crate) const JOIN_K: &str = "join_k";
pub(crate) const K_THEN: &str = "k_then";
pub(crate) const K_ELSE: &str = "k_else";
pub(crate) const LOOP_COND_CHECK: &str = "loop_cond_check";
