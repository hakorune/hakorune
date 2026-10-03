//! Lexical-receiver nullable-handle local call emission from sealed rows.
use super::*;
use crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1 as EmittedCall;

/// Emit one source-issued `local x = recv.m(..)` call whose sealed
/// local-call row carries `Nullable` and whose minted disposition row
/// corroborates `InvokeCallResultKind::NullableHandle` — a claim-local
/// receiver, the callee's `NullableObject` claim, and the same sealed
/// argument evidence the i64 lane materializes (literal/scalar/borrowed
/// actuals and nested call rows). The received binding is an owned
/// nullable Home: the caller's exits owe `HomeReleaseIfLive`, never an
/// unconditional release, and prior Homes unwind on the fault edge. The
/// recorded binding group keeps the physical producer inventory the
/// finalized-call visitor and the exit cleanup both claim.
pub(in crate::mir::builder) fn emit_local_lexical_nullable(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    if row.call_site() != &owned_site {
        return Err(freeze("lexical-nullable/call-site-drift"));
    }
    if row.result() != Some(InvokeCallResultKind::NullableHandle) {
        return Err(freeze("lexical-nullable/result-mismatch"));
    }
    let relation = ledger
        .nullable_call_source(&owned_site)
        .ok_or_else(|| freeze("lexical-nullable/source-missing"))?;
    if relation.prior_homes().is_empty()
        || !relation.prior_homes().contains(&row.receiver_binding())
    {
        return Err(freeze("lexical-nullable/receiver-home-missing"));
    }
    if relation.arguments().len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-nullable/arity-mismatch"));
    }
    let class = ledger
        .nullable_result_class(row.target())
        .ok_or_else(|| freeze("lexical-nullable/class-drift"))?
        .to_owned();
    // The `CallReceived` commit for this exact site mints the canonical
    // object the live arm carries and selects the checked release — the
    // `Void` arm keeps no residence.
    ledger.begin_nullable_call_emission(&owned_site, row.target())?;
    let unwind = ledger.nullable_call_prior_home_unwind(&owned_site)?;
    let receiver = state
        .take_exact_lexical_read(owner, row.receiver_site().node(), row.receiver_binding())
        .map_err(|error| format!("[freeze:contract][lexical-nullable/receiver/{error:?}]"))?;
    let frame = state.borrow_fault_frame(builder)?;
    let normal_landing = builder.next_block_id();
    let outward = builder.next_block_id();
    let result = builder.next_value_id();
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    // Prior Homes unwind newest-first on the fault edge — the received
    // nullable does not exist there yet, so the checked release never
    // runs on the fault path.
    let fault_landing = cleanup_chain(builder, frame, unwind, outward, &mut bindings)?;
    let prepared = lexical_i64::prepare_arguments(
        builder,
        state,
        ledger,
        owner,
        receiver,
        relation.arguments(),
        &row,
        &mut bindings,
    )?;
    let call = prepared.materialize_with_ledger(owner, &row, relation.arguments(), ledger)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::NullableHandle,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, result_type(InvokeCallResultKind::NullableHandle, Some(class.as_ref()))?);
    let emitted = EmittedCall::new(
        row,
        prepared,
        (origin, invoke),
        (normal_landing, projection),
    );
    bindings.push(emitted.outer_bindings().0.clone());
    bindings.push(emitted.outer_bindings().1.clone());
    // The binding group is the finalized-call inventory — the exit
    // cleanup claims it in source order exactly like the scalar lane —
    // while the `CallReceived` commit owns the received Home and the
    // checked release at the caller's exits.
    ledger.record_root_lexical_call_bindings(
        owner,
        owned_site.clone(),
        bindings.clone(),
        emitted,
    )?;
    ledger.record_handle_call_emission(&owned_site, result, bindings)?;
    Ok(result)
}
