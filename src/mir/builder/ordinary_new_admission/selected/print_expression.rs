//! Execute the retained whole-expression recipe through the existing writers.
//! No AST interpretation, target selection or value-class inference lives here.
use super::*;
use crate::mir::instruction::StringInvokeOperation as Operation;
use super::expression_value::pending_operations;
use crate::mir::normal_callable_semantic_package::{
    EmittedPrintExpressionProjectionV1, LexicalInstanceCallDispositionRowV1,
};
use crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1;
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    frame: Rc<CheckedPrintExpressionFrameV1>,
    consume_receiver: &dyn Fn(&LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
) -> Result<(), String> {
    ledger.validate_print_expression_source_v1(&frame)?;
    let owner = frame.source().owner();
    if state.owner() != owner { return Err(freeze("print-expression/state-owner")); }
    let expression_value::EmittedExpressionValueV1 {
        value, body: expression_value::EmittedExpressionBodyV1 {
            values, calls, path_reads, reference_reads, acquired, homes, fault_frame, entry, mut bindings,
        }, purpose_acquisition: _,
    } = expression_value::emit(builder, state, ledger, frame.evaluation(),
        &mut |builder, state, call, acquired, values, bindings| terminal_call::emit_expression_call(
            builder, state, ledger, frame.evaluation(), call, acquired, values, bindings, consume_receiver,
        ),
    )?;
    let mut writer = RawRootGraphWriterV1;
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (clean, MirInstruction::Jump { target: next, edge_args: None });
    writer.append_terminated(builder, clean, continuation.1.clone(), &mut bindings)?;
    let normal: Vec<_> = acquired.iter().rev().map(|(_, value)|
        InvokeOperation::String(Operation::End { value: *value })).collect();
    let fault = pending_operations(&acquired, &homes);
    let after: Vec<_> = (0..normal.len()).map(|index| ((index + 1)..fault.len()).collect()).collect();
    let graph = root_graph::emit_cleanup_suffixes_with_writer(&mut writer, builder, fault_frame, clean,
        &normal, &fault, &after, &(0..fault.len()).collect::<Vec<_>>(), &mut bindings)?;
    let print = writer.emit_current(builder, MirInstruction::Invoke {
        operation: InvokeOperation::String(Operation::Print { value }), fault_frame,
        normal_landing: graph.clean, fault_landing: graph.acquisition_fault,
    })?;
    bindings.push(print.clone());
    builder.start_new_block(next)?;
    // Shared FaultFrame/read observations may be lent by several call nodes;
    // the physical instruction itself is retained once in this enclosing pool.
    let mut unique = Vec::with_capacity(bindings.len());
    for binding in bindings { if !unique.contains(&binding) { unique.push(binding); } }
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
    let packet = Rc::new(EmittedPrintExpressionProjectionV1::new(frame, values, calls, path_reads.into_values().collect(), reference_reads, entry, print,
        continuation, unique.clone(), ledger, function)?);
    ledger.prepare_print_expression_bindings(owner, packet, unique)?.commit()
}
