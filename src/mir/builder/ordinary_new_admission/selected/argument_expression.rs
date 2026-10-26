//! One checked Argument purpose uses the shared expression writer/continuation.
use super::*;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::{LexicalCallArgumentProjectionV1 as Projection, ExpressionCallTransientPrefixV1};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceExprSiteV1};

pub(in crate::mir::builder) fn retained_argument_frame_v1(
    frames: &[std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>],
    call: &OwnedExprSiteV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>>, String> {
    let mut matches = frames.iter().filter(|frame| {
        frame.source().owner() == call.owner()
            && std::rc::Rc::ptr_eq(frame.source_loan().plan(), frame.source())
            && frame.evaluation().reference_coverage_is_exact()
            && matches!(
                frame.source().root(),
                crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                    call: original,
                    ordinal: original_ordinal,
                    ..
                } if original.owner() == call.owner()
                    && original.site() == call.site()
                    && *original_ordinal == ordinal
                    && original.arguments().get(ordinal as usize).is_some_and(|argument|
                        argument.ordinal() == ordinal && argument.site() == site)
                    && frame.source().nodes().first().is_some_and(|root| root.site() == site)
            )
    });
    let Some(frame) = matches.next() else {
        return Ok(None);
    };
    if matches.next().is_some() {
        return Err(freeze("argument-expression/retained-frame-ambiguous"));
    }
    Ok(Some(std::rc::Rc::clone(frame)))
}

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
    // The direct caller completes scalar arguments before its Call. Owned Text
    // must enter through the lifecycle caller that retains the ordered packet.
    ledger.argument_expression_frame_v1(call, ordinal, site)?;
    emit_with_prefix(builder, state, ledger, call, ordinal, site, consume_receiver, None, None)
}

pub(in crate::mir::builder) fn emit_with_prefix(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    call: &OwnedExprSiteV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
    consume_receiver: &dyn Fn(
        &crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String>,
    inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<(ValueId, Projection), String> {
    let frame = ledger.qualified_argument_expression_frame_v1(call, ordinal, site)?;
    emit_with_checked_frame(
        builder,
        state,
        ledger,
        call,
        ordinal,
        site,
        frame,
        &[],
        consume_receiver,
        inherited,
        static_source,
    )
}

pub(in crate::mir::builder) fn emit_with_checked_frame(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    call: &OwnedExprSiteV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
    frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
    nested_argument_frames: &[std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>],
    consume_receiver: &dyn Fn(
        &crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String>,
    inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<(ValueId, Projection), String> {
    let source_identity = frame.source().owner() == call.owner()
        && std::rc::Rc::ptr_eq(frame.source_loan().plan(), frame.source())
        && frame.evaluation().reference_coverage_is_exact()
        && matches!(
            frame.source().root(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                call: original,
                ordinal: original_ordinal,
                ..
            } if original.owner() == call.owner()
                && original.site() == call.site()
                && *original_ordinal == ordinal
                && original.arguments().get(ordinal as usize).is_some_and(|argument|
                    argument.ordinal() == ordinal && argument.site() == site)
                && frame.source().nodes().first().is_some_and(|root| root.site() == site)
        );
    if !source_identity {
        return Err(freeze("argument-expression/checked-frame-identity"));
    }
    if state.owner() != call.owner() {
        return Err(freeze("argument-expression/state-owner"));
    }
    let mut expression = expression_value::emit_with_dynamic_calls(
        builder,
        state,
        ledger,
        frame.evaluation(),
        inherited,
        None,
        &mut |builder, state, call, acquired, values, bindings| {
            terminal_call::emit_expression_call_with_dynamic_and_argument_frames(
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
                nested_argument_frames,
            )
        },
    )?;
    expression_value::materialize_text_root(builder, &mut expression, frame.materializes_root(), frame.evaluation())?;
    let (value, terminal, continuation) = if frame.is_dynamic_carrier_append_v1() {
        let Operand::DynamicCarrier(value) = expression.value.as_operand()?.clone() else {
            return Err(freeze("argument-expression/dynamic-carrier-result"));
        };
        let (terminal, continuation) =
            expression_value::finish_dynamic_initializer_continuation(builder, &mut expression)?;
        (value, terminal, continuation)
    } else { match expression.value.as_operand()?.clone() {
        Operand::NormalInteger(value) if frame.is_i64() => {
            let (terminal, continuation) = expression_value::finish_scalar_continuation(builder, &mut expression)?;
            (value, terminal, continuation)
        }
        Operand::OwnedString(value) if !frame.is_i64() => {
            let (terminal, continuation) = expression_value::finish_deferred_argument_continuation(builder, &mut expression)?;
            (value, terminal, continuation)
        }
        _ => return Err(freeze("argument-expression/result")),
    }};
    let projection = Projection::from_expression_v1(
        frame,
        expression.body.values,
        expression.body.calls,
        expression.body.dynamic_calls,
        expression.body.path_reads.into_values().collect(),
        expression.body.reference_reads,
        expression.body.entry,
        terminal,
        continuation,
        expression.purpose_acquisition,
        expression.body.bindings,
        nested_argument_frames,
        ledger,
        builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?,
        inherited,
    )?;
    Ok((value, projection))
}
