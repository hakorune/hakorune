//! Pass 4: PHI Sealing - Complete PHI Nodes with Latch Values
//!
//! This pass completes the PHI nodes created in Pass 3 by adding the latch
//! input (the backedge from loop body to header). It also handles continue
//! statements which create additional predecessors to the header.
//!
//! # Responsibilities
//!
//! 1. **Latch Value Discovery**: Find the final value of each variable at the latch
//! 2. **PHI Input Completion**: Update incomplete PHI nodes with:
//!    - Preheader input (already set in Pass 3)
//!    - Continue inputs (from continue statements)
//!    - Latch input (backedge)
//! 3. **PHI Optimization**: Skip PHI update if all inputs have same value
//! 4. **Header Bypass**: Skip PHI updates in JoinIR experimental path
//!
//! # Algorithm
//!
//! For each variable (pinned and carrier):
//! 1. Collect all PHI inputs:
//!    - Preheader: `preheader_copy`
//!    - Continues: Variable values at continue blocks
//!    - Latch: Variable value at latch block
//! 2. Sanitize inputs: Remove duplicates using BTreeMap
//! 3. Optimize: If all values are same, skip PHI update
//! 4. Update: Call `update_phi_inputs()` with complete input list
//!
//! # Example
//!
//! ```ignore
//! // After Pass 3 (incomplete PHI):
//! // r4 = phi [r3, @preheader]    // carrier "x"
//!
//! // Loop body execution:
//! // @body:
//! //   r10 = r4 + 1               // x = x + 1
//! //   jump @latch
//! // @latch:
//! //   jump @header                // backedge
//!
//! builder.seal_phis(&mut ops, latch_id, &[], &HashSet::new(), false)?;
//!
//! // After Pass 4 (complete PHI):
//! // r4 = phi [r3, @preheader], [r10, @latch]
//! ```

use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

use crate::mir::phi_core::loopform::builder_core::{LoopFormBuilder, LoopFormOps};
use crate::mir::phi_core::loopform::utils::is_loopform_debug_enabled;

/// Pass 4: Seal PHI nodes after loop body lowering
///
/// Completes PHI nodes with latch + continue inputs, converting them from:
///   phi [preheader_val, preheader]
/// to:
///   phi [preheader_val, preheader], [continue_val, continue_bb]..., [latch_val, latch]
///
/// # Parameters
/// - `latch_id`: The block that closes the canonical backedge to `header`.
/// - `continue_snapshots`: Per-`continue` block variable snapshots.
///   Each entry represents a predecessor of `header` created by `continue`.
/// - `_writes`: Variables modified in loop body (Step 5-1: 選択肢2)
///   Used to distinguish true carriers from loop-invariant variables
///   (Currently unused - PHI optimization uses optimize_same_value() instead)
/// - `header_bypass`: Phase 27.4C: Header φ バイパスフラグ
///   true の場合、Header φ 生成がスキップされているため、φ lookup も行わない
pub fn seal_phis<O: LoopFormOps>(
    builder: &mut LoopFormBuilder,
    ops: &mut O,
    latch_id: BasicBlockId,
    continue_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
    _writes: &std::collections::HashSet<String>,
    header_bypass: bool,
) -> Result<(), String> {
    let debug = std::env::var("NYASH_LOOPFORM_DEBUG").is_ok();

    if debug {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[loopform/seal_phis] header={:?} preheader={:?} latch={:?} continue_snapshots={}",
            builder.header_id,
            builder.preheader_id,
            latch_id,
            continue_snapshots.len()
        ));
    }

    // Phase 27.4C Refactor: Delegate to specialized methods
    seal_pinned_phis(builder, ops, latch_id, continue_snapshots, header_bypass)?;
    seal_carrier_phis(builder, ops, latch_id, continue_snapshots, header_bypass)?;

    Ok(())
}

/// Phase 27.4C Refactor: Seal Pinned 変数 PHIs
///
/// Pinned 変数（ループ不変パラメータ）の PHI ノード入力を finalize する。
/// Header φ バイパス時は φ lookup をスキップし、preheader_copy をそのまま使用。
fn seal_pinned_phis<O: LoopFormOps>(
    builder: &LoopFormBuilder,
    ops: &mut O,
    latch_id: BasicBlockId,
    continue_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
    header_bypass: bool,
) -> Result<(), String> {
    let debug = is_loopform_debug_enabled();

    for pinned in &builder.pinned {
        if header_bypass {
            // Phase 27.4C: JoinIR 実験経路では Pinned 変数の φ lookup をスキップ
            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/seal_phis/27.4C] SKIP pinned '{}' phi update (header bypass active)",
                    pinned.name
                ));
            }
            continue;
        }

        // ========================================
        // Phase 59: PhiInputCollector インライン化
        // ========================================
        // Step 1: 入力収集
        let mut raw_inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
        raw_inputs.push((builder.preheader_id, pinned.preheader_copy));

        for (cid, snapshot) in continue_snapshots {
            if let Some(&value) = snapshot.get(&pinned.name) {
                raw_inputs.push((*cid, value));
            }
        }

        let latch_value = ops
            .get_variable_at_block(&pinned.name, latch_id)
            .unwrap_or(pinned.header_phi);
        raw_inputs.push((latch_id, latch_value));

        // Step 2: sanitize (BTreeMapで重複削除＆ソート)
        let mut sanitized: std::collections::BTreeMap<BasicBlockId, ValueId> =
            std::collections::BTreeMap::new();
        for (bb, val) in &raw_inputs {
            sanitized.insert(*bb, *val);
        }
        let inputs: Vec<(BasicBlockId, ValueId)> = sanitized.into_iter().collect();

        // Step 3: optimize_same_value
        let same_value = if inputs.is_empty() {
            None
        } else if inputs.len() == 1 {
            Some(inputs[0].1)
        } else {
            let first_val = inputs[0].1;
            if inputs.iter().all(|(_, val)| *val == first_val) {
                Some(first_val)
            } else {
                None
            }
        };

        if let Some(same_val) = same_value {
            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/seal_phis] OPTIMIZED pinned '{}': phi={:?} → same_value={:?} (loop-invariant)",
                    pinned.name, pinned.header_phi, same_val
                ));
            }
            continue;
        }

        if debug {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[loopform/seal_phis] pinned '{}' phi={:?} inputs={:?}",
                pinned.name, pinned.header_phi, inputs
            ));
        }

        ops.update_phi_inputs(builder.header_id, pinned.header_phi, inputs)?;
    }

    Ok(())
}

/// Phase 27.4C Refactor: Seal Carrier 変数 PHIs
///
/// Carrier 変数（ループ内変数）の PHI ノード入力を finalize する。
/// Header φ バイパス時は φ lookup をスキップし、preheader_copy をそのまま使用。
fn seal_carrier_phis<O: LoopFormOps>(
    builder: &mut LoopFormBuilder,
    ops: &mut O,
    latch_id: BasicBlockId,
    continue_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
    header_bypass: bool,
) -> Result<(), String> {
    let debug = is_loopform_debug_enabled();

    for carrier in &mut builder.carriers {
        if header_bypass {
            // Phase 27.4C: JoinIR 実験経路では Carrier 変数の φ lookup をスキップ
            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/seal_phis/27.4C] SKIP carrier '{}' phi update (header bypass active)",
                    carrier.name
                ));
            }
            continue;
        }

        carrier.latch_value = ops
            .get_variable_at_block(&carrier.name, latch_id)
            .ok_or_else(|| {
                format!(
                    "carrier '{}' not found at latch {:?}",
                    carrier.name, latch_id
                )
            })?;

        // ========================================
        // Phase 59: PhiInputCollector インライン化
        // ========================================
        // Step 1: 入力収集
        let mut raw_inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
        raw_inputs.push((builder.preheader_id, carrier.preheader_copy));

        for (cid, snapshot) in continue_snapshots {
            if let Some(&value) = snapshot.get(&carrier.name) {
                raw_inputs.push((*cid, value));
            }
        }

        raw_inputs.push((latch_id, carrier.latch_value));

        // Step 2: sanitize (BTreeMapで重複削除＆ソート)
        let mut sanitized: std::collections::BTreeMap<BasicBlockId, ValueId> =
            std::collections::BTreeMap::new();
        for (bb, val) in &raw_inputs {
            sanitized.insert(*bb, *val);
        }
        let inputs: Vec<(BasicBlockId, ValueId)> = sanitized.into_iter().collect();

        // Step 3: optimize_same_value
        let same_value = if inputs.is_empty() {
            None
        } else if inputs.len() == 1 {
            Some(inputs[0].1)
        } else {
            let first_val = inputs[0].1;
            if inputs.iter().all(|(_, val)| *val == first_val) {
                Some(first_val)
            } else {
                None
            }
        };

        if let Some(same_val) = same_value {
            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/seal_phis] OPTIMIZED carrier '{}': phi={:?} → same_value={:?} (misclassified as carrier, actually loop-invariant)",
                    carrier.name, carrier.header_phi, same_val
                ));
            }
            continue;
        }

        if debug {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[loopform/seal_phis] carrier '{}' phi={:?} inputs={:?}",
                carrier.name, carrier.header_phi, inputs
            ));
        }

        ops.update_phi_inputs(builder.header_id, carrier.header_phi, inputs)?;
    }

    Ok(())
}
