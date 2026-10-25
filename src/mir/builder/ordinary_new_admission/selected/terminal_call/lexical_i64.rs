//! Existing local/nested lexical I64 emission from sealed source rows.
use super::*;
use crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1;
use crate::mir::normal_callable_semantic_package::{
    CallPacketSourceLoanV1 as SourceLoan, CallPacketSourceV1 as CallSource,
    EmittedLexicalCallProjectionV1 as EmittedCall,
    LexicalCallArgumentProjectionV1 as ArgumentProjection,
    PreparedLexicalCallProjectionV1 as PreparedCall,
    ExpressionCallArgumentSourceV1 as ArgumentSource, ExpressionCallTransientPrefixV1, CallValidationContextV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
type Binding = (crate::mir::BasicBlockId, MirInstruction);
#[path = "lexical_i64_expression.rs"]
mod expression;
pub(in crate::mir::builder) use expression::{emit as emit_expression_call, emit_with_prefix as emit_expression_call_with_prefix, emit_with_dynamic_and_argument_frames as emit_expression_call_with_dynamic_and_argument_frames, emit_bool as emit_bool_condition_call, StaticExpressionCallIssuerV1};
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
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<ValueId, String> {
    emit_local_lexical_source(
        builder,
        state,
        ledger,
        owner,
        site,
        CallSource::instance(row),
        consume_receiver,
        static_source,
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
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
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
        ArgumentSource::from_local(relation),
        row,
        &mut bindings,
        None,
        Some(consume_receiver),
        static_source,
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
    source: ArgumentSource<'_>,
    original: CallSource,
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
    expression: Option<&ExpressionCallPrefixV1<'_>>,
    consume_receiver: Option<&dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<EmittedCall, String> {
    let row = original.loan();
    let prior_homes = source.prior_homes();
    let sealed_arguments = source.arguments();
    if source.site() != row.call_site() || source.site().owner() != owner {
        return Err(freeze("lexical-i64/original-call-source"));
    }
    if let (Some(prefix), SourceLoan::Instance(instance)) = (expression, row) {
        // Source-selected expression emission bypasses generic method descent.
        // Borrow that same locator once, before receiver/argument MIR effects.
        prefix.consume_receiver(instance)?;
    }
    let kind = match row.result() {
        Some(kind @ (InvokeCallResultKind::I64 | InvokeCallResultKind::Unit | InvokeCallResultKind::OwnedString)) => kind,
        Some(InvokeCallResultKind::Bool) => match row {
            SourceLoan::Instance(instance)
                if ledger.checked_local_scalar_call_result_v1(instance)? == InvokeCallResultKind::Bool => InvokeCallResultKind::Bool,
            SourceLoan::Static { .. } => { row.validate_static(ledger)?; InvokeCallResultKind::Bool },
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
        let borrowed_scalar_receiver = ledger.lexical_i64_call_source(row.call_site())
                .filter(|observation| observation.prior_homes() == prior_homes
                    && observation.arguments() == sealed_arguments)
                .and_then(|observation| observation.borrowed_scalar())
                .is_some_and(|loan| loan.source() == instance.source_target()
                    && instance.scalar_result_source_v1() == Some(loan)
                    && match loan.kind() {
                        crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => kind == InvokeCallResultKind::I64,
                        crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => kind == InvokeCallResultKind::Bool,
                    })
            && (instance.source_target().is_self_receiver()
                || ledger.checked_stored_entry_receiver_v1(instance.source_target())?);
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
        let borrowed_map_receiver = instance.source_target().borrowed_receiver_source_v1().is_some()
            && kind == InvokeCallResultKind::Unit
            && ledger.unit_discard_source_v1(row.call_site()).is_some_and(|observation|
                observation.unit_source() == Some(instance.source_target())
                    && observation.prior_homes() == prior_homes && observation.arguments() == sealed_arguments)
            && ledger.checked_borrowed_map_receiver_source_v1(instance.source_target())?;
        if !borrowed_scalar_receiver && !borrowed_alias && !borrowed_result_receiver && !mixed_receiver && !borrowed_map_receiver && !prior_homes.contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(binding)) {
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
            let (receiver, read) = lexical_receiver::prepare(builder, state, owner, instance, ledger, lexical_receiver::Fault::Call(&unwind), bindings)?;
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
    let shared_frame = fault_frame_binding(builder, state, frame)?;
    if !bindings.contains(&shared_frame) { bindings.push(shared_frame); }
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
        Some(source),
        consume_receiver,
        expression.and_then(|prefix| prefix.static_source()).or(static_source),
    )?;
    let cancellation = if kind == InvokeCallResultKind::OwnedString {
        vec![InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End {
            value: result.ok_or_else(|| freeze("lexical-i64-result-mismatch"))?,
        })]
    } else { Vec::new() };
    let (fault_landing, cleanup) = prepare_argument_cleanup_v1(
        builder, ledger, &prepared, frame, outward, unwind, &cancellation, bindings)?;
    if let Some(read) = stored_read { prepared = prepared.with_stored_receiver(read)?; }
    let call = prepared.materialize_using(owner, row, sealed_arguments, Some(ledger))?;
    let packet = if let Some(result) = result {
        let (invoke, projection) = emit_call_normal(
            builder, call, kind, frame, normal_landing, fault_landing, result, result_type(kind, None)?,
        )?;
        bindings.push(invoke.clone());
        bindings.push(projection.clone());
        EmittedCall::from_source(original, prepared, invoke, projection)
    } else {
        let invoke = emit_call_normal_start(builder, call, kind, frame, normal_landing, fault_landing)?;
        bindings.push(invoke.clone());
        EmittedCall::from_unit_source(original, prepared, invoke, normal_landing)?
    };
    finish_argument_cleanup_v1(builder, normal_landing, packet, cleanup, bindings)
}

pub(super) type ArgumentCleanupV1 = (BasicBlockId, BasicBlockId, Binding, Vec<Binding>);

/// The same Call argument stack ends on Normal and before prior Homes on Fault.
pub(super) fn prepare_argument_cleanup_v1(
    builder: &mut MirBuilder, ledger: &OrdinaryNewClaimLedgerV1, prepared: &PreparedCall,
    frame: ValueId, outward: BasicBlockId, unwind: Vec<InvokeOperation>,
    normal_fault_cancellation: &[InvokeOperation], bindings: &mut Vec<Binding>,
) -> Result<(BasicBlockId, Option<ArgumentCleanupV1>), String> {
    let normal_ends = prepared.deferred_argument_ends_v1(CallValidationContextV1::Lowering(ledger))?;
    let result = if normal_ends.is_empty() {
        append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, bindings)?;
        (cleanup_chain(builder, frame, unwind, outward, bindings)?, None)
    } else {
        let mut fault_operations = normal_ends.clone();
        fault_operations.extend_from_slice(normal_fault_cancellation);
        fault_operations.extend(unwind);
        let mut seen = std::collections::BTreeSet::new();
        for operation in &fault_operations {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }) = operation {
                if !seen.insert(*value) { return Err(freeze("call-argument/duplicate-acquisition")); }
            }
        }
        let clean = builder.next_block_id();
        let next = builder.next_block_id();
        let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
        append_block(builder, clean, continuation.1.clone(), bindings)?;
        let after_normal: Vec<_> = (0..normal_ends.len())
            .map(|index| ((index + 1)..fault_operations.len()).collect()).collect();
        // A result exists only after Call Normal. Its cancellation belongs to
        // Normal cleanup Fault suffixes, never the original Call Fault edge.
        let call_fault: Vec<_> = (0..normal_ends.len())
            .chain((normal_ends.len() + normal_fault_cancellation.len())..fault_operations.len()).collect();
        let graph = super::super::root_graph::emit_cleanup_suffixes_with_writer(
            &mut RawRootGraphWriterV1, builder, frame, clean, &normal_ends, &fault_operations,
            &after_normal, &call_fault, bindings)?;
        (graph.acquisition_fault, Some((graph.clean, next, continuation, graph.normal_origins)))
    };
    Ok(result)
}

pub(super) fn finish_argument_cleanup_v1(
    builder: &mut MirBuilder, normal_landing: BasicBlockId, packet: EmittedCall,
    cleanup: Option<ArgumentCleanupV1>, bindings: &mut Vec<Binding>,
) -> Result<EmittedCall, String> {
    let packet = match cleanup {
        Some((clean, next, continuation, normal)) => {
            let ingress = (normal_landing, MirInstruction::Jump { target: clean, edge_args: None });
            builder.emit_instruction(ingress.1.clone())?;
            bindings.push(ingress.clone());
            builder.start_new_block(next)?;
            packet.with_argument_cleanup_v1(ingress, continuation, normal)
        }
        None => Ok(packet),
    }?;
    packet.validate_original_argument_order_v1(builder.function_state.current_function.as_ref()
        .ok_or_else(|| freeze("no-function"))?)?;
    Ok(packet)
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
        None,
        None,
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
    consume_receiver: Option<&dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<PreparedCall, String> {
    prepare_arguments_with_prefix(builder, state, ledger, owner, receiver, source, row, bindings, None,
        ledger.local_call_for_owner(owner, row.call_site().site()).map(ArgumentSource::from_local), consume_receiver,
        static_source)
}

/// Lend an already checked original purpose into the same argument writer.
pub(super) fn prepare_arguments_for_original_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1,
    receiver: ExactLexicalReadV1, row: &LexicalInstanceCallDispositionRowV1,
    original: ArgumentSource<'_>, bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<PreparedCall, String> {
    prepare_arguments_with_prefix(builder, state, ledger, owner, Some(receiver), original.arguments(),
        SourceLoan::Instance(row), bindings, None, Some(original), Some(consume_receiver), None)
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
    original: Option<ArgumentSource<'_>>,
    consume_receiver: Option<&dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<PreparedCall, String> {
    if original.is_some_and(|original| original.site() != row.call_site() || original.arguments() != source) {
        return Err(freeze("lexical-i64/argument-original-source"));
    }
    let mut arguments = Vec::with_capacity(source.len());
    for (position, (argument, site)) in source.iter().zip(row.argument_sites()).enumerate() {
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
                let prefix = original.map(|original| ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                    original, position, &arguments, CallValidationContextV1::Lowering(ledger),
                    expression.and_then(|prefix| prefix.transient()))).transpose()?;
                if consume_receiver.is_none() && arguments.iter().map(|argument| argument.deferred_text_values_v1(
                    CallValidationContextV1::Lowering(ledger))).collect::<Result<Vec<_>, _>>()?.iter().any(|values| !values.is_empty()) {
                    return Err(freeze("lexical-i64/argument-receiver-scope"));
                }
                let scoped = prefix.zip(consume_receiver).map(|(prefix, consumer)| ExpressionCallPrefixV1::for_argument(
                    prefix, consumer, expression.and_then(|outer| outer.static_source()).or(static_source)));
                ArgumentProjection::CallResult(Box::new(emit_lexical_i64_call(
                    builder, state, ledger, owner, ArgumentSource::from_nested(inner),
                    CallSource::instance(inner_row), bindings,
                    scoped.as_ref().or(expression), consume_receiver, static_source,
                )?))
            }
            LocalCallArgumentV1::BorrowedActual {
                ordinal,
                site: original_site,
            }
            | LocalCallArgumentV1::Expression {
                ordinal,
                site: original_site,
            } => {
                let expression_only = matches!(argument, LocalCallArgumentV1::Expression { .. });
                if original_site != site {
                    return Err(freeze("lexical-i64/borrowed-site"));
                }
                let Some(actuals) = row.borrowed_actuals(ledger)? else {
                    return Err(freeze("lexical-i64/borrowed-actuals-missing"));
                };
                let actual = actuals
                    .iter()
                    .find(|actual| actual.ordinal == *ordinal && &actual.site == site)
                    .ok_or_else(|| freeze("lexical-i64/borrowed-actual-missing"))?;
                use crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1 as Source;
                match &actual.source {
                    Source::Expression(original_frame) => {
                        let canonical = ledger.qualified_argument_expression_frame_v1(row.call_site(), *ordinal, site)?;
                        if !std::rc::Rc::ptr_eq(original_frame.frame(), &canonical) || *ordinal as usize != position {
                            return Err(freeze("lexical-i64/borrowed-expression-original"));
                        }
                        let prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                            original.ok_or_else(|| freeze("lexical-i64/argument-original-source"))?, position,
                            &arguments, CallValidationContextV1::Lowering(ledger), expression.and_then(|prefix| prefix.transient()))?;
                        let consumer = consume_receiver.ok_or_else(|| freeze("lexical-i64/argument-receiver-scope"))?;
                        let (_, projection) = super::super::argument_expression::emit_with_prefix(
                            builder, state, ledger, row.call_site(), *ordinal, site, consumer, Some(&prefix),
                            expression.and_then(|outer| outer.static_source()).or(static_source))?;
                        for binding in projection.expression_bindings_v1() {
                            // The function frame is a shared dependency. Every
                            // acquisition/effect remains independently recorded.
                            if matches!(binding.1, MirInstruction::FaultFrameEnter { .. }) && bindings.contains(binding) { continue; }
                            bindings.push(binding.clone());
                        }
                        projection
                    }
                    _ if expression_only => {
                        return Err(freeze("lexical-i64/expression-source-required"));
                    }
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
                            | Source::LocalObjectBinding { binding, .. }
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
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<(), String> {
    ledger.corroborate_unit_discard_v1(&row)?;
    let site = row.call_site().clone();
    let observation = ledger.unit_discard_source_v1(&site).ok_or_else(|| freeze("unit-call/source-missing"))?;
    let mut bindings = Vec::new();
    let packet = emit_lexical_i64_call(builder, state, ledger, site.owner(), ArgumentSource::from_local(observation),
        CallSource::instance(row), &mut bindings, None, Some(consume_receiver), static_source)?;
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
    let (receiver, read) = lexical_receiver::prepare(builder, state, owner, &row, ledger, lexical_receiver::Fault::Constructor { frame, landing: fault_landing }, &mut bindings)?;
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
