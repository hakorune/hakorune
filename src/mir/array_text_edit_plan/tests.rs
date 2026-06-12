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
