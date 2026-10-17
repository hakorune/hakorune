//! Sole ordered expression descent, independent of the terminal consumer.
//! Returns live acquisitions; the purpose wrapper owns cleanup and publication.
use super::*;
use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as ResultKind;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionEvaluationV1, LocalCallObservationV1};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, ResolvedLiteralSourceV1 as Literal};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

/// No independent binding group is published here. Every retained original
/// receipt and live temporary moves to the checked terminal consumer together.
pub(super) struct EmittedExpressionValueV1 {
    pub(super) value: Operand,
    pub(super) values: Vec<Option<Operand>>,
    pub(super) calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
    pub(super) path_reads: std::collections::BTreeMap<OwnedExprSiteV1, crate::mir::builder::ExactLexicalReadV1>,
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
        &LocalCallObservationV1, &[(usize, ValueId)], &mut Vec<Binding>,
    ) -> Result<EmittedLexicalCallProjectionV1, String>,
) -> Result<EmittedExpressionValueV1, String> {
    let owner = frame.source().owner();
    if state.owner() != owner { return Err(freeze("expression-value/state-owner")); }
    let homes = ledger.prior_home_unwind_for(frame.prior_homes())?;
    let source = frame.source();
    let mut pending = Vec::new();
    let mut path_slots = std::collections::BTreeMap::new();
    let mut direct_fields = std::collections::BTreeMap::new();
    // Fulfill every field-root read before any FaultFrame, CFG or value effect.
    // Reservation remains in original source order; binding nested receivers
    // still waits for the actual ancestor field writer below.
    for &index in frame.ordered_nodes() {
        if frame.transfers_node(index) { continue; }
        let node = &source.nodes()[index];
        if frame.node_is_identity(index) && !matches!(node.source(), Node::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) { continue; }
        let site = OwnedExprSiteV1::new(owner, node.site().clone());
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
    for &index in frame.ordered_nodes() {
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
            Node::MethodCall => {
                let call = frame.calls().iter().find(|call| call.site().site() == node.site())
                    .ok_or_else(|| freeze("print-expression/call-source-missing"))?;
                let packet = Rc::new(emit_call(builder, state, call, &acquired, &mut bindings)?);
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
                match operation.envelope().normal_result() {
                    ResultKind::NormalInteger => {
                        let (Operand::NormalInteger(lhs), Operand::NormalInteger(rhs)) = (left, right) else {
                            return Err(freeze("print-expression/integer-operands"));
                        };
                        let completed = crate::mir::builder::ops::arithmetic::build_arithmetic_op_recorded(
                            builder, crate::mir::BinaryOp::Add, lhs, rhs)?;
                        let binding = completed.original().ok_or_else(|| freeze("print-expression/integer-append"))?.clone();
                        let value = binding.1.dst_value().ok_or_else(|| freeze("print-expression/integer-result"))?;
                        bindings.push(binding);
                        Operand::NormalInteger(value)
                    }
                    ResultKind::OwnedStringValue => {
                        let normal = writer.reserve_block(builder)?;
                        let pending = pending_operations(&acquired, &homes);
                        let graph = root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, fault_frame,
                            normal, &[], &pending, &[], &(0..pending.len()).collect::<Vec<_>>(), &mut bindings)?;
                        let invoke = writer.emit_current(builder, MirInstruction::Invoke {
                            operation: InvokeOperation::String(Operation::Add { left, right }), fault_frame,
                            normal_landing: normal, fault_landing: graph.acquisition_fault,
                        })?;
                        let value = builder.next_value_id();
                        builder.start_new_block(normal)?;
                        let projection = MirInstruction::InvokeNormalResult { invoke_block: invoke.0, dst: value };
                        writer.insert_projection(builder, normal, projection.clone())?;
                        bindings.extend([invoke, (normal, projection)]);
                        builder.function_state.type_ctx.value_types.insert(value, MirType::String);
                        acquired.push((index, value));
                        Operand::OwnedString(value)
                    }
                    _ => return Err(freeze("print-expression/operator-result")),
                }
            }
            _ => return Err(freeze("print-expression/source-node")),
        };
        values[index] = Some(value);
    }
    let value = values[0].clone().ok_or_else(|| freeze("print-expression/root-value"))?;
    Ok(EmittedExpressionValueV1 {
        value, values, calls, path_reads, acquired, homes, fault_frame, entry, bindings,
    })
}

pub(super) fn pending_operations(acquired: &[(usize, ValueId)], homes: &[InvokeOperation]) -> Vec<InvokeOperation> {
    acquired.iter().rev().map(|(_, value)| InvokeOperation::String(Operation::End { value: *value }))
        .chain(homes.iter().cloned()).collect()
}
