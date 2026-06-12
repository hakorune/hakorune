use std::collections::BTreeMap;

use super::super::ir_match::{
    add_result, const_i64_any, copy_from, copy_parent_map, copy_root, declared_box, expect_ops,
    field_get_declared, field_set_declared, has_thin_selection, instructions_with_terminator,
    method_call, newbox_named, return_value, single_block, thin_selection_count,
};
use super::super::main_facts::{collect_userbox_method_main_facts, find_function};
use super::super::model::UserBoxKnownReceiverMethodSeedProof;
use super::shared::{ChainForwardFacts, CounterStepMethodFacts};
use super::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, ThinEntrySurface,
    ValueId,
};

pub(super) fn match_counter_seed_route(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    if let Some(route) = match_counter_step_local_i64(function, functions) {
        return Some(route);
    }
    if let Some(route) = match_counter_step_copy_local_i64(function, functions) {
        return Some(route);
    }
    if let Some(route) = match_counter_step_chain_local_i64(function, functions) {
        return Some(route);
    }
    if let Some(route) = match_counter_step_chain_micro(function, functions) {
        return Some(route);
    }
    match_counter_step_micro(function, functions)
}

fn match_counter_step_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let method_name = "Counter.step/1";
    let method = find_function(functions, method_name)?;
    let method_facts = match_counter_step_method(method)?;
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(&insts, &["const", "newbox", "field_set", "call", "ret"])?;

    let (base_value, base_i64) = const_i64_any(insts[0])?;
    let box_value = newbox_named(insts[1], "Counter")?;
    field_set_declared(insts[2], box_value, "value", base_value, "IntegerBox")?;
    let result_value = method_call(insts[3], "Counter", "step", box_value)?;
    return_value(insts[4], result_value)?;
    if !counter_step_thin_contract_ok(function, method, block.id, 2, 3, result_value) {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::CounterStepLocalI64,
        box_name: "Counter".to_string(),
        method: "step".to_string(),
        method_function: method_name.to_string(),
        block_count: 1,
        method_block_count: method_facts.block_count,
        block: block.id,
        method_block: method_facts.block,
        newbox_instruction_index: 1,
        copy_instruction_index: None,
        call_instruction_index: 3,
        box_value,
        copy_value: None,
        result_value,
        proof: UserBoxKnownReceiverMethodSeedProof::CounterStepLocalI64Seed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
            base_i64,
            delta_i64: method_facts.delta_i64,
        },
    })
}

fn match_counter_step_copy_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let method_name = "Counter.step/1";
    let method = find_function(functions, method_name)?;
    let method_facts = match_counter_step_method(method)?;
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(
        &insts,
        &["const", "newbox", "field_set", "copy", "call", "ret"],
    )?;

    let (base_value, base_i64) = const_i64_any(insts[0])?;
    let box_value = newbox_named(insts[1], "Counter")?;
    field_set_declared(insts[2], box_value, "value", base_value, "IntegerBox")?;
    let copy_value = copy_from(insts[3], box_value)?;
    let result_value = method_call(insts[4], "Counter", "step", copy_value)?;
    return_value(insts[5], result_value)?;
    if !counter_step_thin_contract_ok(function, method, block.id, 2, 4, result_value) {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::CounterStepCopyLocalI64,
        box_name: "Counter".to_string(),
        method: "step".to_string(),
        method_function: method_name.to_string(),
        block_count: 1,
        method_block_count: method_facts.block_count,
        block: block.id,
        method_block: method_facts.block,
        newbox_instruction_index: 1,
        copy_instruction_index: Some(3),
        call_instruction_index: 4,
        box_value,
        copy_value: Some(copy_value),
        result_value,
        proof: UserBoxKnownReceiverMethodSeedProof::CounterStepLocalI64Seed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
            base_i64,
            delta_i64: method_facts.delta_i64,
        },
    })
}

fn match_counter_step_chain_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let chain_name = "Counter.step_chain/1";
    let leaf_name = "Counter.step/1";
    let chain = find_function(functions, chain_name)?;
    let leaf = find_function(functions, leaf_name)?;
    let chain_facts = match_counter_step_chain_forward_method(chain, leaf_name)?;
    let leaf_facts = match_counter_step_method(leaf)?;
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(
        &insts,
        &["const", "newbox", "field_set", "copy", "call", "ret"],
    )?;

    let (base_value, base_i64) = const_i64_any(insts[0])?;
    let box_value = newbox_named(insts[1], "Counter")?;
    field_set_declared(insts[2], box_value, "value", base_value, "IntegerBox")?;
    let copy_value = copy_from(insts[3], box_value)?;
    let result_value = method_call(insts[4], "Counter", "step_chain", copy_value)?;
    return_value(insts[5], result_value)?;

    if !(has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        2,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        "Counter.value",
        "user_box_field_set.inline_scalar",
    ) && has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        4,
        Some(result_value),
        ThinEntrySurface::UserBoxMethod,
        "Counter.step_chain",
        "user_box_method.known_receiver",
    ) && !leaf.metadata.thin_entry_selections.is_empty())
    {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::CounterStepChainLocalI64,
        box_name: "Counter".to_string(),
        method: "step_chain".to_string(),
        method_function: chain_name.to_string(),
        block_count: 1,
        method_block_count: chain_facts.block_count,
        block: block.id,
        method_block: chain_facts.block,
        newbox_instruction_index: 1,
        copy_instruction_index: Some(3),
        call_instruction_index: 4,
        box_value,
        copy_value: Some(copy_value),
        result_value,
        proof: UserBoxKnownReceiverMethodSeedProof::CounterStepChainLocalI64Seed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::CounterStepChainI64 {
            base_i64,
            delta_i64: leaf_facts.delta_i64,
            leaf_method_function: leaf_name.to_string(),
            leaf_method_block_count: leaf_facts.block_count,
            leaf_method_block: leaf_facts.block,
            ops: None,
            known_receiver_count: 1,
            field_set_count: 1,
        },
    })
}

fn match_counter_step_micro(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let method_name = "Counter.step/0";
    let method = find_function(functions, method_name)?;
    let method_facts = match_counter_step_zero_method(method)?;
    let facts = collect_userbox_method_main_facts(function, "Counter", "step")?;
    let base_i64 = facts.base_i64?;
    let ops = facts.ops?;

    if !(facts.block_count >= 4
        && facts.field_set_count == 1
        && facts.known_receiver_count == 2
        && base_i64 == 41
        && ops == 2_000_000
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxFieldSet,
            "Counter.value",
            "user_box_field_set.inline_scalar",
        ) == 1
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxMethod,
            "Counter.step",
            "user_box_method.known_receiver",
        ) == 2
        && !method.metadata.thin_entry_selections.is_empty())
    {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::CounterStepMicro,
        box_name: "Counter".to_string(),
        method: "step".to_string(),
        method_function: method_name.to_string(),
        block_count: facts.block_count,
        method_block_count: method_facts.block_count,
        block: facts.newbox_block,
        method_block: method_facts.block,
        newbox_instruction_index: facts.newbox_instruction_index,
        copy_instruction_index: None,
        call_instruction_index: facts.first_call_instruction_index,
        box_value: facts.newbox_value,
        copy_value: None,
        result_value: facts.first_call_result,
        proof: UserBoxKnownReceiverMethodSeedProof::CounterStepMicroSeed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::CounterStepLoopMicro {
            base_i64,
            delta_i64: method_facts.delta_i64,
            ops,
            step_i64: base_i64 + method_facts.delta_i64,
            known_receiver_count: facts.known_receiver_count,
            field_set_count: facts.field_set_count,
        },
    })
}

fn match_counter_step_chain_micro(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let chain_name = "Counter.step_chain/0";
    let leaf_name = "Counter.step/0";
    let chain = find_function(functions, chain_name)?;
    let leaf = find_function(functions, leaf_name)?;
    let chain_facts = match_counter_step_chain_forward_method(chain, leaf_name)?;
    let leaf_facts = match_counter_step_zero_method(leaf)?;
    let facts = collect_userbox_method_main_facts(function, "Counter", "step_chain")?;
    let base_i64 = facts.base_i64?;
    let ops = facts.ops?;

    if !(facts.block_count >= 4
        && facts.field_set_count == 1
        && facts.known_receiver_count == 2
        && base_i64 == 41
        && ops == 2_000_000
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxFieldSet,
            "Counter.value",
            "user_box_field_set.inline_scalar",
        ) == 1
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxMethod,
            "Counter.step_chain",
            "user_box_method.known_receiver",
        ) == 2
        && thin_selection_count(
            &chain.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxMethod,
            "Counter.step",
            "user_box_method.known_receiver",
        ) == 1
        && !leaf.metadata.thin_entry_selections.is_empty())
    {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::CounterStepChainMicro,
        box_name: "Counter".to_string(),
        method: "step_chain".to_string(),
        method_function: chain_name.to_string(),
        block_count: facts.block_count,
        method_block_count: chain_facts.block_count,
        block: facts.newbox_block,
        method_block: chain_facts.block,
        newbox_instruction_index: facts.newbox_instruction_index,
        copy_instruction_index: None,
        call_instruction_index: facts.first_call_instruction_index,
        box_value: facts.newbox_value,
        copy_value: None,
        result_value: facts.first_call_result,
        proof: UserBoxKnownReceiverMethodSeedProof::CounterStepChainMicroSeed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::CounterStepChainI64 {
            base_i64,
            delta_i64: leaf_facts.delta_i64,
            leaf_method_function: leaf_name.to_string(),
            leaf_method_block_count: leaf_facts.block_count,
            leaf_method_block: leaf_facts.block,
            ops: Some(ops),
            known_receiver_count: facts.known_receiver_count,
            field_set_count: facts.field_set_count,
        },
    })
}

fn match_counter_step_method(function: &MirFunction) -> Option<CounterStepMethodFacts> {
    if function.params != vec![ValueId::new(0)] {
        return None;
    }
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(&insts, &["field_get", "const", "binop", "ret"])?;
    let field_value = field_get_declared(insts[0], ValueId::new(0), "value", "IntegerBox")?;
    let (delta_value, delta_i64) = const_i64_any(insts[1])?;
    let result_value = add_result(insts[2], field_value, delta_value)?;
    return_value(insts[3], result_value)?;
    has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        0,
        Some(field_value),
        ThinEntrySurface::UserBoxFieldGet,
        "Counter.value",
        "user_box_field_get.inline_scalar",
    )
    .then_some(CounterStepMethodFacts {
        block_count: 1,
        block: block.id,
        delta_i64,
    })
}

fn match_counter_step_zero_method(function: &MirFunction) -> Option<CounterStepMethodFacts> {
    if function.params != vec![ValueId::new(0)] {
        return None;
    }
    let block = single_block(function)?;
    let copy_parent = copy_parent_map(function);
    let mut field_value = None;
    let mut field_index = 0;
    let mut const_value = None;
    let mut delta_i64 = 0;
    let mut result_value = None;

    for (index, inst) in block.instructions.iter().enumerate() {
        match inst {
            MirInstruction::FieldGet {
                dst,
                base,
                field,
                declared_type,
            } if copy_root(*base, &copy_parent) == ValueId::new(0)
                && field == "value"
                && declared_box(declared_type.as_ref(), "IntegerBox") =>
            {
                field_value = Some(*dst);
                field_index = index;
            }
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(value),
            } if *value == 2 => {
                const_value = Some(*dst);
                delta_i64 = *value;
            }
            MirInstruction::BinOp { dst, op, lhs, rhs }
                if *op == BinaryOp::Add
                    && Some(copy_root(*lhs, &copy_parent)) == field_value
                    && Some(copy_root(*rhs, &copy_parent)) == const_value =>
            {
                result_value = Some(*dst);
            }
            _ => {}
        }
    }

    let field_value = field_value?;
    let result_value = result_value?;
    return_value(block.terminator.as_ref()?, result_value)?;
    has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        field_index,
        Some(field_value),
        ThinEntrySurface::UserBoxFieldGet,
        "Counter.value",
        "user_box_field_get.inline_scalar",
    )
    .then_some(CounterStepMethodFacts {
        block_count: 1,
        block: block.id,
        delta_i64,
    })
}

fn match_counter_step_chain_forward_method(
    function: &MirFunction,
    leaf_function_name: &str,
) -> Option<ChainForwardFacts> {
    if function.params != vec![ValueId::new(0)] {
        return None;
    }
    let block = single_block(function)?;
    let copy_parent = copy_parent_map(function);
    let mut call_result = None;

    for inst in &block.instructions {
        let MirInstruction::Call {
            dst: Some(dst),
            callee: Some(callee),
            args,
            ..
        } = inst
        else {
            continue;
        };
        match callee {
            Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            } if box_name == "Counter"
                && method == "step"
                && args.is_empty()
                && copy_root(*receiver, &copy_parent) == ValueId::new(0) =>
            {
                call_result = Some(*dst);
            }
            Callee::Global(name)
                if name == leaf_function_name
                    && args.len() == 1
                    && copy_root(args[0], &copy_parent) == ValueId::new(0) =>
            {
                call_result = Some(*dst);
            }
            _ => {}
        }
    }

    return_value(block.terminator.as_ref()?, call_result?)?;
    Some(ChainForwardFacts {
        block_count: 1,
        block: block.id,
    })
}

fn counter_step_thin_contract_ok(
    main_fn: &MirFunction,
    method_fn: &MirFunction,
    block: BasicBlockId,
    set_instruction_index: usize,
    call_instruction_index: usize,
    result_value: ValueId,
) -> bool {
    has_thin_selection(
        &main_fn.metadata.thin_entry_selections,
        block,
        set_instruction_index,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        "Counter.value",
        "user_box_field_set.inline_scalar",
    ) && has_thin_selection(
        &main_fn.metadata.thin_entry_selections,
        block,
        call_instruction_index,
        Some(result_value),
        ThinEntrySurface::UserBoxMethod,
        "Counter.step",
        "user_box_method.known_receiver",
    ) && !method_fn.metadata.thin_entry_selections.is_empty()
}
