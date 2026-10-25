//! Exact cleanup stack construction for expression-owned values and carriers.
use super::*;

pub(in crate::mir::builder::ordinary_new_admission::selected) fn pending_operations(
    acquired: &[(usize, ValueId)],
    dynamic_carriers: &[(usize, ValueId)],
    temporary_order: &[usize],
    homes: &[InvokeOperation],
) -> Result<Vec<InvokeOperation>, String> {
    let mut seen = std::collections::BTreeSet::new();
    if acquired.len() + dynamic_carriers.len() != temporary_order.len()
        || temporary_order.iter().any(|node| !seen.insert(*node))
    {
        return Err(freeze("expression/temporary-order"));
    }
    let mut pending = Vec::with_capacity(temporary_order.len() + homes.len());
    for node in temporary_order.iter().rev() {
        let string = acquired.iter().find(|(candidate, _)| candidate == node);
        let carrier = dynamic_carriers
            .iter()
            .find(|(candidate, _)| candidate == node);
        match (string, carrier) {
            (Some((_, value)), None) => {
                pending.push(InvokeOperation::String(Operation::End { value: *value }))
            }
            (None, Some((_, carrier))) => {
                pending.push(InvokeOperation::DynamicCarrierEnd { carrier: *carrier })
            }
            _ => return Err(freeze("expression/temporary-coverage")),
        }
    }
    pending.extend(homes.iter().cloned());
    Ok(pending)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn bool_condition_temporary_operations_v1(
    frame: &CheckedExpressionEvaluationV1,
    comparison: usize,
    body: &EmittedExpressionBodyV1,
) -> Result<Vec<InvokeOperation>, String> {
    let order = frame
        .scalar_invoke_input_order_v1(comparison)
        .ok_or_else(|| freeze("bool-condition/comparison-input-order"))?;
    let expected: std::collections::BTreeSet<_> = order
        .iter()
        .copied()
        .filter(|node| frame.owned_result_nodes().contains(node))
        .collect();
    if body.temporary_order.len() != expected.len()
        || body
            .temporary_order
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            != expected
    {
        return Err(freeze("bool-condition/temporary-order-coverage"));
    }
    if body
        .dynamic_carriers
        .iter()
        .map(|(node, _)| *node)
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != body.dynamic_carriers.len()
    {
        return Err(freeze("bool-condition/dynamic-temporary-duplicate"));
    }
    let mut operations = std::collections::BTreeMap::new();
    for &(node, value) in &body.acquired {
        if !expected.contains(&node)
            || body
                .values
                .get(node)
                .and_then(Option::as_ref)
                .and_then(|value| value.as_operand().ok())
                != Some(&Operand::OwnedString(value))
            || operations
                .insert(node, InvokeOperation::String(Operation::End { value }))
                .is_some()
        {
            return Err(freeze("bool-condition/string-temporary-coverage"));
        }
    }
    for packet in &body.dynamic_calls {
        let call = packet.source();
        let node = frame
            .source()
            .nodes()
            .iter()
            .position(|row| row.site() == call.site().site())
            .ok_or_else(|| freeze("bool-condition/dynamic-call-node"))?;
        if !expected.contains(&node)
            || !call.dynamic_invocation_result_lifecycle_v1().is_some()
            || !frame.calls().iter().any(|original| original == call)
            || !body.dynamic_carriers.contains(&(node, packet.value()))
            || body
                .values
                .get(node)
                .and_then(Option::as_ref)
                .and_then(|value| value.as_operand().ok())
                != Some(&Operand::DynamicCarrier(packet.value()))
            || operations
                .insert(
                    node,
                    InvokeOperation::DynamicCarrierEnd {
                        carrier: packet.value(),
                    },
                )
                .is_some()
        {
            return Err(freeze("bool-condition/dynamic-temporary-coverage"));
        }
    }
    for &(node, value) in &body.dynamic_carriers {
        if operations.contains_key(&node) {
            continue;
        }
        let carrier_value_mismatch = body
            .values
            .get(node)
            .and_then(Option::as_ref)
            .and_then(|value| value.as_operand().ok())
            != Some(&Operand::DynamicCarrier(value));
        let source_producer = if let Some(core) = frame.cores().get(&node) {
            matches!(
                crate::mir::normal_callable_semantic_package::checked_expression_core_operation_v1(
                    core,
                    &body.values,
                )?,
                InvokeOperation::DynamicArrayGet { .. }
            )
        } else if let Some(operation) = frame
            .operations()
            .iter()
            .find(|operation| operation.node() == node)
        {
            operation.normal_result_v1() == Some(ResultKind::SelfContainedNonAliasingDynamicCarrier)
                && operation
                    .single_envelope()
                    .is_some_and(|envelope| envelope.domain().family() == OperatorFamily::Sub)
        } else {
            false
        };
        if !expected.contains(&node)
            || carrier_value_mismatch
            || !source_producer
            || operations
                .insert(node, InvokeOperation::DynamicCarrierEnd { carrier: value })
                .is_some()
        {
            return Err(freeze("bool-condition/dynamic-core-temporary-coverage"));
        }
    }
    if operations
        .keys()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        != expected
    {
        return Err(freeze("bool-condition/owned-temporary-coverage"));
    }
    body.temporary_order
        .iter()
        .rev()
        .map(|node| {
            operations
                .remove(node)
                .ok_or_else(|| freeze("bool-condition/temporary-order-operation"))
        })
        .collect()
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_dynamic_initializer_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
) -> Result<(Binding, Binding), String> {
    let Operand::DynamicCarrier(value) = expression.value.as_operand()? else {
        return Err(freeze("initializer-expression/dynamic-carrier-result"));
    };
    let projection = expression
        .body
        .bindings
        .iter()
        .find(|(_, row)| {
            matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == *value)
        })
        .cloned()
        .ok_or_else(|| freeze("initializer-expression/dynamic-carrier-projection"))?;
    if expression
        .body
        .bindings
        .iter()
        .filter(|(_, row)| {
            matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == *value)
        })
        .count()
        != 1
    {
        return Err(freeze("initializer-expression/dynamic-carrier-projection"));
    }
    if !matches!(expression.body.dynamic_carriers.last(), Some((0, carrier)) if carrier == value)
        || expression
            .body
            .dynamic_carriers
            .iter()
            .filter(|(_, carrier)| carrier == value)
            .count()
            != 1
    {
        return Err(freeze("initializer-expression/dynamic-carrier-order"));
    }
    let MirInstruction::InvokeNormalResult { invoke_block, .. } = projection.1 else {
        return Err(freeze("initializer-expression/dynamic-carrier-projection"));
    };
    let invoke = expression
        .body
        .bindings
        .iter()
        .find(|(block, row)| {
            *block == invoke_block
                && matches!(
                    row,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::DynamicInvocation(_)
                            | InvokeOperation::DynamicArrayGet { .. }
                            | InvokeOperation::DynamicCarrierSubI64 { .. },
                        normal_landing,
                        ..
                    } if *normal_landing == projection.0
                )
        })
        .ok_or_else(|| freeze("initializer-expression/dynamic-carrier-invoke"))?;
    if expression
        .body
        .bindings
        .iter()
        .filter(|(block, row)| {
            *block == invoke_block
                && matches!(
                    row,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::DynamicInvocation(_)
                            | InvokeOperation::DynamicArrayGet { .. }
                            | InvokeOperation::DynamicCarrierSubI64 { .. },
                        ..
                    }
                )
        })
        .count()
        != 1
        || !expression.body.bindings.contains(invoke)
    {
        return Err(freeze("initializer-expression/dynamic-carrier-invoke"));
    }
    expression.body.dynamic_carriers.pop();
    if expression.body.temporary_order.pop() != Some(0) {
        return Err(freeze("initializer-expression/dynamic-carrier-order"));
    }
    super::finish_body_continuation(builder, &mut expression.body, None, None, false)
}
