//! Source-issued Map flow consumption; no AST descent or key/ownership decisions.
use super::*;
use crate::mir::instruction::{MapInvokeOperation as Map, MapValueKind};
use crate::mir::resolved_semantics::home_new_prefix::{
    MapEntryStoreClassV1, MapValueSource, SourceScalarKind,
};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, ResolvedInitializerRelationV1};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1,
    relation: &ResolvedInitializerRelationV1,
) -> Result<ValueId, String> {
    ledger.begin_map_emission(site, relation)?;
    emit_flow(builder, state, ledger, site).map(|(result, _)| result)
}

/// Emit a `%{...}` literal in call-argument position. The sealed
/// `CallArgument` flow row is the sole membership evidence; the emitted
/// lease stays caller-owned — the callee borrows the storage pointer for
/// the call's duration and the caller's own exit chain Ends it. Returns
/// the lease value and the `NewMap` projection pair as argument evidence.
pub(in crate::mir::builder) fn emit_argument(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1,
) -> Result<(ValueId, (BasicBlockId, MirInstruction)), String> {
    ledger.begin_map_argument_emission(site)?;
    let (result, projection) = emit_flow(builder, state, ledger, site)?;
    Ok((result, projection.expect("Map New records its projection")))
}

/// Emit a `return %{...}` literal. The sealed ReturnBoundary flow row is
/// the sole membership evidence; there is no initializer relation and no
/// local to install into — the emitted lease is consumed by `Return`.
pub(in crate::mir::builder) fn emit_return(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1,
) -> Result<ValueId, String> {
    ledger.begin_map_return_emission(site)?;
    emit_flow(builder, state, ledger, site).map(|(result, _)| result)
}

fn emit_flow(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1,
) -> Result<(ValueId, Option<(BasicBlockId, MirInstruction)>), String> {
    if state.owner() != site.owner() {
        return Err(freeze("map-owner"));
    }
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
    let (result, new_projection) =
        invoke(builder, frame, Map::New, allocation_fault, &mut bindings)?;
    let result = result.expect("Map New produces an opaque result");
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Box("MapBox".to_string()));
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
        // Only pure scalar materialization, sealed inline payloads, or an
        // exact bound read is allowed here. Keep the prepared Key normal
        // block exclusive to its install. The store class is the sealed
        // row's own predicate — the undertaking verifies the same
        // classification before catalog mutation.
        let pending = match entry.store_class() {
            MapEntryStoreClassV1::Scalar => {
                let Some(source) = entry.value_source() else {
                    return Err(freeze("map-value-consumer-missing"));
                };
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
                    _ => return Err(freeze("map-value-consumer-missing")),
                };
                PendingInstall::Value(value, kind)
            }
            MapEntryStoreClassV1::Transferred => {
                PendingInstall::Indexed(state.read_variable(entry.site().node())?)
            }
            MapEntryStoreClassV1::Borrowed => {
                // A self-rooted handle borrow stores the handle's i64
                // bits under the BorrowedHandle tag — the payload end is
                // a no-op (the map never owns the handle) and a checked
                // i64 read on the tagged entry Faults instead of
                // conflating the bits with a scalar. Live map-local and
                // kind-less local borrows have no physical reference lane
                // and stay frozen here; verify already refuses them
                // before lowering.
                let Some(MapValueSource::BorrowedHandle(binding)) = entry.value_source() else {
                    return Err(freeze("map-value-consumer-missing"));
                };
                let value = state.read_variable(entry.site().node())?;
                state
                    .value_for_exact_binding(site.owner(), *binding)
                    .map_err(|_| freeze("map-borrow-binding"))?;
                PendingInstall::Value(value, MapValueKind::BorrowedHandle)
            }
            MapEntryStoreClassV1::Text => {
                // The sealed String row carries the owned payload; the
                // text rides the install inline like a prepared key.
                let Some(MapValueSource::String(text)) = entry.value_source() else {
                    return Err(freeze("map-value-consumer-missing"));
                };
                PendingInstall::Text(text.to_string())
            }
            MapEntryStoreClassV1::EmptyArray => {
                // A `[]` literal carries no payload at all: the sealed
                // empty element list is the whole meaning, so the
                // install needs no value operand.
                PendingInstall::EmptyArray
            }
            MapEntryStoreClassV1::Opaque => {
                return Err(freeze("map-value-consumer-missing"));
            }
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
        .0
        .expect("key prepare produces an opaque result");
        let operation = match pending {
            PendingInstall::Value(value, kind) => Map::InstallValue {
                map: result,
                key,
                value,
                kind,
            },
            PendingInstall::Indexed(value) => Map::InstallIndexed {
                map: result,
                key,
                value,
                object: ledger.map_candidate_object(entry, value)?,
            },
            PendingInstall::Text(utf8) => Map::InstallText {
                map: result,
                key,
                utf8,
            },
            PendingInstall::EmptyArray => Map::InstallEmptyArray { map: result, key },
        };
        let outcome = invoke(builder, frame, operation, precommit, &mut bindings)?
            .0
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
    Ok((result, new_projection))
}

/// One pending entry install: a materialized scalar/indexed value, an
/// inline sealed payload, or a marker install that needs no operand.
enum PendingInstall {
    Value(ValueId, MapValueKind),
    Indexed(ValueId),
    Text(String),
    EmptyArray,
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
) -> Result<(Option<ValueId>, Option<(BasicBlockId, MirInstruction)>), String> {
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
    let projection = value
        .map(|dst| {
            let projection = MirInstruction::InvokeNormalResult {
                invoke_block: origin,
                dst,
            };
            builder.emit_instruction(projection.clone())?;
            bindings.push((normal, projection.clone()));
            Ok::<(BasicBlockId, MirInstruction), String>((normal, projection))
        })
        .transpose()?;
    Ok((value, projection))
}
