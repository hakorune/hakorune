//! Sole checked FieldSet writer; source permission remains with its caller's packet.
use super::*;
use crate::mir::normal_callable_semantic_package::CheckedFieldWriteGraphV1;

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    field: hakorune_mir_defs::CanonicalFieldRefV1, mode: crate::mir::instruction::FieldSetModeV1,
    receiver: ValueId, value: ValueId,
    operations: Vec<InvokeOperation>,
) -> Result<CheckedFieldWriteGraphV1, String> {
    // Pin/cache targets and the Fault frame may be inserted ahead of an
    // already observed RHS. Retain only this writer's genuinely new prefixes.
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let prior_values: std::collections::BTreeSet<_> = function.params.iter().copied().chain(
        function.blocks.values().flat_map(|body| body.all_instructions()).filter_map(|row| row.dst_value())).collect();
    let pinned_receiver = builder.local_field_base(receiver);
    let pinned_value = builder.local_arg(value);
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let index = &mut builder.function_state.local_ssa_correspondence;
    let pins = [index.loan(function, receiver, pinned_receiver).ok_or_else(|| freeze("borrowed-field-write/receiver-pin"))?,
        index.loan(function, value, pinned_value).ok_or_else(|| freeze("borrowed-field-write/value-pin"))?];
    let mut bindings = Vec::new();
    for pin in &pins { index.require_member(pin)?; pin.verify(function)?; pin.append_observed_bindings_v1(&mut bindings); }
    let frame = state.borrow_fault_frame(builder)?;
    let frame_binding = fault_frame_binding(builder, state, frame)?;
    if !bindings.contains(&frame_binding) { bindings.push(frame_binding.clone()); }
    let mut post_rhs_prefix = Vec::new();
    for row in pins.iter().filter_map(|pin| pin.target_binding()).chain(std::iter::once(&frame_binding)) {
        if matches!(row.1, MirInstruction::Copy { .. } | MirInstruction::FaultFrameEnter { .. })
            && row.1.dst_value().is_some_and(|value| !prior_values.contains(&value))
            && !post_rhs_prefix.contains(row) { post_rhs_prefix.push(row.clone()); }
    }
    let mut fault_bindings = Vec::new();
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut fault_bindings)?;
    let fault = cleanup_chain(builder, frame, operations, outward, &mut fault_bindings)?;
    bindings.extend(fault_bindings.iter().cloned());
    let mut invoke = Vec::new();
    let (result, projection) = super::invoke_operation::emit(builder, frame,
        InvokeOperation::FieldSet { mode, field, base: pinned_receiver, value: pinned_value }, fault, &mut invoke)?;
    if result.is_some() || projection.is_some() { return Err(freeze("borrowed-field-write/unit-result")); }
    let invoke = invoke.pop().ok_or_else(|| freeze("borrowed-field-write/invoke-missing"))?;
    bindings.push(invoke.clone());
    let normal = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let next = builder.next_block_id();
    let continuation = MirInstruction::Jump { target: next, edge_args: None };
    builder.emit_instruction(continuation.clone())?;
    let continuation = (normal, continuation);
    bindings.push(continuation.clone());
    builder.start_new_block(next)?;
    Ok(CheckedFieldWriteGraphV1::new(pins, invoke, continuation, bindings, fault_bindings, post_rhs_prefix))
}
