//! The existing Call emitter borrows its enclosing expression's live prefix.
//! No additional source call, Home, or packet inventory is created here.
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::normal_callable_semantic_package::EmittedDynamicInvocationCallProjectionV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::normal_callable_semantic_package::ExpressionCallTransientPrefixV1;
use crate::mir::normal_callable_semantic_package::ExpressionCallArgumentSourceV1;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedBoolConditionFrameV1, CheckedExpressionEvaluationV1, LocalCallObservationV1,
};
use crate::mir::resolved_semantics::{OwnedExprSiteV1};
use super::super::super::expression_value::ExpressionCallEmissionV1;

fn in_arguments(arguments: &[LocalCallArgumentV1], site: &OwnedExprSiteV1) -> bool {
    arguments.iter().any(|argument| matches!(argument, LocalCallArgumentV1::CallResult(inner)
        if inner.site() == site || in_arguments(inner.arguments(), site)))
}

/// One expression-owned prefix for the same physical Call writer. Bool
/// conditions own no String temporaries; their existing prior Homes still unwind.
pub(in crate::mir::builder) type StaticExpressionCallIssuerV1<'a> = dyn Fn(
    &mut MirBuilder,
    &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
    &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
    &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
) -> Result<crate::mir::normal_callable_semantic_package::CallPacketSourceV1, String> + 'a;

pub(super) struct ExpressionCallPrefixV1<'a> {
    kind: ExpressionCallPrefixKindV1<'a>,
    static_source: Option<&'a StaticExpressionCallIssuerV1<'a>>,
    consume_receiver: &'a dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
}
enum ExpressionCallPrefixKindV1<'a> {
    Evaluation(ExpressionCallTransientPrefixV1<'a>),
    Bool { frame: &'a CheckedBoolConditionFrameV1, call: &'a LocalCallObservationV1 },
}
impl<'a> ExpressionCallPrefixV1<'a> {
    pub(super) fn original_observation_v1(&self) -> Option<&LocalCallObservationV1> {
        match &self.kind {
            ExpressionCallPrefixKindV1::Evaluation(prefix) => prefix.original_observation_v1(),
            ExpressionCallPrefixKindV1::Bool { call, .. } => Some(call),
        }
    }
    pub(super) fn for_argument(prefix: ExpressionCallTransientPrefixV1<'a>, consume_receiver: &'a dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>, static_source: Option<&'a StaticExpressionCallIssuerV1<'a>>) -> Self {
        Self { kind: ExpressionCallPrefixKindV1::Evaluation(prefix), consume_receiver, static_source }
    }
    pub(super) fn static_source(&self) -> Option<&StaticExpressionCallIssuerV1<'_>> { self.static_source }
    pub(super) fn transient(&self) -> Option<&ExpressionCallTransientPrefixV1<'_>> {
        match &self.kind { ExpressionCallPrefixKindV1::Evaluation(prefix) => Some(prefix), _ => None }
    }
    pub(super) fn consume_receiver(&self, row: &LexicalInstanceCallDispositionRowV1) -> Result<(), String> {
        (self.consume_receiver)(row)
    }
    pub(super) fn unwind(&self, site: &OwnedExprSiteV1, prior_homes: &[LiveHomeV1])
        -> Result<Vec<InvokeOperation>, String> {
        match &self.kind {
            ExpressionCallPrefixKindV1::Evaluation(prefix) => prefix.unwind_for_call(site, prior_homes),
            ExpressionCallPrefixKindV1::Bool { frame, call } => {
                if prior_homes != frame.prior_homes()
                    || (site != call.site() && !in_arguments(call.arguments(), site)) {
                    return Err(freeze("bool-condition-call/context-drift"));
                }
                Ok(Vec::new())
            }
        }
    }
}

/// The enclosing source-scoped descent retains this real Call packet. No
/// standalone group overlaps the condition's source/control prefix.
pub(in crate::mir::builder) fn emit_bool(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, frame: &CheckedBoolConditionFrameV1,
    call: &LocalCallObservationV1, bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<EmittedCall, String> {
    if !frame.calls().iter().any(|original| std::ptr::eq(original, call))
        || call.prior_homes() != frame.prior_homes()
        || ledger.lexical_i64_call_source(call.site()) != Some(call)
        || state.owner() != frame.source().owner()
    { return Err(freeze("bool-condition-call/source-drift")); }
    let prefix = ExpressionCallPrefixV1 { kind: ExpressionCallPrefixKindV1::Bool { frame, call }, consume_receiver, static_source: None };
    let row = ledger.take_lexical_instance_call(call.owner(), call.site().site())?
        .ok_or_else(|| freeze("bool-condition-call/disposition-missing"))?;
    emit_lexical_i64_call(builder, state, ledger, call.owner(), ArgumentSource::from_local(call),
        CallSource::instance(row), bindings, Some(&prefix), Some(consume_receiver), None)
}

/// This is the same real Call writer, returning its packet to the enclosing
/// prefix instead of publishing an overlapping standalone Call binding group.
pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    call: &LocalCallObservationV1,
    acquired: &[(usize, ValueId)],
    values: &[Option<EvaluatedExpressionValueV1>],
    bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<EmittedCall, String> {
    emit_with_prefix(builder, state, ledger, frame, call, acquired, values, bindings, consume_receiver, static_source, None)
}

pub(in crate::mir::builder) fn emit_with_prefix(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    call: &LocalCallObservationV1,
    acquired: &[(usize, ValueId)],
    values: &[Option<EvaluatedExpressionValueV1>],
    bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
    inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
) -> Result<EmittedCall, String> {
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let prefix = ExpressionCallPrefixV1 { kind: ExpressionCallPrefixKindV1::Evaluation(
        ExpressionCallTransientPrefixV1::from_evaluation(frame, call, acquired, values, function, inherited)?), consume_receiver, static_source };
    if !frame.calls().iter().any(|original| std::ptr::eq(original, call))
        || state.owner() != frame.source().owner() {
        return Err(freeze("expression-call/source-drift"));
    }
    let source = match call.result_source() {
        Some(crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::Ordinary(loan)) if loan.static_source().is_some() =>
            static_source.ok_or_else(|| freeze("expression-call/disposition-missing"))?(
                    builder, loan.static_source().expect("checked Static source"), frame, call)?,
        _ => CallSource::instance(ledger.take_lexical_instance_call(call.owner(), call.site().site())?
            .ok_or_else(|| freeze("expression-call/disposition-missing"))?),
    };
    emit_lexical_i64_call(builder, state, ledger, call.owner(), ArgumentSource::from_local(call),
        source, bindings, Some(&prefix), Some(consume_receiver), static_source)
}

/// DynamicInvocation has its own source-bound Invoke projection. It never
/// consumes a lexical Call disposition or enters the legacy Call packet.
pub(in crate::mir::builder) fn emit_with_dynamic(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    call: &LocalCallObservationV1,
    acquired: &[(usize, ValueId)],
    values: &[Option<EvaluatedExpressionValueV1>],
    bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
) -> Result<ExpressionCallEmissionV1, String> {
    emit_with_dynamic_and_argument_frames(
        builder, state, ledger, frame, call, acquired, values, bindings,
        consume_receiver, static_source, &[],
    )
}

pub(in crate::mir::builder) fn emit_with_dynamic_and_argument_frames(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: &CheckedExpressionEvaluationV1,
    call: &LocalCallObservationV1,
    acquired: &[(usize, ValueId)],
    values: &[Option<EvaluatedExpressionValueV1>],
    bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&StaticExpressionCallIssuerV1<'_>>,
    retained_argument_frames: &[std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>],
) -> Result<ExpressionCallEmissionV1, String> {
    if !frame.calls().iter().any(|original| std::ptr::eq(original, call))
        || state.owner() != frame.source().owner() {
        return Err(freeze("expression-call/source-drift"));
    }
    if !matches!(call.result_source(), Some(
        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::DynamicInvocation(_))) {
        return emit_with_prefix(builder, state, ledger, frame, call, acquired, values, bindings,
            consume_receiver, static_source, None).map(ExpressionCallEmissionV1::Lexical);
    }

    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let owned_site = OwnedExprSiteV1::new(call.owner(), call.site().site().clone());
    let prefix = ExpressionCallPrefixV1 {
        kind: ExpressionCallPrefixKindV1::Evaluation(ExpressionCallTransientPrefixV1::from_evaluation(
            frame, call, acquired, values, function, None)?),
        consume_receiver,
        static_source,
    };
    let Some(crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::DynamicInvocation(loan)) = call.result_source() else {
        return Err(freeze("dynamic-expression/source-loan"));
    };
    let target = loan.target();
    if call.arguments().len() != loan.argument_sites().len() {
        return Err(freeze("dynamic-expression/argument-census"));
    }
    let receiver_read = state.take_exact_lexical_read(
        call.owner(), target.receiver_site().node(), target.receiver_binding(),
    ).map_err(|error| error.to_string())?;
    let receiver_value = receiver_read.value_for(
        call.owner(), target.receiver_site().node(), target.receiver_binding(),
    ).map_err(|error| error.to_string())?;
    let context = CallValidationContextV1::Lowering(ledger);
    let mut argument_projections = Vec::with_capacity(call.arguments().len());
    for (position, (argument, site)) in call.arguments().iter().zip(loan.argument_sites()).enumerate() {
        let crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Expression {
            ordinal, site: actual_site,
        } = argument else {
            return Err(freeze("dynamic-expression/argument-purpose"));
        };
        if *ordinal as usize != position || actual_site != site {
            return Err(freeze("dynamic-expression/argument-source"));
        }
        let argument_frame = match super::super::argument_expression::retained_argument_frame_v1(
            retained_argument_frames,
            call.site(),
            *ordinal,
            site,
        )? {
            Some(frame) => frame,
            None => ledger.qualified_argument_expression_frame_v1(call.site(), *ordinal, site)?,
        };
        if !argument_frame.is_i64() {
            return Err(freeze("dynamic-expression/argument-class"));
        }
        let argument_prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
            ExpressionCallArgumentSourceV1::from_local(call), position, &argument_projections,
            context, prefix.transient(),
        )?;
        let (value, projection) = super::super::argument_expression::emit_with_checked_frame(
            builder,
            state,
            ledger,
            call.site(),
            *ordinal,
            site,
            argument_frame,
            retained_argument_frames,
            consume_receiver,
            Some(&argument_prefix),
            static_source,
        )?;
        let projected = projection.value_for_source_v1(argument, call.owner(), site, None, Some(context))?;
        if projected != value {
            return Err(freeze("dynamic-expression/argument-value"));
        }
        for binding in projection.expression_bindings_v1() {
            if matches!(binding.1, MirInstruction::FaultFrameEnter { .. }) && bindings.contains(binding) {
                continue;
            }
            bindings.push(binding.clone());
        }
        argument_projections.push(projection);
    }
    let operation = EmittedDynamicInvocationCallProjectionV1::operation_from_source_v1(
        frame, call, &argument_projections, context,
        crate::mir::instruction::DynamicInvocationOperandV1::TaggedDynamic(receiver_value),
    )?;
    let mut unwind = prefix.unwind(&owned_site, call.prior_homes())?;
    unwind.extend(ledger.prior_home_unwind_for(call.prior_homes())?);
    let fault_frame = state.borrow_fault_frame(builder)?;
    let shared_frame = fault_frame_binding(builder, state, fault_frame)?;
    if !bindings.contains(&shared_frame) { bindings.push(shared_frame); }
    let mut writer = RawRootGraphWriterV1;
    let (value, result, pair) = super::super::super::expression_value::emit_scalar_operation_with_pending_v1(
        builder, &mut writer, InvokeOperation::DynamicInvocation(operation.clone()), fault_frame,
        &unwind, bindings,
    )?;
    if result != crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier {
        return Err(freeze("dynamic-expression/result-kind"));
    }
    let packet = EmittedDynamicInvocationCallProjectionV1::from_source_v1(
        frame, call, receiver_read, operation, argument_projections, context,
        pair[0].clone(), pair[1].clone(),
    )?;
    if packet.value() != value { return Err(freeze("dynamic-expression/result-value")); }
    Ok(ExpressionCallEmissionV1::Dynamic(std::rc::Rc::new(packet)))
}
