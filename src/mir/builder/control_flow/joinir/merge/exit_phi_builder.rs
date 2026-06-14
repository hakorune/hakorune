//! JoinIR Exit PHI Builder
//!
//! Constructs the exit block PHI nodes that merge return values
//! from all inlined JoinIR functions.
//!
//! Phase 4 Extraction: Separated from merge_joinir_mir_blocks (lines 581-615)
//! Phase 33-13: Extended to support carrier PHIs for multi-carrier loops

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::{BasicBlock, BasicBlockId, ValueId};
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

    if builder.scope_ctx.current_function.is_none() {
        return Ok((None, carrier_phis));
    }

    if let Some(ref mut func) = builder.scope_ctx.current_function {
        func.add_block(BasicBlock::new(exit_block_id));
    }

    let mut txn = PhiTxn::begin("joinir_exit_phi_builder");
    let result = build_exit_phi_with_txn(
        builder,
        &mut txn,
        exit_block_id,
        exit_phi_inputs,
        carrier_inputs,
        debug,
        verbose,
        &trace,
        &mut carrier_phis,
    );

    match result {
        Ok(exit_phi_result_id) => {
            txn.commit()?;
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
            Ok((exit_phi_result_id, carrier_phis))
        }
        Err(err) => match txn.abort_on_err(builder, err) {
            Err(abort_err) => Err(abort_err),
            Ok(()) => unreachable!("PhiTxn::abort_on_err returns Err"),
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn build_exit_phi_with_txn(
    builder: &mut crate::mir::builder::MirBuilder,
    txn: &mut PhiTxn,
    exit_block_id: BasicBlockId,
    exit_phi_inputs: &[(BasicBlockId, ValueId)],
    carrier_inputs: &BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,
    debug: bool,
    verbose: bool,
    trace: &crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace,
    carrier_phis: &mut BTreeMap<String, ValueId>,
) -> Result<Option<ValueId>, String> {
    // Phase 189-Fix: If we collected return values, create a PHI in exit block.
    // This merges all return values from JoinIR functions into a single value.
    let phi_result = if !exit_phi_inputs.is_empty() {
        let phi_dst = next_exit_phi_dst(builder, "expr_result")?;
        let token = txn.define_provisional_phi(
            builder,
            exit_block_id,
            phi_dst,
            "joinir_exit_phi:expr_result_define",
        )?;
        txn.patch_phi_inputs(
            builder,
            token,
            exit_phi_inputs.to_vec(),
            "joinir_exit_phi:expr_result_patch",
        )?;
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

    // Phase 33-13: Create PHI for each carrier variable.
    // This ensures that carrier exit values are properly merged when
    // there are multiple paths to the exit block.
    for (carrier_name, inputs) in carrier_inputs {
        if inputs.is_empty() {
            continue;
        }

        let phi_dst = next_exit_phi_dst(builder, carrier_name)?;
        let token = txn.define_provisional_phi(
            builder,
            exit_block_id,
            phi_dst,
            "joinir_exit_phi:carrier_define",
        )?;
        txn.patch_phi_inputs(
            builder,
            token,
            inputs.clone(),
            "joinir_exit_phi:carrier_patch",
        )?;

        carrier_phis.insert(carrier_name.clone(), phi_dst);

        // DEBUG-177: Exit block PHI creation for carrier debugging.
        trace.stderr_if(
            &format!(
                "[DEBUG-177] Exit block PHI (carrier '{}'): {:?} = phi {:?}",
                carrier_name, phi_dst, inputs
            ),
            verbose,
        );
    }

    Ok(phi_result)
}

fn next_exit_phi_dst(
    builder: &mut crate::mir::builder::MirBuilder,
    tag: &str,
) -> Result<ValueId, String> {
    let func = builder
        .scope_ctx
        .current_function
        .as_mut()
        .ok_or_else(|| format!("[freeze:contract][joinir_exit_phi/no_function] tag={tag}"))?;
    Ok(func.next_value_id())
}
