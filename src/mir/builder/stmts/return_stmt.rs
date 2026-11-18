//! Return instruction emission and protected-region completion.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::{
    ensure_cleanup_exit_allowed_v1, CleanupExitKindV1,
};
use crate::mir::builder::control_flow::plan::facts::match_return_facts::check_match_return_source;
use crate::mir::{MirBuilder, MirInstruction, ValueId};

pub(in crate::mir::builder) fn check_match_return_before_value(
    builder: &MirBuilder,
    value: &ASTNode,
) -> Result<(), String> {
    if builder.function_state.protected_region.return_defer.is_active() {
        return Ok(());
    }
    check_match_return_source(value, crate::config::env::joinir_dev::strict_enabled())
        .map_err(|freeze| freeze.to_string())
}

/// Emit the final return instruction from an already-evaluated value.
///
/// This is the shared shell used by ordinary lowering and fastmem lowering.
pub(in crate::mir::builder) fn emit_return_from_value(
    builder: &mut MirBuilder,
    return_value: ValueId,
) -> Result<ValueId, String> {
    if builder
        .function_state
        .protected_region
        .return_defer
        .is_active()
    {
        // Defer: copy into the configured slot and jump to the configured target.
        if let Some(destination) = builder
            .function_state
            .protected_region
            .return_defer
            .active_destination()?
        {
            let slot = destination.slot();
            let target = destination.target();
            builder
                .function_state
                .protected_region
                .return_defer
                .mark_emitted();
            builder.emit_instruction(MirInstruction::Copy {
                dst: slot,
                src: return_value,
            })?;
            crate::mir::builder::metadata::propagate::propagate(builder, return_value, slot);
            if !builder.is_current_block_terminated() {
                crate::mir::builder::emission::branch::emit_jump(builder, target)?;
            }
            Ok(return_value)
        } else {
            unreachable!("active defer always has a configured destination")
        }
    } else {
        // Normal return
        builder.emit_instruction(MirInstruction::Return {
            value: Some(return_value),
        })?;
        Ok(return_value)
    }
}

/// Lower the exact `return;` source shape.
///
/// Value-bearing Return is owned by `drive_value_return_statement_v1`.
pub(in crate::mir::builder) fn build_void_return_statement(
    builder: &mut MirBuilder,
) -> Result<ValueId, String> {
    ensure_cleanup_exit_allowed_v1(&builder.function_state, CleanupExitKindV1::Return)?;
    let return_value = crate::mir::builder::emission::constant::emit_void(builder)?;
    emit_return_from_value(builder, return_value)
}
