//! Compatibility re-export while loop_scan_methods_v0 facts live under `facts/`.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::loop_scan_methods_v0::{
    try_extract_loop_scan_methods_v0_facts, LoopScanMethodsV0Facts,
};
