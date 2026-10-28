//! Original field assignment through the sole checked writer, with exact operands.
use super::*;
use crate::mir::normal_callable_semantic_package::{FieldWriteSourceLoanV1, EmittedBorrowedFieldWriteV1,
    FieldWriteValueV1};
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: FieldWriteSourceLoanV1,
    field_name: &str, reads: [FieldWriteValueV1; 2],
) -> Result<(), String> {
    let mode = source.require_current_execution(crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))?;
    let owner = source.frame().source().owner();
    if state.owner() != owner { return Err(freeze("borrowed-field-write/source-site")); }
    let [FieldWriteValueV1::Lexical(receiver), value] = reads else { return Err(freeze("field-write/receiver-proof")) };
    let (binding, site) = source.receiver()?;
    let base = receiver.value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string())?;
    if source.field_name()? != field_name { return Err(freeze("borrowed-field-write/source-site")); }
    let field = source.field()?;
    let (value_id, prefix, relations) = match (&value, source.borrowed()) {
        (FieldWriteValueV1::Lexical(read), Some(formal)) => {
            let ty = crate::mir::numeric_substrate::exact_numeric_mir_type_from_declared_name(
                formal.field_source().declared_type_name(), crate::mir::numeric_substrate::NumericTarget::host())
                .ok_or_else(|| freeze("borrowed-field-write/numeric-declaration"))?;
            if crate::mir::numeric_substrate::exact_numeric_type_requires_dynamic_integer_range_check(&ty)
                && formal.field_source().declared_type_name() != Some("usize")
            { return Err(freeze("borrowed-field-write/exact-range-check-unavailable")); }
            let (binding, _, site) = formal.value();
            (read.value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string())?,
                Vec::new(), Vec::new())
        },
        (FieldWriteValueV1::Expression(rhs), None) => {
            if source.qualified_rhs().is_none_or(|original| !Rc::ptr_eq(original, rhs.recipe())) {
                return Err(freeze("field-write-rhs/source-drift"));
            }
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
            rhs.validate_lowering(ledger, function)?;
            (rhs.value(), rhs.bindings()?, rhs.relations())
        },
        _ => return Err(freeze("field-write/operand-origin")),
    };
    let deferred = match &value {
        FieldWriteValueV1::Expression(rhs) => rhs.deferred_child_ends_v1(
            crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))?,
        _ => Vec::new(),
    };
    let operations = ledger.prior_home_unwind_for(source.frame().prior_homes())?;
    let graph = super::checked_field_write::emit(builder, state, field, mode, base, value_id, operations, deferred)?
        .with_operand_projection(prefix, relations);
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedBorrowedFieldWriteV1::new(source, receiver, value, graph, ledger, function)?);
    ledger.record_borrowed_field_write_bindings_v1(packet)
}

pub(in crate::mir::builder) fn emit_owned_rhs_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, source: &FieldWriteSourceLoanV1,
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: &terminal_call::StaticExpressionCallIssuerV1<'_>,
) -> Result<Rc<crate::mir::normal_callable_semantic_package::EmittedFieldWriteRhsV1>, String> {
    let recipe = source.qualified_rhs().ok_or_else(|| freeze("field-write-rhs/source-drift"))?;
    let evaluation = recipe.evaluation().ok_or_else(|| freeze("field-write-rhs/source-drift"))?;
    let mode = source.require_current_execution(crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))?;
    if !mode.consumes_text_candidate_on_fault_v1() { return Err(freeze("field-write/operand-origin")); }
    let mut expression = expression_value::emit_with_dynamic_calls(builder, state, ledger, evaluation,
        None, None, &mut |builder, state, call, acquired, values, bindings|
            terminal_call::emit_expression_call_with_dynamic_and_argument_frames(
                builder, state, ledger, evaluation, call, acquired, values, bindings,
                consume_receiver, Some(static_source), &[]))?;
    expression_value::materialize_text_root(builder, &mut expression,
        evaluation.owned_text_root_node_v1().is_none(), evaluation)?;
    let value = match expression.value.as_operand()? {
        crate::mir::instruction::StringInvokeOperand::OwnedString(value) => *value,
        _ => return Err(freeze("field-write/operand-origin")),
    };
    let (terminal, continuation) = expression_value::finish_deferred_argument_continuation(builder, &mut expression)?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    crate::mir::normal_callable_semantic_package::EmittedFieldWriteRhsV1::from_owned_expression_v1(
        Rc::clone(recipe), expression.body, value, expression.purpose_acquisition,
        terminal, continuation, ledger, function).map(Rc::new)
}
