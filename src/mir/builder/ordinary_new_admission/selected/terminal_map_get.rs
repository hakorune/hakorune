//! Stored Map get purposes share the original key/receiver and sole Invoke writer.
use super::*;
use crate::mir::normal_callable_semantic_package::{CallValidationContextV1, StoredMapGetReturnSourceLoanV1,
    StoredMapGetReturnEmissionV1, StoredMapGetReadEmissionV1, StoredMapGetLocalEmissionV1, StoredMapGetSourceLoanV1};
use crate::mir::resolved_semantics::home_new_prefix::{MapFieldGetLocalV1, CheckedExpressionEvaluationV1};
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

fn emit_read(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: &StoredMapGetSourceLoanV1,
    key: &Rc<CheckedExpressionEvaluationV1>,
) -> Result<(StoredMapGetReadEmissionV1, ValueId), String> {
    let receiver_source = source.receiver();
    let receiver = state.take_exact_lexical_read(source.get().owner(), receiver_source.receiver_site().node(), receiver_source.receiver())
        .map_err(|error| error.to_string())?;
    receiver_source.require_receiver_v1(CallValidationContextV1::Lowering(ledger),
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?, &receiver)?;
    // The retained original key is passive. Do not materialize or transfer it.
    let expression = expression_value::emit(builder, state, ledger, key,
        &mut |_, _, _, _, _, _| Err(freeze("map-get-shape")))?;
    if expression.purpose_acquisition.is_some() || !expression.body.acquired.is_empty()
        || !expression.body.calls.is_empty() || !expression.body.path_reads.is_empty() {
        return Err(freeze("map-get-shape"));
    }
    let mut body = expression.body;
    let frame = body.fault_frame;
    let (terminal, continuation) = expression_value::finish_value_continuation_v1(builder, &mut body)?;
    let read = StoredMapGetReadEmissionV1::issue(source.clone(), Rc::clone(key), receiver,
        body.values, body.reference_reads, body.entry, terminal, continuation, body.bindings, ledger,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    Ok((read, frame))
}

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    source: StoredMapGetReturnSourceLoanV1,
) -> Result<ValueId, String> {
    let owner = source.terminal().owner();
    let exit = source.terminal().return_site().clone();
    let (read, _) = emit_read(builder, state, ledger, source.get_source(), source.parts().1)?;
    let stored = StoredMapGetReturnEmissionV1::issue(source, read, ledger,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    let operation = stored.operation(CallValidationContextV1::Lowering(ledger),
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    let value = builder.next_value_id();
    // The result has no owning lease; do not bind a fresh object result.
    emit_root_home_exit_payload(builder, state, ledger, owner, &exit, Some(value), value,
        Some(RootExitIngress::MapGet { operation, stored: Some(Box::new(stored)) }))
}

/// Affine transport until the real Local writer completes. No group or receipt
/// is published here, and the raw borrowed Normal is never an integer operand.
pub(in crate::mir::builder) struct PendingLocalV1 {
    ordinal: u32, frame: Rc<MapFieldGetLocalV1>, read: StoredMapGetReadEmissionV1,
    value: ValueId, invoke: Binding, normal: Binding, faults: Vec<Binding>,
}
impl PendingLocalV1 {
    pub(in crate::mir::builder) fn ordinal(&self) -> u32 { self.ordinal }
    pub(in crate::mir::builder) fn value(&self) -> ValueId { self.value }
    pub(in crate::mir::builder) fn publish(self,
        receipt: &crate::mir::builder::stmts::CompletedLocalBindingV1,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<(), String> {
        if receipt.ordinal() != self.ordinal || receipt.initializer() != self.value {
            return Err(freeze("initializer-expression/local-receipt"));
        }
        let copy = receipt.copy().cloned().ok_or_else(|| freeze("initializer-expression/local-copy-receipt"))?;
        let packet = StoredMapGetLocalEmissionV1::issue(self.frame, self.read, self.value, receipt.local(), copy,
            self.invoke, self.normal, self.faults, ledger, function)?;
        ledger.record_stored_map_get_local_bindings_v1(Rc::new(packet))
    }
}
pub(in crate::mir::builder) fn emit_local(builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<MapFieldGetLocalV1>, ordinal: u32,
) -> Result<PendingLocalV1, String> {
    ledger.validate_stored_map_get_local_source_v1(&frame)?;
    let operations = ledger.prior_home_unwind_for(frame.prior_homes())?;
    let (read, fault_frame) = emit_read(builder, state, ledger, frame.source(), frame.key())?;
    let operation = read.operation(CallValidationContextV1::Lowering(ledger),
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    let mut faults = Vec::new();
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame }, &mut faults)?;
    let fault = cleanup_chain(builder, fault_frame, operations, outward, &mut faults)?;
    let mut invoke_rows = Vec::new();
    let (value, normal) = invoke_operation::emit(builder, fault_frame, operation, fault, &mut invoke_rows)?;
    let value = value.ok_or_else(|| freeze("map-get-shape"))?;
    let normal = normal.ok_or_else(|| freeze("map-get-shape"))?;
    if invoke_rows.len() != 2 { return Err(freeze("map-get-shape")); }
    let invoke = invoke_rows.remove(0);
    Ok(PendingLocalV1 { ordinal, frame, read, value, invoke, normal, faults })
}
