use super::*;
use crate::mir::MirModule;
use crate::mir::{
    BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirType, ValueId,
};

fn make_module_with_single_const_return(value: i64) -> MirModule {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let dst = ValueId(1);
    let block = function
        .get_block_mut(entry)
        .expect("entry block must exist");
    block.add_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Integer(value),
    });
    block.add_instruction(MirInstruction::Return { value: Some(dst) });
    module.add_function(function);
    module
}

#[test]
fn wasm_shape_table_matches_min_const_return_contract() {
    let module = make_module_with_single_const_return(-1);
    let found = match_native_shape(&module).expect("shape table should match");
    assert_eq!(found.shape.id(), "wsm.p4.main_return_i32_const.v0");
    assert_eq!(found.value, -1);
}

#[test]
fn wasm_shape_table_rejects_non_const_return_contract() {
    let mut module = make_module_with_single_const_return(7);
    let entry = module
        .get_function_mut("main")
        .expect("main should exist")
        .get_block_mut(BasicBlockId(0))
        .expect("entry block should exist");
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(9),
    });

    assert!(
        match_native_shape(&module).is_none(),
        "shape table must fail-fast outside strict pilot shape"
    );
}

#[test]
fn wasm_shape_table_matches_const_copy_return_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let block = function
        .get_block_mut(entry)
        .expect("entry block must exist");
    let const_dst = ValueId(1);
    let copy_dst = ValueId(2);
    block.add_instruction(MirInstruction::Const {
        dst: const_dst,
        value: ConstValue::Integer(8),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: copy_dst,
        src: const_dst,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(copy_dst),
    });
    module.add_function(function);

    let found = match_native_shape(&module).expect("const-copy-return shape should match");
    assert_eq!(found.shape.id(), "wsm.p5.main_return_i32_const_via_copy.v0");
    assert_eq!(found.value, 8);
}

#[test]
fn wasm_shape_table_matches_const_binop_return_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let block = function
        .get_block_mut(entry)
        .expect("entry block must exist");
    let lhs = ValueId(1);
    let rhs = ValueId(2);
    let out = ValueId(3);
    block.add_instruction(MirInstruction::Const {
        dst: lhs,
        value: ConstValue::Integer(40),
    });
    block.add_instruction(MirInstruction::Const {
        dst: rhs,
        value: ConstValue::Integer(2),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: out,
        op: BinaryOp::Add,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return { value: Some(out) });
    module.add_function(function);

    let found = match_native_shape(&module).expect("const-binop-return shape should match");
    assert_eq!(found.shape.id(), "wsm.p9.main_return_i32_const_binop.v0");
    assert_eq!(found.value, 42);
}

#[test]
fn wasm_shape_table_rejects_const_binop_div_by_zero_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let block = function
        .get_block_mut(entry)
        .expect("entry block must exist");
    let lhs = ValueId(1);
    let rhs = ValueId(2);
    let out = ValueId(3);
    block.add_instruction(MirInstruction::Const {
        dst: lhs,
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Const {
        dst: rhs,
        value: ConstValue::Integer(0),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: out,
        op: BinaryOp::Div,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return { value: Some(out) });
    module.add_function(function);

    assert!(
        match_native_shape(&module).is_none(),
        "const-binop-return must fail-fast on invalid arithmetic"
    );
}
