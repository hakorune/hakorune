/*!
 * MIR optimizer entry surface.
 *
 * The implementation lives in `core.rs`; this file keeps the public module
 * surface small and stable.
 */

mod core;
mod diagnostics;
#[cfg(test)]
mod tests;

pub use core::{phase29x_opt_safeset, MirOptimizer, PHASE29X_OPT_SAFESET};
