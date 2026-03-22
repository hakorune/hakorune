//! Phase 29z P0: RC insertion pass - Minimal overwrite release
//!
//! Facade module. The implementation lives in `rc_insertion_helpers`.

use crate::mir::MirModule;

pub use super::rc_insertion_helpers::RcInsertionStats;

#[cfg(not(feature = "rc-insertion-minimal"))]
pub fn insert_rc_instructions(module: &mut MirModule) -> RcInsertionStats {
    let mut stats = RcInsertionStats::default();
    for (_name, func) in &module.functions {
        stats.functions_processed += 1;
        stats.blocks_visited += func.blocks.len();
    }
    stats
}

#[cfg(feature = "rc-insertion-minimal")]
pub use super::rc_insertion_helpers::insert_rc_instructions;
