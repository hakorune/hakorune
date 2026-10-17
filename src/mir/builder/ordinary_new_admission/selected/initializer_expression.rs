//! Affine Local evaluation transport; publication waits for the existing receipt.
use super::*;
use super::expression_value::{self, EmittedExpressionValueV1};
use crate::mir::instruction::{StringInvokeOperand as Operand};
use crate::mir::normal_callable_semantic_package::{EmittedInitializerExpressionProjectionV1, LexicalInstanceCallDispositionRowV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

pub(in crate::mir::builder) struct PendingInitializerExpressionV1 {
    ordinal: u32,
    frame: Rc<CheckedInitializerExpressionFrameV1>,
    expression: EmittedExpressionValueV1,
    terminal: Binding,
    continuation: Binding,
}
impl PendingInitializerExpressionV1 {
    pub(in crate::mir::builder) fn ordinal(&self) -> u32 { self.ordinal }
    pub(in crate::mir::builder) fn value(&self) -> ValueId {
        match self.expression.value { Operand::NormalInteger(value) => value, _ => unreachable!("selected scalar Local policy") }
    }
    pub(in crate::mir::builder) fn placement(&self, value: ValueId) -> Result<crate::mir::builder::stmts::variable_stmt::LocalValuePlacement, String> {
        if value != self.value() { return Err(freeze("initializer-expression/placement-value")); }
        Ok(crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::Copy)
    }
    pub(in crate::mir::builder) fn publish(self,
        receipt: &crate::mir::builder::stmts::CompletedLocalBindingV1,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<(), String> {
        if receipt.ordinal() != self.ordinal || receipt.initializer() != self.value() { return Err(freeze("initializer-expression/local-receipt")); }
        let expression = self.expression;
        let mut bindings = expression.bindings.clone();
        if let Some(copy) = receipt.copy() { bindings.push(copy.clone()); }
        let packet = Rc::new(EmittedInitializerExpressionProjectionV1::new(self.frame,
            receipt.initializer(), receipt.local(), receipt.copy().cloned(), expression.values,
            expression.calls, expression.path_reads.into_values().collect(), expression.reference_reads,
            expression.entry, self.terminal, self.continuation, expression.bindings, ledger, function)?);
        ledger.prepare_initializer_expression_bindings(packet, bindings)?.commit()
    }
}
pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedInitializerExpressionFrameV1>, ordinal: u32,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<PendingInitializerExpressionV1, String> {
    // Text root transfer needs its source-aware ownership handoff. Preserve the
    // existing fence until that selected source view and both consumers close.
    if !frame.is_i64() { return Err(freeze("text-initializer/execution-pending")); }
    if crate::mir::type_contracts::local_slot::is_exact_numeric_local_type(
        frame.source().root().initializer().and_then(|row| row.declared_type_name())) {
        return Err(freeze("initializer-expression/local-contract-receipt-pending"));
    }
    ledger.validate_initializer_expression_source_v1(&frame)?;
    let mut expression = expression_value::emit(builder, state, ledger, frame.evaluation(),
        &mut |builder, state, call, acquired, values, bindings| terminal_call::emit_expression_call(
            builder, state, ledger, frame.evaluation(), call, acquired, values, bindings, consume_receiver))?;
    if !matches!(expression.value, Operand::NormalInteger(_)) { return Err(freeze("initializer-expression/scalar-result")); }
    let (terminal, continuation) = expression_value::finish_scalar_continuation(builder, &mut expression)?;
    Ok(PendingInitializerExpressionV1 { ordinal, frame, expression, terminal, continuation })
}
