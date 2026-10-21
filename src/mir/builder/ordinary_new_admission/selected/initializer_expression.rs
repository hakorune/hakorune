//! Affine Local evaluation transport; publication waits for the existing receipt.
use super::*;
use super::expression_value::{self, EmittedExpressionValueV1};
use crate::mir::instruction::{StringInvokeOperand as Operand};
use crate::mir::normal_callable_semantic_package::{EmittedInitializerExpressionProjectionV1, LexicalInstanceCallDispositionRowV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

/// One pending Local transport, with purpose-specific value and publication laws.
pub(in crate::mir::builder) enum PendingInitializerExpressionV1 {
    Expression(PendingEvaluatedInitializerV1),
    StoredMapGet(terminal_map_get::PendingLocalV1),
}
impl PendingInitializerExpressionV1 {
    pub(in crate::mir::builder) fn ordinal(&self) -> u32 {
        match self { Self::Expression(p) => p.ordinal(), Self::StoredMapGet(p) => p.ordinal() }
    }
    pub(in crate::mir::builder) fn value(&self) -> ValueId {
        match self { Self::Expression(p) => p.value(), Self::StoredMapGet(p) => p.value() }
    }
    pub(in crate::mir::builder) fn placement(&self, value: ValueId) -> Result<crate::mir::builder::stmts::variable_stmt::LocalValuePlacement, String> {
        match self {
            Self::Expression(p) => p.placement(value),
            Self::StoredMapGet(p) if p.value() == value => Ok(crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::Copy),
            _ => Err(freeze("initializer-expression/placement-value")),
        }
    }
    pub(in crate::mir::builder) fn publish(self,
        receipt: &crate::mir::builder::stmts::CompletedLocalBindingV1,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<(), String> {
        match self { Self::Expression(p) => p.publish(receipt, ledger, function), Self::StoredMapGet(p) => p.publish(receipt, ledger, function) }
    }
}

pub(in crate::mir::builder) struct PendingEvaluatedInitializerV1 {
    ordinal: u32,
    frame: Rc<CheckedInitializerExpressionFrameV1>,
    expression: EmittedExpressionValueV1,
    terminal: Binding,
    continuation: Binding,
}
impl PendingEvaluatedInitializerV1 {
    pub(in crate::mir::builder) fn ordinal(&self) -> u32 { self.ordinal }
    pub(in crate::mir::builder) fn value(&self) -> ValueId {
        match self.expression.value.as_operand().expect("checked Local initializer operand") { Operand::NormalInteger(value) | Operand::OwnedString(value) => *value, _ => unreachable!("checked Local initializer value") }
    }
    pub(in crate::mir::builder) fn placement(&self, value: ValueId) -> Result<crate::mir::builder::stmts::variable_stmt::LocalValuePlacement, String> {
        if value != self.value() { return Err(freeze("initializer-expression/placement-value")); }
        Ok(if self.frame.is_text_residence() {
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
            expression.body.calls, expression.body.path_reads.into_values().collect(), expression.body.reference_reads,
            expression.body.entry, self.terminal, self.continuation, expression.body.bindings, ledger, function)?);
        ledger.prepare_initializer_expression_bindings(packet, bindings)?.commit()
    }
}
pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedInitializerExpressionFrameV1>, ordinal: u32,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<PendingInitializerExpressionV1, String> {
    if !frame.is_i64() && !frame.is_text_residence() { return Err(freeze("text-initializer/execution-pending")); }
    if crate::mir::type_contracts::local_slot::is_exact_numeric_local_type(
        frame.source().root().initializer().and_then(|row| row.declared_type_name())) {
        return Err(freeze("initializer-expression/local-contract-receipt-pending"));
    }
    ledger.validate_initializer_expression_source_v1(&frame)?;
    let mut expression = expression_value::emit(builder, state, ledger, frame.evaluation(),
        &mut |builder, state, call, acquired, values, bindings| terminal_call::emit_expression_call(
            builder, state, ledger, frame.evaluation(), call, acquired, values, bindings, consume_receiver, static_source))?;
    let (terminal, continuation) = if frame.is_text_residence() {
        expression_value::finish_residence_continuation(builder, &mut expression)?
    } else { expression_value::finish_scalar_continuation(builder, &mut expression)? };
    Ok(PendingInitializerExpressionV1::Expression(PendingEvaluatedInitializerV1 { ordinal, frame, expression, terminal, continuation }))
}
