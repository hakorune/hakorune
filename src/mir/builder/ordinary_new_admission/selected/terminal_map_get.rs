//! Original stored Map Return uses the shared expression and root graph owners.
use super::*;
use crate::mir::normal_callable_semantic_package::{CallValidationContextV1, StoredMapGetReturnSourceLoanV1, StoredMapGetReturnEmissionV1};

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    source: StoredMapGetReturnSourceLoanV1,
) -> Result<ValueId, String> {
    let owner = source.terminal().owner();
    let exit = source.terminal().return_site().clone();
    let (_, key, receiver_site) = source.parts();
    let receiver = state.take_exact_lexical_read(owner, receiver_site.node(), source.get_source().receiver().receiver())
        .map_err(|error| error.to_string())?;
    source.require_receiver(CallValidationContextV1::Lowering(ledger),
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?, &receiver)?;
    // The retained original key is passive. Do not materialize or transfer it.
    let expression = expression_value::emit(builder, state, ledger, key,
        &mut |_, _, _, _, _, _| Err(freeze("map-get-shape")))?;
    if expression.purpose_acquisition.is_some() || !expression.body.acquired.is_empty()
        || !expression.body.calls.is_empty() || !expression.body.path_reads.is_empty() {
        return Err(freeze("map-get-shape"));
    }
    let mut body = expression.body;
    let (terminal, continuation) = expression_value::finish_value_continuation_v1(builder, &mut body)?;
    let stored = StoredMapGetReturnEmissionV1::issue(source, receiver, body.values,
        body.reference_reads, body.entry, terminal, continuation, body.bindings, ledger,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    let operation = stored.operation(CallValidationContextV1::Lowering(ledger),
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    let value = builder.next_value_id();
    // The result has no owning lease; do not bind a fresh object result.
    emit_root_home_exit_payload(builder, state, ledger, owner, &exit, Some(value), value,
        Some(RootExitIngress::MapGet { operation, stored: Some(Box::new(stored)) }))
}
