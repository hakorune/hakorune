//! Physical consumption of one prepared exact New claim, never a target issuer.
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewConstructorDispositionV1,
    PreparedTerminalI64AddReturnV1,
};
use crate::mir::{BasicBlock, BasicBlockId, Callee, MirBuilder, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    claim: OrdinaryNewAdmissionClaimV1,
    arguments: Vec<ValueId>,
) -> Result<ValueId, String> {
    if state.owner() != claim.site().owner() || arguments.len() != claim.arity() {
        return Err(freeze("owner-or-argument-count"));
    }
    let prior = ledger.begin_new_emission(claim.site())?;
    let site = claim.site().clone();
    let object = claim.object();
    let class = claim.class().to_owned();
    let frame = state.borrow_fault_frame(builder)?;
    let result = builder.next_value_id();
    let frame_binding = {
        let function = builder.function_state.current_function.as_ref()
            .ok_or_else(|| freeze("no-function"))?;
        state.validate_fault_frame(function)?;
        let entry = function.blocks.get(&function.entry_block)
            .ok_or_else(|| freeze("no-entry"))?;
        let mut definitions = entry.all_instructions().filter(|instruction|
            matches!(instruction, MirInstruction::FaultFrameEnter { dst, .. } if *dst == frame));
        let definition = definitions.next().ok_or_else(|| freeze("frame-definition-missing"))?.clone();
        if definitions.next().is_some() { return Err(freeze("frame-definition-duplicate")); }
        (function.entry_block, definition)
    };
    let mut bindings = vec![frame_binding];
    let outward = builder.next_block_id();
    append_block(builder, outward, MirInstruction::ReturnFault { fault_frame: frame }, &mut bindings)?;
    let prior_operations = prior.into_iter().map(|(object, value)|
        InvokeOperation::HomeRelease { object, value }).collect::<Vec<_>>();
    let allocation_fault = cleanup_chain(builder, frame, prior_operations, outward, &mut bindings)?;
    let constructor = claim.constructor();
    let birth_fault = if matches!(constructor, OrdinaryNewConstructorDispositionV1::Birth(_)) {
        cleanup_chain(builder, frame,
            vec![InvokeOperation::ReclaimUnpublished { object, value: result }],
            allocation_fault, &mut bindings)?
    } else { allocation_fault };
    let origin = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let normal = builder.next_block_id();
    let allocation = MirInstruction::Invoke {
        operation: InvokeOperation::NewBox { object }, fault_frame: frame,
        normal_landing: normal, fault_landing: allocation_fault,
    };
    builder.emit_instruction(allocation.clone())?;
    bindings.push((origin, allocation));
    builder.start_new_block(normal)?;
    let projection = MirInstruction::InvokeNormalResult { invoke_block: origin, dst: result };
    builder.emit_instruction(projection.clone())?;
    bindings.push((normal, projection));
    // Existing type metadata is a one-way source projection, not object identity.
    builder.function_state.type_ctx.value_types.insert(result, MirType::Box(class.clone()));
    builder.function_state.type_ctx.value_origin_newbox.insert(result, class);
    if let OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
        let effects = recipe.physical_effect_mask();
        let MirInstruction::Call(call) = MirInstruction::call(None,
            Callee::BirthConstructor { key: recipe.target(), receiver: result }, arguments, effects)
            else { unreachable!("canonical Call constructor") };
        let after_birth = builder.next_block_id();
        let birth = MirInstruction::Invoke {
            operation: InvokeOperation::Call(call), fault_frame: frame,
            normal_landing: after_birth, fault_landing: birth_fault,
        };
        builder.emit_instruction(birth.clone())?;
        bindings.push((normal, birth));
        builder.start_new_block(after_birth)?;
    }
    ledger.record_new_emission(&site, result, bindings)?;
    Ok(result)
}

pub(in crate::mir::builder) fn emit_root_home_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    value: ValueId,
) -> Result<ValueId, String> {
    let operations = ledger.begin_root_home_exit()?;
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = Vec::new();
    let mut clean = builder.next_block_id();
    let mut fault = builder.next_block_id();
    append_block(builder, clean, MirInstruction::Return { value: Some(value) }, &mut bindings)?;
    append_block(builder, fault, MirInstruction::ReturnFault { fault_frame: frame }, &mut bindings)?;
    let count = operations.len();
    for (index, (object, home)) in operations.into_iter().rev().enumerate() {
        let operation = InvokeOperation::HomeRelease { object, value: home };
        // A clean call's Fault skips its own retry and joins the remaining
        // fault-pending suffix. Later Normal outcomes cannot clear that Fault.
        let next_clean = cleanup_step(builder, frame, operation.clone(), clean, fault, &mut bindings)?;
        if index + 1 < count {
            fault = cleanup_step(builder, frame, operation, fault, fault, &mut bindings)?;
        }
        clean = next_clean;
    }
    let origin = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let jump = MirInstruction::Jump { target: clean, edge_args: None };
    builder.emit_instruction(jump.clone())?;
    bindings.push((origin, jump));
    ledger.record_root_home_exit(bindings)?;
    Ok(value)
}

pub(in crate::mir::builder) fn emit_terminal_i64_add_return(
    builder: &mut MirBuilder, ledger: &OrdinaryNewClaimLedgerV1,
    prepared: PreparedTerminalI64AddReturnV1,
) -> Result<ValueId, String> {
    let mut values = [ValueId(0); 2];
    for (index, (site, base, field)) in prepared.reads.into_iter().enumerate() {
        let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
        let dst = builder.next_value_id();
        builder.emit_instruction(MirInstruction::ObjectFieldGet { dst, base, field: field.clone() })?;
        builder.function_state.type_ctx.value_types.insert(dst, MirType::Integer);
        ledger.record_terminal_field_read(&site, block, dst, base, field)?;
        values[index] = dst;
    }
    let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let result = builder.next_value_id();
    builder.emit_instruction(MirInstruction::BinOp { dst: result, op: crate::mir::BinaryOp::Add,
        lhs: values[0], rhs: values[1] })?;
    builder.function_state.type_ctx.value_types.insert(result, MirType::Integer);
    ledger.record_terminal_i64_add(block, result, values[0], values[1])?;
    ledger.complete_terminal_i64_add_return(result)?;
    Ok(result)
}

fn cleanup_chain(
    builder: &mut MirBuilder, frame: ValueId, operations: Vec<InvokeOperation>,
    tail: BasicBlockId, bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let mut next = tail;
    // Build links backwards; execution preserves the source-issued order.
    for operation in operations.into_iter().rev() {
        next = cleanup_step(builder, frame, operation, next, next, bindings)?;
    }
    Ok(next)
}

fn cleanup_step(
    builder: &mut MirBuilder, frame: ValueId, operation: InvokeOperation,
    normal_next: BasicBlockId, fault_next: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let origin = builder.next_block_id();
    let normal = builder.next_block_id();
    let fault = builder.next_block_id();
    for (landing, target) in [(normal, normal_next), (fault, fault_next)] {
        append_block(builder, landing, MirInstruction::Jump { target, edge_args: None }, bindings)?;
    }
    append_block(builder, origin, MirInstruction::Invoke {
        operation, fault_frame: frame, normal_landing: normal, fault_landing: fault,
    }, bindings)?;
    Ok(origin)
}

fn append_block(
    builder: &mut MirBuilder, id: BasicBlockId, terminator: MirInstruction,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(), String> {
    let function = builder.function_state.current_function.as_mut()
        .ok_or_else(|| freeze("no-function"))?;
    if function.blocks.contains_key(&id) { return Err(freeze("duplicate-block")); }
    let mut block = BasicBlock::new(id);
    block.set_terminator(terminator.clone());
    function.add_block(block);
    bindings.push((id, terminator));
    Ok(())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/emission/{reason}]")
}
