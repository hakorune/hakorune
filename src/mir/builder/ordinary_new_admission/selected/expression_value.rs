//! Sole ordered expression descent, independent of the terminal consumer.
//! Returns live acquisitions; the purpose wrapper owns cleanup and publication.
use super::*;
use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as ResultKind;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::normal_callable_semantic_package::{EmittedLexicalCallProjectionV1, ExpressionReferenceReadV1, CallValidationContextV1, ExpressionCallTransientPrefixV1};
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1, LocalCallObservationV1};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, ResolvedLiteralSourceV1 as Literal};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

/// No independent binding group is published here. Every retained original
/// receipt and live temporary moves to the checked terminal consumer together.
pub(super) struct EmittedExpressionValueV1 {
    pub(super) value: Operand,
    pub(super) purpose_acquisition: Option<[Binding; 2]>,
    pub(super) body: EmittedExpressionBodyV1,
}

/// One evaluated source scope. A comparison borrows its inputs without
/// inventing a root value or re-rooting the original source expression.
pub(super) struct EmittedExpressionBodyV1 {
    pub(super) values: Vec<Option<Operand>>,
    pub(super) calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
    pub(super) path_reads: std::collections::BTreeMap<OwnedExprSiteV1, crate::mir::builder::ExactLexicalReadV1>,
    pub(super) reference_reads: std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
    pub(super) acquired: Vec<(usize, ValueId)>,
    pub(super) homes: Vec<InvokeOperation>,
    pub(super) fault_frame: ValueId,
    pub(super) entry: Binding,
    pub(super) bindings: Vec<Binding>,
}

pub(super) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    emit_call: &mut impl FnMut(
        &mut MirBuilder, &mut CallableSemanticLoweringState,
        &LocalCallObservationV1, &[(usize, ValueId)], &[Option<Operand>], &mut Vec<Binding>,
    ) -> Result<EmittedLexicalCallProjectionV1, String>,
) -> Result<EmittedExpressionValueV1, String> {
    emit_with_prefix(builder, state, ledger, frame, None, emit_call)
}

pub(super) fn emit_with_prefix(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    emit_call: &mut impl FnMut(
        &mut MirBuilder, &mut CallableSemanticLoweringState,
        &LocalCallObservationV1, &[(usize, ValueId)], &[Option<Operand>], &mut Vec<Binding>,
    ) -> Result<EmittedLexicalCallProjectionV1, String>,
) -> Result<EmittedExpressionValueV1, String> {
    let body = emit_body_with_prefix(builder, state, ledger, frame, None, inherited, emit_call)?;
    let value = body.values[0].clone().ok_or_else(|| freeze("print-expression/root-value"))?;
    Ok(EmittedExpressionValueV1 { value, purpose_acquisition: None, body })
}

pub(super) fn emit_comparison_inputs_v1(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, frame: &CheckedExpressionEvaluationV1, comparison: usize,
) -> Result<EmittedExpressionBodyV1, String> {
    emit_body_with_prefix(builder, state, ledger, frame, Some(comparison), None,
        &mut |_, _, _, _, _, _| Err(freeze("bool-condition/call-source")))
}

fn emit_body_with_prefix(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, frame: &CheckedExpressionEvaluationV1,
    comparison: Option<usize>, inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    emit_call: &mut impl FnMut(&mut MirBuilder, &mut CallableSemanticLoweringState,
        &LocalCallObservationV1, &[(usize, ValueId)], &[Option<Operand>], &mut Vec<Binding>)
        -> Result<EmittedLexicalCallProjectionV1, String>,
) -> Result<EmittedExpressionBodyV1, String> {
    let order = match comparison {
        Some(node) => frame.text_comparison_input_order_v1(node).ok_or_else(|| freeze("bool-condition/text-comparison-source"))?,
        None => frame.ordered_nodes().to_vec(),
    };
    let owner = frame.source().owner();
    if state.owner() != owner { return Err(freeze("expression-value/state-owner")); }
    if !frame.reference_coverage_is_exact() { return Err(freeze("expression/reference-source-coverage")); }
    // Reject unconnected carriers/finite selection before any committed read,
    // FaultFrame or MIR effect; a retained source class is not a physical value.
    // Corroborate every original finite requirement before the retained
    // multi-envelope execution fence. Preparation consumes no source read.
    for (_, original) in frame.references().iter().filter(|(node, _)| order.contains(node)) {
        if let CheckedExpressionReferenceV1::FiniteEntry { read, exclusion } = original {
            let prepared = state.prepare_exact_lexical_read(owner, read.requirement().read().node(), read.requirement().binding())
                .map_err(|error| error.to_string())?;
            let (_, _, domain) = ledger.validate_prepared_finite_entry_read_v1(read.requirement(),
                builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?, prepared.observation_v1())?;
            if !read.corroborates_finite_domain_v1(&domain) || read.leaf_class(exclusion.as_deref()).is_none() {
                return Err(freeze("expression/reference-initializer-domain"));
            }
        }
    }
    if frame.operations().iter().any(|operation| operation.single_envelope().is_none()) {
        return Err(freeze("expression/finite-envelope-execution-pending"));
    }
    for &index in &order {
        if frame.transfers_node(index) || frame.node_is_identity(index) { continue; }
        if matches!(frame.references().get(&index), Some(CheckedExpressionReferenceV1::Scalar(read)) if read.original_read().is_none()) {
            return Err(freeze("expression/reference-scalar-obligation"));
        }
        if let Some(CheckedExpressionReferenceV1::ArgumentEntry(requirement)) = frame.references().get(&index) {
            let prepared = state.prepare_exact_lexical_read(owner, requirement.read().node(), requirement.binding()).map_err(|error| error.to_string())?;
            ledger.validate_prepared_text_entry_read_v1(requirement,
                builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?, prepared.observation_v1())?;
        }
        if matches!(frame.source().nodes()[index].source(), Node::Reference(_))
            && !matches!(frame.references().get(&index), Some(CheckedExpressionReferenceV1::StaticText(_) | CheckedExpressionReferenceV1::Scalar(_) | CheckedExpressionReferenceV1::Residence(_)
                | CheckedExpressionReferenceV1::ArgumentEntry(_) | CheckedExpressionReferenceV1::FiniteEntry { .. })) {
            return Err(freeze("expression/reference-execution-pending"));
        }
    }
    let mut homes = inherited.map(|prefix| prefix.for_argument(frame)).transpose()?.unwrap_or_default();
    homes.extend(ledger.prior_home_unwind_for(frame.prior_homes())?);
    let source = frame.source();
    let mut pending = Vec::new();
    let mut path_slots = std::collections::BTreeMap::new();
    let mut reference_slots = std::collections::BTreeMap::new();
    let mut direct_fields = std::collections::BTreeMap::new();
    // Fulfill every field-root read before any FaultFrame, CFG or value effect.
    // Reservation remains in original source order; binding nested receivers
    // still waits for the actual ancestor field writer below.
    for &index in &order {
        if frame.transfers_node(index) { continue; }
        let node = &source.nodes()[index];
        if frame.node_is_identity(index) && !matches!(node.source(), Node::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) { continue; }
        let site = OwnedExprSiteV1::new(owner, node.site().clone());
        if let Some(original) = frame.references().get(&index) {
            let read = state.prepare_exact_lexical_read(owner, node.site().node(), original.binding()).map_err(|error| error.to_string())?;
            let producer = ExpressionReferenceReadV1::prepare(frame, index, read.observation_v1(), CallValidationContextV1::Lowering(ledger),
                builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
            reference_slots.insert(index, (pending.len(), producer));
            pending.push(read);
        }
        if let Some(path) = ledger.statement_field_path_v1(&site, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))? {
            if !ledger.reserve_result_path_field_v1(&site, |binding| {
                let root_site = OwnedExprSiteV1::new(owner, path.root_site().clone());
                let slot = match path_slots.get(&root_site) {
                    Some(slot) => *slot,
                    None => {
                        let read = state.prepare_exact_lexical_read(owner, path.root_site().node(), binding)
                            .map_err(|error| error.to_string())?;
                        let slot = pending.len();
                        pending.push(read);
                        path_slots.insert(root_site, slot);
                        slot
                    }
                };
                pending[slot].value_for(owner, path.root_site().node(), binding).map_err(|error| error.to_string())
            })? { return Err(freeze("print-expression/path-source-missing")); }
        } else if let Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) = node.source() {
            let mut slot = None;
            let (base, descriptor) = ledger.take_local_field_read(&site, |binding| {
                let read = state.prepare_exact_lexical_read(owner, object.node(), binding).map_err(|error| error.to_string())?;
                let value = read.value_for(owner, object.node(), binding).map_err(|error| error.to_string())?;
                slot = Some(pending.len());
                pending.push(read);
                Ok(value)
            })?.ok_or_else(|| freeze("print-expression/field-source-missing"))?;
            direct_fields.insert(index, (base, descriptor, slot.ok_or_else(|| freeze("print-expression/field-exact-read-missing"))?));
        }
    }
    let mut reads: Vec<_> = state.commit_exact_lexical_reads(pending).map_err(|error| error.to_string())?
        .into_iter().map(Some).collect();
    let mut path_reads = std::collections::BTreeMap::new();
    for (site, slot) in path_slots {
        path_reads.insert(site, reads[slot].take().ok_or_else(|| freeze("expression-value/read-slot"))?);
    }
    let reference_reads = reference_slots.into_iter().map(|(node, (slot, producer))| {
        Ok((node, producer.finish(reads[slot].take().ok_or_else(|| freeze("expression-value/reference-slot"))?)))
    }).collect::<Result<std::collections::BTreeMap<_, _>, String>>()?;
    let fault_frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, fault_frame)?];
    let mut writer = RawRootGraphWriterV1;
    let start = writer.reserve_block(builder)?;
    let entry = writer.emit_current(builder, MirInstruction::Jump { target: start, edge_args: None })?;
    bindings.push(entry.clone());
    builder.start_new_block(start)?;
    let mut values: Vec<Option<Operand>> = vec![None; source.nodes().len()];
    let mut field_outputs: Vec<Option<ValueId>> = vec![None; source.nodes().len()];
    let mut acquired = Vec::new();
    let mut calls = Vec::new();
    for &index in &order {
        if frame.transfers_node(index) { continue; }
        let node = &source.nodes()[index];
        if frame.node_is_identity(index) && !matches!(node.source(), Node::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) { continue; }
        let site = OwnedExprSiteV1::new(owner, node.site().clone());
        if let Some(path) = ledger.statement_field_path_v1(&site, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))? {
            let base = if let Some(parent) = path.parent() {
                source.nodes().iter().position(|node| node.site() == parent.site())
                    .and_then(|index| field_outputs[index]).ok_or_else(|| freeze("print-expression/path-parent-missing"))?
            } else {
                let root_site = OwnedExprSiteV1::new(owner, path.root_site().clone());
                path_reads.get(&root_site).ok_or_else(|| freeze("print-expression/path-root-missing"))?
                    .value_for(owner, path.root_site().node(), path.root()).map_err(|error| error.to_string())?
            };
            let descriptor = ledger.bind_result_path_receiver_v1(&site, base)?;
            let value = if descriptor.requires_checked_field_read_v1() {
                let normal = writer.reserve_block(builder)?;
                let pending = pending_operations(&acquired, &homes);
                let graph = root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, fault_frame,
                    normal, &[], &pending, &[], &(0..pending.len()).collect::<Vec<_>>(), &mut bindings)?;
                let (value, pair) = builder.append_checked_received_field_read_v1(base, &descriptor,
                    fault_frame, normal, graph.acquisition_fault)?;
                ledger.record_enclosed_checked_local_field_read_v1(&site, &descriptor, None, pair[0].clone(), pair[1].clone())?;
                bindings.extend(pair);
                value
            } else {
                let (value, binding) = builder.complete_exact_field_read_v1(base, descriptor.field(), &site, ledger,
                    crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(descriptor))?;
                bindings.push(binding);
                value
            };
            field_outputs[index] = Some(value);
            if !frame.node_is_identity(index) { values[index] = Some(Operand::NormalInteger(value)); }
            continue;
        }
        let value = match node.source() {
            Node::Literal(Literal::String(text)) => Operand::StaticText { utf8: text.to_string() },
            Node::Literal(Literal::Integer(literal)) => {
                let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
                let value = crate::mir::builder::emission::constant::emit_integer(builder, *literal)?;
                bindings.push((block, MirInstruction::Const { dst: value, value: crate::mir::ConstValue::Integer(*literal) }));
                Operand::NormalInteger(value)
            }
            Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }) => {
                let site = OwnedExprSiteV1::new(owner, node.site().clone());
                let (base, source, slot) = direct_fields.remove(&index)
                    .ok_or_else(|| freeze("print-expression/field-source-missing"))?;
                let original_read = reads[slot].take();
                let value = if source.requires_checked_field_read_v1() {
                    let normal = writer.reserve_block(builder)?;
                    let pending = pending_operations(&acquired, &homes);
                    let graph = root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, fault_frame,
                        normal, &[], &pending, &[], &(0..pending.len()).collect::<Vec<_>>(), &mut bindings)?;
                    let (value, pair) = builder.append_checked_received_field_read_v1(base, &source,
                        fault_frame, normal, graph.acquisition_fault)?;
                    ledger.record_enclosed_checked_local_field_read_v1(&site, &source,
                        Some(Rc::new(original_read.ok_or_else(|| freeze("print-expression/field-exact-read-missing"))?)), pair[0].clone(), pair[1].clone())?;
                    bindings.extend(pair);
                    value
                } else {
                    let (value, binding) = builder.complete_exact_field_read_v1(base, source.field(), &site, ledger,
                        crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(source))?;
                    bindings.push(binding);
                    value
                };
                Operand::NormalInteger(value)
            }
            Node::Reference(_) => {
                let observed = reference_reads.get(&index).ok_or_else(|| freeze("expression/reference-read-missing"))?;
                observed.operand(frame, index, CallValidationContextV1::Lowering(ledger))?
            }
            Node::MethodCall if frame.cores().contains_key(&index) => {
                let operation = crate::mir::normal_callable_semantic_package::checked_expression_core_operation_v1(&frame.cores()[&index], &values)?;
                let value = emit_string_value(builder, &mut writer, operation, index, fault_frame, &mut acquired, &homes, &mut bindings)?;
                state.complete_expression_core_source_v1(&frame.cores()[&index])?;
                value
            }
            Node::MethodCall => {
                let call = frame.calls().iter().find(|call| call.site().site() == node.site())
                    .ok_or_else(|| freeze("print-expression/call-source-missing"))?;
                let packet = Rc::new(emit_call(builder, state, call, &acquired, &values, &mut bindings)?);
                let value = packet.value_with_ledger(owner, call.arguments(), ledger)?;
                calls.push(packet);
                Operand::NormalInteger(value)
            }
            Node::Binary(binary) => {
                let operand = |site| source.nodes().iter().position(|node| node.site() == site)
                    .and_then(|node| values[node].clone()).ok_or_else(|| freeze("print-expression/operand-missing"));
                let left = operand(binary.lhs())?;
                let right = operand(binary.rhs())?;
                let operation = frame.operations().iter().find(|loan| loan.node() == index)
                    .ok_or_else(|| freeze("print-expression/operator-source-missing"))?;
                match operation.single_envelope().ok_or_else(|| freeze("expression/finite-envelope-execution-pending"))?.normal_result() {
                    ResultKind::NormalInteger => {
                        let (Operand::NormalInteger(lhs), Operand::NormalInteger(rhs)) = (left, right) else {
                            return Err(freeze("print-expression/integer-operands"));
                        };
                        let completed = crate::mir::builder::ops::arithmetic::build_arithmetic_op_recorded(
                            builder, operation.integer_operator_v1().ok_or_else(|| freeze("print-expression/operator-source-missing"))?, lhs, rhs)?;
                        let binding = completed.original().ok_or_else(|| freeze("print-expression/integer-append"))?.clone();
                        let value = binding.1.dst_value().ok_or_else(|| freeze("print-expression/integer-result"))?;
                        bindings.push(binding);
                        Operand::NormalInteger(value)
                    }
                    ResultKind::OwnedStringValue => {
                        emit_string_value(builder, &mut writer, Operation::Add { left, right }, index,
                            fault_frame, &mut acquired, &homes, &mut bindings)?
                    }
                    _ => return Err(freeze("print-expression/operator-result")),
                }
            }
            _ => return Err(freeze("print-expression/source-node")),
        };
        values[index] = Some(value);
    }
    crate::mir::normal_callable_semantic_package::validate_expression_acquisition_scope_v1(frame, comparison, None, &acquired, &values,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    Ok(EmittedExpressionBodyV1 { values, calls, path_reads, reference_reads, acquired, homes, fault_frame, entry, bindings })
}

pub(super) fn pending_operations(acquired: &[(usize, ValueId)], homes: &[InvokeOperation]) -> Vec<InvokeOperation> {
    acquired.iter().rev().map(|(_, value)| InvokeOperation::String(Operation::End { value: *value }))
        .chain(homes.iter().cloned()).collect()
}


fn emit_string_value(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: Operation,
    node: usize, fault_frame: ValueId, acquired: &mut Vec<(usize, ValueId)>,
    homes: &[InvokeOperation], bindings: &mut Vec<Binding>,
) -> Result<Operand, String> {
    let (value, result) = emit_string_operation_v1(builder, writer, operation, node, fault_frame, acquired, homes, bindings)?;
    match result {
        crate::mir::instruction::InvokeNormalResultKind::OwnedString => Ok(Operand::OwnedString(value)),
        crate::mir::instruction::InvokeNormalResultKind::I64 => Ok(Operand::NormalInteger(value)),
        _ => Err(freeze("expression/string-value-result")),
    }
}

/// One canonical String operation writer, shared with recursive Bool descent.
/// The enclosing purpose determines when this node is reached and owns cleanup.
pub(super) fn emit_string_operation_v1(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: Operation,
    node: usize, fault_frame: ValueId, acquired: &mut Vec<(usize, ValueId)>,
    homes: &[InvokeOperation], bindings: &mut Vec<Binding>,
) -> Result<(ValueId, crate::mir::instruction::InvokeNormalResultKind), String> {
    let pending = pending_operations(acquired, homes);
    let (value, result, _) = emit_string_operation_with_pending_v1(builder, writer, operation,
        fault_frame, &pending, bindings)?;
    if result == crate::mir::instruction::InvokeNormalResultKind::OwnedString { acquired.push((node, value)); }
    Ok((value, result))
}

/// Physical Invoke/Normal writer shared by source nodes and purpose acquisition.
/// Only the caller that owns a source node may add it to the source-node stack.
fn emit_string_operation_with_pending_v1(
    builder: &mut MirBuilder, writer: &mut RawRootGraphWriterV1, operation: Operation,
    fault_frame: ValueId, pending: &[InvokeOperation], bindings: &mut Vec<Binding>,
) -> Result<(ValueId, crate::mir::instruction::InvokeNormalResultKind, [Binding; 2]), String> {
    let result = operation.normal_result_kind().ok_or_else(|| freeze("expression/string-result"))?;
    let normal = writer.reserve_block(builder)?;
    let graph = root_graph::emit_cleanup_suffixes_with_writer(writer, builder, fault_frame,
        normal, &[], pending, &[], &(0..pending.len()).collect::<Vec<_>>(), bindings)?;
    let invoke = writer.emit_current(builder, MirInstruction::Invoke {
        operation: InvokeOperation::String(operation), fault_frame,
        normal_landing: normal, fault_landing: graph.acquisition_fault,
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
        _ => return Err(freeze("expression/string-result")),
    };
    builder.function_state.type_ctx.value_types.insert(value, kind);
    Ok((value, result, pair))
}

/// I64 purpose completion retains the scalar result and ends only its child
/// Text acquisitions. Prior Homes remain live on Normal and unwind on Fault.
pub(super) fn finish_scalar_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1)
    -> Result<(Binding, Binding), String> {
    finish_continuation(builder, expression, None, None)
}

/// The destination's original initializer frame is the sole issuer of this
/// transfer. Child temporaries remain inside this expression's cleanup scope.
pub(super) fn finish_residence_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1)
    -> Result<(Binding, Binding), String> {
    let Operand::OwnedString(root) = expression.value else { return Err(freeze("expression/residence-root")); };
    if expression.body.acquired.last() != Some(&(0, root)) { return Err(freeze("expression/residence-acquisition-order")); }
    finish_continuation(builder, expression, Some(root), None)
}

/// An owned argument lends its entire remaining stack to the enclosing Call.
/// The newest root prevents earlier children from ending before that Call.
pub(super) fn finish_deferred_argument_continuation(
    builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1,
) -> Result<(Binding, Binding), String> {
    let Operand::OwnedString(root) = expression.value else {
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
pub(super) fn materialize_text_root(
    builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1, required: bool,
) -> Result<(), String> {
    if !required { return Ok(()); }
    if !expression.value.is_text() || expression.purpose_acquisition.is_some() {
        return Err(freeze("rebind-expression/materialize-domain"));
    }
    let pending = pending_operations(&expression.body.acquired, &expression.body.homes);
    let (value, _, pair) = emit_string_operation_with_pending_v1(builder, &mut RawRootGraphWriterV1,
        Operation::Materialize { value: expression.value.clone() }, expression.body.fault_frame,
        &pending, &mut expression.body.bindings)?;
    expression.purpose_acquisition = Some(pair);
    expression.value = Operand::OwnedString(value);
    Ok(())
}

pub(super) fn finish_rebind_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1,
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
    ledger: &OrdinaryNewClaimLedgerV1,
) -> Result<(Binding, Binding), String> {
    if !frame.is_text_residence() { return finish_scalar_continuation(builder, expression); }
    let Operand::OwnedString(root) = expression.value else { return Err(freeze("expression/residence-root")); };
    if expression.purpose_acquisition.is_some() != frame.materializes_root()
        || !frame.materializes_root() && expression.body.acquired.last() != Some(&(0, root)) { return Err(freeze("expression/residence-acquisition-order")); }
    let displaced = ledger.text_rebind_displaced_end_v1(frame)?;
    finish_continuation(builder, expression, Some(root), displaced)
}

fn finish_continuation(builder: &mut MirBuilder, expression: &mut EmittedExpressionValueV1,
    residence: Option<ValueId>, displaced: Option<InvokeOperation>)
    -> Result<(Binding, Binding), String> {
    if residence.is_none() && !matches!(expression.value, Operand::NormalInteger(_)) {
        return Err(freeze("expression/scalar-continuation-result"));
    }
    finish_body_continuation(builder, &mut expression.body, residence, displaced, expression.purpose_acquisition.is_some())
}

pub(super) fn finish_comparison_continuation_v1(builder: &mut MirBuilder, body: &mut EmittedExpressionBodyV1)
    -> Result<(Binding, Binding), String> {
    finish_body_continuation(builder, body, None, None, false)
}

fn finish_body_continuation(builder: &mut MirBuilder, body: &mut EmittedExpressionBodyV1,
    residence: Option<ValueId>, displaced: Option<InvokeOperation>, materialized: bool,
) -> Result<(Binding, Binding), String> {
    let mut writer = RawRootGraphWriterV1;
    if let Some(value) = residence {
        let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
        let replaces = match displaced.as_ref() {
            Some(InvokeOperation::String(Operation::End { value })) => Some(*value),
            Some(_) => return Err(freeze("expression/residence-replacement-kind")),
            None => None,
        };
        let install = MirInstruction::StringResidenceInstall { value, replaces };
        builder.emit_instruction(install.clone())?;
        body.bindings.push((block, install));
        if !materialized { body.acquired.pop(); }
    }
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
    writer.append_terminated(builder, clean, continuation.1.clone(), &mut body.bindings)?;
    let mut normal: Vec<_> = body.acquired.iter().rev().map(|(_, value)| InvokeOperation::String(Operation::End { value: *value })).collect();
    let mut homes = body.homes.clone();
    if let Some(value) = residence { homes.insert(0, InvokeOperation::String(Operation::End { value })); }
    let fault = pending_operations(&body.acquired, &homes);
    let mut normal_indices: Vec<_> = (0..normal.len()).collect();
    if let Some(old) = displaced {
        let index = fault.iter().position(|operation| *operation == old)
            .ok_or_else(|| freeze("rebind-expression/old-home-missing"))?;
        if fault.iter().filter(|operation| **operation == old).count() != 1 || normal_indices.contains(&index) {
            return Err(freeze("rebind-expression/old-home-duplicate"));
        }
        normal_indices.push(index);
        normal.push(old);
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
