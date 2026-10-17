//! Execute the retained whole-expression recipe through the existing writers.
//! No AST interpretation, target selection or value-class inference lives here.
use super::*;
use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as ResultKind;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::normal_callable_semantic_package::{
    EmittedPrintExpressionProjectionV1, LexicalInstanceCallDispositionRowV1,
};
use crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, ResolvedLiteralSourceV1 as Literal};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedPrintExpressionFrameV1>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<(), String> {
    ledger.validate_print_expression_source_v1(&frame)?;
    let owner = frame.source().owner();
    if state.owner() != owner { return Err(freeze("print-expression/state-owner")); }
    let homes = ledger.prior_home_unwind_for(frame.prior_homes())?;
    let fault_frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, fault_frame)?];
    let mut writer = RawRootGraphWriterV1;
    let start = writer.reserve_block(builder)?;
    let entry = writer.emit_current(builder, MirInstruction::Jump { target: start, edge_args: None })?;
    bindings.push(entry.clone());
    builder.start_new_block(start)?;
    let source = frame.source();
    let mut values: Vec<Option<Operand>> = vec![None; source.nodes().len()];
    let mut field_outputs: Vec<Option<ValueId>> = vec![None; source.nodes().len()];
    let mut path_reads = std::collections::BTreeMap::<OwnedExprSiteV1, crate::mir::builder::ExactLexicalReadV1>::new();
    let mut acquired = Vec::new();
    let mut calls = Vec::new();
    for &index in frame.ordered_nodes() {
        if frame.transfers_node(index) { continue; }
        let node = &source.nodes()[index];
        if frame.node_is_identity(index) && !matches!(node.source(), Node::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) { continue; }
        let site = OwnedExprSiteV1::new(owner, node.site().clone());
        if let Some(path) = ledger.statement_field_path_v1(&site, crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))? {
            let mut root = None;
            if !ledger.reserve_result_path_field_v1(&site, |binding| {
                let root_site = OwnedExprSiteV1::new(owner, path.root_site().clone());
                if !path_reads.contains_key(&root_site) {
                    let read = state.take_exact_lexical_read(owner, path.root_site().node(), binding).map_err(|error| error.to_string())?;
                    path_reads.insert(root_site.clone(), read);
                }
                let read = &path_reads[&root_site];
                let value = read.value_for(owner, path.root_site().node(), binding).map_err(|error| error.to_string())?;
                root = Some(value);
                Ok(value)
            })? { return Err(freeze("print-expression/path-source-missing")); }
            let base = if let Some(parent) = path.parent() {
                source.nodes().iter().position(|node| node.site() == parent.site())
                    .and_then(|index| field_outputs[index]).ok_or_else(|| freeze("print-expression/path-parent-missing"))?
            } else { root.ok_or_else(|| freeze("print-expression/path-root-missing"))? };
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
            Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) => {
                let site = OwnedExprSiteV1::new(owner, node.site().clone());
                let mut original_read = None;
                let (base, source) = ledger.take_local_field_read(&site, |binding| {
                    let read = state.take_exact_lexical_read(owner, object.node(), binding).map_err(|error| error.to_string())?;
                    let value = read.value_for(owner, object.node(), binding).map_err(|error| error.to_string())?;
                    original_read = Some(read);
                    Ok(value)
                })?.ok_or_else(|| freeze("print-expression/field-source-missing"))?;
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
                let packet = Rc::new(terminal_call::emit_expression_call(builder, state, ledger, &frame,
                    call, &acquired, &mut bindings, consume_receiver)?);
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
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
    writer.append_terminated(builder, clean, continuation.1.clone(), &mut bindings)?;
    let normal: Vec<_> = acquired.iter().rev().map(|(_, value)|
        InvokeOperation::String(Operation::End { value: *value })).collect();
    let fault = pending_operations(&acquired, &homes);
    let after: Vec<_> = (0..normal.len()).map(|index| ((index + 1)..fault.len()).collect()).collect();
    let graph = root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, fault_frame, clean,
        &normal, &fault, &after, &(0..fault.len()).collect::<Vec<_>>(), &mut bindings)?;
    let print = writer.emit_current(builder, MirInstruction::Invoke {
        operation: InvokeOperation::String(Operation::Print { value }), fault_frame,
        normal_landing: graph.clean, fault_landing: graph.acquisition_fault,
    })?;
    bindings.push(print.clone());
    builder.start_new_block(next)?;
    // Shared FaultFrame/read observations may be lent by several call nodes;
    // the physical instruction itself is retained once in this enclosing pool.
    let mut unique = Vec::with_capacity(bindings.len());
    for binding in bindings { if !unique.contains(&binding) { unique.push(binding); } }
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedPrintExpressionProjectionV1::new(frame, values, calls, path_reads.into_values().collect(), entry, print,
        continuation, unique.clone(), ledger, function)?);
    ledger.prepare_print_expression_bindings(owner, packet, unique)?.commit()
}

fn pending_operations(acquired: &[(usize, ValueId)], homes: &[InvokeOperation]) -> Vec<InvokeOperation> {
    acquired.iter().rev().map(|(_, value)| InvokeOperation::String(Operation::End { value: *value }))
        .chain(homes.iter().cloned()).collect()
}
