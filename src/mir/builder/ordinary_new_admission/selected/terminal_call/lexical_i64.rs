//! Existing local/nested lexical I64 emission from sealed source rows.
use super::*;
use crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1;
use crate::mir::normal_callable_semantic_package::{
    CallPacketSourceLoanV1 as SourceLoan, CallPacketSourceV1 as CallSource,
    EmittedLexicalCallProjectionV1 as EmittedCall,
    LexicalCallArgumentProjectionV1 as ArgumentProjection,
    PreparedLexicalCallProjectionV1 as PreparedCall,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
type Binding = (crate::mir::BasicBlockId, MirInstruction);

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
    emit_local_lexical_source(
        builder,
        state,
        ledger,
        owner,
        site,
        CallSource::instance(row),
    )
}

/// Both original source kinds use the same retained continuation and emitter.
pub(in crate::mir::builder) fn emit_local_lexical_source(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: CallSource,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    if row.loan().call_site() != &owned_site {
        return Err(freeze("lexical-i64/call-site-drift"));
    }
    let relation = ledger
        .lexical_i64_call_source(&owned_site)
        .ok_or_else(|| freeze("lexical-i64-source-missing"))?;
    let mut bindings = Vec::new();
    let emitted = emit_lexical_i64_call(
        builder,
        state,
        ledger,
        owner,
        relation.prior_homes(),
        relation.arguments(),
        row,
        &mut bindings,
    )?;
    // The disposition issuer already routed this site onto the lifecycle
    // lane; the exit entries claim the recorded group in source order —
    // nested argument-position calls fold their emitted instructions
    // into this one group (they have no destination binding of their
    // own).
    let result = emitted.value_with_ledger(owner, relation.arguments(), ledger)?;
    ledger.record_root_lexical_call_bindings(owner, owned_site, bindings, emitted)?;
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
    original: CallSource,
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<EmittedCall, String> {
    let row = original.loan();
    if row.result() != Some(InvokeCallResultKind::I64) {
        return Err(freeze("lexical-i64-result-mismatch"));
    }
    if let SourceLoan::Instance(instance) = row {
        if prior_homes.is_empty() || !prior_homes.contains(&instance.receiver_binding()?) {
            return Err(freeze("lexical-i64-receiver-home-missing"));
        }
    }
    row.validate_static(ledger)?;
    let unwind = ledger.prior_home_unwind_for(prior_homes)?;
    if sealed_arguments.len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-i64-arity-mismatch"));
    }
    let receiver = match row {
        SourceLoan::Instance(instance) => Some(
            state
                .take_exact_lexical_read(
                    owner,
                    instance.receiver_site().node(),
                    instance.receiver_binding()?,
                )
                .map_err(|error| format!("[freeze:contract][lexical-i64/receiver/{error:?}]"))?,
        ),
        SourceLoan::Static { .. } => None,
        SourceLoan::SelectedStaticLoop { .. } => {
            return Err(freeze("selected-static-loop/canonical-entry-required"));
        }
    };
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
    let prepared = prepare_arguments_for_source(
        builder,
        state,
        ledger,
        owner,
        receiver,
        sealed_arguments,
        row,
        bindings,
    )?;
    let call = prepared.materialize_using(owner, row, sealed_arguments, Some(ledger))?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
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
    let invoke = (origin, invoke);
    let projection = (normal_landing, projection);
    bindings.push(invoke.clone());
    bindings.push(projection.clone());
    Ok(EmittedCall::from_source(
        original, prepared, invoke, projection,
    ))
}

/// Materialize only the source-issued receiver/ordered arguments. The caller
/// owns its outer Invoke; nested calls retain their original affine rows here.
pub(in crate::mir::builder) fn prepare_arguments(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    receiver: ExactLexicalReadV1,
    source: &[LocalCallArgumentV1],
    row: &LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<Binding>,
) -> Result<PreparedCall, String> {
    prepare_arguments_for_source(
        builder,
        state,
        ledger,
        owner,
        Some(receiver),
        source,
        SourceLoan::Instance(row),
        bindings,
    )
}

pub(super) fn prepare_arguments_for_source(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    receiver: Option<ExactLexicalReadV1>,
    source: &[LocalCallArgumentV1],
    row: SourceLoan<'_>,
    bindings: &mut Vec<Binding>,
) -> Result<PreparedCall, String> {
    let mut arguments = Vec::with_capacity(source.len());
    for (argument, site) in source.iter().zip(row.argument_sites()) {
        let projection = match argument {
            LocalCallArgumentV1::Integer(literal) => {
                let block = builder
                    .function_state
                    .current_block
                    .ok_or_else(|| freeze("no-block"))?;
                let dst = crate::mir::builder::emission::constant::emit_integer(builder, *literal)?;
                let producer = (
                    block,
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(*literal),
                    },
                );
                bindings.push(producer.clone());
                ArgumentProjection::Integer(producer)
            }
            LocalCallArgumentV1::Scalar(binding) => ArgumentProjection::Scalar(
                state
                    .take_exact_lexical_read(owner, site.node(), *binding)
                    .map_err(|error| {
                        format!("[freeze:contract][lexical-i64/argument/{error:?}]")
                    })?,
            ),
            LocalCallArgumentV1::CallResult(inner) => {
                if inner.site().owner() != owner || inner.site().site() != site {
                    return Err(freeze("lexical-i64/argument-site-mismatch"));
                }
                let inner_row = ledger
                    .take_lexical_instance_call(owner, site)?
                    .ok_or_else(|| freeze("lexical-i64/inner-disposition-missing"))?;
                ArgumentProjection::CallResult(Box::new(emit_lexical_i64_call(
                    builder,
                    state,
                    ledger,
                    owner,
                    inner.prior_homes(),
                    inner.arguments(),
                    CallSource::instance(inner_row),
                    bindings,
                )?))
            }
            LocalCallArgumentV1::BorrowedActual {
                ordinal,
                site: original,
            } => {
                if original != site {
                    return Err(freeze("lexical-i64/borrowed-site"));
                }
                let actuals = row
                    .borrowed_actuals(ledger)?
                    .ok_or_else(|| freeze("lexical-i64/borrowed-actuals-missing"))?;
                let actual = actuals
                    .iter()
                    .find(|actual| actual.ordinal == *ordinal && &actual.site == site)
                    .ok_or_else(|| freeze("lexical-i64/borrowed-actual-missing"))?;
                use crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1 as Source;
                match &actual.source {
                    Source::Integer(literal) => {
                        let block = builder
                            .function_state
                            .current_block
                            .ok_or_else(|| freeze("no-block"))?;
                        let dst = crate::mir::builder::emission::constant::emit_integer(
                            builder, *literal,
                        )?;
                        let binding = (
                            block,
                            MirInstruction::Const {
                                dst,
                                value: crate::mir::ConstValue::Integer(*literal),
                            },
                        );
                        bindings.push(binding.clone());
                        ArgumentProjection::BorrowedLiteral {
                            ordinal: *ordinal,
                            site: site.clone(),
                            formal: actual.formal,
                            binding,
                        }
                    }
                    Source::Bool(literal) => {
                        let block = builder
                            .function_state
                            .current_block
                            .ok_or_else(|| freeze("no-block"))?;
                        let dst =
                            crate::mir::builder::emission::constant::emit_bool(builder, *literal)?;
                        let binding = (
                            block,
                            MirInstruction::Const {
                                dst,
                                value: crate::mir::ConstValue::Bool(*literal),
                            },
                        );
                        bindings.push(binding.clone());
                        ArgumentProjection::BorrowedLiteral {
                            ordinal: *ordinal,
                            site: site.clone(),
                            formal: actual.formal,
                            binding,
                        }
                    }
                    Source::Null => {
                        let block = builder
                            .function_state
                            .current_block
                            .ok_or_else(|| freeze("no-block"))?;
                        let dst = crate::mir::builder::emission::constant::emit_null(builder)?;
                        let binding = (
                            block,
                            MirInstruction::Const {
                                dst,
                                value: crate::mir::ConstValue::Null,
                            },
                        );
                        bindings.push(binding.clone());
                        ArgumentProjection::BorrowedLiteral {
                            ordinal: *ordinal,
                            site: site.clone(),
                            formal: actual.formal,
                            binding,
                        }
                    }
                    source => {
                        let (binding, entry) = match source {
                            Source::Scalar { binding, .. }
                            | Source::TypedHome { binding, .. }
                            | Source::EntryReceiver { binding, .. }
                            | Source::DeclaredFormal { binding, .. }
                            | Source::ReceivedNullable { binding, .. } => (*binding, None),
                            Source::Forwarded { binding, formal } => {
                                let rows = ledger.borrowed_ordinary_entry_values_v1(owner)?;
                                let entry = rows
                                    .iter()
                                    .find(|(_, root, _)| root == formal)
                                    .copied()
                                    .ok_or_else(|| freeze("lexical-i64/forwarded-entry-missing"))?;
                                (*binding, Some(entry))
                            }
                            _ => return Err(freeze("lexical-i64/borrowed-source")),
                        };
                        let read = state
                            .take_exact_lexical_read(owner, site.node(), binding)
                            .map_err(|error| {
                                format!("[freeze:contract][lexical-i64/borrowed-read/{error:?}]")
                            })?;
                        ArgumentProjection::BorrowedRead {
                            ordinal: *ordinal,
                            site: site.clone(),
                            formal: actual.formal,
                            read,
                            entry,
                        }
                    }
                }
            }
            other => {
                return Err(format!(
                    "[freeze:contract][lexical-i64/argument-class]{other:?}"
                ))
            }
        };
        arguments.push(projection);
    }
    PreparedCall::for_source(row, receiver, arguments)
}
