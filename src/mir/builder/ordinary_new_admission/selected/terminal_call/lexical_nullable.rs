//! Lexical-receiver nullable-handle local call emission from sealed rows.
use super::*;
use crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1 as EmittedCall;

/// Emit one source-issued `local x = recv.m(..)` call whose sealed
/// local-call row carries Handle or Nullable and whose minted disposition row
/// corroborates the Object result — a claim-local
/// receiver, the callee's Object/NullableObject claim, and the same sealed
/// argument evidence the i64 lane materializes (literal/scalar/borrowed
/// actuals and nested call rows). The received Home owes HomeRelease for
/// Handle and HomeReleaseIfLive for Nullable; prior Homes unwind on Fault. The
/// recorded binding group keeps the physical producer inventory the
/// finalized-call visitor and the exit cleanup both claim.
pub(in crate::mir::builder) fn emit_local_lexical_object(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
) -> Result<ValueId, String> {
    emit_object(builder, state, ledger, owner, site, row, None)
}

/// The same physical owner consumes completed entry-receiver Object packets.
/// The declared locator is corroboration, never a replacement source issuer.
pub(in crate::mir::builder) fn emit_receiver_object(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
    key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    receiver: ValueId,
    read: crate::mir::builder::ExactLexicalReadV1,
) -> Result<ValueId, String> {
    emit_object(
        builder,
        state,
        ledger,
        owner,
        site,
        row,
        Some((key, receiver, read)),
    )
}

fn emit_object(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
    entry: Option<(
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        ValueId,
        crate::mir::builder::ExactLexicalReadV1,
    )>,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    if row.call_site() != &owned_site {
        return Err(freeze("lexical-nullable/call-site-drift"));
    }
    let result_kind = row
        .result()
        .ok_or_else(|| freeze("lexical-nullable/result-mismatch"))?;
    if !matches!(
        result_kind,
        InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle
    ) || (entry.is_none()
        && result_kind == InvokeCallResultKind::Handle
        && !row.source_target().has_object_source_requirement())
    {
        return Err(freeze("lexical-nullable/result-mismatch"));
    }
    let relation = match result_kind {
        InvokeCallResultKind::Handle => ledger.handle_call_source(&owned_site),
        _ => ledger.nullable_call_source(&owned_site),
    }
    .ok_or_else(|| freeze("lexical-nullable/source-missing"))?;
    let borrowed_receiver = row.source_target().is_self_receiver();
    if borrowed_receiver != entry.is_some()
        || (!borrowed_receiver
            && (relation.prior_homes().is_empty()
                || !relation.prior_homes().contains(&row.receiver_binding()?)))
    {
        return Err(freeze("lexical-nullable/receiver-home-missing"));
    }
    let source_arguments = if borrowed_receiver {
        ledger.receiver_object_packet_arguments_v1(&row)?
    } else if row.source_target().has_object_source_requirement() {
        let arguments = ledger.object_packet_arguments_v1(&row)?;
        if arguments != relation.arguments() {
            return Err(freeze("lexical-object/flow-arguments-drift"));
        }
        arguments
    } else {
        relation.arguments()
    };
    if source_arguments.len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-nullable/arity-mismatch"));
    }
    let class = match result_kind {
        InvokeCallResultKind::Handle => ledger.callable_result_class(row.target()),
        _ => ledger.nullable_result_class(row.target()),
    }
    .ok_or_else(|| freeze("lexical-nullable/class-drift"))?;
    let result_type = result_type(result_kind, Some(class))?;
    let (receiver, locator) = match entry {
        Some((key, expected, read)) => (read, Some((key, expected))),
        None => (
            state
                .take_exact_lexical_read(owner, row.receiver_site().node(), row.receiver_binding()?)
                .map_err(|error| {
                    format!("[freeze:contract][lexical-nullable/receiver/{error:?}]")
                })?,
            None,
        ),
    };
    if let Some((key, expected)) = locator {
        if row.target() != key
            || receiver
                .value_for(owner, row.receiver_site().node(), row.receiver_binding()?)
                .map_err(|error| error.to_string())?
                != expected
        {
            return Err(freeze("receiver-object/locator-drift"));
        }
        let observation = ledger
            .receiver_call_observation(&owned_site)
            .ok_or_else(|| freeze("receiver-object/observation-missing"))?;
        let class_matches = matches!((result_kind, observation.class()),
            (InvokeCallResultKind::Handle, crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::Object(name))
            | (InvokeCallResultKind::NullableHandle, crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(name))
                if name.as_ref() == class);
        if observation.callee() != key
            || !class_matches
            || relation
                .local_binding()
                .is_none_or(|(_, destination)| destination != observation.destination())
        {
            return Err(freeze("receiver-object/observation-drift"));
        }
    }
    let unwind = ledger.prior_home_unwind_for(relation.prior_homes())?;
    // The `CallReceived` commit for this exact site mints the canonical
    // object the live arm carries and selects the checked release — the
    // `Void` arm keeps no residence.
    if row.source_target().has_object_source_requirement() {
        ledger.begin_object_packet_call_emission(&row)?;
    } else {
        match result_kind {
            InvokeCallResultKind::Handle => {
                ledger.begin_handle_call_emission(&owned_site, row.callee_owner())?
            }
            _ => ledger.begin_nullable_call_emission(&owned_site, row.target())?,
        }
    }
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
        source_arguments,
        &row,
        &mut bindings,
    )?;
    let call = prepared.materialize_with_ledger(owner, &row, source_arguments, ledger)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: result_kind,
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
        .insert(result, result_type);
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
    let packet = ledger.record_root_lexical_call_bindings(
        owner,
        owned_site.clone(),
        bindings.clone(),
        emitted,
    )?;
    if borrowed_receiver {
        ledger.record_receiver_object_call_emission_v1(&owned_site, result, bindings, packet)?;
    } else {
        ledger.record_handle_call_emission(&owned_site, result, bindings)?;
    }
    Ok(result)
}
