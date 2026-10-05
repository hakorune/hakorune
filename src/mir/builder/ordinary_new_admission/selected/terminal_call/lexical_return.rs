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
    let mut arguments = Vec::new();
    let stored = row.source_target().stored_receiver();
    let (site, binding) = match stored {
        Some((binding, site, _, _)) => (site, binding),
        None => (row.receiver_site(), row.receiver_binding()?),
    };
    let receiver = state
        .take_exact_lexical_read(owner, site.node(), binding)
        .map_err(|error| error.to_string())?;
    let field_read = if let Some((_, site, field, _)) = stored {
        let base = receiver
            .value_for(owner, site.node(), binding)
            .map_err(|error| error.to_string())?;
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| freeze("no-block"))?;
        let dst = builder.next_value_id();
        let instruction = MirInstruction::ObjectFieldGet { dst, base, field };
        builder.emit_instruction(instruction.clone())?;
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Box(row.target().owner().into()));
        let read = (block, instruction);
        arguments.push(read.clone());
        Some(read)
    } else {
        None
    };
    let mut prepared = lexical_i64::prepare_arguments(
        builder,
        state,
        ledger,
        owner,
        receiver,
        &source,
        &row,
        &mut arguments,
    )?;
    if let Some(read) = field_read {
        prepared = prepared.with_stored_receiver(read)?;
    }
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
