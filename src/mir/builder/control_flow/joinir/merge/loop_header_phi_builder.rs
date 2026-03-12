//! Loop Header PHI Builder
//!
//! Phase 33-16: Generates PHI nodes at loop header blocks to track carrier values.
//! Phase 33-17-B: Refactored to separate data structures from builder logic.
//!
//! ## Problem
//!
//! JoinIR uses function parameters (i_param, i_exit) to pass values between
//! iterations. When inlined into MIR, these parameters have no SSA definition.
//! This causes SSA-undef errors when exit PHIs reference remapped parameters.
//!
//! ## Solution
//!
//! Generate a PHI node at the loop header block for each carrier variable:
//!
//! ```text
//! loop_header:
//!   i_phi = PHI [(entry_block, i_init), (latch_block, i_next)]
//!   // rest of loop...
//! ```
//!
//! The PHI's dst becomes the "current value" of the carrier during iteration.
//! Exit paths should reference this PHI dst, not the undefined parameters.
//!
//! ## Usage
//!
//! Called from merge pipeline between Phase 3 (remap_values) and Phase 4
//! (instruction_rewriter).

use super::super::trace;
use super::dev_log;
use super::header_pred_policy;
use super::loop_header_phi_info::{CarrierPhiEntry, LoopHeaderPhiInfo};
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

/// Builder for loop header PHIs
///
/// Generates PHI nodes at the loop header block to track carrier values.
pub struct LoopHeaderPhiBuilder;

impl LoopHeaderPhiBuilder {
    /// Generate header PHIs for loop carriers
    ///
    /// # Arguments
    ///
    /// * `builder` - MirBuilder for allocating ValueIds
    /// * `header_block` - The loop header block ID
    /// * `entry_block` - The block that jumps to header on first iteration
    /// * `carrier_info` - Carrier variable metadata from pattern lowerer
    /// * `remapper` - ID remapper for looking up remapped init values
    /// * `debug` - Enable debug output
    ///
    /// # Returns
    ///
    /// LoopHeaderPhiInfo with allocated PHI dsts.
    /// Note: latch_incoming is not yet set - that happens in instruction_rewriter.
    ///
    /// # Phase 228 Update
    ///
    /// Added CarrierInit and CarrierRole to carrier tuples:
    /// * `CarrierInit::FromHost` - Use host_id directly as PHI init value
    /// * `CarrierInit::BoolConst(val)` - Generate explicit bool constant for ConditionOnly carriers
    /// * `CarrierInit::LoopLocalZero` - Generate const 0 for loop-local derived carriers (no host slot)
    ///
    /// # Phase 255 P2 Update
    ///
    /// Added `loop_invariants` parameter for variables that are referenced inside
    /// the loop body but do not change across iterations. These need header PHI nodes
    /// (with same value from all incoming edges) but do NOT need exit PHI nodes.
    pub fn build(
        builder: &mut crate::mir::builder::MirBuilder,
        header_block: BasicBlockId,
        entry_block: BasicBlockId,
        loop_var_name: &str,
        loop_var_init: ValueId,
        carriers: &[(
            String,
            ValueId,
            crate::mir::join_ir::lowering::carrier_info::CarrierInit,
            crate::mir::join_ir::lowering::carrier_info::CarrierRole,
        )], // Phase 228: Added CarrierInit and CarrierRole
        loop_invariants: &[(String, ValueId)], // Phase 255 P2: Loop invariant variables
        expr_result_is_loop_var: bool,
        debug: bool,
    ) -> Result<LoopHeaderPhiInfo, String> {
        let trace = trace::trace();
        if debug {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 33-16: Building header PHIs at {:?}",
                    header_block
                ),
                true,
            );
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir]   Loop var '{}' init={:?}, entry_block={:?}",
                    loop_var_name, loop_var_init, entry_block
                ),
                true,
            );
        }

        let mut info = LoopHeaderPhiInfo::empty(header_block);

        // Allocate PHI for loop variable
        let loop_var_phi_dst = builder.next_value_id();

        // Phase 72: Observe PHI dst allocation
        #[cfg(debug_assertions)]
        crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(loop_var_phi_dst);

        // Phase 131-11-H: Set PHI type from entry incoming (init value) only
        // Ignore backedge to avoid circular dependency in type inference
        if let Some(init_type) = builder.type_ctx.value_types.get(&loop_var_init).cloned() {
            builder
                .type_ctx
                .value_types
                .insert(loop_var_phi_dst, init_type.clone());

            trace.stderr_if(
                &format!(
                    "[carrier/phi] Loop var '{}': dst=%{} entry_type={:?} (backedge ignored)",
                    loop_var_name,
                    loop_var_phi_dst.as_u32(),
                    init_type
                ),
                debug || crate::config::env::builder_carrier_phi_debug(),
            );
        }

        info.carrier_phis.insert(
            loop_var_name.to_string(),
            CarrierPhiEntry {
                phi_dst: loop_var_phi_dst,
                entry_incoming: (entry_block, loop_var_init),
                latch_incoming: None,
                role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState, // Phase 227: Loop var is always LoopState
            },
        );
        // Phase 177-STRUCT-2: Record insertion order
        info.carrier_order.push(loop_var_name.to_string());

        if debug {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir]   Loop var PHI: {:?} = phi [(from {:?}, {:?}), (latch TBD)]",
                    loop_var_phi_dst, entry_block, loop_var_init
                ),
                true,
            );
        }

        // Allocate PHIs for other carriers
        for (name, host_id, init, role) in carriers {
            // Phase 86: Use centralized CarrierInit builder
            let init_value =
                super::carrier_init_builder::init_value(builder, &init, *host_id, &name, debug)?;

            let phi_dst = builder.next_value_id();

            // Phase 72: Observe PHI dst allocation
            #[cfg(debug_assertions)]
            crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(phi_dst);

            // Phase 131-11-H: Set PHI type from entry incoming (init value) only
            // Ignore backedge to avoid circular dependency in type inference
            if let Some(init_type) = builder.type_ctx.value_types.get(&init_value).cloned() {
                builder
                    .type_ctx
                    .value_types
                    .insert(phi_dst, init_type.clone());

                trace.stderr_if(
                    &format!(
                        "[carrier/phi] Carrier '{}': dst=%{} entry_type={:?} (backedge ignored)",
                        name,
                        phi_dst.as_u32(),
                        init_type
                    ),
                    debug || crate::config::env::builder_carrier_phi_debug(),
                );
            }

            info.carrier_phis.insert(
                name.clone(),
                CarrierPhiEntry {
                    phi_dst,
                    entry_incoming: (entry_block, init_value),
                    latch_incoming: None,
                    role: *role, // Phase 228: Use role from carrier_info
                },
            );
            // Phase 177-STRUCT-2: Record insertion order
            info.carrier_order.push(name.clone());

            if debug {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir]   Carrier '{}' PHI: {:?} = phi [(from {:?}, {:?}), (latch TBD)], role={:?}",
                        name, phi_dst, entry_block, init_value, role
                    ),
                    true,
                );
            }
        }

        // Phase 255 P2: Generate PHI nodes for loop invariants
        // Loop invariants need header PHI (same value from all edges) but NOT exit PHI
        for (name, host_id) in loop_invariants {
            let invariant_phi_dst = builder.next_value_id();

            // Phase 72: Observe PHI dst allocation
            #[cfg(debug_assertions)]
            crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(invariant_phi_dst);

            // Phase 131-11-H: Set PHI type from entry incoming (init value) only
            if let Some(init_type) = builder.type_ctx.value_types.get(host_id).cloned() {
                builder
                    .type_ctx
                    .value_types
                    .insert(invariant_phi_dst, init_type.clone());

                trace.stderr_if(
                    &format!(
                        "[carrier/phi] Loop invariant '{}': dst=%{} entry_type={:?} (same value all iterations)",
                        name,
                        invariant_phi_dst.as_u32(),
                        init_type
                    ),
                    debug || crate::config::env::builder_carrier_phi_debug(),
                );
            }

            // Register as CarrierPhiEntry with LoopState role
            // Note: latch_incoming will be set to the same PHI dst by instruction_rewriter
            info.carrier_phis.insert(
                name.clone(),
                CarrierPhiEntry {
                    phi_dst: invariant_phi_dst,
                    entry_incoming: (entry_block, *host_id),
                    latch_incoming: None, // Will be set to phi_dst (same value) by instruction_rewriter
                    role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                },
            );
            // Phase 177-STRUCT-2: Record insertion order
            info.carrier_order.push(name.clone());

            if debug {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir]   Loop invariant '{}' PHI: {:?} = phi [(from {:?}, {:?}), (latch same value)]",
                        name, invariant_phi_dst, entry_block, host_id
                    ),
                    true,
                );
            }
        }

        // Set expr_result if this pattern returns the loop var
        if expr_result_is_loop_var {
            info.expr_result_phi = Some(loop_var_phi_dst);
            if debug {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir]   expr_result = {:?} (loop var PHI)",
                        loop_var_phi_dst
                    ),
                    true,
                );
            }
        }

        Ok(info)
    }

    /// Finalize header PHIs by inserting them into the MIR block
    ///
    /// Called after instruction_rewriter has set all latch_incoming values.
    ///
    /// # Arguments
    ///
    /// * `builder` - MirBuilder containing the current function
    /// * `info` - LoopHeaderPhiInfo with all incoming edges set
    /// * `debug` - Enable debug output
    pub fn finalize(
        builder: &mut crate::mir::builder::MirBuilder,
        info: &LoopHeaderPhiInfo,
        debug: bool,
    ) -> Result<(), String> {
        let trace = trace::trace();
        let dev_debug = dev_log::dev_enabled(debug);
        if debug {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 33-16: Finalizing header PHIs at {:?}",
                    info.header_block
                ),
                true,
            );
        }

        // Validate all latch incoming are set
        for (name, entry) in &info.carrier_phis {
            if entry.latch_incoming.is_none() {
                return Err(format!(
                    "Phase 33-16: Carrier '{}' has no latch incoming set",
                    name
                ));
            }
        }

        // Phase 257 P1.2-FIX: Capture builder's current_block BEFORE getting mutable reference
        // This is the host entry block that will emit_jump to the loop header
        let host_entry_block_opt = builder.current_block;

        if dev_debug {
            trace.stderr_if(
                &format!(
                    "[joinir/header-phi] Host entry block (will jump to header): {:?}",
                    host_entry_block_opt
                ),
                true,
            );
        }

        // Get the header block from current function
        let current_func = builder
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or("Phase 33-16: No current function when finalizing header PHIs")?;

        // Phase 257 P1.2: Compute entry predecessor from CFG (correction, not just validation)
        use crate::mir::verification::utils::compute_predecessors;

        // Step 0: Update successors from terminators (instruction_rewriter sets terminator directly, bypassing set_terminator())
        // This ensures the CFG is ready for compute_predecessors()
        if dev_debug {
            trace.stderr_if(
                &format!(
                    "[joinir/header-phi] Step 0: Updating successors for {} blocks",
                    current_func.blocks.len()
                ),
                true,
            );
        }
        for (bid, block) in current_func.blocks.iter_mut() {
            if block.terminator.is_some() {
                block.successors.clear();
                if let Some(ref terminator) = block.terminator {
                    match terminator {
                        crate::mir::MirInstruction::Branch {
                            then_bb, else_bb, ..
                        } => {
                            block.successors.insert(*then_bb);
                            block.successors.insert(*else_bb);
                            if dev_debug {
                                trace.stderr_if(
                                    &format!(
                                        "[joinir/header-phi] Step 0: bb{} Branch → [{}, {}]",
                                        bid.0, then_bb.0, else_bb.0
                                    ),
                                    true,
                                );
                            }
                        }
                        crate::mir::MirInstruction::Jump { target, .. } => {
                            block.successors.insert(*target);
                            if dev_debug {
                                trace.stderr_if(
                                    &format!(
                                        "[joinir/header-phi] Step 0: bb{} Jump → bb{}",
                                        bid.0, target.0
                                    ),
                                    true,
                                );
                            }
                        }
                        _ => {
                            if dev_debug {
                                trace.stderr_if(
                                    &format!(
                                        "[joinir/header-phi] Step 0: bb{} has other terminator",
                                        bid.0
                                    ),
                                    true,
                                );
                            }
                        }
                    }
                }
            } else if dev_debug {
                trace.stderr_if(
                    &format!("[joinir/header-phi] Step 0: bb{} has NO terminator", bid.0),
                    true,
                );
            }
        }

        // Step 1: Compute CFG predecessors
        let preds = compute_predecessors(current_func);
        let header_preds = preds.get(&info.header_block).ok_or_else(|| {
            format!(
                "[loop_header_phi_builder] Loop header bb{} has no predecessors in CFG",
                info.header_block.0
            )
        })?;

        // Step 2: Identify latch block (all carriers must agree)
        let latch_block = {
            let mut latch_opt: Option<BasicBlockId> = None;
            for (name, entry) in &info.carrier_phis {
                let (carrier_latch, _) = entry.latch_incoming.ok_or_else(|| {
                    format!(
                        "[loop_header_phi_builder] Carrier '{}' missing latch_incoming",
                        name
                    )
                })?;

                if let Some(expected_latch) = latch_opt {
                    if carrier_latch != expected_latch {
                        return Err(format!(
                            "[loop_header_phi_builder] Latch block mismatch: carrier '{}' has bb{}, expected bb{}",
                            name, carrier_latch.0, expected_latch.0
                        ));
                    }
                } else {
                    latch_opt = Some(carrier_latch);
                }
            }
            latch_opt.ok_or_else(|| {
                "[loop_header_phi_builder] No carriers found (cannot determine latch)".to_string()
            })?
        };

        // Step 3: Compute entry/latch predecessors (SSOT).
        // Phase 257 P1.2-FIX: Multiple entry preds are OK (bb0 host + bb10 JoinIR main)
        let header_pred_groups = header_pred_policy::split_header_preds(
            info,
            header_preds,
            host_entry_block_opt,
            latch_block,
        );
        let entry_preds = header_pred_groups.entry_preds;
        let latch_preds = header_pred_groups.latch_preds;

        if dev_debug && header_pred_groups.host_entry_added {
            if let Some(host_entry_block) = host_entry_block_opt {
                trace.stderr_if(
                    &format!(
                        "[joinir/header-phi] Added host_entry_block bb{} to entry_preds (terminator not set yet)",
                        host_entry_block.0
                    ),
                    true,
                );
            }
        }

        // Step 4: Validate at least one entry predecessor
        if entry_preds.is_empty() {
            return Err(format!(
                "[loop_header_phi_builder] No entry predecessor found for header bb{} \
                 (all preds are latch bb{}). Hint: Check JoinIR merger - missing entry edge?",
                info.header_block.0, latch_block.0
            ));
        }

        if dev_debug {
            let host_desc =
                host_entry_block_opt.map_or_else(|| "None".to_string(), |bb| format!("bb{}", bb.0));
            trace.stderr_if(
                &format!(
                    "[joinir/header-phi] Entry predecessors: {:?} (latch=bb{}, host={}, total_preds={}, latch_preds={:?})",
                    entry_preds, latch_block.0, host_desc, header_preds.len(), latch_preds
                ),
                true,
            );
        }

        // Step 5: Validate no entry_pred is the header itself (catch CFG bug)
        for &entry_pred in &entry_preds {
            if entry_pred == info.header_block {
                return Err(format!(
                    "[loop_header_phi_builder] Entry predecessor bb{} is loop header itself (bb{}). \
                     This indicates a CFG construction bug (self-loop without latch).",
                    entry_pred.0, info.header_block.0
                ));
            }
        }

        let header_block = current_func
            .blocks
            .get_mut(&info.header_block)
            .ok_or_else(|| {
                format!(
                    "Phase 33-16: Header block {:?} not found in current function",
                    info.header_block
                )
            })?;

        // Insert PHIs at the beginning of the header block (before other instructions)
        // Sorted by carrier name for determinism
        let mut phi_instructions: Vec<MirInstruction> = Vec::new();

        // Step 6: Insert PHIs with inputs from ALL entry preds + latch
        // Phase 257 P1.2-FIX: Handle multiple entry predecessors (bb0 host + bb10 JoinIR main)
        for (name, entry) in &info.carrier_phis {
            let (_stored_entry_block, entry_val) = entry.entry_incoming; // Use value only
            let (_latch_block_stored, latch_val) = entry.latch_incoming.unwrap();

            // Build PHI inputs: entry preds use init value, latch preds use next value
            let mut phi_inputs = Vec::new();
            for &entry_pred in &entry_preds {
                phi_inputs.push((entry_pred, entry_val));
            }
            for &latch_pred in &latch_preds {
                phi_inputs.push((latch_pred, latch_val));
            }

            let phi = MirInstruction::Phi {
                dst: entry.phi_dst,
                inputs: phi_inputs.clone(),
                type_hint: None,
            };

            if crate::config::env::joinir_dev::debug_enabled() {
                let caller = std::panic::Location::caller();
                builder
                    .metadata_ctx
                    .record_value_caller(entry.phi_dst, caller);
                if let Some(loc) = builder
                    .metadata_ctx
                    .value_origin_callers()
                    .get(&entry.phi_dst)
                    .cloned()
                {
                    current_func
                        .metadata
                        .value_origin_callers
                        .insert(entry.phi_dst, loc);
                }
            }

            phi_instructions.push(phi);

            if dev_debug {
                let phi_desc = phi_inputs
                    .iter()
                    .map(|(bb, val)| format!("bb{} → {:?}", bb.0, val))
                    .collect::<Vec<_>>()
                    .join(", ");
                trace.stderr_if(
                    &format!(
                        "[joinir/header-phi]   Carrier '{}': phi {:?} = [{}]",
                        name, entry.phi_dst, phi_desc
                    ),
                    true,
                );
            }
        }

        // Prepend PHIs to existing instructions
        let mut new_instructions = phi_instructions;
        new_instructions.append(&mut header_block.instructions);
        header_block.instructions = new_instructions;

        // Also prepend spans for the PHIs
        let mut new_spans: Vec<crate::ast::Span> = (0..info.carrier_phis.len())
            .map(|_| crate::ast::Span::unknown())
            .collect();
        new_spans.append(&mut header_block.instruction_spans);
        header_block.instruction_spans = new_spans;

        if dev_debug {
            trace.stderr_if(
                &format!(
                    "[joinir/header-phi]   Header block now has {} instructions",
                    header_block.instructions.len()
                ),
                true,
            );
        }

        Ok(())
    }
}
