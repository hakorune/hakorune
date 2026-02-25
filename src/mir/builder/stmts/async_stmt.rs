//! Async Operations Module
//!
//! **Purpose**: Handle async operations (nowait/await)
//!
//! **Responsibilities**:
//! - nowait statement: Create Future values (Phase‑0: sequential evaluation + FutureNew)
//! - await expression: Wait for Future completion with safepoints
//! - Future type propagation and variable registration
//!
//! **Phase Notes**:
//! - Phase 84: Future type registration for await expressions
//! - SlotRegistry integration for async variables

use super::super::{MirBuilder, MirInstruction, MirType, ValueId};
use crate::ast::ASTNode;

/// Nowait: Phase‑0 semantics (sequential evaluation + FutureNew)
///
/// # Arguments
/// * `builder` - The MIR builder instance
/// * `variable` - Variable name to bind the Future to
/// * `expression` - Expression to spawn asynchronously
///
/// # Returns
/// ValueId of the created Future
///
/// # Phase Notes
/// - Phase‑0: evaluate `expression` now, then wrap as a resolved Future via `FutureNew`
/// - Registers Future<T> type and updates variable_map
pub(in crate::mir::builder) fn build_nowait_statement(
    builder: &mut MirBuilder,
    variable: String,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let expression_value = builder.build_expression(expression)?;
    let future_id = builder.next_value_id();
    builder.emit_instruction(MirInstruction::FutureNew {
        dst: future_id,
        value: expression_value,
    })?;
    let inner = builder
        .type_ctx
        .value_types
        .get(&expression_value)
        .cloned()
        .unwrap_or(MirType::Unknown);
    builder
        .type_ctx
        .value_types
        .insert(future_id, MirType::Future(Box::new(inner)));
    builder
        .variable_ctx
        .variable_map
        .insert(variable.clone(), future_id);
    if let Some(reg) = builder.comp_ctx.current_slot_registry.as_mut() {
        reg.ensure_slot(&variable, None);
    }
    Ok(future_id)
}

/// Await: insert Safepoint before/after and emit Await
///
/// # Arguments
/// * `builder` - The MIR builder instance
/// * `expression` - Expression that evaluates to a Future
///
/// # Returns
/// ValueId of the awaited result (inner type of Future<T>)
///
/// # Phase Notes
/// - Phase 84: Type propagation from Future<T> to result T
/// - Safepoints inserted before/after await for GC safety
pub(in crate::mir::builder) fn build_await_expression(
    builder: &mut MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let future_value = builder.build_expression(expression)?;
    builder.emit_instruction(MirInstruction::Safepoint)?;
    let result_id = builder.next_value_id();
    builder.emit_instruction(MirInstruction::Await {
        dst: result_id,
        future: future_value,
    })?;
    let result_type = match builder.type_ctx.value_types.get(&future_value) {
        Some(MirType::Future(inner)) => (**inner).clone(),
        _ => MirType::Unknown,
    };
    builder.type_ctx.value_types.insert(result_id, result_type);
    builder.emit_instruction(MirInstruction::Safepoint)?;
    Ok(result_id)
}
