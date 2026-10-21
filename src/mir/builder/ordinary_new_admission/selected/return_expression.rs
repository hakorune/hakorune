//! Original ReturnValue evaluates once and retains its result across cleanup.
use super::*;
use super::expression_value;
use std::rc::Rc;
use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1;

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState, ledger: &OrdinaryNewClaimLedgerV1,
    source: Rc<CheckedExpressionEvaluationV1>,
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<ValueId, String> {
    let owner = source.source().owner(); let site = source.source().root().statement().clone();
    let mut expression = expression_value::emit(builder, state, ledger, &source,
        &mut |builder, state, call, acquired, values, bindings| terminal_call::emit_expression_call(
            builder, state, ledger, &source, call, acquired, values, bindings, consume_receiver, None))?;
    let text = match source.root_class() {
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String) => true,
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64) => false,
        _ => return Err(freeze("expression/residence-root")),
    };
    let (terminal, continuation) = if text {
        expression_value::materialize_text_root(builder, &mut expression, source.owned_root_node_v1().is_none())?;
        expression_value::finish_residence_continuation(builder, &mut expression)?
    } else { expression_value::finish_scalar_continuation(builder, &mut expression)? };
    let value = match expression.value.as_operand()? {
        crate::mir::instruction::StringInvokeOperand::OwnedString(value) if text => *value,
        crate::mir::instruction::StringInvokeOperand::NormalInteger(value) if !text => *value,
        _ => return Err(freeze("expression/residence-root")),
    };
    let body = expression.body;
    ledger.retain_return_expression_v1(source, value, body.values, body.calls,
        body.path_reads.into_values().collect(), body.reference_reads, body.entry, terminal, continuation,
        expression.purpose_acquisition, body.bindings,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    emit_root_home_exit(builder, state, ledger, owner, &site, value)
}
