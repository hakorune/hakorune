/*!
 * MIR-owned route plans for string direct-set source windows.
 *
 * This module owns the `substring(source, 0, split)`,
 * `substring(source, split, ...)`, `substring_concat3_hhhii(...)` direct-set
 * legality proof so `.inc` codegen can consume a pre-decided route tag instead
 * of rediscovering raw MIR instruction windows.
 */

use super::string_corridor_recognizer::{
    match_substring_call, match_substring_concat3_helper_call, string_source_identity,
    StringSourceIdentity,
};
use super::{
    build_value_def_map, resolve_value_origin, BasicBlock, BasicBlockId, ConstValue, MirFunction,
    MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringDirectSetWindowProof {
    PiecewiseConcat3DirectSetSourceWindow,
}

impl std::fmt::Display for StringDirectSetWindowProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PiecewiseConcat3DirectSetSourceWindow => {
                f.write_str("piecewise_concat3_direct_set_source_window")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDirectSetWindowRoute {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub second_instruction_index: usize,
    pub concat_instruction_index: usize,
    pub source_value: ValueId,
    pub prefix_value: ValueId,
    pub suffix_value: ValueId,
    pub middle_value: ValueId,
    pub split_value: ValueId,
    pub result_value: ValueId,
    pub subrange_start: ValueId,
    pub subrange_end: ValueId,
    pub skip_instruction_indices: Vec<usize>,
    pub proof: StringDirectSetWindowProof,
}

pub fn refresh_module_string_direct_set_window_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_direct_set_window_routes(function);
    }
}

pub fn refresh_function_string_direct_set_window_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some((prefix_value, source_value, start, split, _)) = match_substring_call(inst)
            else {
                continue;
            };
            if !is_const_integer(function, &def_map, start, 0) {
                continue;
            }
            if let Some(route) = match_piecewise_concat3_direct_set_source_window(
                function,
                &def_map,
                block,
                block_id,
                instruction_index,
                source_value,
                prefix_value,
                split,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.string_direct_set_window_routes = routes;
}

#[allow(clippy::too_many_arguments)]
fn match_piecewise_concat3_direct_set_source_window(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    block_id: BasicBlockId,
    instruction_index: usize,
    source_value: ValueId,
    prefix_value: ValueId,
    split_value: ValueId,
) -> Option<StringDirectSetWindowRoute> {
    let instructions = block.instructions.as_slice();
    let second_instruction_index = instruction_index + 1;
    let (suffix_value, second_source, second_start, _, _) =
        match_substring_call(instructions.get(second_instruction_index)?)?;

    if !same_root(function, def_map, second_source, source_value)
        || !same_root(function, def_map, second_start, split_value)
    {
        return None;
    }

    let prefix_chain = follow_single_use_copy_chain_to_use(
        instructions,
        second_instruction_index + 1,
        prefix_value,
    )?;
    let suffix_chain = follow_single_use_copy_chain_to_use(
        instructions,
        second_instruction_index + 1,
        suffix_value,
    )?;
    if prefix_chain.use_index != suffix_chain.use_index {
        return None;
    }

    let concat_instruction_index = prefix_chain.use_index;
    let helper = match_substring_concat3_helper_call(instructions.get(concat_instruction_index)?)?;
    if !same_root(function, def_map, helper.left, prefix_chain.final_value)
        || !same_root(function, def_map, helper.right, suffix_chain.final_value)
        || !value_is_const_string(function, def_map, helper.middle)
        || !has_direct_set_consumer(function, helper.dst)
    {
        return None;
    }

    let mut skip_instruction_indices = Vec::new();
    skip_instruction_indices.push(second_instruction_index);
    skip_instruction_indices.extend(prefix_chain.copy_indices);
    skip_instruction_indices.extend(suffix_chain.copy_indices);
    skip_instruction_indices.push(concat_instruction_index);

    Some(StringDirectSetWindowRoute {
        block: block_id,
        instruction_index,
        second_instruction_index,
        concat_instruction_index,
        source_value: root(function, def_map, source_value),
        prefix_value,
        suffix_value,
        middle_value: root(function, def_map, helper.middle),
        split_value: root(function, def_map, split_value),
        result_value: helper.dst,
        subrange_start: root(function, def_map, helper.start),
        subrange_end: root(function, def_map, helper.end),
        skip_instruction_indices,
        proof: StringDirectSetWindowProof::PiecewiseConcat3DirectSetSourceWindow,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SingleUseChain {
    use_index: usize,
    final_value: ValueId,
    copy_indices: Vec<usize>,
}

fn follow_single_use_copy_chain_to_use(
    instructions: &[MirInstruction],
    mut start_index: usize,
    seed_value: ValueId,
) -> Option<SingleUseChain> {
    let mut cursor = seed_value;
    let mut copy_indices = Vec::new();

    loop {
        let use_index = find_single_use_instruction(instructions, start_index, cursor)?;
        if let Some(MirInstruction::Copy { dst, src }) = instructions.get(use_index) {
            if *src == cursor {
                copy_indices.push(use_index);
                cursor = *dst;
                start_index = use_index + 1;
                continue;
            }
        }
        return Some(SingleUseChain {
            use_index,
            final_value: cursor,
            copy_indices,
        });
    }
}

fn find_single_use_instruction(
    instructions: &[MirInstruction],
    start_index: usize,
    value: ValueId,
) -> Option<usize> {
    let mut found = None;
    for (index, inst) in instructions.iter().enumerate().skip(start_index) {
        if !inst.used_values().contains(&value) {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some(index);
    }
    found
}

fn has_direct_set_consumer(function: &MirFunction, value: ValueId) -> bool {
    function
        .metadata
        .value_consumer_facts
        .get(&value)
        .is_some_and(|facts| facts.direct_set_consumer)
}

fn same_root(function: &MirFunction, def_map: &ValueDefMap, lhs: ValueId, rhs: ValueId) -> bool {
    root(function, def_map, lhs) == root(function, def_map, rhs)
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn is_const_integer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    expected: i64,
) -> bool {
    let value = root(function, def_map, value);
    let Some((block_id, index)) = def_map.get(&value).copied() else {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{Callee, EffectMask, FunctionSignature, MirType, ValueConsumerFacts};

    fn method_call(
        dst: Option<u32>,
        receiver: u32,
        method: &str,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    fn extern_call(dst: u32, name: &str, args: Vec<u32>) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(ValueId::new(dst)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(name.to_string())),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    fn make_function() -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![MirType::Box("StringBox".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.params.push(ValueId::new(1));
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(0),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(4),
            },
            MirInstruction::Const {
                dst: ValueId::new(4),
                value: ConstValue::Integer(16),
            },
            method_call(Some(5), 1, "substring", vec![2, 3]),
            method_call(Some(6), 1, "substring", vec![3, 4]),
            MirInstruction::Const {
                dst: ValueId::new(7),
                value: ConstValue::String("xx".to_string()),
            },
            extern_call(
                8,
                "nyash.string.substring_concat3_hhhii",
                vec![5, 7, 6, 2, 4],
            ),
            method_call(None, 20, "set", vec![2, 8]),
        ]);
        function.metadata.value_consumer_facts.insert(
            ValueId::new(8),
            ValueConsumerFacts {
                direct_set_consumer: true,
            },
        );
        function
    }

    #[test]
    fn detects_piecewise_concat3_direct_set_source_window() {
        let mut function = make_function();
        refresh_function_string_direct_set_window_routes(&mut function);

        assert_eq!(function.metadata.string_direct_set_window_routes.len(), 1);
        let route = &function.metadata.string_direct_set_window_routes[0];
        assert_eq!(route.block, BasicBlockId::new(0));
        assert_eq!(route.instruction_index, 3);
        assert_eq!(route.second_instruction_index, 4);
        assert_eq!(route.concat_instruction_index, 6);
        assert_eq!(route.source_value, ValueId::new(1));
        assert_eq!(route.prefix_value, ValueId::new(5));
        assert_eq!(route.suffix_value, ValueId::new(6));
        assert_eq!(route.middle_value, ValueId::new(7));
        assert_eq!(route.split_value, ValueId::new(3));
        assert_eq!(route.result_value, ValueId::new(8));
        assert_eq!(route.subrange_start, ValueId::new(2));
        assert_eq!(route.subrange_end, ValueId::new(4));
        assert_eq!(route.skip_instruction_indices, vec![4, 6]);
        assert_eq!(
            route.proof,
            StringDirectSetWindowProof::PiecewiseConcat3DirectSetSourceWindow
        );
    }

    #[test]
    fn rejects_when_result_has_no_direct_set_consumer_fact() {
        let mut function = make_function();
        function.metadata.value_consumer_facts.clear();

        refresh_function_string_direct_set_window_routes(&mut function);

        assert!(function.metadata.string_direct_set_window_routes.is_empty());
    }
}
