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
    let source = ledger
        .borrowed_terminal_arguments_v1(owner, exit)?
        .ok_or_else(|| freeze("lexical-terminal/arguments-missing"))?;
    let receiver = state
        .take_exact_lexical_read(owner, row.receiver_site().node(), row.receiver_binding())
        .map_err(|error| error.to_string())?;
    let mut arguments = Vec::new();
    let prepared = lexical_i64::prepare_arguments(
        builder,
        state,
        ledger,
        owner,
        receiver,
        &source,
        &row,
        &mut arguments,
    )?;
    // The original source lender supplies the selected actual/formal relation;
    // final incoming/use verification and writer/C preserve both ABI lanes.
    let call = prepared.materialize_with_ledger(owner, &row, &source, ledger)?;
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        exit,
        Some(value),
        value,
        Some(RootExitIngress::Call(Emission {
            source: Source::Lexical { row, prepared },
            arguments,
            call,
            result: InvokeCallResultKind::I64,
        })),
    )
}
