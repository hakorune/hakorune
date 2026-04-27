/*!
 * MIR-owned array/text same-cell edit routes.
 *
 * This module owns the current H27 edit contract:
 * `array.get(i).length() -> split = len / 2 -> same-slot insert-mid const`.
 * Backends may consume this metadata to select a helper call and skip the
 * covered instructions, but they must not rediscover edit legality from raw
 * MIR JSON.
 */

use std::collections::HashMap;

use super::string_corridor_recognizer::{match_len_call, match_substring_call};
use super::{
    build_value_def_map, definitions::Callee, resolve_value_origin, BasicBlock, BasicBlockId,
    BinaryOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
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
}

impl ArrayTextEditProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArrayGetLenHalfInsertMidSameSlot => "array_get_lenhalf_insert_mid_same_slot",
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
    index_value: ValueId,
    source_value: ValueId,
    length_value: ValueId,
    split_value: ValueId,
    result_value: ValueId,
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

    let middle_value = match instructions.get(cursor)? {
        MirInstruction::Const {
            dst,
            value: ConstValue::String(_),
        } => *dst,
        _ => return None,
    };
    let middle_text = match_const_string(function, def_map, middle_value)?;
    skip.push(cursor);
    cursor += 1;

    let pair_value = match_add_either(
        instructions.get(cursor)?,
        left_final,
        middle_value,
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

    let (set_array, set_index, set_value) = match_set_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, set_array, array_value)
        || !same_root(function, def_map, set_index, index_value)
        || !same_root(function, def_map, set_value, result_value)
    {
        return None;
    }
    let set_instruction_index = cursor;
    skip.push(cursor);

    let consumed = [
        source_value,
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
        pair_value,
        pair_final,
        result_value,
    ];
    if has_uncovered_use(function, def_map, block_id, cursor, &consumed) {
        return None;
    }

    Some(ArrayTextEditRoute {
        block: block_id,
        get_instruction_index,
        set_instruction_index,
        array_value: root(function, def_map, array_value),
        index_value: root(function, def_map, index_value),
        source_value: root(function, def_map, source_value),
        length_value,
        split_value,
        result_value,
        middle_byte_len: middle_text.len(),
        middle_value,
        middle_text,
        skip_instruction_indices: skip,
        edit_kind: ArrayTextEditKind::InsertMidConst,
        split_policy: ArrayTextEditSplitPolicy::SourceLenDivConst { divisor: 2 },
        proof: ArrayTextEditProof::ArrayGetLenHalfInsertMidSameSlot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

    #[test]
    fn detects_lenhalf_insert_mid_same_slot_edit_route() {
        let mut function = test_function();
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(len_call(12, 11));
        block.add_instruction(copy(13, 12));
        block.add_instruction(const_i(14, 0));
        block.add_instruction(copy(15, 13));
        block.add_instruction(const_i(16, 2));
        block.add_instruction(binop(17, BinaryOp::Div, 15, 16));
        block.add_instruction(substring_call(18, 11, 14, 17));
        block.add_instruction(substring_call(19, 11, 17, 13));
        block.add_instruction(copy(20, 18));
        block.add_instruction(copy(21, 19));
        block.add_instruction(copy(22, 20));
        block.add_instruction(const_s(23, "xx"));
        block.add_instruction(binop(24, BinaryOp::Add, 22, 23));
        block.add_instruction(copy(25, 21));
        block.add_instruction(binop(26, BinaryOp::Add, 24, 25));
        block.add_instruction(array_set(27, 1, 2, 26));
        block.set_terminator(MirInstruction::Return { value: None });

        refresh_function_array_text_edit_routes(&mut function);

        assert_eq!(function.metadata.array_text_edit_routes.len(), 1);
        let route = &function.metadata.array_text_edit_routes[0];
        assert_eq!(route.array_value(), ValueId::new(1));
        assert_eq!(route.index_value(), ValueId::new(2));
        assert_eq!(route.source_value(), ValueId::new(10));
        assert_eq!(route.length_value(), ValueId::new(12));
        assert_eq!(route.split_value(), ValueId::new(17));
        assert_eq!(route.result_value(), ValueId::new(26));
        assert_eq!(route.middle_value(), ValueId::new(23));
        assert_eq!(route.middle_text(), "xx");
        assert_eq!(route.middle_byte_len(), 2);
        assert_eq!(route.edit_kind(), "insert_mid_const");
        assert_eq!(route.split_policy(), "source_len_div_const(2)");
        assert_eq!(route.proof(), "array_get_lenhalf_insert_mid_same_slot");
        assert!(route.is_lenhalf_insert_mid_same_slot());
        assert_eq!(
            route.skip_instruction_indices(),
            &(1..=17).collect::<Vec<_>>()
        );
    }

    fn test_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("ArrayBox".to_string()), MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.params = vec![ValueId::new(1), ValueId::new(2)];
        function
    }

    fn entry_block(function: &mut MirFunction) -> &mut BasicBlock {
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
    }

    fn const_i(dst: u32, value: i64) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::Integer(value),
        }
    }

    fn const_s(dst: u32, value: &str) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::String(value.to_string()),
        }
    }

    fn copy(dst: u32, src: u32) -> MirInstruction {
        MirInstruction::Copy {
            dst: ValueId::new(dst),
            src: ValueId::new(src),
        }
    }

    fn binop(dst: u32, op: BinaryOp, lhs: u32, rhs: u32) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(lhs),
            rhs: ValueId::new(rhs),
        }
    }

    fn array_get(dst: u32, array: u32, index: u32) -> MirInstruction {
        method_call(
            Some(dst),
            "RuntimeDataBox",
            "get",
            array,
            vec![ValueId::new(index)],
        )
    }

    fn len_call(dst: u32, receiver: u32) -> MirInstruction {
        method_call(Some(dst), "RuntimeDataBox", "length", receiver, vec![])
    }

    fn substring_call(dst: u32, receiver: u32, start: u32, end: u32) -> MirInstruction {
        method_call(
            Some(dst),
            "RuntimeDataBox",
            "substring",
            receiver,
            vec![ValueId::new(start), ValueId::new(end)],
        )
    }

    fn array_set(_marker: u32, array: u32, index: u32, value: u32) -> MirInstruction {
        method_call(
            None,
            "RuntimeDataBox",
            "set",
            array,
            vec![ValueId::new(index), ValueId::new(value)],
        )
    }

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }
}
