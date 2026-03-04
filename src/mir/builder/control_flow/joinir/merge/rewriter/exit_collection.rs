//! Exit value collection utilities for instruction rewriter
//!
//! Phase 260 P0.1 Step 6: Extracted from instruction_rewriter.rs
//! Handles Return→Jump conversion and exit value collection using ExitArgsCollectorBox.
//!
//! ## Responsibilities
//!
//! 1. Collect exit values from terminator edge-args (Phase 246-EX)
//! 2. Populate exit_phi_inputs and carrier_inputs
//! 3. Handle fallback path when no jump_args present
//!
//! ## Logging
//!
//! These functions do not log internally. The caller (instruction_rewriter.rs)
//! should handle logging with its local log! macro context.

use crate::mir::builder::control_flow::joinir::merge::exit_args_collector::ExitArgsCollectorBox;
use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, ExitReconnectMode};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, EdgeArgs, ValueId};
use std::collections::BTreeMap;

use super::super::loop_header_phi_info::LoopHeaderPhiInfo;

/// Result of exit value collection
#[derive(Debug)]
pub(in crate::mir::builder::control_flow::joinir::merge) struct ExitCollectionResult {
    /// EdgeArgs to use for the exit jump (remapped to HOST value space)
    pub edge_args: Option<EdgeArgs>,
    /// Whether collection succeeded (used for control flow)
    pub collected: bool,
    /// Collected expr_result (if any) - caller should add to exit_phi_inputs
    pub expr_result: Option<(BasicBlockId, ValueId)>,
    /// Collected carrier values - caller should add to carrier_inputs
    /// Format: Vec<(carrier_name, (block_id, value_id))> - allows multiple values per carrier
    pub carrier_values: Vec<(String, (BasicBlockId, ValueId))>,
}

/// Collect exit values from terminator edge-args
///
/// Phase 246-EX: Uses legacy jump_args metadata to recover all original Jump args.
/// Remaps values from JoinIR value space to HOST value space.
///
/// # Arguments
///
/// * `old_block` - Source block with terminator edge-args (legacy hidden inside API)
/// * `boundary` - JoinInlineBoundary with exit_bindings and layout info
/// * `remapper` - JoinIR→HOST value remapper
/// * `new_block_id` - Target block ID in HOST MIR
/// * `strict_exit` - Whether to enforce strict exit value contract
///
/// # Returns
///
/// * `Ok(ExitCollectionResult)` - Collection result with edge_args if found
/// * `Err(String)` - Error if strict mode fails
pub(in crate::mir::builder::control_flow::joinir::merge) fn collect_exit_values_from_edge_args(
    old_block: &crate::mir::BasicBlock,
    boundary: &JoinInlineBoundary,
    remapper: &crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper,
    new_block_id: BasicBlockId,
    strict_exit: bool,
) -> Result<ExitCollectionResult, String> {
    // Phase 246-EX: Read edge-args via BasicBlock API (legacy-only is hidden inside)
    if let Some(edge_args) = old_block.edge_args_from_terminator() {
        if edge_args.layout != boundary.jump_args_layout {
            let msg = format!(
                "[joinir/exit-line] edge-args layout mismatch: block={:?} edge={:?} boundary={:?}",
                old_block.id, edge_args.layout, boundary.jump_args_layout
            );
            if strict_exit {
                return Err(msg);
            } else {
                if crate::config::env::is_joinir_debug() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[DEBUG-177] {}", msg));
                }
            }
        }
        // The jump_args are in JoinIR value space, remap them to HOST
        let remapped_args: Vec<ValueId> = edge_args
            .values
            .iter()
            .map(|&arg| remapper.remap_value(arg))
            .collect();

        // Phase 118 P2: Use ExitArgsCollectorBox to collect exit values
        let result_edge_args = EdgeArgs {
            layout: edge_args.layout,
            values: remapped_args,
        };

        let collector = ExitArgsCollectorBox::new();
        let collection_result = collector.collect(
            &boundary.exit_bindings,
            &result_edge_args.values,
            new_block_id,
            strict_exit,
            result_edge_args.layout,
        )?;

        // Build result
        let expr_result = collection_result
            .expr_result_value
            .map(|v| (new_block_id, v));

        Ok(ExitCollectionResult {
            edge_args: Some(result_edge_args),
            collected: true,
            expr_result,
            carrier_values: collection_result.carrier_values,
        })
    } else {
        Ok(ExitCollectionResult {
            edge_args: None,
            collected: false,
            expr_result: None,
            carrier_values: Vec::new(),
        })
    }
}

/// Add carrier values from collection result to carrier_inputs map
///
/// # Arguments
///
/// * `carrier_inputs` - Output map to populate
/// * `carrier_values` - Collected carrier values: Vec<(carrier_name, (block_id, value_id))>
pub(in crate::mir::builder::control_flow::joinir::merge) fn add_carrier_values_to_inputs(
    carrier_inputs: &mut BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,
    carrier_values: &[(String, (BasicBlockId, ValueId))],
) {
    for (carrier_name, (block_id, value_id)) in carrier_values {
        carrier_inputs
            .entry(carrier_name.clone())
            .or_insert_with(Vec::new)
            .push((*block_id, *value_id));
    }
}

/// Handle fallback path when block has no jump_args metadata
///
/// Phase 246-EX: Uses header PHI dst (old behavior) for blocks without jump_args.
/// Phase 131 P1.5: For DirectValue mode, uses host_slot as fallback.
///
/// # Arguments
///
/// * `boundary` - JoinInlineBoundary with exit_bindings and loop_var_name
/// * `loop_header_phi_info` - Info about loop header PHI values
/// * `new_block_id` - Target block ID in HOST MIR
///
/// # Returns
///
/// * `exit_phi_input` - Optional (block_id, value_id) for exit PHI
/// * `carrier_values` - Map of carrier_name → (block_id, value_id) pairs
pub(in crate::mir::builder::control_flow::joinir::merge) fn handle_fallback_exit_collection(
    boundary: &JoinInlineBoundary,
    loop_header_phi_info: &LoopHeaderPhiInfo,
    new_block_id: BasicBlockId,
) -> (Option<(BasicBlockId, ValueId)>, BTreeMap<String, (BasicBlockId, ValueId)>) {
    let mut exit_phi_input = None;
    let mut carrier_values = BTreeMap::new();

    // Try to use loop_var_name's header PHI for exit_phi_inputs
    if let Some(loop_var_name) = &boundary.loop_var_name {
        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(loop_var_name) {
            exit_phi_input = Some((new_block_id, phi_dst));
        }
    }

    // Phase 227: Filter out ConditionOnly carriers from exit PHI
    // Phase 131 P1.5: For DirectValue mode, if no header PHI, use host_slot (initial value)
    for binding in &boundary.exit_bindings {
        if binding.role == CarrierRole::ConditionOnly {
            continue;
        }

        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(&binding.carrier_name) {
            carrier_values.insert(binding.carrier_name.clone(), (new_block_id, phi_dst));
        } else if boundary.exit_reconnect_mode == ExitReconnectMode::DirectValue {
            // Phase 131 P1.5: DirectValue mode fallback - use host_slot (initial value)
            // This handles k_exit blocks that don't have jump_args and no header PHI
            carrier_values.insert(
                binding.carrier_name.clone(),
                (new_block_id, binding.host_slot),
            );
        }
    }

    (exit_phi_input, carrier_values)
}

/// Collect exit values for k_exit tail call lowering
///
/// Phase 131: Handles the k_exit tail-call normalization path.
/// Similar to collect_exit_values_from_edge_args but uses k_exit_edge_args.
///
/// # Arguments
///
/// * `boundary` - JoinInlineBoundary with exit_bindings and layout info
/// * `k_exit_edge_args` - EdgeArgs from k_exit block (optional)
/// * `fallback_args` - Fallback args if k_exit_edge_args is None
/// * `remapper` - JoinIR→HOST value remapper
/// * `new_block_id` - Target block ID in HOST MIR
/// * `strict_exit` - Whether to enforce strict exit value contract
///
/// # Returns
///
/// * `Ok((Some(EdgeArgs), expr_result, carrier_values))` - Collected values
/// * `Ok((None, None, empty))` - No boundary (non-strict mode)
/// * `Err(String)` - Error in strict mode without boundary
pub(in crate::mir::builder::control_flow::joinir::merge) fn collect_k_exit_values(
    boundary: Option<&JoinInlineBoundary>,
    k_exit_edge_args: Option<&EdgeArgs>,
    fallback_args: &[ValueId],
    remapper: &crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper,
    new_block_id: BasicBlockId,
    strict_exit: bool,
) -> Result<
    (
        Option<EdgeArgs>,
        Option<(BasicBlockId, ValueId)>,
        Vec<(String, (BasicBlockId, ValueId))>,
    ),
    String,
> {
    if let Some(b) = boundary {
        let collector = ExitArgsCollectorBox::new();
        let exit_values: Vec<ValueId> = if let Some(edge_args) = k_exit_edge_args {
            edge_args
                .values
                .iter()
                .map(|&arg| remapper.remap_value(arg))
                .collect()
        } else {
            fallback_args.to_vec()
        };

        let edge_args = EdgeArgs {
            layout: b.jump_args_layout,
            values: exit_values,
        };

        let collection_result = collector.collect(
            &b.exit_bindings,
            &edge_args.values,
            new_block_id,
            strict_exit,
            edge_args.layout,
        )?;

        let expr_result = collection_result
            .expr_result_value
            .map(|v| (collection_result.block_id, v));

        Ok((Some(edge_args), expr_result, collection_result.carrier_values))
    } else if strict_exit {
        Err(crate::mir::join_ir::lowering::error_tags::freeze_with_hint(
            "phase131/k_exit/no_boundary",
            "k_exit tail call detected without JoinInlineBoundary",
            "k_exit must be handled as fragment exit; ensure boundary is passed when merging JoinIR fragments",
        ))
    } else {
        Ok((None, None, Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_carrier_values_to_inputs_empty() {
        let mut carrier_inputs = BTreeMap::new();
        let carrier_values: Vec<(String, (BasicBlockId, ValueId))> = Vec::new();

        add_carrier_values_to_inputs(&mut carrier_inputs, &carrier_values);

        assert!(carrier_inputs.is_empty());
    }

    #[test]
    fn test_add_carrier_values_to_inputs_single() {
        let mut carrier_inputs = BTreeMap::new();
        let carrier_values = vec![(
            "sum".to_string(),
            (BasicBlockId::new(1), ValueId::new(42)),
        )];

        add_carrier_values_to_inputs(&mut carrier_inputs, &carrier_values);

        assert_eq!(carrier_inputs.len(), 1);
        assert_eq!(
            carrier_inputs.get("sum"),
            Some(&vec![(BasicBlockId::new(1), ValueId::new(42))])
        );
    }

    #[test]
    fn test_add_carrier_values_to_inputs_multiple_same_carrier() {
        let mut carrier_inputs = BTreeMap::new();
        carrier_inputs.insert(
            "sum".to_string(),
            vec![(BasicBlockId::new(0), ValueId::new(10))],
        );

        let carrier_values = vec![(
            "sum".to_string(),
            (BasicBlockId::new(1), ValueId::new(42)),
        )];

        add_carrier_values_to_inputs(&mut carrier_inputs, &carrier_values);

        assert_eq!(carrier_inputs.len(), 1);
        let sum_inputs = carrier_inputs.get("sum").unwrap();
        assert_eq!(sum_inputs.len(), 2);
        assert_eq!(sum_inputs[0], (BasicBlockId::new(0), ValueId::new(10)));
        assert_eq!(sum_inputs[1], (BasicBlockId::new(1), ValueId::new(42)));
    }
}
