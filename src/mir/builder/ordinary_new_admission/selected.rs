//! Physical consumption of one prepared exact New claim, never a target issuer.
use crate::mir::instruction::InvokeCallResultKind;
#[path = "selected/arguments.rs"]
mod arguments;
#[path = "selected/map.rs"]
pub(in crate::mir::builder) mod map;
#[path = "selected/terminal_call.rs"]
pub(in crate::mir::builder) mod terminal_call;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewTrivialArgumentV1, PreparedTerminalI64AddReturnV1, PreparedTerminalI64FieldReturnV1,
    PreparedTerminalMapGetReturnV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlock, BasicBlockId, Callee, MirBuilder, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    claim: OrdinaryNewAdmissionClaimV1,
) -> Result<ValueId, String> {
    if state.owner() != claim.site().owner() {
        return Err(freeze("owner-or-argument-count"));
    }
    let site = claim.site().clone();
    let object = claim.object();
    let class = claim.class().to_owned();
    let arity = claim.arity();
    let rows = claim
        .argument_rows()
        .map_err(|_| freeze("argument-source-unavailable"))?
        .to_vec();
    let constructor = claim.constructor();
    emit_selected_new(
        builder,
        state,
        ledger,
        site,
        object,
        class,
        arity,
        constructor,
        &rows,
    )
}

/// Physical consumption of one return-position result claim. The emitted
/// object value is the `Return { value }` operand the claim stands for —
/// one emit shape, no local destination step.
pub(in crate::mir::builder) fn emit_result(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
) -> Result<ValueId, String> {
    if state.owner() != claim.site().owner() {
        return Err(freeze("owner-or-argument-count"));
    }
    let site = claim.site().clone();
    let object = claim.object();
    let class = claim.class().to_owned();
    let arity = claim.arity();
    let rows = claim
        .argument_rows()
        .map_err(|_| freeze("argument-source-unavailable"))?
        .to_vec();
    let constructor = claim.constructor();
    emit_selected_new(
        builder,
        state,
        ledger,
        site,
        object,
        class,
        arity,
        constructor,
        &rows,
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_selected_new(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    class: String,
    arity: usize,
    constructor: OrdinaryNewConstructorDispositionV1,
    argument_rows: &[OrdinaryNewTrivialArgumentV1],
) -> Result<ValueId, String> {
    let arguments =
        arguments::materialize_arguments(builder, state, ledger, &site, arity, argument_rows)?;
    let (prior, reclaim_origin) = ledger.begin_new_emission(&site)?;
    let frame = state.borrow_fault_frame(builder)?;
    let result = builder.next_value_id();
    let frame_binding = fault_frame_binding(builder, state, frame)?;
    let mut bindings = vec![frame_binding];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let allocation_fault = cleanup_chain(builder, frame, prior, outward, &mut bindings)?;
    let mut reclaim = None;
    let birth_fault = if matches!(constructor, OrdinaryNewConstructorDispositionV1::Birth(_)) {
        let origin = reclaim_origin.ok_or_else(|| freeze("reclaim-origin-missing"))?;
        // Birth discharged its source-sealed partial fields before Fault.
        // This caller reclaims only unpublished storage, then prior Homes.
        let tail = cleanup_step(
            builder,
            frame,
            InvokeOperation::ReclaimUnpublished {
                object: origin.object(),
                value: result,
            },
            allocation_fault,
            allocation_fault,
            &mut bindings,
        )?;
        let (block, instruction) = bindings
            .last()
            .cloned()
            .ok_or_else(|| freeze("reclaim-origin-binding-missing"))?;
        reclaim = Some((origin.clone(), block, instruction));
        tail
    } else {
        if reclaim_origin.is_some() {
            return Err(freeze("reclaim-origin-unexpected"));
        }
        allocation_fault
    };
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal = builder.next_block_id();
    let allocation = MirInstruction::Invoke {
        operation: InvokeOperation::NewBox { object },
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: allocation_fault,
    };
    builder.emit_instruction(allocation.clone())?;
    bindings.push((origin, allocation));
    builder.start_new_block(normal)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    bindings.push((normal, projection));
    // Existing type metadata is a one-way source projection, not object identity.
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Box(class.clone()));
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(result, class);
    if let OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
        let effects = recipe.physical_effect_mask();
        let MirInstruction::Call(call) = MirInstruction::call(
            None,
            Callee::BirthConstructor {
                key: recipe.target(),
                receiver: result,
            },
            arguments.clone(),
            effects,
        ) else {
            unreachable!("canonical Call constructor")
        };
        let after_birth = builder.next_block_id();
        let birth = MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call,
                result: InvokeCallResultKind::Unit,
            },
            fault_frame: frame,
            normal_landing: after_birth,
            fault_landing: birth_fault,
        };
        builder.emit_instruction(birth.clone())?;
        bindings.push((normal, birth));
        builder.start_new_block(after_birth)?;
    }
    ledger.record_new_emission(&site, result, arguments, reclaim, bindings)?;
    Ok(result)
}

pub(in crate::mir::builder) fn emit_root_home_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    value: ValueId,
) -> Result<ValueId, String> {
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        site,
        Some(value),
        value,
        None,
    )
}

pub(in crate::mir::builder) fn emit_root_home_unit_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
) -> Result<ValueId, String> {
    let statement_result = crate::mir::builder::emission::constant::emit_void(builder)?;
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        site,
        None,
        statement_result,
        None,
    )
}

/// One physically-emitting terminal ingress: the existing terminal Call or
/// the readable-Map checked get. Both share the same Normal/Fault cleanup
/// graph; the ledger entry kind keeps them apart.
pub(super) enum RootExitIngress {
    Call(terminal_call::Emission),
    MapGet { map: ValueId, utf8: String },
}

fn emit_root_home_exit_payload(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    return_value: Option<ValueId>,
    statement_result: ValueId,
    ingress: Option<RootExitIngress>,
) -> Result<ValueId, String> {
    let operations = ledger.begin_root_home_exit(owner, site)?;
    let mut bindings = Vec::new();
    let mut clean = builder.next_block_id();
    append_block(
        builder,
        clean,
        MirInstruction::Return {
            value: return_value,
        },
        &mut bindings,
    )?;
    let count = operations.len();
    let mut origins = Vec::with_capacity(count);
    // Empty-Home Plain has no Fault edge. Do not issue a disconnected terminal
    // that finishing would remove while its recorded binding stayed live —
    // and do not materialize a frame definition no Invoke would consume.
    if !operations.is_empty() || ingress.is_some() {
        let frame = state.borrow_fault_frame(builder)?;
        let mut fault = builder.next_block_id();
        append_block(
            builder,
            fault,
            MirInstruction::ReturnFault { fault_frame: frame },
            &mut bindings,
        )?;
        for (index, origin) in operations.into_iter().rev().enumerate() {
            let operation = origin.operation().clone();
            // A clean ingress's Fault skips its own retry and joins the
            // remaining fault-pending suffix. Later Normal outcomes cannot
            // clear that Fault.
            let next_clean = cleanup_step(
                builder,
                frame,
                operation.clone(),
                clean,
                fault,
                &mut bindings,
            )?;
            let (block, instruction) = bindings
                .last()
                .cloned()
                .ok_or_else(|| freeze("root-home-release-binding-missing"))?;
            origins.push((origin, block, instruction));
            if ingress.is_some() || index + 1 < count {
                fault = cleanup_step(builder, frame, operation, fault, fault, &mut bindings)?;
            }
            clean = next_clean;
        }
        origins.reverse();
        match ingress {
            Some(RootExitIngress::Call(call)) => {
                let (invoke, projection) = terminal_call::emit_ingress(
                    builder,
                    frame,
                    statement_result,
                    clean,
                    fault,
                    InvokeOperation::Call {
                        call: call.call,
                        result: call.result,
                    },
                    &mut bindings,
                )?;
                let frame_binding = fault_frame_binding(builder, state, frame)?;
                ledger.record_root_call_exit(
                    owner,
                    site,
                    call.source.finish(invoke.clone(), projection.clone()),
                    call.arguments,
                    invoke,
                    projection,
                    frame_binding,
                    origins,
                    bindings,
                )?;
                return Ok(statement_result);
            }
            Some(RootExitIngress::MapGet { map, utf8 }) => {
                let (invoke, projection) = terminal_call::emit_ingress(
                    builder,
                    frame,
                    statement_result,
                    clean,
                    fault,
                    InvokeOperation::Map(
                        crate::mir::instruction::MapInvokeOperation::CheckedGetI64 { map, utf8 },
                    ),
                    &mut bindings,
                )?;
                let frame_binding = fault_frame_binding(builder, state, frame)?;
                ledger.record_root_map_get_exit(
                    owner,
                    site,
                    invoke,
                    projection,
                    frame_binding,
                    origins,
                    bindings,
                )?;
                return Ok(statement_result);
            }
            None => {}
        }
    }
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let jump = MirInstruction::Jump {
        target: clean,
        edge_args: None,
    };
    builder.emit_instruction(jump.clone())?;
    bindings.push((origin, jump));
    ledger.record_root_home_exit(owner, site, origins, bindings)?;
    Ok(statement_result)
}

pub(in crate::mir::builder) fn emit_terminal_i64_add_return(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    prepared: PreparedTerminalI64AddReturnV1,
) -> Result<ValueId, String> {
    let return_site = prepared.return_site.clone();
    let mut values = [ValueId(0); 2];
    for (index, (site, base, field)) in prepared.reads.into_iter().enumerate() {
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| freeze("no-block"))?;
        let dst = builder.next_value_id();
        builder.emit_instruction(MirInstruction::ObjectFieldGet {
            dst,
            base,
            field: field.clone(),
        })?;
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Integer);
        ledger.record_terminal_field_read(&site, block, dst, base, field)?;
        values[index] = dst;
    }
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let result = builder.next_value_id();
    builder.emit_instruction(MirInstruction::BinOp {
        dst: result,
        op: crate::mir::BinaryOp::Add,
        lhs: values[0],
        rhs: values[1],
    })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    ledger.record_terminal_i64_add(&return_site, block, result, values[0], values[1])?;
    ledger.complete_terminal_i64_add_return(&return_site, result)?;
    Ok(result)
}

pub(in crate::mir::builder) fn emit_terminal_i64_field_return(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    prepared: PreparedTerminalI64FieldReturnV1,
) -> Result<ValueId, String> {
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let result = builder.next_value_id();
    builder.emit_instruction(MirInstruction::ObjectFieldGet {
        dst: result,
        base: prepared.base,
        field: prepared.field.clone(),
    })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    ledger.record_terminal_field_read(
        &prepared.site,
        block,
        result,
        prepared.base,
        prepared.field,
    )?;
    ledger.record_terminal_i64_field_return(
        prepared.site.owner(),
        &prepared.return_site,
        result,
    )?;
    Ok(result)
}

/// Emit the bounded readable-Map terminal: one checked `get` read whose i64
/// projection becomes the returned value. The ingress shares the terminal
/// Call's Normal/Fault cleanup graph — an owned map local still owes its
/// End on both paths; a borrowed formal releases nothing here.
pub(in crate::mir::builder) fn emit_terminal_map_get_return(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    prepared: PreparedTerminalMapGetReturnV1,
) -> Result<ValueId, String> {
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        &prepared.return_site,
        Some(value),
        value,
        Some(RootExitIngress::MapGet {
            map: prepared.map,
            utf8: prepared.utf8,
        }),
    )
}

fn cleanup_chain(
    builder: &mut MirBuilder,
    frame: ValueId,
    operations: Vec<InvokeOperation>,
    tail: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let mut next = tail;
    // Build links backwards; execution preserves the source-issued order.
    for operation in operations.into_iter().rev() {
        next = cleanup_step(builder, frame, operation, next, next, bindings)?;
    }
    Ok(next)
}

fn cleanup_step(
    builder: &mut MirBuilder,
    frame: ValueId,
    operation: InvokeOperation,
    normal_next: BasicBlockId,
    fault_next: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let origin = builder.next_block_id();
    let normal = builder.next_block_id();
    let fault = builder.next_block_id();
    for (landing, target) in [(normal, normal_next), (fault, fault_next)] {
        append_block(
            builder,
            landing,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
            bindings,
        )?;
    }
    append_block(
        builder,
        origin,
        MirInstruction::Invoke {
            operation,
            fault_frame: frame,
            normal_landing: normal,
            fault_landing: fault,
        },
        bindings,
    )?;
    Ok(origin)
}

pub(in crate::mir::builder) fn append_block(
    builder: &mut MirBuilder,
    id: BasicBlockId,
    terminator: MirInstruction,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| freeze("no-function"))?;
    if function.blocks.contains_key(&id) {
        return Err(freeze("duplicate-block"));
    }
    let mut block = BasicBlock::new(id);
    block.set_terminator(terminator.clone());
    function.add_block(block);
    bindings.push((id, terminator));
    Ok(())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/emission/{reason}]")
}

pub(in crate::mir::builder) fn fault_frame_binding(
    builder: &MirBuilder,
    state: &CallableSemanticLoweringState,
    frame: ValueId,
) -> Result<(BasicBlockId, MirInstruction), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("no-function"))?;
    state.validate_fault_frame(function)?;
    let entry = function
        .blocks
        .get(&function.entry_block)
        .ok_or_else(|| freeze("no-entry"))?;
    let mut definitions = entry.all_instructions().filter(|instruction|
        matches!(instruction, MirInstruction::FaultFrameEnter { dst, .. } if *dst == frame));
    let definition = definitions
        .next()
        .ok_or_else(|| freeze("frame-definition-missing"))?
        .clone();
    if definitions.next().is_some() {
        return Err(freeze("frame-definition-duplicate"));
    }
    Ok((function.entry_block, definition))
}
