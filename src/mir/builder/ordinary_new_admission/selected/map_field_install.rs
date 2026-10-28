//! Exact source Map field set; sole checked Invoke and cleanup writers.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    CallValidationContextV1, MapFieldInstallEmissionV1, PreparedMapFieldInstallV1,
};
use crate::mir::resolved_semantics::home_new_prefix::MapFieldInstallV1;
use std::rc::Rc;

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    source: Rc<MapFieldInstallV1>,
) -> Result<(), String> {
    ledger.validate_map_field_install_source_v1(&source)?;
    // Refuse missing End plans before changing the physical graph.
    let prior = ledger.prior_home_unwind_for(source.prior_homes())?;
    let normal_homes = ledger.prior_home_unwind_for(source.normal_homes())?;
    let receiver = state
        .take_exact_lexical_read(
            source.site().owner(),
            source.receiver_read().node(),
            source.source().receiver_v1().receiver(),
        )
        .map_err(|error| error.to_string())?;
    let key = expression_value::emit(
        builder,
        state,
        ledger,
        source.key(),
        &mut |_, _, _, _, _, _| Err(freeze("map-get-shape")),
    )?;
    if key.purpose_acquisition.is_some()
        || !key.body.acquired.is_empty()
        || !key.body.calls.is_empty()
        || !key.body.path_reads.is_empty()
    {
        return Err(freeze("map-get-shape"));
    }
    let mut body = key.body;
    let frame = body.fault_frame;
    let (terminal, continuation) =
        expression_value::finish_value_continuation_v1(builder, &mut body)?;
    let candidate = match (source.source().installed_local_v1(source.site()), source.binding()) {
        (Some((_, _, set)), Some(binding)) => Some(
            state
                .take_exact_lexical_read(set.owner(), set.arguments()[1].site().node(), binding)
                .map_err(|error| error.to_string())?,
        ),
        (None, None) => None,
        _ => return Err(freeze("map-get-shape")),
    };
    let prepared = PreparedMapFieldInstallV1::issue(
        Rc::clone(&source),
        receiver,
        candidate,
        body.values,
        body.reference_reads,
        body.entry,
        terminal,
        continuation,
        body.bindings,
        ledger,
        builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?,
    )?;
    let operation = prepared.operation(
        CallValidationContextV1::Lowering(ledger),
        builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?,
    )?;
    let outward = builder.next_block_id();
    let mut outward_rows = Vec::new();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut outward_rows,
    )?;
    let mut prior_faults = outward_rows.clone();
    let fault = cleanup_chain(builder, frame, prior, outward, &mut prior_faults)?;
    let mut normal_faults = outward_rows;
    let end_fault = cleanup_chain(builder, frame, normal_homes, outward, &mut normal_faults)?;
    let mut rows = Vec::new();
    let (outcome, normal) = invoke_operation::emit(builder, frame, operation, fault, &mut rows)?;
    let outcome = outcome.ok_or_else(|| freeze("map-get-shape"))?;
    let normal = normal.ok_or_else(|| freeze("map-get-shape"))?;
    if rows.len() != 2 {
        return Err(freeze("map-get-shape"));
    }
    let invoke = rows.remove(0);
    rows.clear();
    let (value, projection) = invoke_operation::emit(
        builder,
        frame,
        InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::EndOutcome { outcome }),
        end_fault,
        &mut rows,
    )?;
    if value.is_some() || projection.is_some() || rows.len() != 1 {
        return Err(freeze("map-get-shape"));
    }
    let end = rows.remove(0);
    let current = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let next = builder.next_block_id();
    let jump = MirInstruction::Jump {
        target: next,
        edge_args: None,
    };
    builder.emit_instruction(jump.clone())?;
    builder.start_new_block(next)?;
    let packet = MapFieldInstallEmissionV1::issue(
        prepared,
        invoke,
        normal,
        end,
        (current, jump),
        prior_faults,
        normal_faults,
        ledger,
        builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?,
    )?;
    ledger.record_map_field_install_bindings_v1(Rc::new(packet))
}
