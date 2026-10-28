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
    if source.is_map_field_v1() {
        let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger);
        let path = ledger.statement_field_path_v1(source.site(), context)?
            .ok_or_else(|| freeze("read-argument/field-source-missing"))?;
        if path.parent().is_some() || state.owner() != call.owner() || call.owner() != source.site().owner() {
            return Err(freeze("read-argument/field-owner"));
        }
        let prepared = state.prepare_exact_lexical_read(call.owner(), path.root_site().node(), path.root())
            .map_err(|issue| issue.to_string())?;
        let base = prepared.value_for(call.owner(), path.root_site().node(), path.root()).map_err(|issue| issue.to_string())?;
        if !ledger.reserve_result_path_field_v1(source.site(), |binding| {
            if binding != path.root() { return Err(freeze("read-argument/field-parent")); }
            Ok(base)
        })? { return Err(freeze("read-argument/field-source-missing")); }
        let receiver = std::rc::Rc::new(state.commit_exact_lexical_reads(vec![prepared])
            .map_err(|issue| issue.to_string())?.pop().ok_or_else(|| freeze("read-argument/field-receiver-missing"))?);
        let descriptor = ledger.bind_result_path_receiver_v1(source.site(), base)?;
        builder.complete_source_checked_field_read_v1(base, descriptor.field(), source.site(), ledger,
            &descriptor, std::rc::Rc::clone(&receiver), state, None)?;
        let emitted = EmittedReadArgumentV1::from_map_field_v1(source.clone(), call.clone(), receiver, context,
            builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
        for row in emitted.bindings() { if !bindings.contains(row) { bindings.push(row.clone()); } }
        return Ok(emitted);
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
