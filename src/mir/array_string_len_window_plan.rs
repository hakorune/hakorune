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
use super::value_origin::{build_value_def_map, ValueDefMap};
use super::{
    BasicBlock, BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayStringLenWindowMode {
    LenOnly,
    KeepGetLive,
    SourceOnlyInsertMid,
}

impl ArrayStringLenWindowMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::LenOnly => "len_only",
            Self::KeepGetLive => "keep_get_live",
            Self::SourceOnlyInsertMid => "source_only_insert_mid",
        }
    }

    fn keep_get_live(self) -> bool {
        matches!(self, Self::KeepGetLive)
    }

    fn source_only_insert_mid(self) -> bool {
        matches!(self, Self::SourceOnlyInsertMid)
    }

    fn effect_tags(self) -> &'static [&'static str] {
        match self {
            Self::LenOnly => &["load.cell", "observe.len"],
            Self::KeepGetLive => &["load.cell", "observe.len", "keep.source.live"],
            Self::SourceOnlyInsertMid => &["load.cell", "observe.len", "publish.source.ref"],
        }
    }
}

impl std::fmt::Display for ArrayStringLenWindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayStringLenWindowProof {
    ArrayGetLenNoLaterSourceUse,
    ArrayGetLenKeepSourceLive,
    ArrayGetLenSourceOnlyDirectSet,
}

impl ArrayStringLenWindowProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArrayGetLenNoLaterSourceUse => "array_get_len_no_later_source_use",
            Self::ArrayGetLenKeepSourceLive => "array_get_len_keep_source_live",
            Self::ArrayGetLenSourceOnlyDirectSet => "array_get_len_source_only_direct_set",
        }
    }
}

impl std::fmt::Display for ArrayStringLenWindowProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayStringLenWindowRoute {
    block: BasicBlockId,
    instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
    len_instruction_index: usize,
    len_value: ValueId,
    skip_instruction_indices: Vec<usize>,
    mode: ArrayStringLenWindowMode,
    proof: ArrayStringLenWindowProof,
}

impl ArrayStringLenWindowRoute {
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

    pub fn len_instruction_index(&self) -> usize {
        self.len_instruction_index
    }

    pub fn len_value(&self) -> ValueId {
        self.len_value
    }

    pub fn skip_instruction_indices(&self) -> &[usize] {
        &self.skip_instruction_indices
    }

    pub fn mode(&self) -> &'static str {
        self.mode.as_str()
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn keep_get_live(&self) -> bool {
        self.mode.keep_get_live()
    }

    pub fn source_only_insert_mid(&self) -> bool {
        self.mode.source_only_insert_mid()
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.mode.effect_tags()
    }
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
pub(crate) mod test_support;

#[cfg(test)]
mod tests;
