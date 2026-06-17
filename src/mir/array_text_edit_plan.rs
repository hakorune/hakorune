/*!
 * MIR-owned array/text edit routes.
 *
 * This module owns the current array/text edit contracts:
 * - same-slot `array.get(i).length() -> split = len / 2 -> insert-mid const`
 * - cross-array len-only store evidence where `src.get(i)` feeds `dst.set(i, out)`
 *   and only the resulting length is observed.
 * Backends may consume this metadata to select a helper call and skip the
 * covered instructions, but they must not rediscover edit legality from raw
 * MIR JSON.
 */

use std::collections::HashMap;

use super::string_corridor_recognizer::{match_len_call, match_substring_call};
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{
    definitions::Callee, BasicBlock, BasicBlockId, BinaryOp, ConstValue, MirFunction,
    MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextEditKind {
    InsertMidConst,
}

impl ArrayTextEditKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::InsertMidConst => "insert_mid_const",
        }
    }
}

impl std::fmt::Display for ArrayTextEditKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextEditSplitPolicy {
    SourceLenDivConst { divisor: i64 },
}

impl std::fmt::Display for ArrayTextEditSplitPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceLenDivConst { divisor } => {
                write!(f, "source_len_div_const({divisor})")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextEditProof {
    ArrayGetLenHalfInsertMidSameSlot,
    ArrayGetLenHalfInsertMidDestSlotLenOnly,
}

impl ArrayTextEditProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArrayGetLenHalfInsertMidSameSlot => "array_get_lenhalf_insert_mid_same_slot",
            Self::ArrayGetLenHalfInsertMidDestSlotLenOnly => {
                "array_get_lenhalf_insert_mid_dest_slot_len_only"
            }
        }
    }
}

impl std::fmt::Display for ArrayTextEditProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextEditRoute {
    block: BasicBlockId,
    get_instruction_index: usize,
    set_instruction_index: usize,
    array_value: ValueId,
    destination_array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
    length_value: ValueId,
    split_value: ValueId,
    result_value: ValueId,
    result_len_value: Option<ValueId>,
    middle_value: ValueId,
    middle_text: String,
    middle_byte_len: usize,
    skip_instruction_indices: Vec<usize>,
    edit_kind: ArrayTextEditKind,
    split_policy: ArrayTextEditSplitPolicy,
    proof: ArrayTextEditProof,
}

impl ArrayTextEditRoute {
    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn get_instruction_index(&self) -> usize {
        self.get_instruction_index
    }

    pub fn set_instruction_index(&self) -> usize {
        self.set_instruction_index
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn destination_array_value(&self) -> ValueId {
        self.destination_array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn source_value(&self) -> ValueId {
        self.source_value
    }

    pub fn length_value(&self) -> ValueId {
        self.length_value
    }

    pub fn split_value(&self) -> ValueId {
        self.split_value
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn result_len_value(&self) -> Option<ValueId> {
        self.result_len_value
    }

    pub fn middle_value(&self) -> ValueId {
        self.middle_value
    }

    pub fn middle_text(&self) -> &str {
        &self.middle_text
    }

    pub fn middle_byte_len(&self) -> usize {
        self.middle_byte_len
    }

    pub fn skip_instruction_indices(&self) -> &[usize] {
        &self.skip_instruction_indices
    }

    pub fn edit_kind(&self) -> &'static str {
        self.edit_kind.as_str()
    }

    pub fn split_policy(&self) -> String {
        self.split_policy.to_string()
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn is_lenhalf_insert_mid_same_slot(&self) -> bool {
        self.edit_kind == ArrayTextEditKind::InsertMidConst
            && self.split_policy == (ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 })
            && self.proof == ArrayTextEditProof::ArrayGetLenHalfInsertMidSameSlot
    }

    pub fn is_lenhalf_insert_mid_dest_slot_len_only(&self) -> bool {
        self.edit_kind == ArrayTextEditKind::InsertMidConst
            && self.split_policy == (ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 })
            && self.proof == ArrayTextEditProof::ArrayGetLenHalfInsertMidDestSlotLenOnly
    }
}

pub fn refresh_module_array_text_edit_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_edit_routes(function);
    }
}

pub fn refresh_function_array_text_edit_routes(function: &mut MirFunction) {
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
            if let Some(route) = match_lenhalf_insert_mid_same_slot_route(
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
                continue;
            }
            if let Some(route) = match_lenhalf_insert_mid_dest_slot_len_only_route(
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

    routes.sort_by_key(|route| (route.block.as_u32(), route.get_instruction_index));
    function.metadata.array_text_edit_routes = routes;
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
) -> Option<String> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some(text.clone()),
        _ => None,
    }
}

fn match_string_value_inst(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
) -> Option<(ValueId, String)> {
    let value = match inst {
        MirInstruction::Const {
            dst,
            value: ConstValue::String(_),
        } => *dst,
        MirInstruction::Copy { dst, .. } => *dst,
        _ => return None,
    };
    let text = match_const_string(function, def_map, value)?;
    Some((value, text))
}

fn advance_to_string_value_inst(
    function: &MirFunction,
    def_map: &ValueDefMap,
    instructions: &[MirInstruction],
    mut cursor: usize,
    values: &mut [ValueId],
    skip: &mut Vec<usize>,
) -> Option<(usize, ValueId, String)> {
    while let Some(inst) = instructions.get(cursor) {
        if let Some((value, text)) = match_string_value_inst(function, def_map, inst) {
            return Some((cursor, value, text));
        }
        match inst {
            MirInstruction::Copy { dst, src } => {
                if let Some(slot) = values
                    .iter()
                    .position(|value| same_root(function, def_map, *src, *value))
                {
                    skip.push(cursor);
                    values[slot] = *dst;
                }
                cursor += 1;
            }
            _ => return None,
        }
    }
    None
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

fn match_add_either(
    inst: &MirInstruction,
    lhs: ValueId,
    rhs: ValueId,
    function: &MirFunction,
    def_map: &ValueDefMap,
) -> Option<ValueId> {
    match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs: actual_lhs,
            rhs: actual_rhs,
        } if (same_root(function, def_map, *actual_lhs, lhs)
            && same_root(function, def_map, *actual_rhs, rhs))
            || (same_root(function, def_map, *actual_lhs, rhs)
                && same_root(function, def_map, *actual_rhs, lhs)) =>
        {
            Some(*dst)
        }
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

fn match_len_plus_middle(
    inst: &MirInstruction,
    length: ValueId,
    middle_len: i64,
    function: &MirFunction,
    def_map: &ValueDefMap,
) -> Option<ValueId> {
    let MirInstruction::BinOp {
        dst,
        op: BinaryOp::Add,
        lhs,
        rhs,
    } = inst
    else {
        return None;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    let length_root = root(function, def_map, length);
    let middle_const = |value| {
        let (block, index) = def_map.get(&root(function, def_map, value)).copied()?;
        match function.blocks.get(&block)?.instructions.get(index)? {
            MirInstruction::Const {
                value: ConstValue::Integer(actual),
                ..
            } if *actual == middle_len => Some(()),
            _ => None,
        }
    };

    if lhs_root == length_root && middle_const(*rhs).is_some()
        || rhs_root == length_root && middle_const(*lhs).is_some()
    {
        Some(*dst)
    } else {
        None
    }
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

fn skip_interleaved_copies(
    function: &MirFunction,
    def_map: &ValueDefMap,
    instructions: &[MirInstruction],
    mut index: usize,
    values: &mut [ValueId],
    skip: &mut Vec<usize>,
) -> usize {
    while let Some(MirInstruction::Copy { dst, src }) = instructions.get(index) {
        let Some(slot) = values
            .iter()
            .position(|value| same_root(function, def_map, *src, *value))
        else {
            break;
        };
        skip.push(index);
        values[slot] = *dst;
        index += 1;
    }
    index
}

fn has_uncovered_use(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    covered_block: BasicBlockId,
    covered_until_index: usize,
    consumed_values: &[ValueId],
) -> bool {
    let consumed_roots: Vec<ValueId> = consumed_values
        .iter()
        .copied()
        .map(|value| root(function, def_map, value))
        .collect();

    function.blocks.iter().any(|(block_id, block)| {
        block
            .instructions
            .iter()
            .enumerate()
            .filter(|(index, _)| !(*block_id == covered_block && *index <= covered_until_index))
            .any(|(_, inst)| {
                inst.used_values().into_iter().any(|value| {
                    let value_root = root(function, def_map, value);
                    consumed_roots
                        .iter()
                        .any(|consumed| *consumed == value_root)
                })
            })
    })
}

struct LenHalfInsertMidMatch {
    cursor: usize,
    skip: Vec<usize>,
    carried: ValueId,
    length_value: ValueId,
    length_final: ValueId,
    length_for_split: ValueId,
    divisor_value: ValueId,
    split_value: ValueId,
    left_value: ValueId,
    right_value: ValueId,
    left_final: ValueId,
    right_final: ValueId,
    middle_value: ValueId,
    middle_text: String,
    pair_value: ValueId,
    pair_final: ValueId,
    result_value: ValueId,
}

impl LenHalfInsertMidMatch {
    fn base_consumed_values(&self, source_value: ValueId) -> [ValueId; 15] {
        [
            source_value,
            self.carried,
            self.length_value,
            self.length_final,
            self.length_for_split,
            self.divisor_value,
            self.split_value,
            self.left_value,
            self.right_value,
            self.left_final,
            self.right_final,
            self.middle_value,
            self.pair_value,
            self.pair_final,
            self.result_value,
        ]
    }
}

fn match_lenhalf_insert_mid_prefix(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    block: &BasicBlock,
    get_instruction_index: usize,
    source_value: ValueId,
) -> Option<LenHalfInsertMidMatch> {
    let instructions = block.instructions.as_slice();
    let mut skip = Vec::new();
    let (mut cursor, carried) = skip_copy_chain(
        function,
        def_map,
        instructions,
        get_instruction_index + 1,
        source_value,
        &mut skip,
    );

    let (length_value, length_receiver, _) = match_len_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, length_receiver, carried) {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, length_final) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        length_value,
        &mut skip,
    );
    cursor = next_cursor;

    let left_start = match_const_i64(instructions.get(cursor)?, 0)?;
    skip.push(cursor);
    cursor += 1;

    let (next_cursor, length_for_split) = skip_copy_chain(
        function,
        def_map,
        instructions,
        cursor,
        length_final,
        &mut skip,
    );
    cursor = next_cursor;

    let divisor_value = match_const_i64(instructions.get(cursor)?, 2)?;
    skip.push(cursor);
    cursor += 1;

    let split_value = match_binop(
        instructions.get(cursor)?,
        BinaryOp::Div,
        length_for_split,
        divisor_value,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let (left_value, left_source, left_arg_start, left_arg_end, _) =
        match_substring_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, left_source, carried)
        || !same_root(function, def_map, left_arg_start, left_start)
        || !same_root(function, def_map, left_arg_end, split_value)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let (right_value, right_source, right_arg_start, right_arg_end, _) =
        match_substring_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, right_source, carried)
        || !same_root(function, def_map, right_arg_start, split_value)
        || !same_root(function, def_map, right_arg_end, length_final)
    {
        return None;
    }
    skip.push(cursor);
    cursor += 1;

    let mut concat_values = [left_value, right_value];
    cursor = skip_interleaved_copies(
        function,
        def_map,
        instructions,
        cursor,
        &mut concat_values,
        &mut skip,
    );
    let [left_final, right_final] = concat_values;

    let mut source_values = [left_final, right_final];
    let (middle_cursor, middle_value, middle_text) = advance_to_string_value_inst(
        function,
        def_map,
        instructions,
        cursor,
        &mut source_values,
        &mut skip,
    )?;
    let [left_final, right_final] = source_values;
    cursor = middle_cursor;
    skip.push(cursor);
    cursor += 1;

    let mut final_values = [left_final, right_final, middle_value];
    cursor = skip_interleaved_copies(
        function,
        def_map,
        instructions,
        cursor,
        &mut final_values,
        &mut skip,
    );
    let [left_final, right_final, middle_final] = final_values;

    let pair_value = match_add_either(
        instructions.get(cursor)?,
        left_final,
        middle_final,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let mut final_values = [pair_value, right_final];
    cursor = skip_interleaved_copies(
        function,
        def_map,
        instructions,
        cursor,
        &mut final_values,
        &mut skip,
    );
    let [pair_final, right_final] = final_values;

    let result_value = match_add_either(
        instructions.get(cursor)?,
        pair_final,
        right_final,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    Some(LenHalfInsertMidMatch {
        cursor,
        skip,
        carried,
        length_value,
        length_final,
        length_for_split,
        divisor_value,
        split_value,
        left_value,
        right_value,
        left_final,
        right_final,
        middle_value,
        middle_text,
        pair_value,
        pair_final,
        result_value,
    })
}

#[allow(clippy::too_many_arguments)]
fn match_lenhalf_insert_mid_same_slot_route(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    block: &BasicBlock,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
) -> Option<ArrayTextEditRoute> {
    let instructions = block.instructions.as_slice();
    let mut matched =
        match_lenhalf_insert_mid_prefix(function, def_map, block, get_instruction_index, source_value)?;
    let (set_array, set_index, set_value) = match_set_call(instructions.get(matched.cursor)?)?;
    if !same_root(function, def_map, set_array, array_value)
        || !same_root(function, def_map, set_index, index_value)
        || !same_root(function, def_map, set_value, matched.result_value)
    {
        return None;
    }
    let set_instruction_index = matched.cursor;
    matched.skip.push(matched.cursor);

    let consumed = matched.base_consumed_values(source_value);
    if has_uncovered_use(function, def_map, block_id, matched.cursor, &consumed) {
        return None;
    }

    Some(ArrayTextEditRoute {
        block: block_id,
        get_instruction_index,
        set_instruction_index,
        array_value: root(function, def_map, array_value),
        destination_array_value: root(function, def_map, array_value),
        index_value: root(function, def_map, index_value),
        source_value: root(function, def_map, source_value),
        length_value: matched.length_value,
        split_value: matched.split_value,
        result_value: matched.result_value,
        result_len_value: None,
        middle_byte_len: matched.middle_text.len(),
        middle_value: matched.middle_value,
        middle_text: matched.middle_text,
        skip_instruction_indices: matched.skip,
        edit_kind: ArrayTextEditKind::InsertMidConst,
        split_policy: ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 },
        proof: ArrayTextEditProof::ArrayGetLenHalfInsertMidSameSlot,
    })
}

#[allow(clippy::too_many_arguments)]
fn match_lenhalf_insert_mid_dest_slot_len_only_route(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    block: &BasicBlock,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
) -> Option<ArrayTextEditRoute> {
    let instructions = block.instructions.as_slice();
    let mut matched =
        match_lenhalf_insert_mid_prefix(function, def_map, block, get_instruction_index, source_value)?;

    let (set_array, set_index, set_value) = match_set_call(instructions.get(matched.cursor)?)?;
    if same_root(function, def_map, set_array, array_value)
        || !same_root(function, def_map, set_index, index_value)
        || !same_root(function, def_map, set_value, matched.result_value)
    {
        return None;
    }
    let set_instruction_index = matched.cursor;
    matched.skip.push(matched.cursor);
    matched.cursor += 1;

    let middle_len_value =
        match_const_i64(instructions.get(matched.cursor)?, matched.middle_text.len() as i64)?;
    matched.skip.push(matched.cursor);
    matched.cursor += 1;

    let result_len_value = match_len_plus_middle(
        instructions.get(matched.cursor)?,
        matched.length_final,
        matched.middle_text.len() as i64,
        function,
        def_map,
    )?;
    matched.skip.push(matched.cursor);

    let mut consumed = matched.base_consumed_values(source_value).to_vec();
    consumed.push(middle_len_value);
    if has_uncovered_use(function, def_map, block_id, matched.cursor, &consumed) {
        return None;
    }

    Some(ArrayTextEditRoute {
        block: block_id,
        get_instruction_index,
        set_instruction_index,
        array_value: root(function, def_map, array_value),
        destination_array_value: root(function, def_map, set_array),
        index_value: root(function, def_map, index_value),
        source_value: root(function, def_map, source_value),
        length_value: matched.length_value,
        split_value: matched.split_value,
        result_value: matched.result_value,
        result_len_value: Some(result_len_value),
        middle_byte_len: matched.middle_text.len(),
        middle_value: matched.middle_value,
        middle_text: matched.middle_text,
        skip_instruction_indices: matched.skip,
        edit_kind: ArrayTextEditKind::InsertMidConst,
        split_policy: ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 },
        proof: ArrayTextEditProof::ArrayGetLenHalfInsertMidDestSlotLenOnly,
    })
}

#[cfg(test)]
mod tests;
