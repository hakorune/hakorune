//! A real ephemeral Object packet; it never installs a LOCAL or received Home.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    EmittedLexicalCallProjectionV1, OwnedArrayAppendSourceLoanV1,
};
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    source: &OwnedArrayAppendSourceLoanV1,
    row: LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(Rc<EmittedLexicalCallProjectionV1>, ValueId, Vec<InvokeOperation>), String> {
    let site = source.candidate();
    if state.owner() != site.owner() || source.null_candidate() {
        return Err(freeze("array-append/candidate-owner"));
    }
    ledger.live_owned_array_append_value_v1(source)?;
    source.check_candidate_v1(&row)?;
    let packet = ledger.borrowed_object_call_packet_v1(&row)?;
    let result_kind = packet.result();
    let class = match result_kind {
        InvokeCallResultKind::Handle => ledger.callable_result_class(row.target()),
        InvokeCallResultKind::NullableHandle => ledger.nullable_result_class(row.target()),
        _ => return Err(freeze("array-append/candidate-result-kind")),
    }.ok_or_else(|| freeze("array-append/candidate-class-missing"))?;
    let carrier = result_type(result_kind, Some(class))?;
    let arguments = packet.arguments();
    let (receiver, stored) = lexical_receiver::prepare(builder, state, site.owner(), &row, bindings)?;
    let frame = state.borrow_fault_frame(builder)?;
    let frame_binding = fault_frame_binding(builder, state, frame)?;
    if !bindings.contains(&frame_binding) { bindings.push(frame_binding); }
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, bindings)?;
    // Candidate does not exist on Call Fault. Only original prior Homes unwind.
    let fault = cleanup_chain(builder, frame, ledger.owned_array_append_unwind_v1(source)?, outward, bindings)?;
    let mut prepared = lexical_i64::prepare_arguments(
        builder, state, ledger, site.owner(), receiver, arguments, &row, bindings,
    )?;
    if let Some(read) = stored { prepared = prepared.with_stored_receiver(read)?; }
    let call = prepared.materialize_with_ledger(site.owner(), &row, arguments, ledger)?;
    let result = builder.next_value_id();
    let normal = builder.next_block_id();
    let cancellation = packet.cancellation_operations_v1(result);
    let (invoke, projection) = emit_call_normal(
        builder, call, result_kind, frame, normal, fault, result, carrier,
    )?;
    bindings.push(invoke.clone());
    bindings.push(projection.clone());
    let emitted = EmittedLexicalCallProjectionV1::new(row, prepared, invoke, projection);
    // Reborrow the SAME Taken row after moving it into its immutable packet.
    let original = emitted.original_row()?;
    if ledger.borrowed_object_call_packet_v1(original)?.cancellation_operations_v1(result) != cancellation {
        return Err(freeze("array-append/candidate-cancellation-drift"));
    }
    Ok((Rc::new(emitted), result, cancellation))
}
