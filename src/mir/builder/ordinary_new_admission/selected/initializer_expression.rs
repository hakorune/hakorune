//! Affine Local evaluation transport; publication waits for the existing receipt.
use super::*;
use super::expression_value::{self, EmittedExpressionValueV1};
use crate::mir::normal_callable_semantic_package::{EmittedInitializerExpressionProjectionV1, LexicalInstanceCallDispositionRowV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

/// One pending Local transport, with purpose-specific value and publication laws.
pub(in crate::mir::builder) enum PendingInitializerExpressionV1 {
    Expression(PendingEvaluatedInitializerV1),
    StoredMapGet(terminal_map_get::PendingLocalV1),
    Move { ordinal: u32, value: ValueId },
    Shared {
        ordinal: u32,
        source: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        payload_source: crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1,
        prior_homes: Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>,
        descriptor: crate::mir::instruction::SharedDescriptorV1,
        payload: ValueId,
        value: ValueId,
        bindings: Vec<Binding>,
    },
}
impl PendingInitializerExpressionV1 {
    pub(in crate::mir::builder) fn ordinal(&self) -> u32 {
        match self { Self::Expression(p) => p.ordinal(), Self::StoredMapGet(p) => p.ordinal(), Self::Move { ordinal, .. } | Self::Shared { ordinal, .. } => *ordinal }
    }
    pub(in crate::mir::builder) fn value(&self) -> ValueId {
        match self { Self::Expression(p) => p.value(), Self::StoredMapGet(p) => p.value(), Self::Move { value, .. } | Self::Shared { value, .. } => *value }
    }
    pub(in crate::mir::builder) fn placement(&self, value: ValueId) -> Result<crate::mir::builder::stmts::variable_stmt::LocalValuePlacement, String> {
        match self {
            Self::Expression(p) => p.placement(value),
            Self::StoredMapGet(p) if p.value() == value => Ok(crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::Copy),
            Self::Move { value: source, .. } | Self::Shared { value: source, .. } if *source == value => Ok(crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::ReuseInitializer),
            _ => Err(freeze("initializer-expression/placement-value")),
        }
    }
    pub(in crate::mir::builder) fn publish(self,
        receipt: &crate::mir::builder::stmts::CompletedLocalBindingV1,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<(), String> {
        match self {
            Self::Expression(p) => p.publish(receipt, ledger, function),
            Self::StoredMapGet(p) => p.publish(receipt, ledger, function),
            Self::Move { ordinal, value } if receipt.ordinal() == ordinal && receipt.initializer() == value && receipt.local() == value && receipt.copy().is_none() => Ok(()),
            Self::Move { .. } => Err(freeze("move-initializer/local-receipt")),
            Self::Shared { ordinal, source, payload_source, prior_homes, descriptor, payload, value, bindings } => {
                if receipt.ordinal() != ordinal || receipt.initializer() != value || receipt.local() != value || receipt.copy().is_some() {
                    return Err(freeze("shared-initializer/local-receipt"));
                }
                ledger.prepare_shared_initializer_bindings_v1(source, payload_source, prior_homes,
                    descriptor, payload, value, bindings, function)?.commit()
            }
        }
    }
}

pub(in crate::mir::builder) fn emit_shared_initializer_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    ordinal: u32, source: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    from: crate::mir::resolved_semantics::BindingRefV1, acquisition: OwnedExprSiteV1,
    prior_homes: Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>, value: ValueId, is_clone: bool,
) -> Result<PendingInitializerExpressionV1, String> {
    let (descriptor, payload, unwind) = if is_clone {
        ledger.prepare_shared_clone_handle_v1(from, &acquisition, &prior_homes)?
    } else {
        ledger.prepare_shared_move_payload_v1(from, &acquisition, &prior_homes)?
    };
    if payload != value { return Err(freeze("shared-initializer/payload-value-drift")); }
    let payload_source = crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1::Existing { from, acquisition, is_clone };
    emit_shared_initializer_common_v1(builder, state, ordinal, source, payload_source, prior_homes, descriptor, payload, unwind, is_clone)
}

pub(in crate::mir::builder) fn emit_shared_fresh_initializer_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    ordinal: u32, source: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    site: OwnedExprSiteV1, shared_site: OwnedExprSiteV1, binding: crate::mir::resolved_semantics::BindingRefV1,
    prior_homes: Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>, value: ValueId,
) -> Result<PendingInitializerExpressionV1, String> {
    let (descriptor, payload, unwind) = ledger.prepare_shared_fresh_payload_v1(&site, &shared_site, binding, &prior_homes)?;
    if payload != value { return Err(freeze("shared-fresh/payload-value-drift")); }
    let payload_source = crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1::Fresh { site };
    emit_shared_initializer_common_v1(builder, state, ordinal, source, payload_source, prior_homes, descriptor, payload, unwind, false)
}

fn emit_shared_initializer_common_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ordinal: u32,
    source: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    payload_source: crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1,
    prior_homes: Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>,
    descriptor: crate::mir::instruction::SharedDescriptorV1, payload: ValueId,
    unwind: Vec<InvokeOperation>, is_clone: bool,
) -> Result<PendingInitializerExpressionV1, String> {
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut bindings)?;
    let fault = cleanup_chain(builder, frame, unwind, outward, &mut bindings)?;
    let operation = if is_clone {
        crate::mir::instruction::SharedInvokeOperation::Clone { descriptor: descriptor.clone(), handle: payload }
    } else {
        crate::mir::instruction::SharedInvokeOperation::New { descriptor: descriptor.clone(), payload }
    };
    let result = super::invoke_operation::emit(builder, frame, InvokeOperation::Shared(operation),
        fault, &mut bindings)?.0.ok_or_else(|| freeze("shared-initializer/result-missing"))?;
    Ok(PendingInitializerExpressionV1::Shared { ordinal, source, payload_source, prior_homes, descriptor, payload, value: result, bindings })
}

pub(in crate::mir::builder) struct PendingEvaluatedInitializerV1 {
    ordinal: u32,
    frame: Rc<CheckedInitializerExpressionFrameV1>,
    expression: EmittedExpressionValueV1,
    terminal: Binding,
    continuation: Binding,
    loop_residency: Option<(Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        Vec<crate::mir::builder::BindingValueObservationV2>)>,
}
impl PendingEvaluatedInitializerV1 {
    pub(in crate::mir::builder) fn ordinal(&self) -> u32 { self.ordinal }
    pub(in crate::mir::builder) fn value(&self) -> ValueId {
        self.expression.value.value().expect("checked Local initializer value")
    }
    pub(in crate::mir::builder) fn placement(&self, value: ValueId) -> Result<crate::mir::builder::stmts::variable_stmt::LocalValuePlacement, String> {
        if value != self.value() { return Err(freeze("initializer-expression/placement-value")); }
        Ok(if self.frame.is_text_residence() || self.frame.is_dynamic_invocation_result() {
            crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::ReuseInitializer
        } else { crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::Copy })
    }
    pub(in crate::mir::builder) fn publish(self,
        receipt: &crate::mir::builder::stmts::CompletedLocalBindingV1,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<(), String> {
        if receipt.ordinal() != self.ordinal || receipt.initializer() != self.value() { return Err(freeze("initializer-expression/local-receipt")); }
        let expression = self.expression;
        let mut bindings = expression.body.bindings.clone();
        if let Some(copy) = receipt.copy() { bindings.push(copy.clone()); }
        let packet = Rc::new(EmittedInitializerExpressionProjectionV1::new(self.frame,
            receipt.initializer(), receipt.local(), receipt.copy().cloned(), expression.body.values,
            expression.body.calls, expression.body.dynamic_calls, expression.body.path_reads, expression.body.reference_reads,
            expression.body.entry, self.terminal, self.continuation, self.loop_residency, expression.purpose_acquisition,
            expression.body.bindings, ledger, function)?);
        ledger.prepare_initializer_expression_bindings(packet, bindings)?.commit()
    }
}
pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedInitializerExpressionFrameV1>, ordinal: u32,
    source_loop: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
    retained_argument_frames: &[Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>],
) -> Result<PendingInitializerExpressionV1, String> {
    if !frame.is_i64() && !frame.is_bool() && !frame.is_text_residence() && !frame.is_dynamic_value() {
        return Err(freeze("text-initializer/execution-pending"));
    }
    if crate::mir::type_contracts::local_slot::is_exact_numeric_local_type(
        frame.source().root().initializer().and_then(|row| row.declared_type_name())) {
        return Err(freeze("initializer-expression/local-contract-receipt-pending"));
    }
    ledger.validate_initializer_expression_source_v1(&frame)?;
    let loop_residency = source_loop.map(|product| {
        let reads = product.expression_text_fault_producers_v2(frame.source())?.iter()
            .map(|(binding, _)| state.observe_binding_value_v2(*binding).map_err(|error| error.to_string()))
            .collect::<Result<Vec<_>, String>>()?;
        Ok::<_, String>((product, reads))
    }).transpose()?;
    let unwind = loop_residency.as_ref().map(|(product, reads)|
        ledger.prepare_loop_expression_text_unwind_v2(frame.evaluation(), product, reads,
            builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)).transpose()?;
    if let (Some(unwind), Some((product, _))) = (&unwind, &loop_residency) {
        // Every call of this loop expression Faults with the same live Text carriers.
        let plain = ledger.prior_home_unwind_for(frame.prior_homes())?;
        let ends: Vec<_> = unwind.iter().filter(|operation| !plain.contains(operation)).cloned().collect();
        for call in frame.evaluation().calls() {
            ledger.record_loop_call_text_carrier_ends_v2(call.site().clone(), product.source.condition.prior_homes().len(), ends.clone())?;
        }
    }
    let mut expression = expression_value::emit_with_dynamic_calls(builder, state, ledger, frame.evaluation(), None, unwind.as_deref(),
        &mut |builder, state, call, acquired, values, bindings| terminal_call::emit_expression_call_with_dynamic_and_argument_frames(
            builder, state, ledger, frame.evaluation(), call, acquired, values, bindings,
            consume_receiver, static_source, retained_argument_frames))?;
    expression_value::materialize_text_root(builder, &mut expression, frame.materializes_root(), frame.evaluation())?;
    let (terminal, continuation) = if frame.is_dynamic_invocation_result() {
        expression_value::finish_dynamic_initializer_continuation(builder, &mut expression)?
    } else if frame.is_dynamic_alias() {
        expression_value::finish_value_continuation_v1(builder, &mut expression.body)?
    } else if frame.is_text_residence() {
        expression_value::finish_residence_continuation(builder, &mut expression)?
    } else { expression_value::finish_scalar_continuation(builder, &mut expression)? };
    Ok(PendingInitializerExpressionV1::Expression(PendingEvaluatedInitializerV1 { ordinal, frame, expression, terminal, continuation, loop_residency }))
}
