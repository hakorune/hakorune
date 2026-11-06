//! Generic-loop step extract owner.
//!
//! This module owns step increment extraction order. Keep the order stable:
//! legacy helper fast path, then var-step, next-step, and complex-step probes.


pub(in crate::mir::builder) mod complex_step;
pub(in crate::mir::builder) mod next_step;
pub(in crate::mir::builder) mod shared;
pub(in crate::mir::builder) mod var_step;
