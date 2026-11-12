//! Execute the retained whole-expression recipe through the existing writers.
//! No AST interpretation, target selection or value-class inference lives here.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    ExpressionEmissionV1, LexicalInstanceCallDispositionRowV1,
};
use crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1;
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedPrintExpressionFrameV1>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<(), String> {
    ledger.validate_print_expression_source_v1(&frame)?;
    let owner = frame.source().owner();
    if state.owner() != owner {
        return Err(freeze("print-expression/state-owner"));
    }
    let mut expression = expression_value::emit(
        builder,
        state,
        ledger,
        frame.evaluation(),
        &mut |builder, state, call, acquired, values, bindings| {
            terminal_call::emit_expression_call(
                builder,
                state,
                ledger,
                frame.evaluation(),
                call,
                acquired,
                values,
                bindings,
                consume_receiver,
                static_source,
            )
        },
    )?;
    let (print, continuation) = expression.finish_statement(builder, &frame)?;
    let bindings = expression.body.bindings.clone();
    let packet = Rc::new(ExpressionEmissionV1::new(
        frame,
        expression.body,
        print,
        continuation,
        ledger,
        builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?,
    )?);
    ledger
        .prepare_print_expression_bindings(owner, packet, bindings)?
        .commit()
}
