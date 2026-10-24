//! Checked RHS -> common purpose cleanup -> sole assignment writer -> publication.
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use super::*;
use super::expression_value;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::{EmittedBindingRebindProjectionV1, LexicalInstanceCallDispositionRowV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1;
use crate::mir::builder::stmts::variable_assignment_completion::{build_checked_variable_assignment_with_completion_v1, CheckedRebindDestinationV1};
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedBindingRebindFrameV1>,
    source_loop: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<ValueId, String> {
    let destination = CheckedRebindDestinationV1::prepare(builder, state, ledger, Rc::clone(&frame))?;
    let name = destination.name().to_owned();
    let previous = state.value_for_exact_binding(frame.source().owner(), frame.target()).map_err(|error| error.to_string())?;
    // A LocalContractWrite needs its own result relation. Refuse before RHS
    // effects, never silently invoke the raw assignment after selecting a frame.
    if builder.function_state.binding_ctx.lookup(&name).is_some_and(|binding|
        builder.function_state.current_function.as_ref().is_some_and(|function|
            crate::mir::type_contracts::local_slot::local_slot_contract(function, crate::mir::LocalSlotId::from(binding)).is_some())) {
        return Err(freeze("rebind-expression/local-contract-receipt-pending"));
    }
    let loop_residency = if frame.text_loop_carrier_v1().is_some() {
        Some((source_loop.ok_or_else(|| freeze("rebind-expression/old-text-end"))?,
            state.observe_binding_value_v2(frame.target()).map_err(|error| error.to_string())?))
    } else { None };
    let (displaced, unwind) = match &loop_residency {
        Some((product, read)) => {
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
            let (old, unwind) = ledger.prepare_loop_text_rebind_unwind_v2(&frame, product, read, function)?;
            (Some(old), Some(unwind))
        }
        None => (ledger.text_rebind_displaced_end_v1(&frame)?, None),
    };
    let mut emit_call = |builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
        call: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &[(usize, ValueId)], values: &[Option<EvaluatedExpressionValueV1>], bindings: &mut Vec<(BasicBlockId, MirInstruction)>|
        terminal_call::emit_expression_call(builder, state, ledger, frame.evaluation(), call,
            acquired, values, bindings, consume_receiver, None);
    let mut expression = match &unwind {
        Some(unwind) => expression_value::emit_with_prior_home_unwind(builder, state, ledger, frame.evaluation(), unwind, &mut emit_call)?,
        None => expression_value::emit(builder, state, ledger, frame.evaluation(), &mut emit_call)?,
    };
    expression_value::materialize_text_root(builder, &mut expression, frame.materializes_root())?;
    let dynamic_root = frame.evaluation().root_class()
        == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic);
    if dynamic_root != frame.is_dynamic_loop_carrier_v1() {
        return Err(freeze("rebind-expression/dynamic-carrier-source"));
    }
    let value = match &expression.value {
        EvaluatedExpressionValueV1::Bool(value)
            if frame.evaluation().root_class() == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Bool) => *value,
        EvaluatedExpressionValueV1::Operand(value) => match value {
        Operand::NormalInteger(value) if !frame.is_text_residence() => *value,
        Operand::Dynamic(value) | Operand::DynamicCarrier(value) if frame.is_dynamic_loop_carrier_v1() => *value,
        Operand::OwnedString(value) if frame.is_text_residence() => *value,
        _ => return Err(freeze("rebind-expression/result-class")),
        },
        _ => return Err(freeze("rebind-expression/result-class")),
    };
    let (terminal, continuation) = expression_value::finish_rebind_continuation(builder, &mut expression, &frame, displaced)?;
    if state.value_for_exact_binding(frame.source().owner(), frame.target()).map_err(|error| error.to_string())? != previous
        || builder.function_state.variable_ctx.variable_map.get(&name) != Some(&previous) {
        return Err(freeze("rebind-expression/destination-drift"));
    }
    let block = builder.function_state.current_block.ok_or_else(|| freeze("rebind-expression/no-block"))?;
    let before = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?
        .blocks.get(&block).ok_or_else(|| freeze("no-block"))?.instructions.len();
    let receipt = build_checked_variable_assignment_with_completion_v1(builder, destination, value)
        .map_err(|error| error.bounded_report())?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let effects = &function.blocks.get(&block).ok_or_else(|| freeze("no-block"))?.instructions[before..];
    if !effects.is_empty() { return Err(freeze("rebind-expression/unexpected-assignment-effect")); }
    let assigned = receipt.assigned();
    let bindings = expression.body.bindings.clone();
    let packet = Rc::new(EmittedBindingRebindProjectionV1::new(Rc::clone(&frame), receipt, expression.body.values, expression.body.calls, expression.body.path_reads.into_values().collect(), expression.body.reference_reads,
        expression.body.entry, terminal, continuation, expression.purpose_acquisition, loop_residency, expression.body.bindings, ledger, function)?);
    let pending = ledger.prepare_rebind_expression_bindings(packet, bindings)?;
    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment } = frame.source().root()
        else { return Err(freeze("rebind-expression/source-purpose")) };
    state.record_checked_rebind_v1(assignment.target_site().node(), Rc::clone(&frame), assigned)?;
    pending.commit()?;
    Ok(assigned)
}
