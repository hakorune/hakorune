//! The existing Call emitter borrows its enclosing expression's live prefix.
//! No additional source call, Home, or packet inventory is created here.
use super::*;
use crate::mir::instruction::{StringInvokeOperation, StringInvokeOperand};
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedBoolConditionFrameV1, CheckedExpressionEvaluationV1, LocalCallObservationV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};

fn in_arguments(arguments: &[LocalCallArgumentV1], site: &OwnedExprSiteV1) -> bool {
    arguments.iter().any(|argument| matches!(argument, LocalCallArgumentV1::CallResult(inner)
        if inner.site() == site || in_arguments(inner.arguments(), site)))
}

struct EvaluationCallPrefixV1<'a> {
    frame: &'a CheckedExpressionEvaluationV1,
    call: &'a LocalCallObservationV1,
    acquired: &'a [(usize, ValueId)],
}
impl<'a> EvaluationCallPrefixV1<'a> {
    fn new(
        frame: &'a CheckedExpressionEvaluationV1,
        call: &'a LocalCallObservationV1,
        acquired: &'a [(usize, ValueId)],
        values: &[Option<StringInvokeOperand>],
        function: &MirFunction,
    ) -> Result<Self, String> {
        super::super::super::expression_acquisition_prefix::validate(frame, Some(call), acquired, values, function)?;
        Ok(Self { frame, call, acquired })
    }

    pub(super) fn unwind(
        &self, site: &OwnedExprSiteV1, prior_homes: &[BindingRefV1],
    ) -> Result<Vec<InvokeOperation>, String> {
        if prior_homes != self.frame.prior_homes()
            || (site != self.call.site() && !in_arguments(self.call.arguments(), site)) {
            return Err(freeze("expression-call/context-drift"));
        }
        Ok(self.acquired.iter().rev().map(|(_, value)|
            InvokeOperation::String(StringInvokeOperation::End { value: *value })).collect())
    }
}

/// One expression-owned prefix for the same physical Call writer. Bool
/// conditions own no String temporaries; their existing prior Homes still unwind.
pub(super) struct ExpressionCallPrefixV1<'a> {
    kind: ExpressionCallPrefixKindV1<'a>,
    consume_receiver: &'a dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
}
enum ExpressionCallPrefixKindV1<'a> {
    Evaluation(EvaluationCallPrefixV1<'a>),
    Bool { frame: &'a CheckedBoolConditionFrameV1, call: &'a LocalCallObservationV1 },
}
impl ExpressionCallPrefixV1<'_> {
    pub(super) fn consume_receiver(&self, row: &LexicalInstanceCallDispositionRowV1) -> Result<(), String> {
        (self.consume_receiver)(row)
    }
    pub(super) fn unwind(&self, site: &OwnedExprSiteV1, prior_homes: &[BindingRefV1])
        -> Result<Vec<InvokeOperation>, String> {
        match &self.kind {
            ExpressionCallPrefixKindV1::Evaluation(prefix) => prefix.unwind(site, prior_homes),
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
    let prefix = ExpressionCallPrefixV1 { kind: ExpressionCallPrefixKindV1::Bool { frame, call }, consume_receiver };
    let row = ledger.take_lexical_instance_call(call.owner(), call.site().site())?
        .ok_or_else(|| freeze("bool-condition-call/disposition-missing"))?;
    emit_lexical_i64_call(builder, state, ledger, call.owner(), call.prior_homes(), call.arguments(),
        CallSource::instance(row), bindings, Some(&prefix))
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
    values: &[Option<StringInvokeOperand>],
    bindings: &mut Vec<Binding>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<EmittedCall, String> {
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let prefix = ExpressionCallPrefixV1 { kind: ExpressionCallPrefixKindV1::Evaluation(
        EvaluationCallPrefixV1::new(frame, call, acquired, values, function)?), consume_receiver };
    if ledger.lexical_i64_call_source(call.site()) != Some(call) || state.owner() != frame.source().owner() {
        return Err(freeze("expression-call/source-drift"));
    }
    let row = ledger.take_lexical_instance_call(call.owner(), call.site().site())?
        .ok_or_else(|| freeze("expression-call/disposition-missing"))?;
    emit_lexical_i64_call(builder, state, ledger, call.owner(), call.prior_homes(), call.arguments(),
        CallSource::instance(row), bindings, Some(&prefix))
}
