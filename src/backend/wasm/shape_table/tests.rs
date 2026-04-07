use super::*;
use crate::mir::MirModule;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature,
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

#[test]
fn wasm_shape_table_detects_p10_loop_extern_candidate_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let loop_bb = BasicBlockId(1);
    let exit_bb = BasicBlockId(2);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        entry,
    );

    {
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let cond = ValueId(1);
        block.add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Branch {
            condition: cond,
            then_bb: loop_bb,
            else_bb: exit_bb,
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut loop_block = BasicBlock::new(loop_bb);
    loop_block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId(99),
        callee: Some(Callee::Extern("env.console.log".to_string())),
        args: vec![ValueId(1)],
        effects: EffectMask::IO,
    });
    loop_block.add_instruction(MirInstruction::Jump {
        target: entry,
        edge_args: None,
    });
    function.add_block(loop_block);

    let mut exit_block = BasicBlock::new(exit_bb);
    let out = ValueId(2);
    exit_block.add_instruction(MirInstruction::Const {
        dst: out,
        value: ConstValue::Integer(0),
    });
    exit_block.add_instruction(MirInstruction::Return { value: Some(out) });
    function.add_block(exit_block);

    module.add_function(function);

    let found = detect_p10_loop_extern_call_candidate(&module).expect("p10 candidate should match");
    assert_eq!(found, "wsm.p10.main_loop_extern_call.v0");
}

#[test]
fn wasm_shape_table_rejects_p10_candidate_without_loop_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        entry,
    );

    let block = function
        .get_block_mut(entry)
        .expect("entry block must exist");
    let v = ValueId(1);
    block.add_instruction(MirInstruction::Const {
        dst: v,
        value: ConstValue::Integer(1),
    });
    block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId(99),
        callee: Some(Callee::Extern("env.console.log".to_string())),
        args: vec![v],
        effects: EffectMask::IO,
    });
    block.add_instruction(MirInstruction::Return { value: Some(v) });
    module.add_function(function);

    assert!(
        detect_p10_loop_extern_call_candidate(&module).is_none(),
        "non-loop extern shape must stay outside p10 candidate contract"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min4_native_promotable_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let loop_bb = BasicBlockId(1);
    let exit_bb = BasicBlockId(2);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        entry,
    );

    {
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let cond = ValueId(1);
        block.add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Branch {
            condition: cond,
            then_bb: loop_bb,
            else_bb: exit_bb,
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut loop_block = BasicBlock::new(loop_bb);
    let c3 = ValueId(3);
    loop_block.add_instruction(MirInstruction::Const {
        dst: c3,
        value: ConstValue::Integer(3),
    });
    loop_block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId(99),
        callee: Some(Callee::Method {
            box_name: "console".to_string(),
            method: "log".to_string(),
            receiver: Some(ValueId(98)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![c3],
        effects: EffectMask::IO,
    });
    loop_block.add_instruction(MirInstruction::Jump {
        target: entry,
        edge_args: None,
    });
    function.add_block(loop_block);

    let mut exit_block = BasicBlock::new(exit_bb);
    let out = ValueId(2);
    exit_block.add_instruction(MirInstruction::Const {
        dst: out,
        value: ConstValue::Integer(0),
    });
    exit_block.add_instruction(MirInstruction::Return { value: Some(out) });
    function.add_block(exit_block);

    module.add_function(function);
    let found = detect_p10_min4_native_promotable_shape(&module)
        .expect("p10 min4 native promotable should match");
    assert_eq!(found, "wsm.p10.main_loop_extern_call.fixed3.v0");
}

#[test]
fn wasm_shape_table_rejects_p10_min4_native_promotable_with_other_calls_contract() {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        entry,
    );

    let block = function
        .get_block_mut(entry)
        .expect("entry block must exist");
    let v = ValueId(1);
    block.add_instruction(MirInstruction::Const {
        dst: v,
        value: ConstValue::Integer(3),
    });
    block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId(99),
        callee: Some(Callee::Extern("env.canvas.fillRect".to_string())),
        args: vec![v],
        effects: EffectMask::IO,
    });
    block.add_instruction(MirInstruction::Return { value: Some(v) });
    module.add_function(function);

    assert!(
        detect_p10_min4_native_promotable_shape(&module).is_none(),
        "shape with non-console extern must stay outside min4 native promotion"
    );
}

fn make_p10_loop_console_method_module(method: &str) -> MirModule {
    let mut module = MirModule::new("test".to_string());
    let entry = BasicBlockId(0);
    let loop_bb = BasicBlockId(1);
    let exit_bb = BasicBlockId(2);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        entry,
    );

    {
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let cond = ValueId(1);
        block.add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Branch {
            condition: cond,
            then_bb: loop_bb,
            else_bb: exit_bb,
            then_edge_args: None,
            else_edge_args: None,
        });
    }

    let mut loop_block = BasicBlock::new(loop_bb);
    let c3 = ValueId(3);
    loop_block.add_instruction(MirInstruction::Const {
        dst: c3,
        value: ConstValue::Integer(3),
    });
    loop_block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId(99),
        callee: Some(Callee::Method {
            box_name: "console".to_string(),
            method: method.to_string(),
            receiver: Some(ValueId(98)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![c3],
        effects: EffectMask::IO,
    });
    loop_block.add_instruction(MirInstruction::Jump {
        target: entry,
        edge_args: None,
    });
    function.add_block(loop_block);

    let mut exit_block = BasicBlock::new(exit_bb);
    let out = ValueId(2);
    exit_block.add_instruction(MirInstruction::Const {
        dst: out,
        value: ConstValue::Integer(0),
    });
    exit_block.add_instruction(MirInstruction::Return { value: Some(out) });
    function.add_block(exit_block);
    module.add_function(function);
    module
}

#[test]
fn wasm_shape_table_detects_p10_min5_expansion_warn_inventory_contract() {
    let module = make_p10_loop_console_method_module("warn");
    let found = detect_p10_min5_expansion_inventory_shape(&module)
        .expect("warn inventory shape should match");
    assert_eq!(
        found,
        "wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min5_expansion_info_inventory_contract() {
    let module = make_p10_loop_console_method_module("info");
    let found = detect_p10_min5_expansion_inventory_shape(&module)
        .expect("info inventory shape should match");
    assert_eq!(
        found,
        "wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min5_expansion_error_inventory_contract() {
    let module = make_p10_loop_console_method_module("error");
    let found = detect_p10_min5_expansion_inventory_shape(&module)
        .expect("error inventory shape should match");
    assert_eq!(
        found,
        "wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min5_expansion_debug_inventory_contract() {
    let module = make_p10_loop_console_method_module("debug");
    let found = detect_p10_min5_expansion_inventory_shape(&module)
        .expect("debug inventory shape should match");
    assert_eq!(
        found,
        "wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0"
    );
}

#[test]
fn wasm_shape_table_rejects_p10_min5_expansion_unknown_method_contract() {
    let module = make_p10_loop_console_method_module("log");
    assert!(
        detect_p10_min5_expansion_inventory_shape(&module).is_none(),
        "log shape belongs to min4 promotion and must stay outside min5 expansion inventory"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min6_warn_native_promotable_contract() {
    let mut module = make_p10_loop_console_method_module("warn");
    let main = module
        .get_function_mut("main")
        .expect("main function exists");
    for block in main.blocks.values_mut() {
        for inst in &mut block.instructions {
            if let MirInstruction::Const { value, .. } = inst {
                if *value == ConstValue::Integer(3) {
                    *value = ConstValue::Integer(4);
                }
            }
        }
    }
    let found = detect_p10_min6_warn_native_promotable_shape(&module)
        .expect("p10 min6 warn native shape should match");
    assert_eq!(found, "wsm.p10.main_loop_extern_call.warn.fixed4.v0");
}

#[test]
fn wasm_shape_table_rejects_p10_min6_warn_native_promotable_for_fixed3_contract() {
    let module = make_p10_loop_console_method_module("warn");
    assert!(
        detect_p10_min6_warn_native_promotable_shape(&module).is_none(),
        "fixed3 warn shape must stay outside min6 promotion and remain min5 inventory"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min7_info_native_promotable_contract() {
    let mut module = make_p10_loop_console_method_module("info");
    let main = module
        .get_function_mut("main")
        .expect("main function exists");
    for block in main.blocks.values_mut() {
        for inst in &mut block.instructions {
            if let MirInstruction::Const { value, .. } = inst {
                if *value == ConstValue::Integer(3) {
                    *value = ConstValue::Integer(4);
                }
            }
        }
    }
    let found = detect_p10_min7_info_native_promotable_shape(&module)
        .expect("p10 min7 info native shape should match");
    assert_eq!(found, "wsm.p10.main_loop_extern_call.info.fixed4.v0");
}

#[test]
fn wasm_shape_table_rejects_p10_min7_info_native_promotable_for_fixed3_contract() {
    let module = make_p10_loop_console_method_module("info");
    assert!(
        detect_p10_min7_info_native_promotable_shape(&module).is_none(),
        "fixed3 info shape must stay outside min7 promotion and remain min5 inventory"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min8_error_native_promotable_contract() {
    let mut module = make_p10_loop_console_method_module("error");
    let main = module
        .get_function_mut("main")
        .expect("main function exists");
    for block in main.blocks.values_mut() {
        for inst in &mut block.instructions {
            if let MirInstruction::Const { value, .. } = inst {
                if *value == ConstValue::Integer(3) {
                    *value = ConstValue::Integer(4);
                }
            }
        }
    }
    let found = detect_p10_min8_error_native_promotable_shape(&module)
        .expect("p10 min8 error native shape should match");
    assert_eq!(found, "wsm.p10.main_loop_extern_call.error.fixed4.v0");
}

#[test]
fn wasm_shape_table_rejects_p10_min8_error_native_promotable_for_fixed3_contract() {
    let module = make_p10_loop_console_method_module("error");
    assert!(
        detect_p10_min8_error_native_promotable_shape(&module).is_none(),
        "fixed3 error shape must stay outside min8 promotion and remain min5 inventory"
    );
}

#[test]
fn wasm_shape_table_detects_p10_min9_debug_native_promotable_contract() {
    let mut module = make_p10_loop_console_method_module("debug");
    let main = module
        .get_function_mut("main")
        .expect("main function exists");
    for block in main.blocks.values_mut() {
        for inst in &mut block.instructions {
            if let MirInstruction::Const { value, .. } = inst {
                if *value == ConstValue::Integer(3) {
                    *value = ConstValue::Integer(4);
                }
            }
        }
    }
    let found = detect_p10_min9_debug_native_promotable_shape(&module)
        .expect("p10 min9 debug native shape should match");
    assert_eq!(found, "wsm.p10.main_loop_extern_call.debug.fixed4.v0");
}

#[test]
fn wasm_shape_table_rejects_p10_min9_debug_native_promotable_for_fixed3_contract() {
    let module = make_p10_loop_console_method_module("debug");
    assert!(
        detect_p10_min9_debug_native_promotable_shape(&module).is_none(),
        "fixed3 debug shape must stay outside min9 promotion and remain min5 inventory"
    );
}
