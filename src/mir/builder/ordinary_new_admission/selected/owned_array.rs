//! Physical construction of the exact source-owned Array holder.
use super::*;
use crate::mir::instruction::OwnedArrayInvokeOperation as Array;
use crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1;

pub(in crate::mir::builder) fn emit_construction(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    source: PreparedOwnedArrayConstructionV1,
) -> Result<ValueId, String> {
    let site = source.site();
    if state.owner() != site.owner() {
        return Err(freeze("owned-array-owner-drift"));
    }
    let descriptor = source.descriptor();
    ledger.begin_owned_array_construction(source)?;
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let allocation_fault = cleanup_chain(
        builder,
        frame,
        ledger.owned_array_construction_unwind(&site)?,
        outward,
        &mut bindings,
    )?;
    let array = super::invoke_operation::emit(
        builder,
        frame,
        InvokeOperation::IntrinsicArrayNew,
        allocation_fault,
        &mut bindings,
    )?
    .0
    .ok_or_else(|| freeze("owned-array-allocation-result"))?;
    // A failed Claim has no owned-slot state. Release only its acquired outer
    // storage, then unwind the original prior Homes in source-issued order.
    let claim_fault = builder.next_block_id();
    let release = MirInstruction::ArrayResidenceRelease { value: array };
    bindings.push((claim_fault, release.clone()));
    append_block(
        builder,
        claim_fault,
        MirInstruction::Jump {
            target: allocation_fault,
            edge_args: None,
        },
        &mut bindings,
    )?;
    builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| freeze("no-function"))?
        .blocks
        .get_mut(&claim_fault)
        .expect("fresh cleanup block")
        .add_instruction(release);
    super::invoke_operation::emit(
        builder,
        frame,
        InvokeOperation::OwnedArray(Array::Claim { array, descriptor }),
        claim_fault,
        &mut bindings,
    )?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(array, MirType::Box("ArrayBox".into()));
    ledger.record_owned_array_construction(&site, array, bindings)?;
    Ok(array)
}

/// Materialize the original read as part of its enclosing argument packet.
/// The caller retains its argument ordinal/formal and exact physical group.
pub(in crate::mir::builder) fn emit_read(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    call: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    source: crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedOwnedArrayReadV1>, String> {
    if state.owner() != call.owner() || call.owner() != source.site().owner() {
        return Err(freeze("owned-array-read-owner"));
    }
    let array = ledger.live_owned_array_read_value_v1(&source)?;
    let receiver = state
        .take_exact_lexical_read(call.owner(), source.receiver()?.node(), source.holder())
        .map_err(|issue| format!("{}: {issue:?}", freeze("owned-array-read-receiver")))?;
    let frame = state.borrow_fault_frame(builder)?;
    let mut fault_bindings = vec![fault_frame_binding(builder, state, frame)?];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut fault_bindings,
    )?;
    let fault = cleanup_chain(
        builder,
        frame,
        ledger.owned_array_read_fault_unwind_v1(call, &source)?,
        outward,
        &mut fault_bindings,
    )?;
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let index = crate::mir::builder::emission::constant::emit_integer(builder, source.index()?)?;
    let index_binding = (
        block,
        MirInstruction::Const {
            dst: index,
            value: crate::mir::ConstValue::Integer(source.index()?),
        },
    );
    let mut read_bindings = Vec::new();
    let (value, projection) = super::invoke_operation::emit(
        builder,
        frame,
        InvokeOperation::OwnedArray(Array::Get {
            array,
            descriptor: source.descriptor(),
            index,
        }),
        fault,
        &mut read_bindings,
    )?;
    let value = value.ok_or_else(|| freeze("owned-array-read-normal-result"))?;
    let projection = projection.ok_or_else(|| freeze("owned-array-read-normal-projection"))?;
    let invoke = read_bindings
        .first()
        .cloned()
        .ok_or_else(|| freeze("owned-array-read-invoke"))?;
    let observed = crate::mir::normal_callable_semantic_package::EmittedOwnedArrayReadV1::new(
        source,
        call.clone(),
        receiver,
        index_binding,
        invoke,
        projection,
        fault_bindings,
        ledger,
    )?;
    observed.check_raw_v1(builder.function_state.current_function.as_ref()
        .ok_or_else(|| freeze("no-function"))?, ledger)?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Unknown);
    for binding in observed.bindings() {
        // One shared function frame is a common dependency, not a repeated
        // effect. Preserve every actual Get/index/Normal/cleanup operation.
        if matches!(binding.1, MirInstruction::FaultFrameEnter { .. }) && bindings.contains(binding) { continue; }
        bindings.push(binding.clone());
    }
    let observed = std::rc::Rc::new(observed);
    ledger.record_owned_array_read_v1(std::rc::Rc::clone(&observed))?;
    Ok(observed)
}

/// A discarded original Append consumes one ephemeral candidate on Normal.
/// Fault cancels that candidate completely before the original holder prefix.
pub(in crate::mir::builder) fn emit_append(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    source: crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1,
    candidate_row: Option<crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1>,
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedOwnedArrayAppendV1>, String> {
    use crate::mir::normal_callable_semantic_package::{
        EmittedOwnedArrayAppendV1, OwnedArrayAppendCandidateV1 as Candidate,
    };
    use std::rc::Rc;
    if state.owner() != source.site().owner() { return Err(freeze("array-append/owner")); }
    let array = ledger.live_owned_array_append_value_v1(&source)?;
    // Receiver is evaluated before the candidate; no AST child is re-lowered.
    let receiver = state.take_exact_lexical_read(source.site().owner(), source.receiver().node(), source.holder())
        .map_err(|issue| issue.to_string())?;
    if source.null_candidate() != candidate_row.is_none() {
        return Err(freeze("array-append/candidate-source-kind"));
    }
    let mut bindings = Vec::new();
    let (candidate, value, cancellation) = if source.null_candidate() {
        let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
        let value = builder.next_value_id();
        let null = MirInstruction::Const { dst: value, value: crate::mir::ConstValue::Null };
        builder.emit_instruction(null.clone())?;
        bindings.push((block, null.clone()));
        (Candidate::Null((block, null)), value, Vec::new())
    } else {
        let (packet, value, cancellation) = super::terminal_call::emit_object_expression(
            builder, state, ledger, &source,
            candidate_row.ok_or_else(|| freeze("array-append/candidate-disposition-missing"))?,
            &mut bindings, consume_receiver,
        )?;
        (Candidate::Call(packet), value, cancellation)
    };
    let frame = state.borrow_fault_frame(builder)?;
    let frame_binding = fault_frame_binding(builder, state, frame)?;
    if !bindings.contains(&frame_binding) { bindings.push(frame_binding); }
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut bindings)?;
    let unwind = cancellation.into_iter().chain(ledger.owned_array_append_unwind_v1(&source)?).collect();
    let fault = cleanup_chain(builder, frame, unwind, outward, &mut bindings)?;
    let mut append = Vec::new();
    let (result, projection) = super::invoke_operation::emit(builder, frame,
        InvokeOperation::OwnedArray(Array::Append { array, descriptor: source.descriptor(), candidate: value }),
        fault, &mut append)?;
    if result.is_some() || projection.is_some() { return Err(freeze("array-append/unit-result-drift")); }
    let invoke = append.pop().ok_or_else(|| freeze("array-append/invoke-missing"))?;
    bindings.push(invoke.clone());
    let normal = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let next = builder.next_block_id();
    let continuation = MirInstruction::Jump { target: next, edge_args: None };
    builder.emit_instruction(continuation.clone())?;
    bindings.push((normal, continuation.clone()));
    builder.start_new_block(next)?;
    let packet = EmittedOwnedArrayAppendV1::new(source, receiver, candidate, invoke,
        (normal, continuation), bindings.clone(), ledger,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    let packet = Rc::new(packet);
    ledger.record_owned_array_append_bindings_v1(Rc::clone(&packet), bindings)?;
    Ok(packet)
}
