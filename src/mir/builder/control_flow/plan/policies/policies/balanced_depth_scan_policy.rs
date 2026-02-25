//! Phase 107: Balanced depth-scan policy (SSOT wrapper)
//!
//! Re-export SSOT from `src/mir/policies/` so joinir code stays stable.

#[allow(unused_imports)]
pub(crate) use crate::mir::policies::balanced_depth_scan::{
    BalancedDepthScanPolicyResult, classify_balanced_depth_scan_array_end,
    classify_balanced_depth_scan_object_end,
};
