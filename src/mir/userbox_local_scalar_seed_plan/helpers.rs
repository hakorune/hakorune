use crate::mir::thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface};
use crate::mir::thin_entry_selection::ThinEntrySelection;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, MirType, ValueId,
};

pub(super) fn const_i64(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_value: i64,
) -> Option<(ValueId, i64)> {
    let MirInstruction::Const {
        dst,
        value: ConstValue::Integer(value),
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst && *value == expected_value).then_some((*dst, *value))
}

pub(super) fn const_f64_bits(
    inst: &MirInstruction,
    expected_dst: ValueId,
) -> Option<(ValueId, u64)> {
    let MirInstruction::Const {
        dst,
        value: ConstValue::Float(value),
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst).then_some((*dst, value.to_bits()))
}

pub(super) fn newbox_named(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_box: &str,
) -> Option<ValueId> {
    let MirInstruction::NewBox {
        dst,
        box_type,
        args,
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst && box_type == expected_box && args.is_empty()).then_some(*dst)
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
    expected_dst: ValueId,
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
    (*dst == expected_dst
        && *base == expected_base
        && field == expected_field
        && declared_box(declared_type.as_ref(), expected_declared_box))
    .then_some(*dst)
}

pub(super) fn copy_from(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_src: ValueId,
) -> Option<ValueId> {
    let MirInstruction::Copy { dst, src } = inst else {
        return None;
    };
    (*dst == expected_dst && *src == expected_src).then_some(*dst)
}

pub(super) fn add_result(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_lhs: ValueId,
    expected_rhs: ValueId,
) -> Option<ValueId> {
    let MirInstruction::BinOp { dst, op, lhs, rhs } = inst else {
        return None;
    };
    (*dst == expected_dst && *op == BinaryOp::Add && *lhs == expected_lhs && *rhs == expected_rhs)
        .then_some(*dst)
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

pub(super) fn op_name(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "const",
        MirInstruction::NewBox { .. } => "newbox",
        MirInstruction::FieldSet { .. } => "field_set",
        MirInstruction::FieldGet { .. } => "field_get",
        MirInstruction::Copy { .. } => "copy",
        MirInstruction::BinOp { .. } => "binop",
        MirInstruction::Return { .. } => "ret",
        _ => "other",
    }
}
