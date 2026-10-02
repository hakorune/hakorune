//! Existing local/nested lexical I64 emission from sealed source rows.
use super::*;

/// Emit one source-issued lexical instance-call local result
/// (`local x = recv.m(...)`) whose co-sealed result contract is `I64`.
/// The sealed local-call relation is sole membership, the disposition row
/// names the unique selected callee, and the callee's literal-only exits
/// keep the exact-i64 contract — this lane never transfers a Home and
/// never consumes a handle argument. The receiver value is the
/// ledger-installed binding value; arguments come from the sealed
/// relation — integer literals and sealed integer-scalar bindings only,
/// zipped against the disposition's exact argument sites. Prior homes
/// (the receiver among them) stay live across the call and unwind
/// newest-first on the fault edge.
pub(in crate::mir::builder) fn emit_local_lexical_i64(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    let relation = ledger
        .lexical_i64_call_source(&owned_site)
        .ok_or_else(|| freeze("lexical-i64-source-missing"))?;
    let mut bindings = Vec::new();
    let result = emit_lexical_i64_call(
        builder,
        state,
        ledger,
        owner,
        relation.prior_homes(),
        relation.arguments(),
        &row,
        &mut bindings,
    )?;
    // The disposition issuer already routed this site onto the lifecycle
    // lane; the exit entries claim the recorded group in source order —
    // nested argument-position calls fold their emitted instructions
    // into this one group (they have no destination binding of their
    // own).
    ledger.record_root_local_call_bindings(owner, owned_site, bindings)?;
    Ok(result)
}

/// Emit one proven-i64 lexical call — a `local x = recv.m(..)`
/// continuation or a `CallResult` argument nested inside one — from its
/// sealed evidence: the prior-Home list (receiver included), the sealed
/// argument rows, and the exact disposition. Every emitted instruction
/// lands on `bindings` in physical order so the enclosing site records
/// one coherent group.
fn emit_lexical_i64_call(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    prior_homes: &[crate::mir::resolved_semantics::BindingRefV1],
    sealed_arguments: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
    row: &LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<ValueId, String> {
    if row.result() != Some(InvokeCallResultKind::I64) {
        return Err(freeze("lexical-i64-result-mismatch"));
    }
    if prior_homes.is_empty() || !prior_homes.contains(&row.receiver_binding()) {
        return Err(freeze("lexical-i64-receiver-home-missing"));
    }
    let unwind = ledger.prior_home_unwind_for(prior_homes)?;
    if sealed_arguments.len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-i64-arity-mismatch"));
    }
    let receiver = state
        .take_exact_lexical_value(owner, row.receiver_site().node(), row.receiver_binding())
        .map_err(|error| format!("[freeze:contract][lexical-i64/receiver/{error:?}]"))?;
    let frame = state.borrow_fault_frame(builder)?;
    let normal_landing = builder.next_block_id();
    let outward = builder.next_block_id();
    let result = builder.next_value_id();
    bindings.push(fault_frame_binding(builder, state, frame)?);
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        bindings,
    )?;
    // The call's fault path unwinds the prior Homes exactly like the
    // Handle lane does — the receiver Home can never leak past a fault.
    let fault_landing = cleanup_chain(builder, frame, unwind, outward, bindings)?;
    let mut arguments = Vec::with_capacity(sealed_arguments.len());
    for (argument, site) in sealed_arguments.iter().zip(row.argument_sites().iter()) {
        // The lexical-i64 lane seals integer literals, integer-scalar
        // bindings, and proven-i64 nested calls only — any other row here
        // means a claim boundary the emitter never admits, so fail
        // closed instead of guessing a value.
        let value = match argument {
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(
                literal,
            ) => crate::mir::builder::emission::constant::emit_integer(builder, *literal)?,
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Scalar(
                binding,
            ) => state
                .take_exact_lexical_value(owner, site.node(), *binding)
                .map_err(|error| format!("[freeze:contract][lexical-i64/argument/{error:?}]"))?,
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::CallResult(
                inner,
            ) => {
                if inner.site().site() != site {
                    return Err(freeze("lexical-i64/argument-site-mismatch"));
                }
                let inner_row = ledger
                    .take_lexical_instance_call(owner, site)?
                    .ok_or_else(|| freeze("lexical-i64/inner-disposition-missing"))?;
                // The inner Invoke runs before this call and feeds the
                // argument slot; its instructions join this call's
                // recorded binding group in physical order.
                emit_lexical_i64_call(
                    builder,
                    state,
                    ledger,
                    owner,
                    inner.prior_homes(),
                    inner.arguments(),
                    &inner_row,
                    bindings,
                )?
            }
            other => {
                return Err(format!(
                    "[freeze:contract][lexical-i64/argument-class]{other:?}"
                ))
            }
        };
        arguments.push(value);
    }
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call: MirCall::new(
                None,
                crate::mir::definitions::Callee::SameModuleInstance {
                    key: row.target().clone(),
                    receiver,
                },
                arguments,
            ),
            result: InvokeCallResultKind::I64,
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
        .insert(result, result_type(InvokeCallResultKind::I64, None)?);
    bindings.push((origin, invoke));
    bindings.push((normal_landing, projection));
    Ok(result)
}
