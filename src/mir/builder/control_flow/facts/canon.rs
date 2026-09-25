//! Facts-side owner surface for analysis-only canon helpers.
//!
//! Facts owns `cond` and `cond_block_view` directly; generic loop canon
//! lives in `control_flow/generic_loop_canon`.

pub(in crate::mir::builder) mod cond;
pub(in crate::mir::builder) mod cond_block_view;
