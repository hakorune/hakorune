//! Shared physical operation/Normal writers; source laws stay with the loans.
use super::*;

pub(super) fn emit_string_value(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: Operation,
    node: usize, fault_frame: ValueId, acquired: &mut Vec<(usize, ValueId)>,
    dynamic_carriers: &[(usize, ValueId)], temporary_order: &mut Vec<usize>,
    homes: &[InvokeOperation], bindings: &mut Vec<Binding>,
) -> Result<Operand, String> {
    let before = acquired.len();
    let (value, result) = emit_string_operation_v1(builder, writer, operation, node, fault_frame,
        acquired, dynamic_carriers, temporary_order, homes, bindings)?;
    if acquired.len() == before + 1 {
        temporary_order.push(node);
    } else if acquired.len() != before {
        return Err(freeze("expression/string-acquisition-count"));
    }
    match result {
        crate::mir::instruction::InvokeNormalResultKind::OwnedString => Ok(Operand::OwnedString(value)),
        crate::mir::instruction::InvokeNormalResultKind::I64 => Ok(Operand::NormalInteger(value)),
        _ => Err(freeze("expression/string-value-result")),
    }
}

/// One canonical String operation writer, shared with recursive Bool descent.
/// The enclosing purpose determines when this node is reached and owns cleanup.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn emit_string_operation_v1(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: Operation,
    node: usize, fault_frame: ValueId, acquired: &mut Vec<(usize, ValueId)>,
    dynamic_carriers: &[(usize, ValueId)], temporary_order: &[usize],
    homes: &[InvokeOperation], bindings: &mut Vec<Binding>,
) -> Result<(ValueId, crate::mir::instruction::InvokeNormalResultKind), String> {
    let pending = FaultCleanupPlanV1::pending(acquired, dynamic_carriers, temporary_order, homes)?;
    let (value, result, _) = emit_scalar_operation_with_pending_v1(builder, writer, InvokeOperation::String(operation),
        fault_frame, &pending, bindings)?;
    if result == crate::mir::instruction::InvokeNormalResultKind::OwnedString { acquired.push((node, value)); }
    Ok((value, result))
}

/// Physical Invoke/Normal writer shared by source nodes and purpose acquisition.
/// Only the caller that owns a source node may add it to the source-node stack.
/// Share source-derived comparison emission across conditions and value consumers.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn emit_checked_scalar_condition_v1(
    builder: &mut MirBuilder, frame: &CheckedExpressionEvaluationV1, node: usize,
    values: &[Option<EvaluatedExpressionValueV1>], receiver: Option<&crate::mir::builder::ExactLexicalReadV1>,
    fault_frame: Option<ValueId>, pending: &FaultCleanupPlanV1, older: Option<BasicBlockId>, bindings: &mut Vec<Binding>,
) -> Result<ValueId, String> {
    let plan = crate::mir::normal_callable_semantic_package::checked_scalar_invoke_operation_v1(frame, node, values, receiver)?;
    let (normal, kind) = match (plan.operation(), plan.direct_integer()) {
        (Some(operation), _) => {
            let frame = fault_frame.ok_or_else(|| freeze("expression/fault-frame-required"))?;
            let mut writer = RawRootGraphWriterV1;
            let normal = writer.reserve_block(builder)?;
            let graph = root_graph::emit_cleanup_suffixes_with_parent_v1(&mut writer, builder, frame, normal, pending, older, bindings)?;
            let (normal, kind, _) = emit_scalar_operation_to_landing_v1(builder, &mut writer, operation.clone(), frame,
                normal, graph.acquisition_fault, bindings)?;
            (normal, kind)
        }
        // Two exact i64 operand values compare directly; nothing is invoked.
        (None, Some(integer)) => (integer, crate::mir::instruction::InvokeNormalResultKind::I64),
        (None, None) => return Err(freeze("bool-condition/text-result")),
    };
    if plan.is_integer_comparison() {
        if kind != crate::mir::instruction::InvokeNormalResultKind::I64 { return Err(freeze("bool-condition/text-result")); }
        let result = builder.next_value_id();
        let comparison = plan.comparison_instruction(normal, result).ok_or_else(|| freeze("bool-condition/text-result"))?;
        let MirInstruction::Compare { dst, op, lhs, rhs } = comparison else { unreachable!() };
        bindings.push(crate::mir::builder::emission::compare::emit_to_recorded(builder, dst, op, lhs, rhs)?);
        Ok(result)
    } else {
        if kind != crate::mir::instruction::InvokeNormalResultKind::Bool { return Err(freeze("bool-condition/text-result")); }
        Ok(normal)
    }
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn emit_scalar_operation_with_pending_v1(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: InvokeOperation,
    fault_frame: ValueId, pending: &FaultCleanupPlanV1, bindings: &mut Vec<Binding>,
) -> Result<(ValueId, crate::mir::instruction::InvokeNormalResultKind, [Binding; 2]), String> {
    operation.normal_result_kind().ok_or_else(|| freeze("expression/string-result"))?;
    let normal = writer.reserve_block(builder)?;
    let graph = root_graph::emit_cleanup_suffixes_with_writer(writer, builder, fault_frame,
        normal, pending, bindings)?;
    emit_scalar_operation_to_landing_v1(builder, writer, operation, fault_frame,
        normal, graph.acquisition_fault, bindings)
}

/// The purpose lends its already checked Fault continuation. This sole writer
/// never replaces constructor field discharge with an expression ReturnFault.
pub(in crate::mir::builder) fn emit_scalar_operation_to_landing_v1(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: InvokeOperation,
    fault_frame: ValueId, normal: BasicBlockId, fault: BasicBlockId, bindings: &mut Vec<Binding>,
) -> Result<(ValueId, crate::mir::instruction::InvokeNormalResultKind, [Binding; 2]), String> {
    let result = operation.normal_result_kind().ok_or_else(|| freeze("expression/string-result"))?;
    let invoke = writer.emit_current(builder, MirInstruction::Invoke {
        operation, fault_frame, normal_landing: normal, fault_landing: fault,
    })?;
    let value = builder.next_value_id();
    builder.start_new_block(normal)?;
    let projection = MirInstruction::InvokeNormalResult { invoke_block: invoke.0, dst: value };
    writer.insert_projection(builder, normal, projection.clone())?;
    let pair = [invoke, (normal, projection)];
    bindings.extend(pair.iter().cloned());
    let kind = match result {
        crate::mir::instruction::InvokeNormalResultKind::OwnedString => MirType::String,
        crate::mir::instruction::InvokeNormalResultKind::I64 => MirType::Integer,
        crate::mir::instruction::InvokeNormalResultKind::Bool => MirType::Bool,
        crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier => MirType::DynamicCarrier,
        _ => return Err(freeze("expression/string-result")),
    };
    builder.function_state.type_ctx.value_types.insert(value, kind);
    Ok((value, result, pair))
}
