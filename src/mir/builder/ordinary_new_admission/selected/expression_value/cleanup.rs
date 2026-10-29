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
    // Transport does not select dispatch. Ordinary nullable Text calls lend
    // their exact checked Call/Normal pair, just as dynamic producers do.
    let carrier_invoke = |binding: &Binding| {
        if binding.0 != invoke_block { return false; }
        let MirInstruction::Invoke { operation, normal_landing, .. } = &binding.1 else {
            return false;
        };
        if *normal_landing != projection.0 { return false; }
        match operation {
            InvokeOperation::DynamicInvocation(_)
                | InvokeOperation::DynamicArrayGet { .. }
                | InvokeOperation::DynamicCarrierSubI64 { .. } => true,
            InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::DynamicCarrier, .. } => {
                let mut packets = expression.body.calls.iter().filter(|packet|
                    packet.original_source().result() == Some(crate::mir::instruction::InvokeCallResultKind::DynamicCarrier)
                        && packet.outer_bindings().is_ok_and(|(invoke, normal)| invoke == binding && normal == &projection));
                packets.next().is_some() && packets.next().is_none()
            }
            _ => false,
        }
    };
    if expression.body.bindings.iter().filter(|binding| carrier_invoke(binding)).count() != 1 {
        return Err(freeze("initializer-expression/dynamic-carrier-invoke"));
    }
    expression.body.dynamic_carriers.pop();
    if expression.body.temporary_order.pop() != Some(0) {
        return Err(freeze("initializer-expression/dynamic-carrier-order"));
    }
    finish_body_continuation(builder, &mut expression.body, None, None, false)
}

/// I64 purpose completion retains the scalar result and ends only its child
/// Text acquisitions. Prior Homes remain live on Normal and unwind on Fault.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_scalar_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1)
    -> Result<(Binding, Binding), String> {
    finish_continuation(builder, expression, None, None, false)
}

/// The destination's original initializer frame is the sole issuer of this
/// transfer. Child temporaries remain inside this expression's cleanup scope.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_residence_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1)
    -> Result<(Binding, Binding), String> {
    finish_residence_with_displaced_continuation(builder, expression, None)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_residence_with_displaced_continuation(builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1, displaced: Option<InvokeOperation>,
) -> Result<(Binding, Binding), String> {
    let Operand::OwnedString(root) = expression.value.as_operand()?.clone() else { return Err(freeze("expression/residence-root")); };
    let materialized = expression.purpose_acquisition.as_ref().and_then(|pair| pair[1].1.dst_value());
    if materialized != Some(root) && expression.body.acquired.last().is_none_or(|(_, value)| *value != root) {
        return Err(freeze("expression/residence-acquisition-order"));
    }
    finish_continuation(builder, expression, Some(root), displaced, false)
}

/// An owned argument lends its entire remaining stack to the enclosing Call.
/// The newest root prevents earlier children from ending before that Call.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_deferred_argument_continuation(
    builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1,
) -> Result<(Binding, Binding), String> {
    let Operand::OwnedString(root) = expression.value.as_operand()?.clone() else {
        return Err(freeze("argument-expression/deferred-root"));
    };
    let materialized = expression.purpose_acquisition.as_ref().and_then(|pair| pair[1].1.dst_value());
    if materialized != Some(root) && expression.body.acquired.last() != Some(&(0, root)) {
        return Err(freeze("argument-expression/deferred-acquisition-order"));
    }
    let mut writer = RawRootGraphWriterV1;
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
    writer.append_terminated(builder, clean, continuation.1.clone(), &mut expression.body.bindings)?;
    let terminal = writer.emit_current(builder, MirInstruction::Jump { target: clean, edge_args: None })?;
    expression.body.bindings.push(terminal.clone());
    builder.start_new_block(next)?;
    Ok((terminal, continuation))
}

/// Materialize after the original source acquisition-prefix validation. The
/// original operand table and node stack remain untouched for both final scans.
/// Shared purpose acquisition; preserve the original evaluation operands.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn materialize_text_root(
    builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1, required: bool,
    source: &CheckedExpressionEvaluationV1,
) -> Result<(), String> {
    if !required { return Ok(()); }
    let operand = expression.value.as_operand()?;
    let snapshot = matches!(operand, Operand::DynamicCarrier(_))
        && source.text_snapshot_root_v1().is_some_and(|node|
            expression.body.values.get(node).and_then(Option::as_ref) == Some(&expression.value));
    if !(operand.is_text() || snapshot) || expression.purpose_acquisition.is_some() {
        return Err(freeze("rebind-expression/materialize-domain"));
    }
    let pending = pending_operations(&expression.body.acquired, &expression.body.dynamic_carriers,
        &expression.body.temporary_order, &expression.body.homes)?;
    let (value, _, pair) = emit_scalar_operation_with_pending_v1(builder, &mut RawRootGraphWriterV1,
        InvokeOperation::String(Operation::Materialize { value: expression.value.as_operand()?.clone() }), expression.body.fault_frame,
        &pending, &mut expression.body.bindings)?;
    expression.purpose_acquisition = Some(pair);
    expression.value = Operand::OwnedString(value).into();
    Ok(())
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_rebind_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1,
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
    displaced: Option<InvokeOperation>,
) -> Result<(Binding, Binding), String> {
    if !frame.is_text_residence() {
        return finish_continuation(builder, expression, None, None, frame.is_dynamic_loop_carrier_v1());
    }
    let Operand::OwnedString(root) = expression.value.as_operand()?.clone() else { return Err(freeze("expression/residence-root")); };
    if expression.purpose_acquisition.is_some() != frame.materializes_root()
        || !frame.materializes_root() && expression.body.acquired.last() != Some(&(0, root)) { return Err(freeze("expression/residence-acquisition-order")); }
    finish_continuation(builder, expression, Some(root), displaced, false)
}

fn finish_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1,
    residence: Option<ValueId>, displaced: Option<InvokeOperation>, dynamic_loop_carrier: bool)
    -> Result<(Binding, Binding), String> {
    let scalar_result = matches!(expression.value,
        EvaluatedExpressionValueV1::Operand(Operand::NormalInteger(_)) | EvaluatedExpressionValueV1::Bool(_))
        || dynamic_loop_carrier && matches!(expression.value,
            EvaluatedExpressionValueV1::Operand(Operand::Dynamic(_) | Operand::DynamicCarrier(_)));
    if residence.is_none() && !scalar_result {
        return Err(freeze("expression/scalar-continuation-result"));
    }
    finish_body_continuation(builder, &mut expression.body, residence, displaced, expression.purpose_acquisition.is_some())
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_value_continuation_v1(builder: &mut MirBuilder, body: &mut EmittedExpressionBodyV1)
    -> Result<(Binding, Binding), String> {
    finish_body_continuation(builder, body, None, None, false)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_bool_condition_continuation_v1(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    normal: Vec<InvokeOperation>,
) -> Result<(Binding, Binding), String> {
    let fault = normal.iter().cloned().chain(body.homes.iter().cloned()).collect();
    finish_body_cleanup(builder, body, normal, fault, None)
}

/// Transfer the exact Normal Dynamic carrier result into its Local receipt.
/// The carrier is intentionally left live here; its Local scope owner must
/// later discharge it with DynamicCarrierEnd on every exit path.
fn finish_body_continuation(builder: &mut MirBuilder, body: &mut EmittedExpressionBodyV1,
    residence: Option<ValueId>, displaced: Option<InvokeOperation>, materialized: bool,
) -> Result<(Binding, Binding), String> {
    if let Some(value) = residence {
        let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
        let replaces = match displaced.as_ref() {
            Some(InvokeOperation::String(Operation::End { value } | Operation::EndIfOwned { value })) => Some(*value),
            Some(_) => return Err(freeze("expression/residence-replacement-kind")),
            None => None,
        };
        let install = MirInstruction::StringResidenceInstall { value, replaces, returning: false };
        builder.emit_instruction(install.clone())?;
        body.bindings.push((block, install));
        if !materialized {
            if body.acquired.pop() != Some((0, value)) || body.temporary_order.pop() != Some(0) {
                return Err(freeze("expression/residence-acquisition-order"));
            }
        }
    }
    let normal = pending_operations(&body.acquired, &body.dynamic_carriers, &body.temporary_order, &[])?;
    let mut homes = body.homes.clone();
    if let Some(value) = residence { homes.insert(0, InvokeOperation::String(Operation::End { value })); }
    let fault = pending_operations(&body.acquired, &body.dynamic_carriers, &body.temporary_order, &homes)?;
    finish_body_cleanup(builder, body, normal, fault, displaced)
}

fn finish_body_cleanup(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    mut normal: Vec<InvokeOperation>,
    fault: Vec<InvokeOperation>,
    displaced: Option<InvokeOperation>,
) -> Result<(Binding, Binding), String> {
    let mut writer = RawRootGraphWriterV1;
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
    writer.append_terminated(builder, clean, continuation.1.clone(), &mut body.bindings)?;
    let mut normal_indices: Vec<_> = (0..normal.len()).collect();
    let mut fault = fault;
    if let Some(old) = displaced {
        let index = fault.iter().position(|operation| *operation == old)
            .ok_or_else(|| freeze("rebind-expression/old-home-missing"))?;
        if fault.iter().filter(|operation| **operation == old).count() != 1 || normal_indices.contains(&index) {
            return Err(freeze("rebind-expression/old-home-duplicate"));
        }
        if normal.is_empty() {
            normal_indices.push(index);
            normal.push(old);
        } else {
            // The installed replacement owes the displaced End before any
            // temporary's End, on Normal and on Fault alike.
            fault.remove(index);
            fault.insert(0, old.clone());
            normal.insert(0, old);
            normal_indices = (0..normal.len()).collect();
        }
    }
    let after = (0..normal.len()).map(|step|
        crate::mir::normal_callable_semantic_package::fault_indices_after_normal_step_v2(fault.len(), &normal_indices, step))
        .collect::<Result<Vec<_>, _>>()?;
    // The Local terminal is an infallible Jump. Its child acquisitions already
    // own their Fault paths; do not emit a detached terminal-acquisition suffix.
    // Without child cleanup, there is no new fallible operation and no Fault
    // suffix consumer. Do not retain a detached ReturnFault for finishing to drop.
    let clean_entry = if normal.is_empty() { clean } else {
        root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, body.fault_frame, clean,
            &normal, &fault, &after, &[], &mut body.bindings)?.clean
    };
    let terminal = writer.emit_current(builder, MirInstruction::Jump { target: clean_entry, edge_args: None })?;
    body.bindings.push(terminal.clone());
    builder.start_new_block(next)?;
    let mut unique = Vec::new();
    for binding in std::mem::take(&mut body.bindings) { if !unique.contains(&binding) { unique.push(binding); } }
    body.bindings = unique;
    Ok((terminal, continuation))
}
