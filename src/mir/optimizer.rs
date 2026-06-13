/*!
 * MIR optimizer entry surface.
 *
 * The implementation lives in `core.rs`; this file keeps the public module
 * surface small and stable.
 */

mod core;
#[cfg(test)]
mod tests;

pub use core::{
    mir_opt_pipeline_groups, phase29x_opt_safeset, MirOptimizer, MIR_OPT_PIPELINE_GROUPS,
    PHASE29X_OPT_SAFESET,
};
