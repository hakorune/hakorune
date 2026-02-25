//! CoreLoopFrame: Standard5 loop block structure with carrier PHI management.
//!
//! ## Design Principles
//! 1. Template does NOT lookup variable_map - carrier_inits is provided by pipeline
//! 2. StepBb mode only (HeaderBb mode is future)
//! 3. CFG wiring is pipeline responsibility (not template)

use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::normalizer::helpers::LoopBlocksStandard5;
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// Frame for Standard5 loop structure with carrier PHI management.
///
/// Contains block IDs and carrier PHI mappings needed for loop construction.
/// Pipeline provides `carrier_inits`; template allocates PHI destinations.
#[derive(Debug)]
pub(in crate::mir::builder) struct CoreLoopFrame {
    // Block IDs (Standard5 layout)
    pub preheader_bb: BasicBlockId,
    pub header_bb: BasicBlockId,
    pub body_bb: BasicBlockId,
    pub step_bb: BasicBlockId,
    pub after_bb: BasicBlockId,

    // Carrier PHI mappings (pipeline provides inits; template allocates PHI dsts)
    pub carrier_inits: BTreeMap<String, ValueId>,
    pub carrier_header_phis: BTreeMap<String, ValueId>,
    pub carrier_step_phis: BTreeMap<String, ValueId>,

    // Continue routing (StepBb fixed)
    pub continue_target: BasicBlockId,
}

/// Build a CoreLoopFrame with Standard5 blocks and carrier PHI allocations.
///
/// ## Arguments
/// - `builder`: MirBuilder for block/value allocation
/// - `carrier_vars`: Set of carrier variable names
/// - `carrier_inits`: Map of carrier variable -> init ValueId (pipeline provides this)
/// - `error_tag`: Error message prefix for diagnostics
///
/// ## Returns
/// `CoreLoopFrame` with all blocks allocated and PHI ValueIds assigned.
///
/// ## Contract
/// - `carrier_inits` must contain all variables in `carrier_vars`
/// - Template does NOT lookup from `variable_map` (pipeline responsibility)
/// - `continue_target` is always `step_bb` (StepBb mode)
pub(in crate::mir::builder) fn build_coreloop_frame(
    builder: &mut MirBuilder,
    carrier_vars: &BTreeSet<String>,
    carrier_inits: &BTreeMap<String, ValueId>,
    error_tag: &str,
) -> Result<CoreLoopFrame, String> {
    // Allocate Standard5 blocks
    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    // Build carrier PHI maps
    let mut carrier_header_phis = BTreeMap::new();
    let mut carrier_step_phis = BTreeMap::new();

    for var in carrier_vars {
        // Get init value (pipeline must provide this)
        let Some(&init_val) = carrier_inits.get(var) else {
            return Err(format!(
                "{error_tag}: carrier_inits missing variable '{}'",
                var
            ));
        };

        // Get type from init value
        let ty = builder
            .type_ctx
            .get_type(init_val)
            .cloned()
            .unwrap_or(MirType::Unknown);

        // Allocate header PHI destination
        let header_phi_dst = builder.alloc_typed(ty.clone());

        // Allocate step PHI destination (StepBb mode: separate ValueId)
        let step_phi_dst = builder.alloc_typed(ty);

        carrier_header_phis.insert(var.clone(), header_phi_dst);
        carrier_step_phis.insert(var.clone(), step_phi_dst);
    }

    // Fail-fast: no carriers is likely a bug
    if carrier_header_phis.is_empty() && !carrier_vars.is_empty() {
        return Err(format!("{error_tag}: carrier PHI allocation failed"));
    }

    Ok(CoreLoopFrame {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
        carrier_inits: carrier_inits.clone(),
        carrier_header_phis,
        carrier_step_phis,
        continue_target: step_bb, // StepBb mode: always step_bb
    })
}

/// Build PHI nodes for step_bb and header_bb (StepBb mode).
///
/// Returns PHIs in order: step PHIs first, then header PHIs.
///
/// ## PHI Structure (StepBb mode)
/// - step_bb PHI: inputs = `[]` (filled by ContinueWithPhiArgs during lowering)
/// - header_bb PHI: inputs = `[(preheader_bb, init), (step_bb, step_phi_dst)]`
///
/// ## Fail-Fast
/// Returns error if frame has empty carrier PHIs.
pub(in crate::mir::builder) fn build_header_step_phis(
    frame: &CoreLoopFrame,
    tag_prefix: &str,
) -> Result<Vec<CorePhiInfo>, String> {
    let mut phis = Vec::new();

    for (var, header_phi_dst) in &frame.carrier_header_phis {
        let Some(&init_val) = frame.carrier_inits.get(var) else {
            return Err(format!(
                "[coreloop_skeleton] {}: carrier_inits missing '{}' during PHI build",
                tag_prefix, var
            ));
        };

        let Some(&step_phi_dst) = frame.carrier_step_phis.get(var) else {
            return Err(format!(
                "[coreloop_skeleton] {}: carrier_step_phis missing '{}' during PHI build",
                tag_prefix, var
            ));
        };

        // Step PHI: empty inputs (filled by ContinueWithPhiArgs)
        phis.push(loop_carriers::build_step_join_phi_info(
            frame.step_bb,
            step_phi_dst,
            format!("{}_step_join_{}", tag_prefix, var),
        ));

        // Header PHI: preheader init + step update
        phis.push(loop_carriers::build_loop_phi_info(
            frame.header_bb,
            frame.preheader_bb,
            frame.step_bb,
            *header_phi_dst,
            init_val,
            step_phi_dst,
            format!("{}_carrier_{}", tag_prefix, var),
        ));
    }

    Ok(phis)
}
