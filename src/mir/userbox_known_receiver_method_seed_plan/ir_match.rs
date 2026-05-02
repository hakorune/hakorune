use crate::mir::{
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface},
    thin_entry_selection::ThinEntrySelection,
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType,
    ValueId,
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn single_block(function: &MirFunction) -> Option<&BasicBlock> {
    let blocks = ordered_blocks(function);
    (blocks.len() == 1).then_some(blocks[0])
}

pub(super) fn instruction_refs(
    function: &MirFunction,
) -> Vec<(BasicBlockId, usize, &MirInstruction)> {
    let mut refs = Vec::new();
    for block in ordered_blocks(function) {
        for (index, inst) in block.instructions.iter().enumerate() {
            refs.push((block.id, index, inst));
        }
        if let Some(terminator) = block.terminator.as_ref() {
            refs.push((block.id, block.instructions.len(), terminator));
        }
    }
    refs
}

pub(super) fn copy_parent_map(function: &MirFunction) -> BTreeMap<ValueId, ValueId> {
    instruction_refs(function)
        .into_iter()
        .filter_map(|(_, _, inst)| {
            if let MirInstruction::Copy { dst, src } = inst {
                Some((*dst, *src))
            } else {
                None
            }
        })
        .collect()
}

pub(super) fn const_i64_map(function: &MirFunction) -> BTreeMap<ValueId, i64> {
    instruction_refs(function)
        .into_iter()
        .filter_map(|(_, _, inst)| {
            if let MirInstruction::Const {
                dst,
                value: ConstValue::Integer(value),
            } = inst
            {
                Some((*dst, *value))
            } else {
                None
            }
        })
        .collect()
}

pub(super) fn copy_root(value: ValueId, copy_parent: &BTreeMap<ValueId, ValueId>) -> ValueId {
    let mut current = value;
    let mut seen = BTreeSet::new();
    while let Some(parent) = copy_parent.get(&current).copied() {
        if !seen.insert(current) {
            break;
        }
        current = parent;
    }
    current
}

pub(super) fn const_i64_any(inst: &MirInstruction) -> Option<(ValueId, i64)> {
    let MirInstruction::Const {
        dst,
        value: ConstValue::Integer(value),
    } = inst
    else {
        return None;
    };
    Some((*dst, *value))
}

pub(super) fn newbox_named(inst: &MirInstruction, expected_box: &str) -> Option<ValueId> {
    let MirInstruction::NewBox {
        dst,
        box_type,
        args,
    } = inst
    else {
        return None;
    };
    (box_type == expected_box && args.is_empty()).then_some(*dst)
}

pub(super) fn field_set_declared(
    inst: &MirInstruction,
    expected_base: ValueId,
    expected_field: &str,
    expected_value: ValueId,
    expected_declared_box: &str,
) -> Option<()> {
    let MirInstruction::FieldSet {
        base,
        field,
        value,
        declared_type,
    } = inst
    else {
        return None;
    };
    (*base == expected_base
        && field == expected_field
        && *value == expected_value
        && declared_box(declared_type.as_ref(), expected_declared_box))
    .then_some(())
}

pub(super) fn field_get_declared(
    inst: &MirInstruction,
    expected_base: ValueId,
    expected_field: &str,
    expected_declared_box: &str,
) -> Option<ValueId> {
    let MirInstruction::FieldGet {
        dst,
        base,
        field,
        declared_type,
    } = inst
    else {
        return None;
    };
    (*base == expected_base
        && field == expected_field
        && declared_box(declared_type.as_ref(), expected_declared_box))
    .then_some(*dst)
}

pub(super) fn copy_from(inst: &MirInstruction, expected_src: ValueId) -> Option<ValueId> {
    let MirInstruction::Copy { dst, src } = inst else {
        return None;
    };
    (*src == expected_src).then_some(*dst)
}

pub(super) fn method_call(
    inst: &MirInstruction,
    expected_box: &str,
    expected_method: &str,
    expected_receiver: ValueId,
) -> Option<ValueId> {
    let MirInstruction::Call {
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
    } = inst
    else {
        return None;
    };
    (box_name == expected_box
        && method == expected_method
        && *receiver == expected_receiver
        && args.is_empty())
    .then_some(*dst)
}

pub(super) fn add_result(
    inst: &MirInstruction,
    expected_lhs: ValueId,
    expected_rhs: ValueId,
) -> Option<ValueId> {
    let MirInstruction::BinOp { dst, op, lhs, rhs } = inst else {
        return None;
    };
    (*op == BinaryOp::Add && *lhs == expected_lhs && *rhs == expected_rhs).then_some(*dst)
}

pub(super) fn return_value(inst: &MirInstruction, expected_value: ValueId) -> Option<()> {
    let MirInstruction::Return { value } = inst else {
        return None;
    };
    (*value == Some(expected_value)).then_some(())
}

pub(super) fn declared_box(ty: Option<&MirType>, expected_box: &str) -> bool {
    matches!(ty, Some(MirType::Box(box_name)) if box_name == expected_box)
}

pub(super) fn has_thin_selection(
    selections: &[ThinEntrySelection],
    block: BasicBlockId,
    instruction_index: usize,
    value: Option<ValueId>,
    surface: ThinEntrySurface,
    subject: &str,
    manifest_row: &str,
) -> bool {
    selections.iter().any(|selection| {
        selection.block == block
            && selection.instruction_index == instruction_index
            && selection.value == value
            && selection.surface == surface
            && selection.subject == subject
            && selection.manifest_row == manifest_row
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
    })
}

pub(super) fn thin_selection_count(
    selections: &[ThinEntrySelection],
    surface: ThinEntrySurface,
    subject: &str,
    manifest_row: &str,
) -> usize {
    selections
        .iter()
        .filter(|selection| {
            selection.surface == surface
                && selection.subject == subject
                && selection.manifest_row == manifest_row
                && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
        })
        .count()
}

pub(super) fn ordered_blocks(function: &MirFunction) -> Vec<&BasicBlock> {
    let mut ids: Vec<BasicBlockId> = function.blocks.keys().copied().collect();
    ids.sort();
    ids.into_iter()
        .filter_map(|id| function.blocks.get(&id))
        .collect()
}

pub(super) fn instructions_with_terminator(block: &BasicBlock) -> Option<Vec<&MirInstruction>> {
    let mut insts: Vec<&MirInstruction> = block.instructions.iter().collect();
    insts.push(block.terminator.as_ref()?);
    Some(insts)
}

pub(super) fn expect_ops(insts: &[&MirInstruction], expected: &[&str]) -> Option<()> {
    if insts.len() != expected.len() {
        return None;
    }
    for (inst, expected) in insts.iter().zip(expected.iter().copied()) {
        if op_name(inst) != expected {
            return None;
        }
    }
    Some(())
}

fn op_name(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "const",
        MirInstruction::NewBox { .. } => "newbox",
        MirInstruction::FieldSet { .. } => "field_set",
        MirInstruction::FieldGet { .. } => "field_get",
        MirInstruction::Copy { .. } => "copy",
        MirInstruction::Call { .. } => "call",
        MirInstruction::BinOp { .. } => "binop",
        MirInstruction::Return { .. } => "ret",
        _ => "other",
    }
}
