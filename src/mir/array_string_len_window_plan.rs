/*!
 * MIR-owned route plans for array string length windows.
 *
 * This module owns the simple `array.get(i) -> copy* -> length` legality proof
 * so `.inc` codegen can consume a pre-decided route tag for direct
 * `nyash.array.string_len_hi` emission instead of rediscovering the MIR shape
 * from JSON.
 */

use super::array_receiver_proof::{
    match_array_get_call, match_array_set_call, receiver_is_proven_array, same_value_root,
    value_root as resolve_array_value_root,
};
use super::string_corridor_recognizer::{
    match_concat_triplet, match_len_call, match_substring_call,
    match_substring_concat3_helper_call, string_source_identity, StringSourceIdentity,
};
use super::{
    build_value_def_map, BasicBlock, BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction,
    MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayStringLenWindowMode {
    LenOnly,
    KeepGetLive,
    SourceOnlyInsertMid,
}

impl std::fmt::Display for ArrayStringLenWindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LenOnly => f.write_str("len_only"),
            Self::KeepGetLive => f.write_str("keep_get_live"),
            Self::SourceOnlyInsertMid => f.write_str("source_only_insert_mid"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayStringLenWindowProof {
    ArrayGetLenNoLaterSourceUse,
    ArrayGetLenKeepSourceLive,
    ArrayGetLenSourceOnlyDirectSet,
}

impl std::fmt::Display for ArrayStringLenWindowProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrayGetLenNoLaterSourceUse => f.write_str("array_get_len_no_later_source_use"),
            Self::ArrayGetLenKeepSourceLive => f.write_str("array_get_len_keep_source_live"),
            Self::ArrayGetLenSourceOnlyDirectSet => {
                f.write_str("array_get_len_source_only_direct_set")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayStringLenWindowRoute {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub source_value: ValueId,
    pub len_instruction_index: usize,
    pub len_value: ValueId,
    pub skip_instruction_indices: Vec<usize>,
    pub mode: ArrayStringLenWindowMode,
    pub proof: ArrayStringLenWindowProof,
}

pub fn refresh_module_array_string_len_window_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_string_len_window_routes(function);
    }
}

pub fn refresh_function_array_string_len_window_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some(get_call) = match_array_get_call(inst) else {
                continue;
            };
            if !receiver_is_proven_array(
                function,
                &def_map,
                get_call.array_value,
                get_call.receiver_box_name,
            ) {
                continue;
            }
            if let Some(route) = match_len_only_route(
                function,
                &def_map,
                block,
                block_id,
                instruction_index,
                get_call.array_value,
                get_call.index_value,
                get_call.output_value,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.array_string_len_window_routes = routes;
}

fn same_root(function: &MirFunction, def_map: &ValueDefMap, lhs: ValueId, rhs: ValueId) -> bool {
    same_value_root(function, def_map, lhs, rhs)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LaterSourceUse {
    NoUse,
    SafeSubstring,
    SourceOnlyDirectSet,
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

fn classify_later_source_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    instructions: &[MirInstruction],
    from_index: usize,
    source_values: &[ValueId],
    array_value: ValueId,
    index_value: ValueId,
    len_value: ValueId,
) -> Option<LaterSourceUse> {
    let source_roots: Vec<ValueId> = source_values
        .iter()
        .copied()
        .map(|value| resolve_array_value_root(function, def_map, value))
        .collect();
    let mut saw_safe_substring = false;
    let mut saw_source_only_direct_set = false;
    let mut index = from_index;
    while index < instructions.len() {
        let inst = &instructions[index];
        let uses_source = inst_uses_any_root(function, def_map, inst, &source_roots);
        if !uses_source {
            index += 1;
            continue;
        }
        if let Some(end_index) = match_source_only_direct_set_window(
            function,
            def_map,
            block_id,
            instructions,
            index,
            &source_roots,
            array_value,
            index_value,
            len_value,
        ) {
            saw_source_only_direct_set = true;
            index = end_index + 1;
            continue;
        }
        if inst_is_safe_substring_source_reuse(function, def_map, inst, &source_roots) {
            saw_safe_substring = true;
            index += 1;
            continue;
        }
        return None;
    }
    Some(if saw_safe_substring {
        LaterSourceUse::SafeSubstring
    } else if saw_source_only_direct_set {
        LaterSourceUse::SourceOnlyDirectSet
    } else {
        LaterSourceUse::NoUse
    })
}

fn inst_uses_any_root(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_roots: &[ValueId],
) -> bool {
    inst.used_values().into_iter().any(|value| {
        let value_root = resolve_array_value_root(function, def_map, value);
        source_roots.iter().any(|source| *source == value_root)
    })
}

fn inst_is_safe_substring_source_reuse(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_roots: &[ValueId],
) -> bool {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            ..
        } if box_name == "RuntimeDataBox" && method == "substring" => {
            let receiver_root = resolve_array_value_root(function, def_map, *receiver);
            source_roots.iter().any(|source| *source == receiver_root)
        }
        _ => false,
    }
}

#[allow(clippy::too_many_arguments)]
fn match_source_only_direct_set_window(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    instructions: &[MirInstruction],
    first_substring_index: usize,
    source_roots: &[ValueId],
    array_value: ValueId,
    index_value: ValueId,
    len_value: ValueId,
) -> Option<usize> {
    let first = match_source_substring(
        function,
        def_map,
        instructions.get(first_substring_index)?,
        source_roots,
    )?;
    let second_substring_index = first_substring_index + 1;
    let second = match_source_substring(
        function,
        def_map,
        instructions.get(second_substring_index)?,
        source_roots,
    )?;

    if !is_const_integer(function, def_map, first.start, 0) {
        return None;
    }
    if !same_root(function, def_map, first.end, second.start) {
        return None;
    }
    if !same_root(function, def_map, second.end, len_value) {
        return None;
    }

    for (set_index, inst) in instructions
        .iter()
        .enumerate()
        .skip(second_substring_index + 1)
    {
        let Some(set_call) = match_array_set_call(inst) else {
            continue;
        };
        if instructions
            .iter()
            .take(set_index)
            .skip(second_substring_index + 1)
            .any(|inst| inst_uses_any_root(function, def_map, inst, source_roots))
        {
            return None;
        }
        if !same_root(function, def_map, set_call.array_value, array_value)
            || !same_root(function, def_map, set_call.index_value, index_value)
            || !value_is_single_set_input(
                function,
                def_map,
                instructions,
                set_call.input_value,
                set_index,
            )
        {
            continue;
        }
        if direct_set_value_is_source_only_concat(
            function,
            def_map,
            block_id,
            set_call.input_value,
            first.dst,
            second.dst,
        ) {
            return Some(set_index);
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceSubstring {
    dst: ValueId,
    start: ValueId,
    end: ValueId,
}

fn match_source_substring(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_roots: &[ValueId],
) -> Option<SourceSubstring> {
    let (dst, receiver, start, end, _) = match_substring_call(inst)?;
    let receiver_root = resolve_array_value_root(function, def_map, receiver);
    if !source_roots.iter().any(|source| *source == receiver_root) {
        return None;
    }
    Some(SourceSubstring { dst, start, end })
}

fn direct_set_value_is_source_only_concat(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    set_input: ValueId,
    left_substring: ValueId,
    right_substring: ValueId,
) -> bool {
    if let Some(triplet) = match_concat_triplet(function, block_id, def_map, set_input) {
        return same_root(function, def_map, triplet.left, left_substring)
            && same_root(function, def_map, triplet.right, right_substring)
            && value_is_const_string(function, def_map, triplet.middle);
    }

    let set_input_root = resolve_array_value_root(function, def_map, set_input);
    let Some((def_block, def_index)) = def_map.get(&set_input_root).copied() else {
        return false;
    };
    if def_block != block_id {
        return false;
    }
    let Some(block) = function.blocks.get(&def_block) else {
        return false;
    };
    let Some(helper) = block
        .instructions
        .get(def_index)
        .and_then(match_substring_concat3_helper_call)
    else {
        return false;
    };
    same_root(function, def_map, helper.left, left_substring)
        && same_root(function, def_map, helper.right, right_substring)
        && value_is_const_string(function, def_map, helper.middle)
}

fn is_const_integer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    expected: i64,
) -> bool {
    let root = resolve_array_value_root(function, def_map, value);
    let Some((block_id, index)) = def_map.get(&root).copied() else {
        return false;
    };
    function
        .blocks
        .get(&block_id)
        .and_then(|block| block.instructions.get(index))
        .is_some_and(|inst| {
            matches!(
                inst,
                MirInstruction::Const {
                    value: ConstValue::Integer(actual),
                    ..
                } if *actual == expected
            )
        })
}

fn value_is_const_string(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> bool {
    matches!(
        string_source_identity(function, def_map, value),
        Some(StringSourceIdentity::ConstString(_))
    )
}

fn value_is_single_set_input(
    function: &MirFunction,
    def_map: &ValueDefMap,
    instructions: &[MirInstruction],
    value: ValueId,
    set_index: usize,
) -> bool {
    let root = resolve_array_value_root(function, def_map, value);
    let mut uses = 0;
    for (index, inst) in instructions.iter().enumerate() {
        for used in inst.used_values() {
            if resolve_array_value_root(function, def_map, used) == root {
                uses += 1;
                if index != set_index {
                    return false;
                }
            }
        }
    }
    uses == 1
}

#[allow(clippy::too_many_arguments)]
fn match_len_only_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    block_id: BasicBlockId,
    instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
) -> Option<ArrayStringLenWindowRoute> {
    let instructions = block.instructions.as_slice();
    let mut copy_skip = Vec::new();
    let (cursor, carried) = skip_copy_chain(
        function,
        def_map,
        instructions,
        instruction_index + 1,
        source_value,
        &mut copy_skip,
    );

    let (len_value, len_receiver, _) = match_len_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, len_receiver, carried) {
        return None;
    }

    let later_use = classify_later_source_use(
        function,
        def_map,
        block_id,
        instructions,
        cursor + 1,
        &[source_value, carried],
        array_value,
        index_value,
        len_value,
    )?;

    let (skip_instruction_indices, mode, proof) = match later_use {
        LaterSourceUse::NoUse => {
            let mut skip = copy_skip;
            skip.push(cursor);
            (
                skip,
                ArrayStringLenWindowMode::LenOnly,
                ArrayStringLenWindowProof::ArrayGetLenNoLaterSourceUse,
            )
        }
        LaterSourceUse::SafeSubstring => (
            vec![cursor],
            ArrayStringLenWindowMode::KeepGetLive,
            ArrayStringLenWindowProof::ArrayGetLenKeepSourceLive,
        ),
        LaterSourceUse::SourceOnlyDirectSet => (
            vec![cursor],
            ArrayStringLenWindowMode::SourceOnlyInsertMid,
            ArrayStringLenWindowProof::ArrayGetLenSourceOnlyDirectSet,
        ),
    };

    Some(ArrayStringLenWindowRoute {
        block: block_id,
        instruction_index,
        array_value,
        index_value,
        source_value,
        len_instruction_index: cursor,
        len_value,
        skip_instruction_indices,
        mode,
        proof,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlock, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature, MirType,
    };

    #[test]
    fn detects_array_get_string_len_route() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 11));
        block.add_instruction(const_i(13, 1));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(12)),
        });

        refresh_function_array_string_len_window_routes(&mut function);

        assert_eq!(function.metadata.array_string_len_window_routes.len(), 1);
        let route = &function.metadata.array_string_len_window_routes[0];
        assert_eq!(route.array_value, ValueId::new(1));
        assert_eq!(route.index_value, ValueId::new(2));
        assert_eq!(route.source_value, ValueId::new(10));
        assert_eq!(route.len_instruction_index, 2);
        assert_eq!(route.len_value, ValueId::new(12));
        assert_eq!(route.skip_instruction_indices, vec![1, 2]);
        assert_eq!(route.mode, ArrayStringLenWindowMode::LenOnly);
        assert_eq!(
            route.proof,
            ArrayStringLenWindowProof::ArrayGetLenNoLaterSourceUse
        );
    }

    #[test]
    fn detects_runtime_data_receiver_from_new_array_box_root() {
        let mut function = test_function(MirType::Box("MapBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(new_box(20, "ArrayBox"));
        block.add_instruction(copy(21, 20));
        block.add_instruction(array_get(10, "RuntimeDataBox", 21, 2));
        block.add_instruction(length_call(12, "RuntimeDataBox", "len", 10));

        refresh_function_array_string_len_window_routes(&mut function);

        assert_eq!(function.metadata.array_string_len_window_routes.len(), 1);
        let route = &function.metadata.array_string_len_window_routes[0];
        assert_eq!(route.array_value, ValueId::new(21));
        assert_eq!(route.len_instruction_index, 3);
        assert_eq!(route.skip_instruction_indices, vec![3]);
    }

    #[test]
    fn rejects_unproven_runtime_data_receiver() {
        let mut function = test_function(MirType::Box("MapBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 10));

        refresh_function_array_string_len_window_routes(&mut function);

        assert!(function.metadata.array_string_len_window_routes.is_empty());
    }

    #[test]
    fn rejects_post_len_source_use() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(length_call(12, "RuntimeDataBox", "size", 11));
        block.add_instruction(copy(13, 10));

        refresh_function_array_string_len_window_routes(&mut function);

        assert!(function.metadata.array_string_len_window_routes.is_empty());
    }

    #[test]
    fn detects_keep_get_live_shape() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 11));
        block.add_instruction(method_call(
            Some(13),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(3), ValueId::new(4)],
        ));

        refresh_function_array_string_len_window_routes(&mut function);

        assert_eq!(function.metadata.array_string_len_window_routes.len(), 1);
        let route = &function.metadata.array_string_len_window_routes[0];
        assert_eq!(route.len_instruction_index, 2);
        assert_eq!(route.skip_instruction_indices, vec![2]);
        assert_eq!(route.mode, ArrayStringLenWindowMode::KeepGetLive);
        assert_eq!(
            route.proof,
            ArrayStringLenWindowProof::ArrayGetLenKeepSourceLive
        );
    }

    #[test]
    fn detects_source_only_insert_mid_direct_set_shape() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 11));
        block.add_instruction(const_i(13, 0));
        block.add_instruction(const_i(14, 2));
        block.add_instruction(binop(15, BinaryOp::Div, 12, 14));
        block.add_instruction(method_call(
            Some(16),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(13), ValueId::new(15)],
        ));
        block.add_instruction(method_call(
            Some(17),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(15), ValueId::new(12)],
        ));
        block.add_instruction(const_s(18, "xx"));
        block.add_instruction(binop(19, BinaryOp::Add, 16, 18));
        block.add_instruction(binop(20, BinaryOp::Add, 19, 17));
        block.add_instruction(array_set(1, 2, 20));

        refresh_function_array_string_len_window_routes(&mut function);

        assert_eq!(function.metadata.array_string_len_window_routes.len(), 1);
        let route = &function.metadata.array_string_len_window_routes[0];
        assert_eq!(route.len_instruction_index, 2);
        assert_eq!(route.skip_instruction_indices, vec![2]);
        assert_eq!(route.mode, ArrayStringLenWindowMode::SourceOnlyInsertMid);
        assert_eq!(
            route.proof,
            ArrayStringLenWindowProof::ArrayGetLenSourceOnlyDirectSet
        );
    }

    #[test]
    fn detects_source_only_piecewise_concat3_direct_set_shape() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 11));
        block.add_instruction(const_i(13, 0));
        block.add_instruction(const_i(14, 2));
        block.add_instruction(binop(15, BinaryOp::Div, 12, 14));
        block.add_instruction(method_call(
            Some(16),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(13), ValueId::new(15)],
        ));
        block.add_instruction(method_call(
            Some(17),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(15), ValueId::new(12)],
        ));
        block.add_instruction(const_s(18, "xx"));
        block.add_instruction(const_i(19, 1));
        block.add_instruction(binop(20, BinaryOp::Add, 12, 19));
        block.add_instruction(extern_call(
            21,
            "nyash.string.substring_concat3_hhhii",
            vec![
                ValueId::new(16),
                ValueId::new(18),
                ValueId::new(17),
                ValueId::new(19),
                ValueId::new(20),
            ],
        ));
        block.add_instruction(array_set(1, 2, 21));

        refresh_function_array_string_len_window_routes(&mut function);

        assert_eq!(function.metadata.array_string_len_window_routes.len(), 1);
        let route = &function.metadata.array_string_len_window_routes[0];
        assert_eq!(route.len_instruction_index, 2);
        assert_eq!(route.skip_instruction_indices, vec![2]);
        assert_eq!(route.mode, ArrayStringLenWindowMode::SourceOnlyInsertMid);
        assert_eq!(
            route.proof,
            ArrayStringLenWindowProof::ArrayGetLenSourceOnlyDirectSet
        );
    }

    #[test]
    fn falls_back_to_keep_live_when_direct_set_result_has_extra_use() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 11));
        block.add_instruction(const_i(13, 0));
        block.add_instruction(const_i(14, 2));
        block.add_instruction(binop(15, BinaryOp::Div, 12, 14));
        block.add_instruction(method_call(
            Some(16),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(13), ValueId::new(15)],
        ));
        block.add_instruction(method_call(
            Some(17),
            "RuntimeDataBox",
            "substring",
            11,
            vec![ValueId::new(15), ValueId::new(12)],
        ));
        block.add_instruction(const_s(18, "xx"));
        block.add_instruction(binop(19, BinaryOp::Add, 16, 18));
        block.add_instruction(binop(20, BinaryOp::Add, 19, 17));
        block.add_instruction(copy(21, 20));
        block.add_instruction(array_set(1, 2, 20));

        refresh_function_array_string_len_window_routes(&mut function);

        assert_eq!(function.metadata.array_string_len_window_routes.len(), 1);
        let route = &function.metadata.array_string_len_window_routes[0];
        assert_eq!(route.skip_instruction_indices, vec![2]);
        assert_eq!(route.mode, ArrayStringLenWindowMode::KeepGetLive);
        assert_eq!(
            route.proof,
            ArrayStringLenWindowProof::ArrayGetLenKeepSourceLive
        );
    }

    #[test]
    fn rejects_substring_arg_source_use_when_receiver_is_not_source() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 10));
        block.add_instruction(method_call(
            Some(13),
            "RuntimeDataBox",
            "substring",
            20,
            vec![ValueId::new(10), ValueId::new(4)],
        ));

        refresh_function_array_string_len_window_routes(&mut function);

        assert!(function.metadata.array_string_len_window_routes.is_empty());
    }

    fn test_function(array_param_type: MirType) -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![array_param_type, MirType::Integer],
            return_type: MirType::Integer,
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

    fn new_box(dst: u32, box_type: &str) -> MirInstruction {
        MirInstruction::NewBox {
            dst: ValueId::new(dst),
            box_type: box_type.to_string(),
            args: vec![],
        }
    }

    fn array_get(dst: u32, box_name: &str, array: u32, index: u32) -> MirInstruction {
        method_call(Some(dst), box_name, "get", array, vec![ValueId::new(index)])
    }

    fn array_set(array: u32, index: u32, value: u32) -> MirInstruction {
        method_call(
            None,
            "RuntimeDataBox",
            "set",
            array,
            vec![ValueId::new(index), ValueId::new(value)],
        )
    }

    fn binop(dst: u32, op: BinaryOp, lhs: u32, rhs: u32) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(lhs),
            rhs: ValueId::new(rhs),
        }
    }

    fn extern_call(dst: u32, name: &str, args: Vec<ValueId>) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(ValueId::new(dst)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(name.to_string())),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn length_call(dst: u32, box_name: &str, method: &str, receiver: u32) -> MirInstruction {
        method_call(Some(dst), box_name, method, receiver, vec![])
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
