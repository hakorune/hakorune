/*!
 * MIR-owned backend route plans for the current array/text loopcarry lane.
 *
 * This module is the route SSOT for the active H21 loopcarry len-store slice.
 * It recognizes the fused window in MIR, then exposes only a thin
 * backend-consumable plan. C shims may consume this plan to emit/skip, but they
 * must not grow new legality for this route.
 */

use std::collections::HashMap;

use super::string_corridor_recognizer::{
    match_substring_call, match_substring_concat3_helper_call,
};
use super::{
    build_value_def_map, definitions::Callee, resolve_value_origin, BasicBlock, BasicBlockId,
    BinaryOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextLoopCarryLenStoreProof {
    InsertMidSubrangeTrailingLen,
}

impl ArrayTextLoopCarryLenStoreProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::InsertMidSubrangeTrailingLen => "insert_mid_subrange_trailing_len",
        }
    }
}

impl std::fmt::Display for ArrayTextLoopCarryLenStoreProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextLoopCarryLenStoreRoute {
    block: BasicBlockId,
    instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
    substring_value: ValueId,
    result_len_value: ValueId,
    middle_value: ValueId,
    middle_length: i64,
    skip_instruction_indices: Vec<usize>,
    proof: ArrayTextLoopCarryLenStoreProof,
}

impl ArrayTextLoopCarryLenStoreRoute {
    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn source_value(&self) -> ValueId {
        self.source_value
    }

    pub fn substring_value(&self) -> ValueId {
        self.substring_value
    }

    pub fn result_len_value(&self) -> ValueId {
        self.result_len_value
    }

    pub fn middle_value(&self) -> ValueId {
        self.middle_value
    }

    pub fn middle_length(&self) -> i64 {
        self.middle_length
    }

    pub fn skip_instruction_indices(&self) -> &[usize] {
        &self.skip_instruction_indices
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn covered_instruction_indices(&self) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(self.instruction_index).chain(self.skip_instruction_indices.iter().copied())
    }
}

pub fn refresh_module_array_text_loopcarry_len_store_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_loopcarry_len_store_routes(function);
    }
}

pub fn refresh_function_array_text_loopcarry_len_store_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some((array_value, index_value, source_value)) = match_array_text_get(inst) else {
                continue;
            };
            if let Some(route) = match_insert_mid_subrange_trailing_len_route(
                function,
                &def_map,
                block,
                block_id,
                instruction_index,
                array_value,
                index_value,
                source_value,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.array_text_loopcarry_len_store_routes = routes;
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn same_root(function: &MirFunction, def_map: &ValueDefMap, lhs: ValueId, rhs: ValueId) -> bool {
    root(function, def_map, lhs) == root(function, def_map, rhs)
}

fn match_array_text_get(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    match inst {
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
        } if args.len() == 1
            && method == "get"
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            Some((*receiver, args[0], *dst))
        }
        _ => None,
    }
}

fn match_const_i64(inst: &MirInstruction, expected: i64) -> Option<ValueId> {
    match inst {
        MirInstruction::Const {
            dst,
            value: ConstValue::Integer(actual),
        } if *actual == expected => Some(*dst),
        _ => None,
    }
}

fn match_const_string(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<i64> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some(text.len() as i64),
        _ => None,
    }
}

fn match_binop(
    inst: &MirInstruction,
    op: BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
    function: &MirFunction,
    def_map: &ValueDefMap,
) -> Option<ValueId> {
    match inst {
        MirInstruction::BinOp {
            dst,
            op: actual,
            lhs: actual_lhs,
            rhs: actual_rhs,
        } if *actual == op
            && same_root(function, def_map, *actual_lhs, lhs)
            && same_root(function, def_map, *actual_rhs, rhs) =>
        {
            Some(*dst)
        }
        _ => None,
    }
}

fn match_extern_call<'a>(
    inst: &'a MirInstruction,
    expected_name: &str,
) -> Option<(ValueId, &'a [ValueId])> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if name == expected_name => Some((*dst, args.as_slice())),
        _ => None,
    }
}

fn match_set_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if args.len() == 2
            && method == "set"
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            Some((*receiver, args[0], args[1]))
        }
        _ => None,
    }
}

fn match_len_call(inst: &MirInstruction) -> Option<(ValueId, ValueId)> {
    super::string_corridor_recognizer::match_len_call(inst)
        .map(|(dst, receiver, _)| (dst, receiver))
}

fn skip_copy_chain(
    function: &MirFunction,
    def_map: &ValueDefMap,
    instructions: &[MirInstruction],
    mut index: usize,
    mut carried: ValueId,
    skip: &mut Vec<usize>,
) -> (usize, ValueId) {
    while let Some(MirInstruction::Copy { dst, src }) = instructions.get(index) {
        if !same_root(function, def_map, *src, carried) {
            break;
        }
        skip.push(index);
        carried = *dst;
        index += 1;
    }
    (index, carried)
}

fn has_later_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    instructions: &[MirInstruction],
    from_index: usize,
    consumed_values: &[ValueId],
) -> bool {
    let consumed_roots: Vec<ValueId> = consumed_values
        .iter()
        .copied()
        .map(|value| root(function, def_map, value))
        .collect();
    instructions.iter().skip(from_index).any(|inst| {
        inst.used_values().into_iter().any(|value| {
            let value_root = root(function, def_map, value);
            consumed_roots
                .iter()
                .any(|consumed| *consumed == value_root)
        })
    })
}

#[allow(clippy::too_many_arguments)]
fn match_insert_mid_subrange_trailing_len_route(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    block: &BasicBlock,
    block_id: BasicBlockId,
    instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
) -> Option<ArrayTextLoopCarryLenStoreRoute> {
    if let Some(route) = match_substring_concat3_direct_set_trailing_len_route(
        function,
        def_map,
        block,
        block_id,
        instruction_index,
        array_value,
        index_value,
        source_value,
    ) {
        return Some(route);
    }

    let instructions = block.instructions.as_slice();
    let mut skip = Vec::new();
    let (mut cursor, carried) = skip_copy_chain(
        function,
        def_map,
        instructions,
        instruction_index + 1,
        source_value,
        &mut skip,
    );

    let (len_value, len_recv) = match_len_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, len_recv, carried) {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, len_final) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        len_value,
        &mut skip,
    );
    cursor = next_cursor;

    let const_two = match_const_i64(instructions.get(cursor)?, 2)?;
    skip.push(cursor);
    cursor += 1;

    let split = match_binop(
        instructions.get(cursor)?,
        BinaryOp::Div,
        len_final,
        const_two,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let start = match_const_i64(instructions.get(cursor)?, 1)?;
    skip.push(cursor);
    cursor += 1;

    let const_one = match_const_i64(instructions.get(cursor)?, 1)?;
    skip.push(cursor);
    cursor += 1;

    let end = match_binop(
        instructions.get(cursor)?,
        BinaryOp::Add,
        len_final,
        const_one,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let middle_value = match instructions.get(cursor)? {
        MirInstruction::Const { dst, .. } => *dst,
        _ => return None,
    };
    let middle_length = match_const_string(function, def_map, middle_value)?;
    skip.push(cursor);
    cursor += 1;

    let (insert_value, insert_args) =
        match_extern_call(instructions.get(cursor)?, "nyash.string.insert_hsi")?;
    if insert_args.len() != 3
        || !same_root(function, def_map, insert_args[0], source_value)
        || !same_root(function, def_map, insert_args[1], middle_value)
        || !same_root(function, def_map, insert_args[2], split)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (substring_value, substring_args) =
        match_extern_call(instructions.get(cursor)?, "nyash.string.substring_hii")?;
    if substring_args.len() != 3
        || !same_root(function, def_map, substring_args[0], insert_value)
        || !same_root(function, def_map, substring_args[1], start)
        || !same_root(function, def_map, substring_args[2], end)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, set_value) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        substring_value,
        &mut skip,
    );
    cursor = next_cursor;

    let (set_array, set_index, set_arg_value) = match_set_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, set_array, array_value)
        || !same_root(function, def_map, set_index, index_value)
        || !same_root(function, def_map, set_arg_value, set_value)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, trailing_recv) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        substring_value,
        &mut skip,
    );
    cursor = next_cursor;

    let (result_len_value, len_recv) = match_len_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, len_recv, trailing_recv) {
        return None;
    }
    skip.push(cursor);

    let consumed = [
        source_value,
        carried,
        len_value,
        len_final,
        const_two,
        split,
        start,
        const_one,
        end,
        middle_value,
        insert_value,
        substring_value,
        set_value,
        trailing_recv,
    ];
    if has_later_use(function, def_map, instructions, cursor + 1, &consumed) {
        return None;
    }

    Some(ArrayTextLoopCarryLenStoreRoute {
        block: block_id,
        instruction_index,
        array_value,
        index_value,
        source_value,
        substring_value,
        result_len_value,
        middle_value,
        middle_length,
        skip_instruction_indices: skip,
        proof: ArrayTextLoopCarryLenStoreProof::InsertMidSubrangeTrailingLen,
    })
}

#[allow(clippy::too_many_arguments)]
fn match_substring_concat3_direct_set_trailing_len_route(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    block: &BasicBlock,
    block_id: BasicBlockId,
    instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
) -> Option<ArrayTextLoopCarryLenStoreRoute> {
    let instructions = block.instructions.as_slice();
    let mut skip = Vec::new();
    let (mut cursor, carried) = skip_copy_chain(
        function,
        def_map,
        instructions,
        instruction_index + 1,
        source_value,
        &mut skip,
    );

    let (len_value, len_recv) = match_len_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, len_recv, carried) {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, len_final) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        len_value,
        &mut skip,
    );
    cursor = next_cursor;

    let left_start = match_const_i64(instructions.get(cursor)?, 0)?;
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, len_for_split) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        len_final,
        &mut skip,
    );
    cursor = next_cursor;

    let const_two = match_const_i64(instructions.get(cursor)?, 2)?;
    skip.push(cursor);
    cursor += 1;

    let split = match_binop(
        instructions.get(cursor)?,
        BinaryOp::Div,
        len_for_split,
        const_two,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let (left_value, left_source, left_arg_start, left_arg_end, _) =
        match_substring_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, left_source, carried)
        || !same_root(function, def_map, left_arg_start, left_start)
        || !same_root(function, def_map, left_arg_end, split)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (right_value, right_source, right_arg_start, right_arg_end, _) =
        match_substring_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, right_source, carried)
        || !same_root(function, def_map, right_arg_start, split)
        || !same_root(function, def_map, right_arg_end, len_final)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let trailing_start = match_const_i64(instructions.get(cursor)?, 1)?;
    skip.push(cursor);
    cursor += 1;

    let const_one = match_const_i64(instructions.get(cursor)?, 1)?;
    skip.push(cursor);
    cursor += 1;

    let trailing_end = match_binop(
        instructions.get(cursor)?,
        BinaryOp::Add,
        len_for_split,
        const_one,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let middle_value = match instructions.get(cursor)? {
        MirInstruction::Const { dst, .. } => *dst,
        _ => return None,
    };
    let middle_length = match_const_string(function, def_map, middle_value)?;
    skip.push(cursor);
    cursor += 1;

    let concat3 = match_substring_concat3_helper_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, concat3.left, left_value)
        || !same_root(function, def_map, concat3.middle, middle_value)
        || !same_root(function, def_map, concat3.right, right_value)
        || !same_root(function, def_map, concat3.start, trailing_start)
        || !same_root(function, def_map, concat3.end, trailing_end)
    {
        return None;
    }
    let substring_value = concat3.dst;
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, set_value) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        substring_value,
        &mut skip,
    );
    cursor = next_cursor;

    let (set_array, set_index, set_arg_value) = match_set_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, set_array, array_value)
        || !same_root(function, def_map, set_index, index_value)
        || !same_root(function, def_map, set_arg_value, set_value)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, _trailing_value) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        substring_value,
        &mut skip,
    );
    cursor = next_cursor;

    let result_len_value = match_binop(
        instructions.get(cursor)?,
        BinaryOp::Sub,
        trailing_end,
        trailing_start,
        function,
        def_map,
    )?;
    skip.push(cursor);

    let consumed = [
        source_value,
        carried,
        len_value,
        len_final,
        left_start,
        len_for_split,
        const_two,
        split,
        left_value,
        right_value,
        trailing_start,
        const_one,
        trailing_end,
        middle_value,
        substring_value,
        set_value,
    ];
    if has_later_use(function, def_map, instructions, cursor + 1, &consumed) {
        return None;
    }

    Some(ArrayTextLoopCarryLenStoreRoute {
        block: block_id,
        instruction_index,
        array_value,
        index_value,
        source_value,
        substring_value,
        result_len_value,
        middle_value,
        middle_length,
        skip_instruction_indices: skip,
        proof: ArrayTextLoopCarryLenStoreProof::InsertMidSubrangeTrailingLen,
    })
}
