//! Source-issued Map flow consumption; no AST descent or key/ownership decisions.
use super::*;
use crate::mir::instruction::{MapInvokeOperation as Map, MapValueKind};
use crate::mir::resolved_semantics::home_new_prefix::{MapValueSource, SourceScalarKind};
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
        // Only pure scalar materialization or an exact bound read is allowed here.
        // Keep the prepared Key normal block exclusive to its install.
        let (value, scalar_kind) = match entry.value_source() {
            Some(source) => {
                let kind = match source.scalar_kind() {
                    Some(SourceScalarKind::Integer) => MapValueKind::I64,
                    Some(SourceScalarKind::Bool) => MapValueKind::Bool,
                    None => return Err(freeze("map-value-consumer-missing")),
                };
                let value = match source {
                    MapValueSource::Integer(n) => {
                        let dst =
                            crate::mir::builder::emission::constant::emit_integer(builder, *n)?;
                        record_literal(
                            builder,
                            dst,
                            crate::mir::ConstValue::Integer(*n),
                            &mut bindings,
                        )?;
                        dst
                    }
                    MapValueSource::Bool(b) => {
                        let dst = crate::mir::builder::emission::constant::emit_bool(builder, *b)?;
                        record_literal(
                            builder,
                            dst,
                            crate::mir::ConstValue::Bool(*b),
                            &mut bindings,
                        )?;
                        dst
                    }
                    MapValueSource::Local { binding, .. } => {
                        let value = state.read_variable(entry.site().node())?;
                        if state
                            .value_for_exact_binding(site.owner(), *binding)
                            .map_err(|_| freeze("map-scalar-binding"))?
                            != value
                        {
                            return Err(freeze("map-scalar-binding-drift"));
                        }
                        value
                    }
                };
                (value, Some(kind))
            }
            None => (state.read_variable(entry.site().node())?, None),
        };
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
        let operation = match scalar_kind {
            Some(kind) => Map::InstallValue {
                map: result,
                key,
                value,
                kind,
            },
            None => Map::InstallIndexed {
                map: result,
                key,
                value,
                object: ledger.map_candidate_object(entry, value)?,
            },
        };
        let outcome = invoke(builder, frame, operation, precommit, &mut bindings)?
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

fn record_literal(
    builder: &MirBuilder,
    dst: ValueId,
    value: crate::mir::ConstValue,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(), String> {
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    bindings.push((block, MirInstruction::Const { dst, value }));
    Ok(())
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
