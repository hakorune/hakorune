//! Source-issued Map flow consumption; no AST descent or key/ownership decisions.
use super::*;
use crate::mir::instruction::MapInvokeOperation as Map;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, ResolvedInitializerRelationV1};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1,
    relation: &ResolvedInitializerRelationV1,
) -> Result<ValueId, String> {
    if state.owner() != site.owner() {
        return Err(freeze("map-owner"));
    }
    ledger.begin_map_emission(site, relation)?;
    let flow = ledger.map_flow(site)?;
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let allocation_fault = cleanup_chain(
        builder,
        frame,
        ledger.map_outer_operations(site, 0)?,
        outward,
        &mut bindings,
    )?;
    let result = invoke(builder, frame, Map::New, allocation_fault, &mut bindings)?
        .expect("Map New produces an opaque result");
    for (index, entry) in flow.entries().iter().enumerate() {
        let precommit = map_fault(
            builder,
            ledger,
            site,
            index,
            result,
            frame,
            outward,
            &mut bindings,
        )?;
        let key = invoke(
            builder,
            frame,
            Map::PrepareKey {
                utf8: entry.key().into(),
            },
            precommit,
            &mut bindings,
        )?
        .expect("key prepare produces an opaque result");
        // The bounded cohort reads an already-acquired local: no intervening MIR
        // child evaluation, no new Home and no key cancellation edge required.
        let value = state.read_variable(entry.site().node())?;
        let object = ledger.map_candidate_object(entry, value)?;
        let outcome = invoke(
            builder,
            frame,
            Map::InstallIndexed {
                map: result,
                key,
                object,
                value,
            },
            precommit,
            &mut bindings,
        )?
        .expect("install produces its detached outcome");
        let committed = map_fault(
            builder,
            ledger,
            site,
            index + 1,
            result,
            frame,
            outward,
            &mut bindings,
        )?;
        invoke(
            builder,
            frame,
            Map::EndOutcome { outcome },
            committed,
            &mut bindings,
        )?;
    }
    ledger.record_map_emission(site, result, bindings)?;
    Ok(result)
}

fn map_fault(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1,
    installed: usize,
    map: ValueId,
    frame: ValueId,
    tail: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let mut operations = vec![InvokeOperation::Map(Map::End { map })];
    operations.extend(ledger.map_outer_operations(site, installed)?);
    cleanup_chain(builder, frame, operations, tail, bindings)
}

fn invoke(
    builder: &mut MirBuilder,
    frame: ValueId,
    operation: Map,
    fault: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<Option<ValueId>, String> {
    let value = operation
        .normal_result_kind()
        .map(|_| builder.next_value_id());
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal = builder.next_block_id();
    let instruction = MirInstruction::Invoke {
        operation: InvokeOperation::Map(operation),
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: fault,
    };
    builder.emit_instruction(instruction.clone())?;
    bindings.push((origin, instruction));
    builder.start_new_block(normal)?;
    if let Some(dst) = value {
        let projection = MirInstruction::InvokeNormalResult {
            invoke_block: origin,
            dst,
        };
        builder.emit_instruction(projection.clone())?;
        bindings.push((normal, projection));
    }
    Ok(value)
}
