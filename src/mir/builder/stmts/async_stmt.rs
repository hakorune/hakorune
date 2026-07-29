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
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};

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
    let mut port = RawLegacyChildLoweringPortV1;
    build_nowait_statement_with_port_v1(builder, &mut port, variable, expression)
}

/// Lower `nowait` while retaining the caller's raw child-descent port.
pub(in crate::mir::builder) fn build_nowait_statement_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    variable: String,
    expression: ASTNode,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let expression_value = drive_legacy_expression_v1(builder, port, expression)?;
    let future_id = builder.next_value_id();
    builder.emit_instruction(MirInstruction::FutureNew {
        dst: future_id,
        value: expression_value,
    })?;
    let inner = builder
        .function_state
        .type_ctx
        .value_types
        .get(&expression_value)
        .cloned()
        .unwrap_or(MirType::Unknown);
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(future_id, MirType::Future(Box::new(inner)));
    builder
        .function_state
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
    let mut port = RawLegacyChildLoweringPortV1;
    build_await_expression_with_port_v1(builder, &mut port, expression)
}

/// Lower `await` while retaining the caller's raw child-descent port.
pub(in crate::mir::builder) fn build_await_expression_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    expression: ASTNode,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let future_value = drive_legacy_expression_v1(builder, port, expression)?;
    builder.emit_instruction(MirInstruction::Safepoint)?;
    let result_id = builder.next_value_id();
    builder.emit_instruction(MirInstruction::Await {
        dst: result_id,
        future: future_value,
    })?;
    let result_type = match builder
        .function_state
        .type_ctx
        .value_types
        .get(&future_value)
    {
        Some(MirType::Future(inner)) => (**inner).clone(),
        _ => MirType::Unknown,
    };
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result_id, result_type);
    builder.emit_instruction(MirInstruction::Safepoint)?;
    Ok(result_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
    use crate::mir::region::function_slot_registry::FunctionSlotRegistry;

    #[test]
    fn nowait_publishes_future_binding_and_slot_after_child_success() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("nowait_owner_receipt/0".to_owned());
        builder.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
        let mut port = RawLegacyChildLoweringPortV1;

        let future = build_nowait_statement_with_port_v1(
            &mut builder,
            &mut port,
            "pending".to_owned(),
            ASTNode::Literal {
                value: LiteralValue::Integer(17),
                span: Span::unknown(),
            },
        )
        .unwrap();

        assert_eq!(
            builder
                .function_state
                .variable_ctx
                .variable_map
                .get("pending"),
            Some(&future)
        );
        assert!(matches!(
            builder.function_state.type_ctx.value_types.get(&future),
            Some(MirType::Future(inner)) if **inner == MirType::Integer
        ));
        assert!(builder
            .comp_ctx
            .current_slot_registry
            .as_ref()
            .and_then(|registry| registry.get_slot("pending"))
            .is_some());
    }
}
