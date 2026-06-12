use std::collections::BTreeMap;

use super::super::ir_match::{
    add_result, const_i64_any, copy_from, copy_parent_map, copy_root, declared_box, expect_ops,
    field_get_declared, field_set_declared, has_thin_selection, instructions_with_terminator,
    method_call, newbox_named, return_value, single_block, thin_selection_count,
};
use super::super::main_facts::{collect_userbox_method_main_facts, find_function};
use super::super::model::UserBoxKnownReceiverMethodSeedProof;
use super::shared::PointSumMethodFacts;
use super::{BasicBlockId, BinaryOp, MirFunction, MirInstruction, ThinEntrySurface, ValueId};

pub(super) fn match_point_seed_route(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    if let Some(route) = match_point_sum_local_i64(function, functions) {
        return Some(route);
    }
    if let Some(route) = match_point_sum_copy_local_i64(function, functions) {
        return Some(route);
    }
    match_point_sum_micro(function, functions)
}

fn match_point_sum_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let method_name = "Point.sum/1";
    let method = find_function(functions, method_name)?;
    let method_facts = match_point_sum_method(method)?;
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(
        &insts,
        &[
            "const",
            "const",
            "newbox",
            "field_set",
            "field_set",
            "call",
            "ret",
        ],
    )?;

    let (x_value, x_i64) = const_i64_any(insts[0])?;
    let (y_value, y_i64) = const_i64_any(insts[1])?;
    let box_value = newbox_named(insts[2], "Point")?;
    field_set_declared(insts[3], box_value, "x", x_value, "IntegerBox")?;
    field_set_declared(insts[4], box_value, "y", y_value, "IntegerBox")?;
    let result_value = method_call(insts[5], "Point", "sum", box_value)?;
    return_value(insts[6], result_value)?;
    if !point_sum_thin_contract_ok(function, method, block.id, 3, 4, 5, result_value) {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::PointSumLocalI64,
        box_name: "Point".to_string(),
        method: "sum".to_string(),
        method_function: method_name.to_string(),
        block_count: 1,
        method_block_count: method_facts.block_count,
        block: block.id,
        method_block: method_facts.block,
        newbox_instruction_index: 2,
        copy_instruction_index: None,
        call_instruction_index: 5,
        box_value,
        copy_value: None,
        result_value,
        proof: UserBoxKnownReceiverMethodSeedProof::PointSumLocalI64Seed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64, y_i64 },
    })
}

fn match_point_sum_copy_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let method_name = "Point.sum/1";
    let method = find_function(functions, method_name)?;
    let method_facts = match_point_sum_method(method)?;
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(
        &insts,
        &[
            "const",
            "const",
            "newbox",
            "field_set",
            "field_set",
            "copy",
            "call",
            "ret",
        ],
    )?;

    let (x_value, x_i64) = const_i64_any(insts[0])?;
    let (y_value, y_i64) = const_i64_any(insts[1])?;
    let box_value = newbox_named(insts[2], "Point")?;
    field_set_declared(insts[3], box_value, "x", x_value, "IntegerBox")?;
    field_set_declared(insts[4], box_value, "y", y_value, "IntegerBox")?;
    let copy_value = copy_from(insts[5], box_value)?;
    let result_value = method_call(insts[6], "Point", "sum", copy_value)?;
    return_value(insts[7], result_value)?;
    if !point_sum_thin_contract_ok(function, method, block.id, 3, 4, 6, result_value) {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::PointSumCopyLocalI64,
        box_name: "Point".to_string(),
        method: "sum".to_string(),
        method_function: method_name.to_string(),
        block_count: 1,
        method_block_count: method_facts.block_count,
        block: block.id,
        method_block: method_facts.block,
        newbox_instruction_index: 2,
        copy_instruction_index: Some(5),
        call_instruction_index: 6,
        box_value,
        copy_value: Some(copy_value),
        result_value,
        proof: UserBoxKnownReceiverMethodSeedProof::PointSumLocalI64Seed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64, y_i64 },
    })
}

fn match_point_sum_micro(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<super::UserBoxKnownReceiverMethodSeedRoute> {
    let method_name = "Point.sum/0";
    let method = find_function(functions, method_name)?;
    let method_facts = match_point_sum_zero_method(method)?;
    let facts = collect_userbox_method_main_facts(function, "Point", "sum")?;
    let x_i64 = facts.x_i64?;
    let y_i64 = facts.y_i64?;
    let ops = facts.ops?;

    if !(facts.block_count >= 4
        && facts.field_set_count == 2
        && facts.known_receiver_count == 2
        && facts.compare_lt_count == 1
        && facts.branch_count == 1
        && facts.ret_count == 1
        && facts.add_count >= 3
        && x_i64 == 1
        && y_i64 == 2
        && ops == 2_000_000
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.x",
            "user_box_field_set.inline_scalar",
        ) == 1
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.y",
            "user_box_field_set.inline_scalar",
        ) == 1
        && thin_selection_count(
            &function.metadata.thin_entry_selections,
            ThinEntrySurface::UserBoxMethod,
            "Point.sum",
            "user_box_method.known_receiver",
        ) == 2
        && !method.metadata.thin_entry_selections.is_empty())
    {
        return None;
    }

    Some(super::UserBoxKnownReceiverMethodSeedRoute {
        kind: super::UserBoxKnownReceiverMethodSeedKind::PointSumMicro,
        box_name: "Point".to_string(),
        method: "sum".to_string(),
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
        proof: UserBoxKnownReceiverMethodSeedProof::PointSumMicroSeed,
        payload: super::UserBoxKnownReceiverMethodSeedPayload::PointSumLoopMicro {
            x_i64,
            y_i64,
            ops,
            sum_i64: x_i64 + y_i64,
            known_receiver_count: facts.known_receiver_count,
            field_set_count: facts.field_set_count,
            compare_lt_count: facts.compare_lt_count,
            branch_count: facts.branch_count,
            jump_count: facts.jump_count,
            ret_count: facts.ret_count,
            add_count: facts.add_count,
        },
    })
}

fn match_point_sum_method(function: &MirFunction) -> Option<PointSumMethodFacts> {
    if function.params != vec![ValueId::new(0)] {
        return None;
    }
    let block = single_block(function)?;
    let insts = instructions_with_terminator(block)?;
    expect_ops(&insts, &["field_get", "field_get", "binop", "ret"])?;
    let get_x = field_get_declared(insts[0], ValueId::new(0), "x", "IntegerBox")?;
    let get_y = field_get_declared(insts[1], ValueId::new(0), "y", "IntegerBox")?;
    let result_value = add_result(insts[2], get_x, get_y)?;
    return_value(insts[3], result_value)?;
    (has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        0,
        Some(get_x),
        ThinEntrySurface::UserBoxFieldGet,
        "Point.x",
        "user_box_field_get.inline_scalar",
    ) && has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        1,
        Some(get_y),
        ThinEntrySurface::UserBoxFieldGet,
        "Point.y",
        "user_box_field_get.inline_scalar",
    ))
    .then_some(PointSumMethodFacts {
        block_count: 1,
        block: block.id,
    })
}

fn match_point_sum_zero_method(function: &MirFunction) -> Option<PointSumMethodFacts> {
    if function.params != vec![ValueId::new(0)] {
        return None;
    }
    let block = single_block(function)?;
    let copy_parent = copy_parent_map(function);
    let mut get_x = None;
    let mut get_y = None;
    let mut get_x_index = 0;
    let mut get_y_index = 0;
    let mut result_value = None;

    for (index, inst) in block.instructions.iter().enumerate() {
        match inst {
            MirInstruction::FieldGet {
                dst,
                base,
                field,
                declared_type,
            } if copy_root(*base, &copy_parent) == ValueId::new(0)
                && declared_box(declared_type.as_ref(), "IntegerBox") =>
            {
                if field == "x" {
                    get_x = Some(*dst);
                    get_x_index = index;
                } else if field == "y" {
                    get_y = Some(*dst);
                    get_y_index = index;
                }
            }
            MirInstruction::BinOp { dst, op, lhs, rhs }
                if *op == BinaryOp::Add
                    && Some(copy_root(*lhs, &copy_parent)) == get_x
                    && Some(copy_root(*rhs, &copy_parent)) == get_y =>
            {
                result_value = Some(*dst);
            }
            _ => {}
        }
    }

    let get_x = get_x?;
    let get_y = get_y?;
    let result_value = result_value?;
    return_value(block.terminator.as_ref()?, result_value)?;
    (has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        get_x_index,
        Some(get_x),
        ThinEntrySurface::UserBoxFieldGet,
        "Point.x",
        "user_box_field_get.inline_scalar",
    ) && has_thin_selection(
        &function.metadata.thin_entry_selections,
        block.id,
        get_y_index,
        Some(get_y),
        ThinEntrySurface::UserBoxFieldGet,
        "Point.y",
        "user_box_field_get.inline_scalar",
    ))
    .then_some(PointSumMethodFacts {
        block_count: 1,
        block: block.id,
    })
}

fn point_sum_thin_contract_ok(
    main_fn: &MirFunction,
    method_fn: &MirFunction,
    block: BasicBlockId,
    set_x_instruction_index: usize,
    set_y_instruction_index: usize,
    call_instruction_index: usize,
    result_value: ValueId,
) -> bool {
    has_thin_selection(
        &main_fn.metadata.thin_entry_selections,
        block,
        set_x_instruction_index,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        "Point.x",
        "user_box_field_set.inline_scalar",
    ) && has_thin_selection(
        &main_fn.metadata.thin_entry_selections,
        block,
        set_y_instruction_index,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        "Point.y",
        "user_box_field_set.inline_scalar",
    ) && has_thin_selection(
        &main_fn.metadata.thin_entry_selections,
        block,
        call_instruction_index,
        Some(result_value),
        ThinEntrySurface::UserBoxMethod,
        "Point.sum",
        "user_box_method.known_receiver",
    ) && !method_fn.metadata.thin_entry_selections.is_empty()
}
