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
