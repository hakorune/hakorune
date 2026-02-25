//! JoinIR Exit PHI Builder
//!
//! Constructs the exit block PHI nodes that merge return values
//! from all inlined JoinIR functions.
//!
//! Phase 4 Extraction: Separated from merge_joinir_mir_blocks (lines 581-615)
//! Phase 33-13: Extended to support carrier PHIs for multi-carrier loops

use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Phase 5: Create exit block with PHI for return values and carrier values
///
/// Phase 189-Fix: Generate exit PHI if there are multiple return values.
/// Phase 33-13: Also generates PHI for each carrier variable.
///
/// Returns:
/// - Option<ValueId>: The expr result PHI dst (if any return values)
/// - BTreeMap<String, ValueId>: Carrier name → PHI dst mapping
pub(super) fn build_exit_phi(
    builder: &mut crate::mir::builder::MirBuilder,
    exit_block_id: BasicBlockId,
    exit_phi_inputs: &[(BasicBlockId, ValueId)],
    carrier_inputs: &BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,
    debug: bool,
) -> Result<(Option<ValueId>, BTreeMap<String, ValueId>), String> {
    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
    let verbose = debug || crate::config::env::joinir_dev_enabled();
    let mut carrier_phis: BTreeMap<String, ValueId> = BTreeMap::new();

    let exit_phi_result_id = if let Some(ref mut func) = builder.scope_ctx.current_function {
        let mut exit_block = BasicBlock::new(exit_block_id);

        // Phase 189-Fix: If we collected return values, create a PHI in exit block
        // This merges all return values from JoinIR functions into a single value
        let phi_result = if !exit_phi_inputs.is_empty() {
            // Phase 132-P2: Use function-level next_value_id() to allocate
            // Previously used builder.value_gen.next() which is module-level, causing ValueId collisions
            // Note: We use func.next_value_id() directly since builder.current_function is already borrowed
            let phi_dst = func.next_value_id();
            if crate::config::env::joinir_dev::debug_enabled() {
                let caller = std::panic::Location::caller();
                builder.metadata_ctx.record_value_caller(phi_dst, caller);
                if let Some(loc) = builder
                    .metadata_ctx
                    .value_origin_callers()
                    .get(&phi_dst)
                    .cloned()
                {
                    func.metadata.value_origin_callers.insert(phi_dst, loc);
                }
            }
            exit_block.instructions.push(MirInstruction::Phi {
                dst: phi_dst,
                inputs: exit_phi_inputs.to_vec(),
                type_hint: None,
            });
            exit_block
                .instruction_spans
                .push(crate::ast::Span::unknown());
            if debug {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir]   Exit block PHI (expr result): {:?} = phi {:?}",
                        phi_dst, exit_phi_inputs
                    ),
                    true,
                );
            }
            Some(phi_dst)
        } else {
            None
        };

        // Phase 33-13: Create PHI for each carrier variable
        // This ensures that carrier exit values are properly merged when
        // there are multiple paths to the exit block
        for (carrier_name, inputs) in carrier_inputs {
            if inputs.is_empty() {
                continue;
            }

            // Phase 132-P2: Use function-level next_value_id() to allocate
            // Previously used builder.value_gen.next() which is module-level, causing ValueId collisions
            // Note: We use func.next_value_id() directly since builder.current_function is already borrowed
            let phi_dst = func.next_value_id();
            if crate::config::env::joinir_dev::debug_enabled() {
                let caller = std::panic::Location::caller();
                builder.metadata_ctx.record_value_caller(phi_dst, caller);
                if let Some(loc) = builder
                    .metadata_ctx
                    .value_origin_callers()
                    .get(&phi_dst)
                    .cloned()
                {
                    func.metadata.value_origin_callers.insert(phi_dst, loc);
                }
            }
            exit_block.instructions.push(MirInstruction::Phi {
                dst: phi_dst,
                inputs: inputs.clone(),
                type_hint: None,
            });
            exit_block
                .instruction_spans
                .push(crate::ast::Span::unknown());

            carrier_phis.insert(carrier_name.clone(), phi_dst);

            // DEBUG-177: Exit block PHI creation for carrier debugging
            trace.stderr_if(
                &format!(
                    "[DEBUG-177] Exit block PHI (carrier '{}'): {:?} = phi {:?}",
                    carrier_name, phi_dst, inputs
                ),
                verbose,
            );
        }

        func.add_block(exit_block);
        if debug {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir]   Created exit block: {:?} with {} carrier PHIs",
                    exit_block_id,
                    carrier_phis.len()
                ),
                true,
            );
        }
        phi_result
    } else {
        None
    };

    Ok((exit_phi_result_id, carrier_phis))
}
