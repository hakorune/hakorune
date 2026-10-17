//! One checked Argument purpose uses the shared expression writer/continuation.
use super::*;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1 as Projection;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceExprSiteV1};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    call: &OwnedExprSiteV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
    consume_receiver: &dyn Fn(
        &crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String>,
) -> Result<(ValueId, Projection), String> {
    let frame = ledger.argument_expression_frame_v1(call, ordinal, site)?;
    if state.owner() != call.owner() {
        return Err(freeze("argument-expression/state-owner"));
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
            )
        },
    )?;
    let Operand::NormalInteger(value) = expression.value else {
        return Err(freeze("argument-expression/result"));
    };
    let (terminal, continuation) =
        expression_value::finish_scalar_continuation(builder, &mut expression)?;
    let projection = Projection::from_expression_v1(
        frame,
        expression.values,
        expression.calls,
        expression.path_reads.into_values().collect(),
        expression.reference_reads,
        expression.entry,
        terminal,
        continuation,
        expression.bindings,
        ledger,
        builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?,
    )?;
    Ok((value, projection))
}
