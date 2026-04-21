/*!
 * MIR-owned route plans for array read-modify-write windows.
 *
 * This module owns the narrow `array.get(i) -> + 1 -> array.set(i, ...)`
 * legality proof so `.inc` codegen can consume a pre-decided route tag instead
 * of rediscovering the MIR shape from JSON.
 */

use super::{
    build_value_def_map, definitions::Callee, resolve_value_origin, BasicBlock, BasicBlockId,
    BinaryOp, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayRmwWindowProof {
    ArrayGetAdd1SetSameSlot,
}

impl std::fmt::Display for ArrayRmwWindowProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrayGetAdd1SetSameSlot => f.write_str("array_get_add1_set_same_slot"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayRmwWindowRoute {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub input_value: ValueId,
    pub const_value: ValueId,
    pub result_value: ValueId,
    pub set_instruction_index: usize,
    pub skip_instruction_indices: Vec<usize>,
    pub proof: ArrayRmwWindowProof,
}

pub fn refresh_module_array_rmw_window_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_rmw_window_routes(function);
    }
}

pub fn refresh_function_array_rmw_window_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some((array_value, index_value, input_value, receiver_box_name)) =
                match_array_get_call(inst)
            else {
                continue;
            };
            if !receiver_is_proven_array(function, &def_map, array_value, receiver_box_name) {
                continue;
            }
            if let Some(route) = match_add1_same_slot_set_route(
                function,
                &def_map,
                block,
                block_id,
                instruction_index,
                array_value,
                index_value,
                input_value,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.array_rmw_window_routes = routes;
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn same_root(function: &MirFunction, def_map: &ValueDefMap, lhs: ValueId, rhs: ValueId) -> bool {
    root(function, def_map, lhs) == root(function, def_map, rhs)
}

fn match_array_get_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId, &str)> {
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
            && matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") =>
        {
            Some((*receiver, args[0], *dst, box_name.as_str()))
        }
        _ => None,
    }
}

fn receiver_is_proven_array(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    receiver_box_name: &str,
) -> bool {
    if receiver_box_name == "ArrayBox" {
        return true;
    }

    let receiver_root = root(function, def_map, receiver);
    if value_is_array_box_type(function.metadata.value_types.get(&receiver_root)) {
        return true;
    }

    if value_is_array_box_newbox(function, def_map, receiver_root) {
        return true;
    }

    function
        .params
        .iter()
        .position(|param| *param == receiver_root)
        .and_then(|index| function.signature.params.get(index))
        .is_some_and(type_is_array_box)
}

fn value_is_array_box_newbox(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> bool {
    let Some(inst) = def_instruction(function, def_map, value) else {
        return false;
    };
    matches!(inst, MirInstruction::NewBox { box_type, .. } if box_type == "ArrayBox")
}

fn def_instruction<'a>(
    function: &'a MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<&'a MirInstruction> {
    let (block, index) = def_map.get(&value).copied()?;
    function.blocks.get(&block)?.instructions.get(index)
}

fn value_is_array_box_type(ty: Option<&MirType>) -> bool {
    ty.is_some_and(type_is_array_box)
}

fn type_is_array_box(ty: &MirType) -> bool {
    matches!(ty, MirType::Box(name) if name == "ArrayBox")
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

fn match_add1_binop(
    inst: &MirInstruction,
    carried: ValueId,
    const_one: ValueId,
    function: &MirFunction,
    def_map: &ValueDefMap,
) -> Option<ValueId> {
    match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if (same_root(function, def_map, *lhs, carried)
            && same_root(function, def_map, *rhs, const_one))
            || (same_root(function, def_map, *rhs, carried)
                && same_root(function, def_map, *lhs, const_one)) =>
        {
            Some(*dst)
        }
        _ => None,
    }
}

fn match_array_set_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
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
            && matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") =>
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
fn match_add1_same_slot_set_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    block_id: BasicBlockId,
    instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    input_value: ValueId,
) -> Option<ArrayRmwWindowRoute> {
    let instructions = block.instructions.as_slice();
    let mut skip = Vec::new();
    let (mut cursor, carried) = skip_copy_chain(
        function,
        def_map,
        instructions,
        instruction_index + 1,
        input_value,
        &mut skip,
    );

    let const_value = match_const_i64(instructions.get(cursor)?, 1)?;
    skip.push(cursor);
    cursor += 1;

    let result_value = match_add1_binop(
        instructions.get(cursor)?,
        carried,
        const_value,
        function,
        def_map,
    )?;
    skip.push(cursor);
    cursor += 1;

    let set_instruction_index = cursor;
    let (set_array, set_index, set_value) = match_array_set_call(instructions.get(cursor)?)?;
    if !same_root(function, def_map, set_array, array_value)
        || !same_root(function, def_map, set_index, index_value)
        || !same_root(function, def_map, set_value, result_value)
    {
        return None;
    }
    skip.push(cursor);

    if has_later_use(
        function,
        def_map,
        instructions,
        cursor + 1,
        &[input_value, carried],
    ) {
        return None;
    }

    Some(ArrayRmwWindowRoute {
        block: block_id,
        instruction_index,
        array_value,
        index_value,
        input_value,
        const_value,
        result_value,
        set_instruction_index,
        skip_instruction_indices: skip,
        proof: ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature};

    #[test]
    fn detects_array_get_add1_set_same_slot_route() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(const_i(12, 1));
        block.add_instruction(binop(13, BinaryOp::Add, 11, 12));
        block.add_instruction(array_set("RuntimeDataBox", 1, 2, 13));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(13)),
        });

        refresh_function_array_rmw_window_routes(&mut function);

        assert_eq!(function.metadata.array_rmw_window_routes.len(), 1);
        let route = &function.metadata.array_rmw_window_routes[0];
        assert_eq!(route.array_value, ValueId::new(1));
        assert_eq!(route.index_value, ValueId::new(2));
        assert_eq!(route.input_value, ValueId::new(10));
        assert_eq!(route.const_value, ValueId::new(12));
        assert_eq!(route.result_value, ValueId::new(13));
        assert_eq!(route.set_instruction_index, 4);
        assert_eq!(route.skip_instruction_indices, vec![1, 2, 3, 4]);
        assert_eq!(route.proof, ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot);
    }

    #[test]
    fn detects_runtime_data_receiver_from_new_array_box_root() {
        let mut function = test_function(MirType::Box("MapBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(new_box(20, "ArrayBox"));
        block.add_instruction(copy(21, 20));
        block.add_instruction(array_get(10, "RuntimeDataBox", 21, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(const_i(12, 1));
        block.add_instruction(binop(13, BinaryOp::Add, 11, 12));
        block.add_instruction(array_set("RuntimeDataBox", 21, 2, 13));

        refresh_function_array_rmw_window_routes(&mut function);

        assert_eq!(function.metadata.array_rmw_window_routes.len(), 1);
        let route = &function.metadata.array_rmw_window_routes[0];
        assert_eq!(route.array_value, ValueId::new(21));
        assert_eq!(route.set_instruction_index, 6);
        assert_eq!(route.skip_instruction_indices, vec![3, 4, 5, 6]);
    }

    #[test]
    fn rejects_non_one_increment() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(const_i(11, 2));
        block.add_instruction(binop(12, BinaryOp::Add, 10, 11));
        block.add_instruction(array_set("RuntimeDataBox", 1, 2, 12));

        refresh_function_array_rmw_window_routes(&mut function);

        assert!(function.metadata.array_rmw_window_routes.is_empty());
    }

    #[test]
    fn rejects_unproven_runtime_data_receiver() {
        let mut function = test_function(MirType::Box("MapBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(const_i(11, 1));
        block.add_instruction(binop(12, BinaryOp::Add, 10, 11));
        block.add_instruction(array_set("RuntimeDataBox", 1, 2, 12));

        refresh_function_array_rmw_window_routes(&mut function);

        assert!(function.metadata.array_rmw_window_routes.is_empty());
    }

    #[test]
    fn rejects_post_set_get_value_use() {
        let mut function = test_function(MirType::Box("ArrayBox".to_string()));
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, "RuntimeDataBox", 1, 2));
        block.add_instruction(copy(11, 10));
        block.add_instruction(const_i(12, 1));
        block.add_instruction(binop(13, BinaryOp::Add, 11, 12));
        block.add_instruction(array_set("RuntimeDataBox", 1, 2, 13));
        block.add_instruction(copy(14, 10));

        refresh_function_array_rmw_window_routes(&mut function);

        assert!(function.metadata.array_rmw_window_routes.is_empty());
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

    fn binop(dst: u32, op: BinaryOp, lhs: u32, rhs: u32) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(lhs),
            rhs: ValueId::new(rhs),
        }
    }

    fn array_get(dst: u32, box_name: &str, array: u32, index: u32) -> MirInstruction {
        method_call(Some(dst), box_name, "get", array, vec![ValueId::new(index)])
    }

    fn array_set(box_name: &str, array: u32, index: u32, value: u32) -> MirInstruction {
        method_call(
            None,
            box_name,
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
