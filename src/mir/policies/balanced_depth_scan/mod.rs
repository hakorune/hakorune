//! Phase 107: Balanced depth-scan policy (json_cur find_balanced_* family)
//!
//! Responsibility (analysis only):
//! - Recognize the `depth` scan loop shape with nested-if + `return i`
//! - Produce a loop-break-compatible break condition + derived recipe inputs
//! - Fail-fast with tagged reasons when the shape is close but unsupported

mod ast_helpers;
mod classify;
mod extract;
mod types;

#[cfg(test)]
mod tests;

pub use classify::{
    classify_balanced_depth_scan_array_end, classify_balanced_depth_scan_object_end, decide,
};
pub use types::BalancedDepthScanPolicyResult;
