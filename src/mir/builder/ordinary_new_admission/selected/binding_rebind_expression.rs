//! Checked RHS -> common scalar cleanup -> sole assignment writer -> publication.
use super::*;
use super::expression_value;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::{EmittedBindingRebindProjectionV1, LexicalInstanceCallDispositionRowV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1;
use crate::mir::builder::stmts::variable_assignment_completion::{build_scalar_variable_assignment_with_completion_v1, ScalarRebindDestinationV1};
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedBindingRebindFrameV1>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<ValueId, String> {
    let destination = ScalarRebindDestinationV1::prepare(builder, state, ledger, Rc::clone(&frame))?;
    let name = destination.name().to_owned();
    let previous = state.value_for_exact_binding(frame.source().owner(), frame.target()).map_err(|error| error.to_string())?;
    // A LocalContractWrite needs its own result relation. Refuse before RHS
    // effects, never silently invoke the raw assignment after selecting a frame.
    if builder.function_state.binding_ctx.lookup(&name).is_some_and(|binding|
        builder.function_state.current_function.as_ref().is_some_and(|function|
            crate::mir::type_contracts::local_slot::local_slot_contract(function, crate::mir::LocalSlotId::from(binding)).is_some())) {
        return Err(freeze("rebind-expression/local-contract-receipt-pending"));
    }
    let mut expression = expression_value::emit(builder, state, ledger, frame.evaluation(),
        &mut |builder, state, call, acquired, values, bindings| terminal_call::emit_expression_call(
            builder, state, ledger, frame.evaluation(), call, acquired, values, bindings, consume_receiver))?;
    let Operand::NormalInteger(value) = expression.value else { return Err(freeze("rebind-expression/result-class")) };
    let (terminal, continuation) = expression_value::finish_scalar_continuation(builder, &mut expression)?;
    if state.value_for_exact_binding(frame.source().owner(), frame.target()).map_err(|error| error.to_string())? != previous
        || builder.function_state.variable_ctx.variable_map.get(&name) != Some(&previous) {
        return Err(freeze("rebind-expression/destination-drift"));
    }
    let block = builder.function_state.current_block.ok_or_else(|| freeze("rebind-expression/no-block"))?;
    let before = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?
        .blocks.get(&block).ok_or_else(|| freeze("no-block"))?.instructions.len();
    let receipt = build_scalar_variable_assignment_with_completion_v1(builder, destination, value)
        .map_err(|error| error.bounded_report())?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let effects = &function.blocks.get(&block).ok_or_else(|| freeze("no-block"))?.instructions[before..];
    if !effects.is_empty() { return Err(freeze("rebind-expression/unexpected-assignment-effect")); }
    let assigned = receipt.assigned();
    let bindings = expression.bindings.clone();
    let packet = Rc::new(EmittedBindingRebindProjectionV1::new(Rc::clone(&frame), receipt, expression.values, expression.calls, expression.path_reads.into_values().collect(), expression.reference_reads,
        expression.entry, terminal, continuation, expression.bindings, ledger, function)?);
    let pending = ledger.prepare_rebind_expression_bindings(packet, bindings)?;
    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment } = frame.source().root()
        else { return Err(freeze("rebind-expression/source-purpose")) };
    state.record_checked_rebind_v1(assignment.target_site().node(), Rc::clone(&frame), assigned)?;
    pending.commit()?;
    Ok(assigned)
}
