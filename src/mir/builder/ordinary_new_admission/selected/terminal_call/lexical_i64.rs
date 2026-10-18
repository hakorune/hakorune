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
#[path = "lexical_i64_expression.rs"]
mod expression;
pub(in crate::mir::builder) use expression::{emit as emit_expression_call, emit_with_prefix as emit_expression_call_with_prefix, emit_bool as emit_bool_condition_call};
use expression::ExpressionCallPrefixV1;

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
        .or_else(|| ledger.borrowed_nullable_call_source_v1(&owned_site))
        .or_else(|| ledger.mixed_call_source_v1(&owned_site))
        .ok_or_else(|| freeze("lexical-i64-source-missing"))?;
    let mixed = match row.loan() {
        SourceLoan::Instance(instance) if instance.source_target().mixed_local_source().is_some() => {
            ledger.begin_mixed_call_emission_v1(instance)?;
            true
        }
        _ => false,
    };
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
        None,
    )?;
    // The disposition issuer already routed this site onto the lifecycle
    // lane; the exit entries claim the recorded group in source order —
    // nested argument-position calls fold their emitted instructions
    // into this one group (they have no destination binding of their
    // own).
    let result = emitted.value_with_ledger(owner, relation.arguments(), ledger)?;
    let commit_bindings = mixed.then(|| bindings.clone());
    let packet = ledger.record_root_lexical_call_bindings(owner, owned_site.clone(), bindings, emitted)?;
    if let Some(bindings) = commit_bindings {
        ledger.record_mixed_call_emission_v1(&owned_site, result, bindings, packet)?;
    }
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
    expression: Option<&ExpressionCallPrefixV1<'_>>,
) -> Result<EmittedCall, String> {
    let row = original.loan();
    if let (Some(prefix), SourceLoan::Instance(instance)) = (expression, row) {
        // Source-selected expression emission bypasses generic method descent.
        // Borrow that same locator once, before receiver/argument MIR effects.
        prefix.consume_receiver(instance)?;
    }
    let kind = match row.result() {
        Some(kind @ (InvokeCallResultKind::I64 | InvokeCallResultKind::Unit)) => kind,
        Some(InvokeCallResultKind::Bool) => match row {
            SourceLoan::Instance(instance)
                if ledger.checked_local_scalar_call_result_v1(instance)? == InvokeCallResultKind::Bool => InvokeCallResultKind::Bool,
            _ => return Err(freeze("lexical-scalar/bool-source-missing")),
        },
        Some(kind @ InvokeCallResultKind::BorrowedNullableHandle { .. }) => match row {
            SourceLoan::Instance(instance) if ledger.corroborate_borrowed_nullable_call_v1(instance)? == kind => kind,
            _ => return Err(freeze("borrowed-nullable-call/source-kind")),
        },
        Some(kind @ InvokeCallResultKind::MixedHandle { .. }) => match row {
            SourceLoan::Instance(instance) if ledger.corroborate_mixed_call_v1(instance)? == kind => kind,
            _ => return Err(freeze("mixed-call/source-kind")),
        },
        _ => return Err(freeze("lexical-i64-result-mismatch")),
    };
    if let SourceLoan::Instance(instance) = row {
        let binding = match instance.source_target().stored_receiver() {
            Some((binding, _, _, _)) => binding,
            None => instance.receiver_binding()?,
        };
        let borrowed_self = instance.source_target().is_self_receiver()
            && ledger.lexical_i64_call_source(row.call_site())
                .filter(|observation| observation.prior_homes() == prior_homes
                    && observation.arguments() == sealed_arguments)
                .and_then(|observation| observation.borrowed_scalar())
                .is_some_and(|loan| loan.source() == instance.source_target()
                    && instance.scalar_result_source_v1() == Some(loan)
                    && match loan.kind() {
                        crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => kind == InvokeCallResultKind::I64,
                        crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => kind == InvokeCallResultKind::Bool,
                    });
        let borrowed_alias = ledger.lexical_i64_call_source(row.call_site())
            .filter(|observation| observation.prior_homes() == prior_homes
                && observation.arguments() == sealed_arguments)
            .map(|observation| ledger.checked_field_alias_receiver_v1(instance, observation, None))
            .transpose()?.unwrap_or(false);
        let borrowed_result_receiver = matches!(kind, InvokeCallResultKind::BorrowedNullableHandle { .. })
            && ledger.borrowed_nullable_call_source_v1(row.call_site()).is_some_and(|observation|
                observation.prior_homes() == prior_homes && observation.arguments() == sealed_arguments)
            && (instance.source_target().is_self_receiver() || instance.source_target().stored_receiver().is_some());
        let mixed_receiver = instance.source_target().mixed_local_source().is_some()
            && ledger.corroborate_mixed_call_v1(instance)? == kind
            && ledger.mixed_call_source_v1(row.call_site()).is_some_and(|observation|
                observation.prior_homes() == prior_homes && observation.arguments() == sealed_arguments
                    && observation.mixed() == instance.source_target().mixed_local_source())
            && (instance.source_target().is_self_receiver() || instance.source_target().stored_receiver().is_some());
        if !borrowed_self && !borrowed_alias && !borrowed_result_receiver && !mixed_receiver && !prior_homes.contains(&binding) {
            return Err(freeze("lexical-i64-receiver-home-missing"));
        }
    }
    row.validate_static(ledger)?;
    let mut unwind = match expression {
        Some(prefix) => prefix.unwind(row.call_site(), prior_homes)?,
        None => Vec::new(),
    };
    unwind.extend(ledger.prior_home_unwind_for(prior_homes)?);
    if sealed_arguments.len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-i64-arity-mismatch"));
    }
    let (receiver, stored_read) = match row {
        SourceLoan::Instance(instance) => {
            let (receiver, read) = lexical_receiver::prepare(builder, state, owner, instance, bindings)?;
            (Some(receiver), read)
        }
        SourceLoan::Static { .. } => (None, None),
        SourceLoan::SelectedStaticLoop { .. } => {
            return Err(freeze("selected-static-loop/canonical-entry-required"));
        }
    };
    let frame = state.borrow_fault_frame(builder)?;
    let normal_landing = builder.next_block_id();
    let outward = builder.next_block_id();
    let result = (kind != InvokeCallResultKind::Unit).then(|| builder.next_value_id());
    bindings.push(fault_frame_binding(builder, state, frame)?);
    let mut prepared = prepare_arguments_with_prefix(
        builder,
        state,
        ledger,
        owner,
        receiver,
        sealed_arguments,
        row,
        bindings,
        expression,
    )?;
    // The enclosing Call's Fault graph belongs after ordered argument
    // evaluation. Argument acquisitions will join this same lifetime boundary;
    // source target admission still precedes every argument effect above.
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        bindings,
    )?;
    // The call's fault path unwinds the prior Homes exactly like the
    // Handle lane does — the receiver Home can never leak past a fault.
    let fault_landing = cleanup_chain(builder, frame, unwind, outward, bindings)?;
    if let Some(read) = stored_read { prepared = prepared.with_stored_receiver(read)?; }
    let call = prepared.materialize_using(owner, row, sealed_arguments, Some(ledger))?;
    if let Some(result) = result {
        let (invoke, projection) = emit_call_normal(
            builder, call, kind, frame, normal_landing, fault_landing, result, result_type(kind, None)?,
        )?;
        bindings.push(invoke.clone());
        bindings.push(projection.clone());
        Ok(EmittedCall::from_source(original, prepared, invoke, projection))
    } else {
        let invoke = emit_call_normal_start(builder, call, kind, frame, normal_landing, fault_landing)?;
        bindings.push(invoke.clone());
        EmittedCall::from_unit_source(original, prepared, invoke, normal_landing)
    }
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
    prepare_arguments_with_prefix(builder, state, ledger, owner, receiver, source, row, bindings, None)
}

fn prepare_arguments_with_prefix(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    receiver: Option<ExactLexicalReadV1>,
    source: &[LocalCallArgumentV1],
    row: SourceLoan<'_>,
    bindings: &mut Vec<Binding>,
    expression: Option<&ExpressionCallPrefixV1<'_>>,
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
                    expression,
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
                    Source::Read(source) => {
                        let observed = crate::mir::builder::ordinary_new_admission::selected::read_argument::emit(
                            builder, state, ledger, row.call_site(), source, bindings,
                        )?;
                        ArgumentProjection::BorrowedReadArgument {
                            ordinal: *ordinal, site: site.clone(), formal: actual.formal, read: observed,
                        }
                    }
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
                            | Source::ReceivedNullable { binding, .. }
                | Source::ReceivedMixed { binding, .. } => (*binding, None),
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

/// Unit statement uses the same receiver/argument/Fault/Call owner as values.
pub(in crate::mir::builder) fn emit_unit_discard(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, row: LexicalInstanceCallDispositionRowV1,
) -> Result<(), String> {
    ledger.corroborate_unit_discard_v1(&row)?;
    let site = row.call_site().clone();
    let observation = ledger.unit_discard_source_v1(&site).ok_or_else(|| freeze("unit-call/source-missing"))?;
    let mut bindings = Vec::new();
    let packet = emit_lexical_i64_call(builder, state, ledger, site.owner(), observation.prior_homes(),
        observation.arguments(), CallSource::instance(row), &mut bindings, None)?;
    packet.call_with_ledger(site.owner(), observation.arguments(), ledger)?;
    ledger.record_root_lexical_call_bindings(site.owner(), site, bindings, packet)?;
    Ok(())
}

/// The existing Call/Normal writer, with receiver liveness and Fault cleanup
/// supplied by the original ordered construction state instead of caller Homes.
pub(in crate::mir::builder) fn emit_constructor_unit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, row: LexicalInstanceCallDispositionRowV1,
    frame: ValueId, fault_landing: BasicBlockId,
) -> Result<(EmittedCall, Vec<Binding>), String> {
    if ledger.constructor_unit_call_source_v1(&row)?.is_none() {
        return Err(freeze("constructor-unit/original-source-missing"));
    }
    let owner = row.call_site().owner();
    let mut bindings = Vec::new();
    let (receiver, read) = lexical_receiver::prepare(builder, state, owner, &row, &mut bindings)?;
    let read = read.ok_or_else(|| freeze("constructor-unit/stored-read-missing"))?;
    let prepared = prepare_arguments(builder, state, ledger, owner, receiver, &[], &row, &mut bindings)?
        .with_stored_receiver(read)?;
    let source = CallSource::instance(row);
    let call = prepared.materialize_using(owner, source.loan(), &[], Some(ledger))?;
    let normal = builder.next_block_id();
    let invoke = emit_call_normal_start(builder, call, InvokeCallResultKind::Unit, frame, normal, fault_landing)?;
    bindings.push(invoke.clone());
    let packet = EmittedCall::from_unit_source(source, prepared, invoke, normal)?;
    packet.call_with_ledger(owner, &[], ledger)?;
    packet.validate_recorded(&bindings)?;
    Ok((packet, bindings))
}
