use super::ir_match::{
    const_i64_map, copy_parent_map, copy_root, declared_box, instruction_refs, ordered_blocks,
};
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
};
use std::collections::BTreeMap;

pub(super) struct UserBoxMethodMainFacts {
    pub(super) block_count: usize,
    pub(super) newbox_block: BasicBlockId,
    pub(super) newbox_instruction_index: usize,
    pub(super) newbox_value: ValueId,
    pub(super) base_i64: Option<i64>,
    pub(super) x_i64: Option<i64>,
    pub(super) y_i64: Option<i64>,
    pub(super) ops: Option<i64>,
    pub(super) first_call_instruction_index: usize,
    pub(super) first_call_result: ValueId,
    pub(super) known_receiver_count: usize,
    pub(super) field_set_count: usize,
    pub(super) compare_lt_count: usize,
    pub(super) branch_count: usize,
    pub(super) jump_count: usize,
    pub(super) ret_count: usize,
    pub(super) add_count: usize,
}

pub(super) fn collect_userbox_method_main_facts(
    function: &MirFunction,
    expected_box: &str,
    expected_method: &str,
) -> Option<UserBoxMethodMainFacts> {
    let block_count = ordered_blocks(function).len();
    let copy_parent = copy_parent_map(function);
    let const_values = const_i64_map(function);
    let mut newbox_count = 0usize;
    let mut newbox_block = None;
    let mut newbox_instruction_index = 0usize;
    let mut newbox_value = None;
    let mut base_i64 = None;
    let mut x_i64 = None;
    let mut y_i64 = None;
    let mut ops = None;
    let mut first_call_instruction_index = 0usize;
    let mut first_call_result = None;
    let mut known_receiver_count = 0usize;
    let mut field_set_count = 0usize;
    let mut compare_lt_count = 0usize;
    let mut branch_count = 0usize;
    let mut jump_count = 0usize;
    let mut ret_count = 0usize;
    let mut add_count = 0usize;

    for (block, index, inst) in instruction_refs(function) {
        match inst {
            MirInstruction::Const {
                value: ConstValue::Integer(value),
                ..
            } if *value == 2_000_000 => {
                ops = Some(*value);
            }
            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } if box_type == expected_box && args.is_empty() => {
                newbox_count += 1;
                if newbox_value.is_none() {
                    newbox_block = Some(block);
                    newbox_instruction_index = index;
                    newbox_value = Some(*dst);
                }
            }
            MirInstruction::FieldSet {
                base,
                field,
                value,
                declared_type,
            } if newbox_value
                .map(|newbox| copy_root(*base, &copy_parent) == newbox)
                .unwrap_or(false)
                && declared_box(declared_type.as_ref(), "IntegerBox") =>
            {
                let literal = const_values.get(&copy_root(*value, &copy_parent)).copied();
                match (expected_box, field.as_str(), literal) {
                    ("Counter", "value", Some(value)) => {
                        base_i64 = Some(value);
                        field_set_count += 1;
                    }
                    ("Point", "x", Some(value)) => {
                        x_i64 = Some(value);
                        field_set_count += 1;
                    }
                    ("Point", "y", Some(value)) => {
                        y_i64 = Some(value);
                        field_set_count += 1;
                    }
                    _ => return None,
                }
            }
            MirInstruction::Call {
                dst: Some(dst),
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } if box_name == expected_box && method == expected_method && args.is_empty() => {
                let Some(newbox) = newbox_value else {
                    return None;
                };
                if copy_root(*receiver, &copy_parent) != newbox {
                    return None;
                }
                known_receiver_count += 1;
                if first_call_result.is_none() {
                    first_call_instruction_index = index;
                    first_call_result = Some(*dst);
                }
            }
            MirInstruction::Compare {
                op: CompareOp::Lt, ..
            } => {
                compare_lt_count += 1;
            }
            MirInstruction::Branch { .. } => branch_count += 1,
            MirInstruction::Jump { .. } => jump_count += 1,
            MirInstruction::Return { .. } => ret_count += 1,
            MirInstruction::BinOp {
                op: BinaryOp::Add, ..
            } => add_count += 1,
            _ => {}
        }
    }

    (newbox_count == 1 && first_call_result.is_some()).then_some(UserBoxMethodMainFacts {
        block_count,
        newbox_block: newbox_block?,
        newbox_instruction_index,
        newbox_value: newbox_value?,
        base_i64,
        x_i64,
        y_i64,
        ops,
        first_call_instruction_index,
        first_call_result: first_call_result?,
        known_receiver_count,
        field_set_count,
        compare_lt_count,
        branch_count,
        jump_count,
        ret_count,
        add_count,
    })
}

pub(super) fn find_function<'a>(
    functions: &'a BTreeMap<String, MirFunction>,
    name: &str,
) -> Option<&'a MirFunction> {
    functions.get(name).or_else(|| {
        functions
            .values()
            .find(|function| function.signature.name == name)
    })
}
