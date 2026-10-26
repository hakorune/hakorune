//! Original ReturnValue evaluates once and retains its result across cleanup.
use super::*;
use super::expression_value;
use std::rc::Rc;
use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1;

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    source: Rc<CheckedExpressionEvaluationV1>,
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<ValueId, String> {
    let owner = source.source().owner(); let site = source.source().root().statement().clone();
    let text = match source.root_class() {
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String) => true,
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64) => false,
        _ => return Err(freeze("expression/residence-root")),
    };
    let after_residency = if text {
        source.return_text_after_read_v2().map_err(freeze)?.map(|(_, read)|
            state.observe_binding_value_v2(read.binding()).map_err(|error| error.to_string())).transpose()?
    } else { None };
    let (displaced, unwind) = match &after_residency {
        Some(read) => {
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
            let (old, unwind) = ledger.prepare_return_text_after_unwind_v2(&source, read, function)?;
            (Some(old), Some(unwind))
        }
        None => (None, None),
    };
    let mut emit_call = |builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
        call: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &[(usize, ValueId)], values: &[Option<crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1>],
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>|
        terminal_call::emit_expression_call(builder, state, ledger, &source, call,
            acquired, values, bindings, consume_receiver, static_source);
    let mut expression = match &unwind {
        Some(unwind) => expression_value::emit_with_prior_home_unwind(builder, state, ledger, &source, unwind, &mut emit_call)?,
        None => expression_value::emit(builder, state, ledger, &source, &mut emit_call)?,
    };
    let (terminal, continuation) = if text {
        expression_value::materialize_text_root(builder, &mut expression, source.owned_text_root_node_v1().is_none(), &source)?;
        expression_value::finish_residence_with_displaced_continuation(builder, &mut expression, displaced)?
    } else { expression_value::finish_scalar_continuation(builder, &mut expression)? };
    let value = match expression.value.as_operand()? {
        crate::mir::instruction::StringInvokeOperand::OwnedString(value) if text => *value,
        crate::mir::instruction::StringInvokeOperand::NormalInteger(value) if !text => *value,
        _ => return Err(freeze("expression/residence-root")),
    };
    let body = expression.body;
    ledger.retain_return_expression_v1(source, value, body.values, body.calls,
        body.path_reads.into_values().collect(), body.reference_reads, body.entry, terminal, continuation,
        expression.purpose_acquisition, after_residency, body.bindings,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    emit_root_home_exit(builder, state, ledger, owner, &site, value)
}
