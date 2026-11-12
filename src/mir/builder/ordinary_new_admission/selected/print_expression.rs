//! Execute the retained whole-expression recipe through the existing writers.
//! No AST interpretation, target selection or value-class inference lives here.
use super::*;
use crate::mir::instruction::StringInvokeOperation as Operation;
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
    let (print, continuation) = if let Some(fault_frame) = expression.body.fault_frame {
        let body = &mut expression.body;
        let mut writer = RawRootGraphWriterV1;
        let next = writer.reserve_block(builder)?;
        let clean = writer.reserve_block(builder)?;
        let continuation = (
            clean,
            MirInstruction::Jump {
                target: next,
                edge_args: None,
            },
        );
        writer.append_terminated(builder, clean, continuation.1.clone(), &mut body.bindings)?;
        let plan = FaultCleanupPlanV1::temporaries_then_homes(
            &body.acquired,
            &body.dynamic_carriers,
            &body.temporary_order,
            &body.homes,
            !frame.is_discard(),
        )?;
        let graph = root_graph::emit_cleanup_suffixes_with_writer(
            &mut writer,
            builder,
            fault_frame,
            clean,
            &plan,
            &mut body.bindings,
        )?;
        let print = if frame.is_discard() {
            writer.emit_current(
                builder,
                MirInstruction::Jump {
                    target: graph.clean,
                    edge_args: None,
                },
            )?
        } else {
            writer.emit_current(
                builder,
                MirInstruction::Invoke {
                    operation: InvokeOperation::String(Operation::Print {
                        value: expression.value.as_operand()?.clone(),
                    }),
                    fault_frame,
                    normal_landing: graph.clean,
                    fault_landing: graph.acquisition_fault,
                },
            )?
        };
        body.bindings.push(print.clone());
        builder.start_new_block(next)?;
        (Some(print), Some(continuation))
    } else {
        if !frame.is_discard() {
            return Err(freeze("expression/fault-frame-required"));
        }
        expression_value::finish_value_continuation_v1(builder, &mut expression.body)?
    };
    let mut unique = Vec::new();
    for row in &expression.body.bindings {
        if !unique.contains(row) {
            unique.push(row.clone());
        }
    }
    expression.body.bindings = unique.clone();
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
        .prepare_print_expression_bindings(owner, packet, unique)?
        .commit()
}
