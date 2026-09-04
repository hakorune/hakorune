//! Top-level descriptive extractors.
//!
//! Route-local helpers use the grouped `generic_loop_canon` owner directly;
//! the former plan-side forwarding shelf has been retired.

pub(in crate::mir::builder) mod common_helpers;
pub(in crate::mir::builder) mod if_phi_join;
pub(in crate::mir::builder) mod loop_simple_while;
