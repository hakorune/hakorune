//! Plan steps (neutral layer - SSOT).
//!
//! This module provides step helpers for Parts.
//! Goal: Single source of truth for all step helpers.

pub(in crate::mir::builder) mod carrier_collect;
pub(in crate::mir::builder) mod effects;
pub(in crate::mir::builder) mod join_payload;
pub(in crate::mir::builder) mod loop_wiring_standard5;
pub(in crate::mir::builder) mod stmt_block;

pub(in crate::mir::builder) use carrier_collect::collect_carrier_inits;
pub(in crate::mir::builder) use effects::effects_to_plans;
pub(in crate::mir::builder) use join_payload::{build_join_payload, build_join_payload_filtered};
pub(in crate::mir::builder) use loop_wiring_standard5::{
    build_standard5_internal_wires, empty_carriers_args,
};
pub(in crate::mir::builder) use stmt_block::lower_stmt_block;
