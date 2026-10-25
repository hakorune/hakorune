//! Prepare the original terminal tree; the existing exit owns the outer Invoke.
use super::*;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
) -> Result<ValueId, String> {
    // The passed affine row must match the original Direct proof before leases
    // or argument instructions are consumed. Missing Object proof never retries I64.
    let direct = ledger.checked_direct_object_return_row_v1(owner, exit, &row)?;
    let terminal = ledger.verified_terminal_call_source_v1(owner, exit)?
        .ok_or_else(|| freeze("lexical-terminal/source-missing"))?;
    terminal.corroborate_row(&row)?;
    let source = terminal.lexical_arguments()
        .ok_or_else(|| freeze("lexical-terminal/arguments-missing"))?;
    let result = terminal.result();
    let class = direct.as_ref().map(|view| view.class());
    let mut arguments = Vec::new();
    let (receiver, field_read) = lexical_receiver::prepare(
        builder, state, owner, &row, ledger, lexical_receiver::Fault::Return(exit), &mut arguments)?;
    let mut prepared = lexical_i64::prepare_arguments(
        builder,
        state,
        ledger,
        owner,
        receiver,
        source,
        &row,
        &mut arguments,
    )?;
    if let Some(read) = field_read {
        prepared = prepared.with_stored_receiver(read)?;
    }
    // The original source lender supplies the selected actual/formal relation;
    // final incoming/use verification and writer/C preserve both ABI lanes.
    let call = prepared.materialize_with_ledger(owner, &row, source, ledger)?;
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, result_type(result, class)?);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        exit,
        Some(value),
        value,
        Some(RootExitIngress::Call(Emission {
            source: Source::Packet {
                row: crate::mir::normal_callable_semantic_package::CallPacketSourceV1::instance(row),
                prepared,
            },
            arguments,
            call,
            result,
        })),
    )
}

#[cfg(test)]
#[path = "lexical_return_tests.rs"]
mod tests;
