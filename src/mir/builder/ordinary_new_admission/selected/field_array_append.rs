//! Selected source field-Array Push delegates to the existing sole Invoke and
//! cleanup writers. Its original normalizer output is never lowered again.
use super::*;
use crate::mir::normal_callable_semantic_package::{FieldArrayAppendSourceLoanV1, EmittedFieldArrayAppendV1, NamedArrayWriteEmissionPortV1};
use crate::mir::builder::control_flow::plan::SourceValueProjectionV1;
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: FieldArrayAppendSourceLoanV1,
    receiver_read: Rc<crate::mir::builder::ExactLexicalReadV1>,
    receiver: ValueId, value: ValueId, argument: &SourceValueProjectionV1,
    emission: &NamedArrayWriteEmissionPortV1,
) -> Result<(), String> {
    source.validate_source(ledger)?;
    if state.owner() != source.site().owner() || argument.root_value() != value
        || receiver_read.value_for(source.site().owner(), source.receiver_site().node(), source.integer_requirement()?.binding())
            .map_err(|issue| issue.to_string())? != receiver { return Err(freeze("field-array-append/source-operands")); }
    let argument = argument.observe(builder, state.owner(), source.integer_argument()?)?;
    // Reject unavailable End plans before changing the control graph.
    let operations = ledger.field_array_append_unwind_v1(&source)?;
    let pinned_receiver = builder.local_recv(receiver);
    let pinned_value = builder.local_arg(value);
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let index = &mut builder.function_state.local_ssa_correspondence;
    let receiver_pin = index.loan(function, receiver, pinned_receiver).ok_or_else(|| freeze("field-array-append/receiver-pin"))?;
    let value_pin = index.loan(function, value, pinned_value).ok_or_else(|| freeze("field-array-append/value-pin"))?;
    let mut bindings = argument.bindings().to_vec();
    for pin in [&receiver_pin, &value_pin] {
        index.require_member(pin)?; pin.verify(function)?; pin.append_observed_bindings_v1(&mut bindings);
    }
    let (invoke, continuation, fault_bindings) = emit_push(builder, state, operations, pinned_receiver,
        crate::mir::instruction::ArrayElementWriteValueV1::Scalar(pinned_value), &mut bindings)?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedFieldArrayAppendV1::new(source, receiver_read, argument,
        [receiver_pin, value_pin], invoke, continuation, bindings, fault_bindings, ledger, function)?);
    let marker = emission.record_checked_write_v1(Rc::clone(&packet))?;
    ledger.record_field_array_append_bindings_v1(packet)?;
    builder.function_state.current_function.as_mut().ok_or_else(|| freeze("no-function"))?
        .metadata.named_array_write_obligations.push(marker);
    Ok(())
}

/// Sole checked field-Push/cleanup graph for both source argument kinds.
fn emit_push(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    operations: Vec<InvokeOperation>, pinned_receiver: ValueId,
    payload: crate::mir::instruction::ArrayElementWriteValueV1,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<((BasicBlockId, MirInstruction), (BasicBlockId, MirInstruction), Vec<(BasicBlockId, MirInstruction)>), String> {
    let frame = state.borrow_fault_frame(builder)?;
    let frame_binding = fault_frame_binding(builder, state, frame)?;
    if !bindings.contains(&frame_binding) { bindings.push(frame_binding); }
    let mut fault_bindings = Vec::new();
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut fault_bindings)?;
    let fault = cleanup_chain(builder, frame, operations, outward, &mut fault_bindings)?;
    bindings.extend(fault_bindings.iter().cloned());
    let site_id = builder.next_array_write_site_id();
    let mut invoke = Vec::new();
    let (result, projection) = super::invoke_operation::emit(builder, frame,
        InvokeOperation::ArrayElementWrite { site_id, kind: crate::mir::ArrayElementWriteKind::Push,
            producer: crate::mir::ArrayWriteProducerKind::MethodCall, receiver: pinned_receiver, index: None, value: payload }, fault, &mut invoke)?;
    if result.is_some() || projection.is_some() { return Err(freeze("field-array-append/unit-result")); }
    let invoke = invoke.pop().ok_or_else(|| freeze("field-array-append/invoke-missing"))?;
    bindings.push(invoke.clone());
    let normal = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let next = builder.next_block_id();
    let continuation = MirInstruction::Jump { target: next, edge_args: None };
    builder.emit_instruction(continuation.clone())?;
    let continuation = (normal, continuation);
    bindings.push(continuation.clone());
    builder.start_new_block(next)?;
    Ok((invoke, continuation, fault_bindings))
}

pub(in crate::mir::builder) fn emit_text(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: FieldArrayAppendSourceLoanV1,
) -> Result<(), String> {
    source.validate_source(ledger)?;
    let frame = source.text_frame().ok_or_else(|| freeze("field-array-append/source-operands"))?;
    if state.owner() != source.site().owner() { return Err(freeze("field-array-append/source-operands")); }
    // Consume the real Me read, then the source-qualified field receiver.
    let mut receiver = None;
    let (base, field) = ledger.take_local_field_read(frame.source().receiver(), |binding| {
        if binding != frame.receiver() { return Err(freeze("field-array-append/source-operands")); }
        let read = state.take_exact_lexical_read(source.site().owner(), frame.receiver_read().node(), binding)
            .map_err(|issue| issue.to_string())?;
        let value = read.value_for(source.site().owner(), frame.receiver_read().node(), binding)
            .map_err(|issue| issue.to_string())?;
        receiver = Some(Rc::new(read));
        Ok(value)
    })?.ok_or_else(|| freeze("field-array-append/source-operands"))?;
    let (array, read_binding) = builder.complete_exact_field_read_v1(base, field.field(), frame.source().receiver(), ledger,
        crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(Rc::clone(&field)))?;
    let receiver = receiver.ok_or_else(|| freeze("field-array-append/source-operands"))?;
    let expression = super::expression_value::emit(builder, state, ledger, frame.evaluation(),
        &mut |_, _, _, _, _, _| Err(freeze("field-array-append/source-operands")))?;
    if !expression.value.is_text() || expression.purpose_acquisition.is_some() || !expression.body.acquired.is_empty()
        || !expression.body.calls.is_empty() || !expression.body.path_reads.is_empty() { return Err(freeze("field-array-append/source-operands")); }
    let payload = crate::mir::instruction::ArrayElementWriteValueV1::Text(expression.value);
    let mut body = expression.body;
    let (terminal, continuation) = super::expression_value::finish_value_continuation_v1(builder, &mut body)?;
    let argument = crate::mir::normal_callable_semantic_package::FieldAppendTextProjectionV1::new(field, read_binding.clone(),
        body.values, body.reference_reads, body.fault_frame, body.entry, terminal, continuation, body.bindings.clone());
    let mut bindings = receiver.original_copy_dependencies_v1()?.to_vec();
    bindings.push(read_binding);
    for row in body.bindings { if !bindings.contains(&row) { bindings.push(row); } }
    argument.append_dependencies(&mut bindings)?;
    let operations = ledger.field_array_append_unwind_v1(&source)?;
    let pinned = builder.local_recv(array);
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let index = &mut builder.function_state.local_ssa_correspondence;
    let pin = index.loan(function, array, pinned).ok_or_else(|| freeze("field-array-append/receiver-pin"))?;
    index.require_member(&pin)?;
    pin.verify(function)?;
    pin.append_observed_bindings_v1(&mut bindings);
    let (invoke, continuation, fault_bindings) = emit_push(builder, state, operations, pinned, payload, &mut bindings)?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedFieldArrayAppendV1::new_text(source, receiver, argument, pin,
        invoke, continuation, bindings, fault_bindings, ledger, function)?);
    let marker = crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1::checked_text_append_marker_v1(&packet)?;
    ledger.record_field_array_append_bindings_v1(packet)?;
    builder.function_state.current_function.as_mut().ok_or_else(|| freeze("no-function"))?
        .metadata.named_array_write_obligations.push(marker);
    Ok(())
}
