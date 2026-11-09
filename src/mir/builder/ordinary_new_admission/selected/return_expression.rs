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
    use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as Class;
    // R16: a scalar root is an i64 or a Bool; only the operand spelling differs.
    let (text, bool_root) = match source.root_class() {
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(Class::String) => (true, false),
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(Class::I64) => (false, false),
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(Class::Bool) => (false, true),
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
    let mut prepared_root = ledger.expression_root_cleanup_v1(&source)?.map(|(operations, faults)| {
        let graph = root_graph::prepare_root_cleanup_graph_v1(builder, state, &mut RawRootGraphWriterV1,
            operations, &faults, ledger.expression_requires_fault_frame_v1(&source))?;
        Ok::<_, String>((graph, faults))
    }).transpose()?;
    let mut emit_call = |builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
        call: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &[(usize, ValueId)], values: &[Option<crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1>],
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>|
        terminal_call::emit_expression_call(builder, state, ledger, &source, call,
            acquired, values, bindings, consume_receiver, static_source);
    let mut expression = if let Some((root, _)) = &prepared_root {
        let continuation = root.bindings.iter().find(|(block, instruction)| *block == root.clean && matches!(instruction, MirInstruction::Invoke { .. } | MirInstruction::Return { .. } | MirInstruction::Jump { .. }))
            .cloned().ok_or_else(|| freeze("root-fault-sequence-shape"))?;
        expression_value::emit_with_root_cleanup_v1(builder, state, ledger, &source,
            (continuation, root.acquisition_fault), &mut emit_call)?
    } else { match &unwind {
        Some(unwind) => expression_value::emit_with_prior_home_unwind(builder, state, ledger, &source, unwind, &mut emit_call)?,
        None => expression_value::emit(builder, state, ledger, &source, &mut emit_call)?,
    }};
    let (terminal, continuation) = if text {
        expression_value::materialize_text_root(builder, &mut expression, source.owned_text_root_node_v1().is_none(), &source)?;
        expression_value::finish_residence_with_displaced_continuation(builder, &mut expression, displaced)?
    } else { expression_value::finish_scalar_continuation(builder, &mut expression)? };
    let value = if bool_root {
        // A Bool root is evaluated as its own value class, never as an operand.
        match &expression.value {
            crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1::Bool(value) => *value,
            _ => return Err(freeze("expression/residence-root")),
        }
    } else {
        match expression.value.as_operand()? {
            crate::mir::instruction::StringInvokeOperand::OwnedString(value) if text => *value,
            crate::mir::instruction::StringInvokeOperand::NormalInteger(value) if !text => *value,
            _ => return Err(freeze("expression/residence-root")),
        }
    };
    if let Some((root, _)) = &mut prepared_root {
        RawRootGraphWriterV1.append_terminated(builder, root.return_block,
            MirInstruction::Return { value: Some(value) }, &mut root.bindings)?;
        root.bindings.rotate_right(1);
        let ingress = match &terminal {
            Some(terminal) => terminal.clone(),
            None => {
                root_graph::separate_plain_entry_v1(builder, state)?;
                RawRootGraphWriterV1.emit_current(builder, MirInstruction::Jump { target: root.clean, edge_args: None })?
            }
        };
        root.bindings.push(ingress);
    }
    let body = expression.body;
    ledger.retain_return_expression_v1(source, value, body.values, body.calls,
        body.path_reads, body.reference_reads, body.entry, body.inline_range, terminal, continuation,
        body.root_cleanup.and_then(|(_, fault)| fault),
        expression.purpose_acquisition, after_residency, body.bindings,
        builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
    emit_root_home_exit(builder, state, ledger, owner, &site, value, prepared_root)
}
