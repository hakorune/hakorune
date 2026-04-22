/*!
 * MIR-owned route plan for temporary UserBox known-receiver method exact seeds.
 *
 * Thin-entry metadata already proves the known receiver method surface and the
 * primitive field lanes. This module binds the local/copy `Counter.step/1` and
 * `Point.sum/1` exact seed shells to a backend route so the C boundary can
 * validate metadata and emit without rescanning raw MIR JSON.
 */

use super::{
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface},
    thin_entry_selection::ThinEntrySelection,
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule,
    MirType, ValueId,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxKnownReceiverMethodSeedKind {
    CounterStepLocalI64,
    CounterStepCopyLocalI64,
    PointSumLocalI64,
    PointSumCopyLocalI64,
}

impl std::fmt::Display for UserBoxKnownReceiverMethodSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CounterStepLocalI64 => f.write_str("counter_step_local_i64"),
            Self::CounterStepCopyLocalI64 => f.write_str("counter_step_copy_local_i64"),
            Self::PointSumLocalI64 => f.write_str("point_sum_local_i64"),
            Self::PointSumCopyLocalI64 => f.write_str("point_sum_copy_local_i64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxKnownReceiverMethodSeedProof {
    CounterStepLocalI64Seed,
    PointSumLocalI64Seed,
}

impl std::fmt::Display for UserBoxKnownReceiverMethodSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CounterStepLocalI64Seed => f.write_str("userbox_counter_step_local_i64_seed"),
            Self::PointSumLocalI64Seed => f.write_str("userbox_point_sum_local_i64_seed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserBoxKnownReceiverMethodSeedPayload {
    CounterStepI64 { base_i64: i64, delta_i64: i64 },
    PointSumI64 { x_i64: i64, y_i64: i64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxKnownReceiverMethodSeedRoute {
    pub kind: UserBoxKnownReceiverMethodSeedKind,
    pub box_name: String,
    pub method: String,
    pub method_function: String,
    pub block_count: usize,
    pub method_block_count: usize,
    pub block: BasicBlockId,
    pub method_block: BasicBlockId,
    pub newbox_instruction_index: usize,
    pub copy_instruction_index: Option<usize>,
    pub call_instruction_index: usize,
    pub box_value: ValueId,
    pub copy_value: Option<ValueId>,
    pub result_value: ValueId,
    pub proof: UserBoxKnownReceiverMethodSeedProof,
    pub payload: UserBoxKnownReceiverMethodSeedPayload,
}

struct CounterStepMethodFacts {
    block_count: usize,
    block: BasicBlockId,
    delta_i64: i64,
}

struct PointSumMethodFacts {
    block_count: usize,
    block: BasicBlockId,
}

pub fn refresh_module_userbox_known_receiver_method_seed_routes(module: &mut MirModule) {
    let routes: Vec<(String, Option<UserBoxKnownReceiverMethodSeedRoute>)> = module
        .functions
        .iter()
        .map(|(name, function)| {
            (
                name.clone(),
                match_userbox_known_receiver_method_seed_route(function, &module.functions),
            )
        })
        .collect();

    for (name, route) in routes {
        if let Some(function) = module.functions.get_mut(&name) {
            function.metadata.userbox_known_receiver_method_seed_route = route;
        }
    }
}

fn match_userbox_known_receiver_method_seed_route(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<UserBoxKnownReceiverMethodSeedRoute> {
    if let Some(route) = match_counter_step_local_i64(function, functions) {
        return Some(route);
    }
    if let Some(route) = match_counter_step_copy_local_i64(function, functions) {
        return Some(route);
    }
    if let Some(route) = match_point_sum_local_i64(function, functions) {
        return Some(route);
    }
    match_point_sum_copy_local_i64(function, functions)
}

fn match_counter_step_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<UserBoxKnownReceiverMethodSeedRoute> {
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

    Some(UserBoxKnownReceiverMethodSeedRoute {
        kind: UserBoxKnownReceiverMethodSeedKind::CounterStepLocalI64,
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
        payload: UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
            base_i64,
            delta_i64: method_facts.delta_i64,
        },
    })
}

fn match_counter_step_copy_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<UserBoxKnownReceiverMethodSeedRoute> {
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

    Some(UserBoxKnownReceiverMethodSeedRoute {
        kind: UserBoxKnownReceiverMethodSeedKind::CounterStepCopyLocalI64,
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
        payload: UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
            base_i64,
            delta_i64: method_facts.delta_i64,
        },
    })
}

fn match_point_sum_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<UserBoxKnownReceiverMethodSeedRoute> {
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

    Some(UserBoxKnownReceiverMethodSeedRoute {
        kind: UserBoxKnownReceiverMethodSeedKind::PointSumLocalI64,
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
        payload: UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64, y_i64 },
    })
}

fn match_point_sum_copy_local_i64(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<UserBoxKnownReceiverMethodSeedRoute> {
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

    Some(UserBoxKnownReceiverMethodSeedRoute {
        kind: UserBoxKnownReceiverMethodSeedKind::PointSumCopyLocalI64,
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
        payload: UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64, y_i64 },
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

fn find_function<'a>(
    functions: &'a BTreeMap<String, MirFunction>,
    name: &str,
) -> Option<&'a MirFunction> {
    functions.get(name).or_else(|| {
        functions
            .values()
            .find(|function| function.signature.name == name)
    })
}

fn single_block(function: &MirFunction) -> Option<&BasicBlock> {
    let blocks = ordered_blocks(function);
    (blocks.len() == 1).then_some(blocks[0])
}

fn const_i64_any(inst: &MirInstruction) -> Option<(ValueId, i64)> {
    let MirInstruction::Const {
        dst,
        value: ConstValue::Integer(value),
    } = inst
    else {
        return None;
    };
    Some((*dst, *value))
}

fn newbox_named(inst: &MirInstruction, expected_box: &str) -> Option<ValueId> {
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

fn field_set_declared(
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

fn field_get_declared(
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

fn copy_from(inst: &MirInstruction, expected_src: ValueId) -> Option<ValueId> {
    let MirInstruction::Copy { dst, src } = inst else {
        return None;
    };
    (*src == expected_src).then_some(*dst)
}

fn method_call(
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

fn add_result(
    inst: &MirInstruction,
    expected_lhs: ValueId,
    expected_rhs: ValueId,
) -> Option<ValueId> {
    let MirInstruction::BinOp { dst, op, lhs, rhs } = inst else {
        return None;
    };
    (*op == BinaryOp::Add && *lhs == expected_lhs && *rhs == expected_rhs).then_some(*dst)
}

fn return_value(inst: &MirInstruction, expected_value: ValueId) -> Option<()> {
    let MirInstruction::Return { value } = inst else {
        return None;
    };
    (*value == Some(expected_value)).then_some(())
}

fn declared_box(ty: Option<&MirType>, expected_box: &str) -> bool {
    matches!(ty, Some(MirType::Box(box_name)) if box_name == expected_box)
}

fn has_thin_selection(
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

fn ordered_blocks(function: &MirFunction) -> Vec<&BasicBlock> {
    let mut ids: Vec<BasicBlockId> = function.blocks.keys().copied().collect();
    ids.sort();
    ids.into_iter()
        .filter_map(|id| function.blocks.get(&id))
        .collect()
}

fn instructions_with_terminator(block: &BasicBlock) -> Option<Vec<&MirInstruction>> {
    let mut insts: Vec<&MirInstruction> = block.instructions.iter().collect();
    insts.push(block.terminator.as_ref()?);
    Some(insts)
}

fn expect_ops(insts: &[&MirInstruction], expected: &[&str]) -> Option<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        definitions::call_unified::{CalleeBoxKind, TypeCertainty},
        thin_entry::{ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryValueClass},
        thin_entry_selection::ThinEntrySelectionState,
        EffectMask, FunctionSignature,
    };

    fn make_function(name: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn make_method(name: &str) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: vec![MirType::Box("Self".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(1),
        );
        function.params = vec![ValueId::new(0)];
        function
    }

    fn add_counter_step_main(function: &mut MirFunction, copy: bool) {
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(const_i(1, 41));
        block.add_instruction(newbox(2, "Counter"));
        block.add_instruction(field_set(2, "value", 1, "IntegerBox"));
        let receiver = if copy {
            block.add_instruction(MirInstruction::Copy {
                dst: ValueId::new(3),
                src: ValueId::new(2),
            });
            ValueId::new(3)
        } else {
            ValueId::new(2)
        };
        let result = if copy { 4 } else { 3 };
        block.add_instruction(method_call_inst(result, "Counter", "step", receiver));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(result)),
        });
        function.metadata.thin_entry_selections = vec![
            selection(
                0,
                2,
                None,
                ThinEntrySurface::UserBoxFieldSet,
                "Counter.value",
                "user_box_field_set.inline_scalar",
                ThinEntryValueClass::InlineI64,
            ),
            selection(
                0,
                if copy { 4 } else { 3 },
                Some(result),
                ThinEntrySurface::UserBoxMethod,
                "Counter.step",
                "user_box_method.known_receiver",
                ThinEntryValueClass::Unknown,
            ),
        ];
    }

    fn counter_step_method() -> MirFunction {
        let mut function = make_method("Counter.step/1");
        let block = function.get_block_mut(BasicBlockId::new(1)).unwrap();
        block.add_instruction(field_get(1, 0, "value", "IntegerBox"));
        block.add_instruction(const_i(2, 2));
        block.add_instruction(binop(3, 1, 2));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        function.metadata.thin_entry_selections = vec![selection(
            1,
            0,
            Some(1),
            ThinEntrySurface::UserBoxFieldGet,
            "Counter.value",
            "user_box_field_get.inline_scalar",
            ThinEntryValueClass::InlineI64,
        )];
        function
    }

    fn add_point_sum_main(function: &mut MirFunction, copy: bool) {
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(const_i(1, 1));
        block.add_instruction(const_i(2, 2));
        block.add_instruction(newbox(3, "Point"));
        block.add_instruction(field_set(3, "x", 1, "IntegerBox"));
        block.add_instruction(field_set(3, "y", 2, "IntegerBox"));
        let receiver = if copy {
            block.add_instruction(MirInstruction::Copy {
                dst: ValueId::new(4),
                src: ValueId::new(3),
            });
            ValueId::new(4)
        } else {
            ValueId::new(3)
        };
        let result = if copy { 5 } else { 4 };
        block.add_instruction(method_call_inst(result, "Point", "sum", receiver));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(result)),
        });
        function.metadata.thin_entry_selections = vec![
            selection(
                0,
                3,
                None,
                ThinEntrySurface::UserBoxFieldSet,
                "Point.x",
                "user_box_field_set.inline_scalar",
                ThinEntryValueClass::InlineI64,
            ),
            selection(
                0,
                4,
                None,
                ThinEntrySurface::UserBoxFieldSet,
                "Point.y",
                "user_box_field_set.inline_scalar",
                ThinEntryValueClass::InlineI64,
            ),
            selection(
                0,
                if copy { 6 } else { 5 },
                Some(result),
                ThinEntrySurface::UserBoxMethod,
                "Point.sum",
                "user_box_method.known_receiver",
                ThinEntryValueClass::Unknown,
            ),
        ];
    }

    fn point_sum_method() -> MirFunction {
        let mut function = make_method("Point.sum/1");
        let block = function.get_block_mut(BasicBlockId::new(1)).unwrap();
        block.add_instruction(field_get(1, 0, "x", "IntegerBox"));
        block.add_instruction(field_get(2, 0, "y", "IntegerBox"));
        block.add_instruction(binop(3, 1, 2));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        function.metadata.thin_entry_selections = vec![
            selection(
                1,
                0,
                Some(1),
                ThinEntrySurface::UserBoxFieldGet,
                "Point.x",
                "user_box_field_get.inline_scalar",
                ThinEntryValueClass::InlineI64,
            ),
            selection(
                1,
                1,
                Some(2),
                ThinEntrySurface::UserBoxFieldGet,
                "Point.y",
                "user_box_field_get.inline_scalar",
                ThinEntryValueClass::InlineI64,
            ),
        ];
        function
    }

    #[test]
    fn userbox_known_receiver_method_seed_detects_counter_step_local_and_copy() {
        for copy in [false, true] {
            let mut module = MirModule::new("counter_step_route_test".to_string());
            let mut main = make_function("main");
            add_counter_step_main(&mut main, copy);
            module.add_function(main);
            module.add_function(counter_step_method());

            refresh_module_userbox_known_receiver_method_seed_routes(&mut module);
            let route = module
                .functions
                .get("main")
                .and_then(|function| {
                    function
                        .metadata
                        .userbox_known_receiver_method_seed_route
                        .as_ref()
                })
                .expect("counter step route");

            assert_eq!(route.box_name, "Counter");
            assert_eq!(route.method, "step");
            assert_eq!(route.copy_value.is_some(), copy);
            assert_eq!(
                route.kind,
                if copy {
                    UserBoxKnownReceiverMethodSeedKind::CounterStepCopyLocalI64
                } else {
                    UserBoxKnownReceiverMethodSeedKind::CounterStepLocalI64
                }
            );
            assert_eq!(
                route.payload,
                UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
                    base_i64: 41,
                    delta_i64: 2
                }
            );
        }
    }

    #[test]
    fn userbox_known_receiver_method_seed_detects_point_sum_local_and_copy() {
        for copy in [false, true] {
            let mut module = MirModule::new("point_sum_route_test".to_string());
            let mut main = make_function("main");
            add_point_sum_main(&mut main, copy);
            module.add_function(main);
            module.add_function(point_sum_method());

            refresh_module_userbox_known_receiver_method_seed_routes(&mut module);
            let route = module
                .functions
                .get("main")
                .and_then(|function| {
                    function
                        .metadata
                        .userbox_known_receiver_method_seed_route
                        .as_ref()
                })
                .expect("point sum route");

            assert_eq!(route.box_name, "Point");
            assert_eq!(route.method, "sum");
            assert_eq!(route.copy_value.is_some(), copy);
            assert_eq!(
                route.kind,
                if copy {
                    UserBoxKnownReceiverMethodSeedKind::PointSumCopyLocalI64
                } else {
                    UserBoxKnownReceiverMethodSeedKind::PointSumLocalI64
                }
            );
            assert_eq!(
                route.payload,
                UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64: 1, y_i64: 2 }
            );
        }
    }

    #[test]
    fn userbox_known_receiver_method_seed_requires_thin_method_selection() {
        let mut module = MirModule::new("counter_step_route_negative_test".to_string());
        let mut main = make_function("main");
        add_counter_step_main(&mut main, false);
        main.metadata
            .thin_entry_selections
            .retain(|selection| selection.surface != ThinEntrySurface::UserBoxMethod);
        module.add_function(main);
        module.add_function(counter_step_method());

        refresh_module_userbox_known_receiver_method_seed_routes(&mut module);
        assert!(module
            .functions
            .get("main")
            .unwrap()
            .metadata
            .userbox_known_receiver_method_seed_route
            .is_none());
    }

    fn const_i(dst: u32, value: i64) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::Integer(value),
        }
    }

    fn newbox(dst: u32, box_type: &str) -> MirInstruction {
        MirInstruction::NewBox {
            dst: ValueId::new(dst),
            box_type: box_type.to_string(),
            args: vec![],
        }
    }

    fn field_set(base: u32, field: &str, value: u32, declared_box: &str) -> MirInstruction {
        MirInstruction::FieldSet {
            base: ValueId::new(base),
            field: field.to_string(),
            value: ValueId::new(value),
            declared_type: Some(MirType::Box(declared_box.to_string())),
        }
    }

    fn field_get(dst: u32, base: u32, field: &str, declared_box: &str) -> MirInstruction {
        MirInstruction::FieldGet {
            dst: ValueId::new(dst),
            base: ValueId::new(base),
            field: field.to_string(),
            declared_type: Some(MirType::Box(declared_box.to_string())),
        }
    }

    fn binop(dst: u32, lhs: u32, rhs: u32) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op: BinaryOp::Add,
            lhs: ValueId::new(lhs),
            rhs: ValueId::new(rhs),
        }
    }

    fn method_call_inst(
        dst: u32,
        box_name: &str,
        method: &str,
        receiver: ValueId,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(ValueId::new(dst)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        }
    }

    fn selection(
        block: u32,
        instruction_index: usize,
        value: Option<u32>,
        surface: ThinEntrySurface,
        subject: &str,
        manifest_row: &'static str,
        value_class: ThinEntryValueClass,
    ) -> ThinEntrySelection {
        ThinEntrySelection {
            block: BasicBlockId::new(block),
            instruction_index,
            value: value.map(ValueId::new),
            surface,
            subject: subject.to_string(),
            manifest_row,
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: ThinEntryCurrentCarrier::BackendTyped,
            value_class,
            demand: ThinEntryDemand::InlineScalar,
            reason: "test".to_string(),
        }
    }
}
