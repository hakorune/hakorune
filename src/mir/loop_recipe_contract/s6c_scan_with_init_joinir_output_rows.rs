//! Owned, logical-only S6C output rows.
//!
//! Every key in this module is copied from the already verified Recipe role
//! seal.  This module never creates a new key space, reads source/MIR, or
//! materializes JoinIR/MIR.  Join transfer and source-call contracts remain
//! borrowed from the source-retaining product by the parent output façade.

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::s6c_scan_with_init::{
    DefinedRoleV2, ExitRoleV2, S6CScanWithInitRecipeRolesRefV2, WriteRoleV2,
};
use super::s6c_scan_with_init_joinir::{
    S6CLogicalCallRoleV1, S6CScanWithInitLogicalJoinInputRefV1,
};
use super::s6c_scan_with_init_rows::{
    S6CRecipeBlockRowRefV2, S6CRecipeOperationRowRefV2, S6CRecipeValueRowRefV2,
    S6CScanWithInitRecipeRowsRefV2,
};
use super::schema_v2::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopConditionV2, LoopExitKindV2, LoopValueClassV2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalOutputRejectV1 {
    Domain(&'static str),
    Row(&'static str),
    Call(&'static str),
    Control(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalCallArgsV1 {
    Empty,
    Pair([LoopValueKeyV1; 2]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CLogicalValueV1 {
    pub(crate) key: LoopValueKeyV1,
    pub(crate) class: LoopValueClassV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CLogicalBindingV1 {
    pub(crate) key: LoopBindingKeyV1,
    pub(crate) class: LoopValueClassV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6CLogicalLoopV1 {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) condition_block: LoopBlockKeyV1,
    pub(crate) condition_value: LoopValueKeyV1,
    pub(crate) body: LoopBlockKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6CLogicalBlockV1 {
    pub(crate) key: LoopBlockKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) items: Box<[LoopItemKeyV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CLogicalCarrierV1 {
    pub(crate) key: LoopCarrierKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: LoopValueClassV2,
    pub(crate) entry_value: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CLogicalExitV1 {
    pub(crate) key: LoopExitKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) value: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CLogicalCallSlotV1 {
    pub(crate) item: LoopItemKeyV1,
    pub(crate) block: LoopBlockKeyV1,
    pub(crate) role: S6CLogicalCallRoleV1,
    pub(crate) receiver: LoopValueKeyV1,
    pub(crate) args: S6CLogicalCallArgsV1,
    pub(crate) result: LoopValueKeyV1,
    pub(crate) result_class: LoopValueClassV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalItemV1 {
    ReadBinding {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
    },
    ConstI64 {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        result: LoopValueKeyV1,
        value: i64,
    },
    BinaryI64 {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        op: LoopBinaryI64OpV2,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    CompareI64 {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        op: LoopCompareI64OpV2,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    CallSlot(S6CLogicalCallSlotV1),
    TextEq {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    If {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        condition: LoopValueKeyV1,
        then_block: LoopBlockKeyV1,
        else_block: Option<LoopBlockKeyV1>,
    },
    WriteBinding {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
    },
    Exit {
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        exit: LoopExitKeyV1,
        value: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CLogicalHeaderV1 {
    pub(crate) root_loop: LoopNodeKeyV1,
    pub(crate) condition_block: LoopBlockKeyV1,
    pub(crate) body_block: LoopBlockKeyV1,
    pub(crate) text_equal_then_block: LoopBlockKeyV1,
    pub(crate) index_binding: LoopBindingKeyV1,
    pub(crate) index_carrier: LoopCarrierKeyV1,
    pub(crate) subject_input: LoopValueKeyV1,
    pub(crate) needle_input: LoopValueKeyV1,
    pub(crate) index_input: LoopValueKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6CLogicalOutputRowsV1 {
    pub(super) header: S6CLogicalHeaderV1,
    loops: Box<[S6CLogicalLoopV1]>,
    blocks: Box<[S6CLogicalBlockV1]>,
    bindings: Box<[S6CLogicalBindingV1]>,
    inputs: Box<[LoopValueKeyV1]>,
    values: Box<[S6CLogicalValueV1]>,
    carriers: Box<[S6CLogicalCarrierV1]>,
    items: Box<[S6CLogicalItemV1]>,
    calls: Box<[S6CLogicalCallSlotV1]>,
    exits: Box<[S6CLogicalExitV1]>,
}

impl S6CLogicalOutputRowsV1 {
    pub(crate) fn loops(&self) -> &[S6CLogicalLoopV1] {
        &self.loops
    }

    pub(crate) fn values(&self) -> &[S6CLogicalValueV1] {
        &self.values
    }

    pub(crate) fn blocks(&self) -> &[S6CLogicalBlockV1] {
        &self.blocks
    }

    pub(crate) fn items(&self) -> &[S6CLogicalItemV1] {
        &self.items
    }

    pub(crate) fn calls(&self) -> &[S6CLogicalCallSlotV1] {
        &self.calls
    }
}

impl S6CLogicalCallSlotV1 {
    pub(crate) const fn role(self) -> S6CLogicalCallRoleV1 {
        self.role
    }
}

pub(crate) fn issue_s6c_logical_output_rows(
    input: S6CScanWithInitLogicalJoinInputRefV1<'_>,
) -> Result<S6CLogicalOutputRowsV1, S6CLogicalOutputRejectV1> {
    let rows = input.rows();
    let roles = input.roles();
    verify_domains(rows)?;

    let root = rows
        .root_loop()
        .ok_or(S6CLogicalOutputRejectV1::Row("root loop"))?;
    let condition_block = rows
        .condition_block()
        .ok_or(S6CLogicalOutputRejectV1::Row("condition block"))?;
    let body_block = rows
        .body_block()
        .ok_or(S6CLogicalOutputRejectV1::Row("body block"))?;
    let then_block = rows
        .text_equal_then_block()
        .ok_or(S6CLogicalOutputRejectV1::Row("TextEq then block"))?;
    let carrier = rows
        .index_carrier()
        .ok_or(S6CLogicalOutputRejectV1::Row("carrier"))?;
    let if_row = rows
        .text_equal_if()
        .ok_or(S6CLogicalOutputRejectV1::Row("TextEq If"))?;
    let exit = rows
        .loop_exit(roles.loop_return())
        .ok_or(S6CLogicalOutputRejectV1::Row("Loop Return"))?;

    let condition = match root.condition {
        LoopConditionV2::Predicate { block, value }
            if block == roles.condition_block() && value == roles.less_condition().result() =>
        {
            (block, value)
        }
        _ => return Err(S6CLogicalOutputRejectV1::Control("loop condition")),
    };
    if root.key != roles.root_loop()
        || root.parent.is_some()
        || root.body != roles.body_block()
        || condition_block.key != roles.condition_block()
        || body_block.key != roles.body_block()
        || then_block.key != roles.text_eq_then_block()
        || carrier.key != roles.index_carrier()
        || carrier.owner_loop != roles.root_loop()
        || carrier.binding != roles.index_binding()
        || carrier.class != LoopValueClassV2::I64
        || carrier.entry_value != roles.index_input()
    {
        return Err(S6CLogicalOutputRejectV1::Control("fixed header"));
    }
    if if_row.condition != roles.text_equal().result()
        || if_row.then_block != roles.text_eq_then_block()
        || if_row.else_block.is_some()
    {
        return Err(S6CLogicalOutputRejectV1::Control("TextEq If parity"));
    }
    let exit_value = match exit.kind {
        LoopExitKindV2::Return { value: Some(value) }
            if exit.key == roles.loop_return().exit()
                && exit.owner_loop == roles.root_loop()
                && value == roles.return_index_read().result() =>
        {
            value
        }
        _ => return Err(S6CLogicalOutputRejectV1::Control("Loop Return parity")),
    };

    let values = fixed_values(rows)?;
    let loops = Box::new([S6CLogicalLoopV1 {
        key: root.key,
        parent: root.parent,
        condition_block: condition.0,
        condition_value: condition.1,
        body: root.body,
    }]);
    let blocks = Box::new([
        copy_block(condition_block),
        copy_block(body_block),
        copy_block(then_block),
    ]);
    let bindings = Box::new([S6CLogicalBindingV1 {
        key: roles.index_binding(),
        class: LoopValueClassV2::I64,
    }]);
    let inputs = Box::new([
        roles.subject_input(),
        roles.needle_input(),
        roles.index_input(),
    ]);
    let carriers = Box::new([S6CLogicalCarrierV1 {
        key: carrier.key,
        owner_loop: carrier.owner_loop,
        binding: carrier.binding,
        class: carrier.class,
        entry_value: carrier.entry_value,
    }]);

    let items = build_items(input, condition_block.key, body_block.key, then_block.key)?;
    let calls = build_calls(input, condition_block.key, body_block.key)?;
    let exits = Box::new([S6CLogicalExitV1 {
        key: exit.key,
        owner_loop: exit.owner_loop,
        value: exit_value,
    }]);
    let output = S6CLogicalOutputRowsV1 {
        header: S6CLogicalHeaderV1 {
            root_loop: roles.root_loop(),
            condition_block: roles.condition_block(),
            body_block: roles.body_block(),
            text_equal_then_block: roles.text_eq_then_block(),
            index_binding: roles.index_binding(),
            index_carrier: roles.index_carrier(),
            subject_input: roles.subject_input(),
            needle_input: roles.needle_input(),
            index_input: roles.index_input(),
        },
        loops,
        blocks,
        bindings,
        inputs,
        values,
        carriers,
        items,
        calls,
        exits,
    };
    verify_output_rows(&output, &roles)?;
    Ok(output)
}

fn verify_domains(
    rows: S6CScanWithInitRecipeRowsRefV2<'_>,
) -> Result<(), S6CLogicalOutputRejectV1> {
    let expected = [
        (rows.loop_count(), 1, "loops"),
        (rows.block_count(), 3, "blocks"),
        (rows.binding_count(), 1, "bindings"),
        (rows.input_count(), 3, "inputs"),
        (rows.value_count(), 15, "values"),
        (rows.item_count(), 15, "items"),
        (rows.carrier_count(), 1, "carriers"),
        (rows.exit_count(), 1, "exits"),
    ];
    expected
        .into_iter()
        .find(|(actual, wanted, _)| actual != wanted)
        .map_or(Ok(()), |(_, _, domain)| {
            Err(S6CLogicalOutputRejectV1::Domain(domain))
        })
}

fn fixed_values(
    rows: S6CScanWithInitRecipeRowsRefV2<'_>,
) -> Result<Box<[S6CLogicalValueV1]>, S6CLogicalOutputRejectV1> {
    let classes = [
        LoopValueClassV2::Text,
        LoopValueClassV2::Text,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
        LoopValueClassV2::Bool,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
        LoopValueClassV2::Text,
        LoopValueClassV2::Bool,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
        LoopValueClassV2::I64,
    ];
    classes
        .into_iter()
        .enumerate()
        .map(|(raw, class)| {
            let key = LoopValueKeyV1::new(raw as u32);
            (rows.value_class(key) == Some(class))
                .then_some(S6CLogicalValueV1 { key, class })
                .ok_or(S6CLogicalOutputRejectV1::Row("value class"))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
}

fn copy_block(row: S6CRecipeBlockRowRefV2<'_>) -> S6CLogicalBlockV1 {
    S6CLogicalBlockV1 {
        key: row.key,
        owner_loop: row.owner_loop,
        items: row.items.to_vec().into_boxed_slice(),
    }
}

fn build_items(
    input: S6CScanWithInitLogicalJoinInputRefV1<'_>,
    condition_block: LoopBlockKeyV1,
    body_block: LoopBlockKeyV1,
    then_block: LoopBlockKeyV1,
) -> Result<Box<[S6CLogicalItemV1]>, S6CLogicalOutputRejectV1> {
    let rows = input.rows();
    let roles = input.roles();
    let read = |role: DefinedRoleV2, block, binding| match rows.operation(role) {
        Some(S6CRecipeOperationRowRefV2::ReadBinding {
            binding: actual,
            result,
        }) if actual == binding && block_has(rows, block, role.item()) => {
            Ok(S6CLogicalItemV1::ReadBinding {
                item: role.item(),
                block,
                binding: actual,
                result,
            })
        }
        _ => Err(S6CLogicalOutputRejectV1::Row("ReadBinding")),
    };
    let condition_index = read(
        roles.condition_index_read(),
        condition_block,
        roles.index_binding(),
    )?;
    let body_index = read(roles.body_index_read(), body_block, roles.index_binding())?;
    let return_index = read(roles.return_index_read(), then_block, roles.index_binding())?;
    let step_index = read(roles.step_index_read(), body_block, roles.index_binding())?;

    let constant = |role: DefinedRoleV2, block| match rows.operation(role) {
        Some(S6CRecipeOperationRowRefV2::ConstI64 { result, value })
            if result == role.result() && block_has(rows, block, role.item()) =>
        {
            Ok(S6CLogicalItemV1::ConstI64 {
                item: role.item(),
                block,
                result,
                value,
            })
        }
        _ => Err(S6CLogicalOutputRejectV1::Row("ConstI64")),
    };
    let slice_one = constant(roles.slice_one(), body_block)?;
    let step_one = constant(roles.step_one(), body_block)?;

    let binary = |role: DefinedRoleV2, block, left, right| match rows.operation(role) {
        Some(S6CRecipeOperationRowRefV2::BinaryI64 {
            op: LoopBinaryI64OpV2::Add,
            left: actual_left,
            right: actual_right,
            result,
        }) if actual_left == left
            && actual_right == right
            && result == role.result()
            && block_has(rows, block, role.item()) =>
        {
            Ok(S6CLogicalItemV1::BinaryI64 {
                item: role.item(),
                block,
                op: LoopBinaryI64OpV2::Add,
                left: actual_left,
                right: actual_right,
                result,
            })
        }
        _ => Err(S6CLogicalOutputRejectV1::Row("BinaryI64")),
    };
    let slice_end = binary(
        roles.slice_end_add(),
        body_block,
        roles.body_index_read().result(),
        roles.slice_one().result(),
    )?;
    let step_add = binary(
        roles.step_add(),
        body_block,
        roles.step_index_read().result(),
        roles.step_one().result(),
    )?;

    let less = match rows.operation(roles.less_condition()) {
        Some(S6CRecipeOperationRowRefV2::CompareI64 {
            op: LoopCompareI64OpV2::Less,
            left,
            right,
            result,
        }) if left == roles.condition_index_read().result()
            && right == roles.length_call().result()
            && result == roles.less_condition().result()
            && block_has(rows, condition_block, roles.less_condition().item()) =>
        {
            S6CLogicalItemV1::CompareI64 {
                item: roles.less_condition().item(),
                block: condition_block,
                op: LoopCompareI64OpV2::Less,
                left,
                right,
                result,
            }
        }
        _ => return Err(S6CLogicalOutputRejectV1::Row("Less")),
    };
    let text_equal = match rows.operation(roles.text_equal()) {
        Some(S6CRecipeOperationRowRefV2::TextEq {
            left,
            right,
            result,
        }) if left == roles.substring_call().result()
            && right == roles.needle_input()
            && result == roles.text_equal().result()
            && block_has(rows, body_block, roles.text_equal().item()) =>
        {
            S6CLogicalItemV1::TextEq {
                item: roles.text_equal().item(),
                block: body_block,
                left,
                right,
                result,
            }
        }
        _ => return Err(S6CLogicalOutputRejectV1::Row("TextEq")),
    };
    let write = match rows.write(roles.step_write()) {
        Some(S6CRecipeOperationRowRefV2::WriteBinding { binding, value })
            if block_has(rows, body_block, roles.step_write().item()) =>
        {
            S6CLogicalItemV1::WriteBinding {
                item: roles.step_write().item(),
                block: body_block,
                binding,
                value,
            }
        }
        _ => return Err(S6CLogicalOutputRejectV1::Row("WriteBinding")),
    };
    let if_item = rows
        .text_equal_if()
        .ok_or(S6CLogicalOutputRejectV1::Control("If"))?;
    let if_item = S6CLogicalItemV1::If {
        item: roles.text_equal_if(),
        block: body_block,
        condition: if_item.condition,
        then_block: if_item.then_block,
        else_block: if_item.else_block,
    };
    let exit = rows
        .loop_exit(roles.loop_return())
        .ok_or(S6CLogicalOutputRejectV1::Control("Exit"))?;
    let exit_item = S6CLogicalItemV1::Exit {
        item: roles.loop_return().item(),
        block: then_block,
        exit: exit.key,
        value: roles.return_index_read().result(),
    };

    let length = call_item(
        input.length().recipe_row(),
        roles.length_call(),
        condition_block,
        S6CLogicalCallRoleV1::Length,
        S6CLogicalCallArgsV1::Empty,
        roles.subject_input(),
    )?;
    let substring = call_item(
        input.substring().recipe_row(),
        roles.substring_call(),
        body_block,
        S6CLogicalCallRoleV1::Substring,
        S6CLogicalCallArgsV1::Pair([
            roles.body_index_read().result(),
            roles.slice_end_add().result(),
        ]),
        roles.subject_input(),
    )?;

    Ok(Box::new([
        condition_index,
        length,
        less,
        body_index,
        slice_one,
        slice_end,
        substring,
        text_equal,
        if_item,
        return_index,
        exit_item,
        step_index,
        step_one,
        step_add,
        write,
    ]))
}

fn call_item(
    row: S6CRecipeOperationRowRefV2<'_>,
    role: DefinedRoleV2,
    block: LoopBlockKeyV1,
    call_role: S6CLogicalCallRoleV1,
    args: S6CLogicalCallArgsV1,
    receiver: LoopValueKeyV1,
) -> Result<S6CLogicalItemV1, S6CLogicalOutputRejectV1> {
    match row {
        S6CRecipeOperationRowRefV2::CallSlot {
            receiver: Some(actual_receiver),
            args: actual_args,
            result: Some(result),
        } if actual_receiver == receiver
            && result == role.result()
            && match (args, actual_args) {
                (S6CLogicalCallArgsV1::Empty, []) => true,
                (S6CLogicalCallArgsV1::Pair(expected), actual) => actual == expected,
                _ => false,
            } =>
        {
            Ok(S6CLogicalItemV1::CallSlot(S6CLogicalCallSlotV1 {
                item: role.item(),
                block,
                role: call_role,
                receiver: actual_receiver,
                args,
                result,
                result_class: if call_role == S6CLogicalCallRoleV1::Length {
                    LoopValueClassV2::I64
                } else {
                    LoopValueClassV2::Text
                },
            }))
        }
        _ => Err(S6CLogicalOutputRejectV1::Call("CallSlot")),
    }
}

fn build_calls(
    input: S6CScanWithInitLogicalJoinInputRefV1<'_>,
    condition_block: LoopBlockKeyV1,
    body_block: LoopBlockKeyV1,
) -> Result<Box<[S6CLogicalCallSlotV1]>, S6CLogicalOutputRejectV1> {
    let roles = input.roles();
    let length = call_item(
        input.length().recipe_row(),
        roles.length_call(),
        condition_block,
        S6CLogicalCallRoleV1::Length,
        S6CLogicalCallArgsV1::Empty,
        roles.subject_input(),
    )?;
    let substring = call_item(
        input.substring().recipe_row(),
        roles.substring_call(),
        body_block,
        S6CLogicalCallRoleV1::Substring,
        S6CLogicalCallArgsV1::Pair([
            roles.body_index_read().result(),
            roles.slice_end_add().result(),
        ]),
        roles.subject_input(),
    )?;
    let calls = [length, substring]
        .into_iter()
        .map(|item| match item {
            S6CLogicalItemV1::CallSlot(call) => call,
            _ => unreachable!("call_item returned a call"),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Ok(calls)
}

fn block_has(
    rows: S6CScanWithInitRecipeRowsRefV2<'_>,
    block: LoopBlockKeyV1,
    item: LoopItemKeyV1,
) -> bool {
    [
        rows.condition_block(),
        rows.body_block(),
        rows.text_equal_then_block(),
    ]
    .into_iter()
    .flatten()
    .any(|row| row.key == block && row.items.contains(&item))
}

fn verify_output_rows(
    output: &S6CLogicalOutputRowsV1,
    roles: &S6CScanWithInitRecipeRolesRefV2<'_>,
) -> Result<(), S6CLogicalOutputRejectV1> {
    if output.loops.len() != 1
        || output.blocks.len() != 3
        || output.bindings.len() != 1
        || output.inputs.len() != 3
        || output.values.len() != 15
        || output.carriers.len() != 1
        || output.items.len() != 15
        || output.calls.len() != 2
        || output.exits.len() != 1
    {
        return Err(S6CLogicalOutputRejectV1::Domain("output rows"));
    }
    if output.header.root_loop != roles.root_loop()
        || output.header.condition_block != roles.condition_block()
        || output.header.body_block != roles.body_block()
        || output.header.text_equal_then_block != roles.text_eq_then_block()
        || output.header.index_binding != roles.index_binding()
        || output.header.index_carrier != roles.index_carrier()
        || output.header.subject_input != roles.subject_input()
        || output.header.needle_input != roles.needle_input()
        || output.header.index_input != roles.index_input()
    {
        return Err(S6CLogicalOutputRejectV1::Row("header parity"));
    }
    if output.inputs.as_ref()
        != [
            roles.subject_input(),
            roles.needle_input(),
            roles.index_input(),
        ]
    {
        return Err(S6CLogicalOutputRejectV1::Row("input parity"));
    }
    if output.calls[0].role != S6CLogicalCallRoleV1::Length
        || output.calls[1].role != S6CLogicalCallRoleV1::Substring
        || output.calls[0].args != S6CLogicalCallArgsV1::Empty
        || output.calls[1].args
            != S6CLogicalCallArgsV1::Pair([
                roles.body_index_read().result(),
                roles.slice_end_add().result(),
            ])
    {
        return Err(S6CLogicalOutputRejectV1::Call("call role/order"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{S6CLogicalCallArgsV1, S6CLogicalCallRoleV1};

    #[test]
    fn logical_output_call_args_are_typed_and_ordered() {
        assert_ne!(
            S6CLogicalCallArgsV1::Empty,
            S6CLogicalCallArgsV1::Pair([
                super::LoopValueKeyV1::new(6),
                super::LoopValueKeyV1::new(8),
            ])
        );
        assert_ne!(
            S6CLogicalCallRoleV1::Length,
            S6CLogicalCallRoleV1::Substring
        );
    }
}
