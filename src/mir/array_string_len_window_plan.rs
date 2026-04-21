/*!
 * MIR-owned route plans for array string length windows.
 *
 * This module owns the simple `array.get(i) -> copy* -> length` legality proof
 * so `.inc` codegen can consume a pre-decided route tag for direct
 * `nyash.array.string_len_hi` emission instead of rediscovering the MIR shape
 * from JSON.
 */

use super::array_receiver_proof::{
    match_array_get_call, receiver_is_proven_array, same_value_root,
    value_root as resolve_array_value_root,
};
use super::string_corridor_recognizer::match_len_call;
use super::{
    build_value_def_map, BasicBlock, BasicBlockId, MirFunction, MirInstruction, MirModule,
    ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayStringLenWindowMode {
    LenOnly,
}

impl std::fmt::Display for ArrayStringLenWindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LenOnly => f.write_str("len_only"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayStringLenWindowProof {
    ArrayGetLenNoLaterSourceUse,
}

impl std::fmt::Display for ArrayStringLenWindowProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrayGetLenNoLaterSourceUse => f.write_str("array_get_len_no_later_source_use"),
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

fn has_later_source_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    instructions: &[MirInstruction],
    from_index: usize,
    source_values: &[ValueId],
) -> bool {
    let source_roots: Vec<ValueId> = source_values
        .iter()
        .copied()
        .map(|value| resolve_array_value_root(function, def_map, value))
        .collect();
    instructions.iter().skip(from_index).any(|inst| {
        inst.used_values().into_iter().any(|value| {
            let value_root = resolve_array_value_root(function, def_map, value);
            source_roots.iter().any(|source| *source == value_root)
        })
    })
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
    let mut skip = Vec::new();
    let (cursor, carried) = skip_copy_chain(
        function,
        def_map,
        instructions,
        instruction_index + 1,
        source_value,
        &mut skip,
    );

    let (len_value, len_receiver, _) = match_len_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, len_receiver, carried) {
        return None;
    }
    skip.push(cursor);

    if has_later_source_use(
        function,
        def_map,
        instructions,
        cursor + 1,
        &[source_value, carried],
    ) {
        return None;
    }

    Some(ArrayStringLenWindowRoute {
        block: block_id,
        instruction_index,
        array_value,
        index_value,
        source_value,
        len_instruction_index: cursor,
        len_value,
        skip_instruction_indices: skip,
        mode: ArrayStringLenWindowMode::LenOnly,
        proof: ArrayStringLenWindowProof::ArrayGetLenNoLaterSourceUse,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, Callee, ConstValue, EffectMask, FunctionSignature, MirType};

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
    fn rejects_keep_get_live_shape_for_follow_up_card() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(length_call(12, "RuntimeDataBox", "length", 10));
        block.add_instruction(method_call(
            Some(13),
            "RuntimeDataBox",
            "substring",
            10,
            vec![ValueId::new(3), ValueId::new(4)],
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
