//! Facts-side owner surface for analysis-only canon helpers.
//!
//! Facts owns `cond`, `cond_block_view`, and `generic_loop` directly.

pub(in crate::mir::builder) mod cond;
pub(in crate::mir::builder) mod cond_block_view;
pub(in crate::mir::builder) mod generic_loop;
