//! One guarded original field assignment through the shared checked writer.
use super::*;
use crate::mir::normal_callable_semantic_package::{BorrowedFieldWriteSourceLoanV1, EmittedBorrowedFieldWriteV1};
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: BorrowedFieldWriteSourceLoanV1,
    field_name: &str, reads: [Rc<crate::mir::builder::ExactLexicalReadV1>; 2],
) -> Result<(), String> {
    source.validate_source(ledger)?;
    let owner = source.frame().source().owner();
    if state.owner() != owner || source.field_name() != field_name { return Err(freeze("borrowed-field-write/source-site")); }
    let ty = crate::mir::numeric_substrate::exact_numeric_mir_type_from_declared_name(
        source.field_source().declared_type_name(), crate::mir::numeric_substrate::NumericTarget::host())
        .ok_or_else(|| freeze("borrowed-field-write/numeric-declaration"))?;
    if crate::mir::numeric_substrate::exact_numeric_type_requires_dynamic_integer_range_check(&ty)
        && source.field_source().declared_type_name() != Some("usize")
    { return Err(freeze("borrowed-field-write/exact-range-check-unavailable")); }
    // usize retains its existing canonical numeric obligation. Final source
    // use closure and backend/ABI checks must all accept this exact Invoke.
    let operations = ledger.prior_home_unwind_for(source.frame().prior_homes())?;
    let values = [source.receiver(), source.value()].into_iter().enumerate().map(|(index, (binding, _, site))|
        reads[index].value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let (receiver, value) = (values[0], values[1]);
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
    if !bindings.contains(&frame_binding) { bindings.push(frame_binding); }
    let mut fault_bindings = Vec::new();
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut fault_bindings)?;
    let fault = cleanup_chain(builder, frame, operations, outward, &mut fault_bindings)?;
    bindings.extend(fault_bindings.iter().cloned());
    let mut invoke = Vec::new();
    let (result, projection) = super::invoke_operation::emit(builder, frame,
        InvokeOperation::FieldSet { field: source.field_source().field(), base: pinned_receiver, value: pinned_value }, fault, &mut invoke)?;
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
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedBorrowedFieldWriteV1::new(source, reads, pins, invoke, continuation,
        bindings, fault_bindings, ledger, function)?);
    ledger.record_borrowed_field_write_bindings_v1(packet)
}
