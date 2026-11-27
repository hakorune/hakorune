//! Checked RHS -> common purpose cleanup -> sole assignment writer -> publication.
use super::expression_value;
use super::*;
use crate::mir::builder::stmts::variable_assignment_completion::{
    build_checked_variable_assignment_with_completion_v1, CheckedRebindDestinationV1,
};
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::normal_callable_semantic_package::{
    ExpressionEmissionV1, LexicalInstanceCallDispositionRowV1,
};
use crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1;
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedBindingRebindFrameV1>,
    source_loop: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<ValueId, String> {
    let destination =
        CheckedRebindDestinationV1::prepare(builder, state, ledger, Rc::clone(&frame))?;
    let name = destination.name().to_owned();
    let previous = destination.previous();
    let loop_residency = if frame.text_loop_carrier_v1().is_some() {
        Some((
            source_loop.ok_or_else(|| freeze("rebind-expression/old-text-end"))?,
            state
                .observe_binding_value_v2(frame.target())
                .map_err(|error| error.to_string())?,
        ))
    } else {
        None
    };
    let (displaced, unwind) = match &loop_residency {
        Some((product, read)) => {
            let function = builder
                .function_state
                .current_function
                .as_ref()
                .ok_or_else(|| freeze("no-function"))?;
            let (old, unwind) =
                ledger.prepare_loop_text_rebind_unwind_v2(&frame, product, read, function)?;
            (Some(old), Some(unwind))
        }
        None => (ledger.text_rebind_displaced_end_v1(&frame)?, None),
    };
    let mut expression = expression_value::emit_binding_write_rhs_v1(
        builder, state, ledger, frame.evaluation(), frame.materializes_root(),
        unwind.as_deref(), consume_receiver, static_source, None,
    )?;
    let dynamic_root = frame.evaluation().root_class()
        == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic,
        );
    if dynamic_root != frame.is_dynamic_loop_carrier_v1() {
        return Err(freeze("rebind-expression/dynamic-carrier-source"));
    }
    let value = match &expression.value {
        EvaluatedExpressionValueV1::BoxReference(value) if frame.declared_read_v1().is_some() => value.value(),
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
    let (terminal, continuation) =
        expression_value::finish_rebind_continuation(builder, &mut expression, &frame, displaced)?;
    if state
        .value_for_exact_binding(frame.source().owner(), frame.target())
        .map_err(|error| error.to_string())?
        != previous
        || builder.function_state.variable_ctx.variable_map.get(&name) != Some(&previous)
    {
        return Err(freeze("rebind-expression/destination-drift"));
    }
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("rebind-expression/no-block"))?;
    let before = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("no-function"))?
        .blocks
        .get(&block)
        .ok_or_else(|| freeze("no-block"))?
        .instructions
        .len();
    let mut receipt = build_checked_variable_assignment_with_completion_v1(builder, destination, value)
        .map_err(|error| error.bounded_report())?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("no-function"))?;
    let effects = &function
        .blocks
        .get(&block)
        .ok_or_else(|| freeze("no-block"))?
        .instructions[before..];
    if !receipt.record_checked_write(block, effects) {
        return Err(freeze("rebind-expression/unexpected-assignment-effect"));
    }
    let assigned = receipt.assigned();
    let mut bindings = expression.body.bindings.clone();
    bindings.extend(receipt.contract_write().cloned());
    let packet = Rc::new(ExpressionEmissionV1::new_rebind_v1(
        Rc::clone(&frame),
        receipt,
        expression.body,
        terminal,
        continuation,
        expression.purpose_acquisition,
        loop_residency,
        ledger,
        function,
    )?);
    let pending = ledger.prepare_rebind_expression_bindings(packet, bindings)?;
    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment {
        assignment,
    } = frame.source().root()
    else {
        return Err(freeze("rebind-expression/source-purpose"));
    };
    state.record_checked_rebind_v1(assignment.target_site().node(), Rc::clone(&frame), assigned)?;
    pending.commit()?;
    Ok(assigned)
}
