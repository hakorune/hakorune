//! One physical read-argument dispatch. The specialized owner issues the read.
use super::*;
use crate::mir::normal_callable_semantic_package::{EmittedReadArgumentV1, ReadActualSourceLoanV1};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    call: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    source: &ReadActualSourceLoanV1,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<EmittedReadArgumentV1, String> {
    if let Some(array) = source.array_source_v1() {
        let read = super::owned_array::emit_read(builder, state, ledger, call, array.clone(), bindings)?;
        return Ok(EmittedReadArgumentV1::from_array_v1(read));
    }
    let (descriptor, parent, receiver_site) = source.field_parts_v1()
        .ok_or_else(|| freeze("read-argument/source-kind"))?;
    let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger);
    if state.owner() != call.owner() || call.owner() != source.site().owner() {
        return Err(freeze("read-argument/field-owner"));
    }
    let parent_value = ledger.returned_field_argument_parent_v1(descriptor, context)?.0;
    let mut receiver = None;
    let (base, taken) = ledger.take_local_field_read(source.site(), |binding| {
        if binding != parent { return Err(freeze("read-argument/field-parent")); }
        let read = state.take_exact_lexical_read(call.owner(), receiver_site.node(), parent)
            .map_err(|issue| issue.to_string())?;
        read.loan_forwarded_copies(parent, parent_value)?;
        let value = read.value_for(call.owner(), receiver_site.node(), parent)
            .map_err(|issue| issue.to_string())?;
        receiver = Some(read);
        Ok(value)
    })?.ok_or_else(|| freeze("read-argument/field-source-missing"))?;
    if !std::rc::Rc::ptr_eq(&taken, descriptor) { return Err(freeze("read-argument/field-source-drift")); }
    let (_, binding) = builder.complete_exact_field_read_v1(base, taken.field(), source.site(), ledger,
        crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(taken))?;
    let emitted = EmittedReadArgumentV1::from_field_v1(std::rc::Rc::clone(descriptor), call.clone(),
        receiver.ok_or_else(|| freeze("read-argument/field-receiver-missing"))?, binding.clone(), context,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    bindings.push(binding);
    Ok(emitted)
}
