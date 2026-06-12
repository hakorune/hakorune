use super::*;
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};

#[test]
fn test_backend_creation() {
    let _backend = WasmBackend::new();
    assert!(true);
}

#[test]
fn test_empty_module_compilation() {
    let mut backend = WasmBackend::new();
    let module = MirModule::new("test".to_string());
    let result = backend.compile_to_wat(module);
    assert!(result.is_ok());
}

#[test]
fn test_compile_to_wasm_skips_unreachable_branch_helper_contract() {
    let mut module = MirModule::new("test".to_string());

    let main_entry = BasicBlockId::new(0);
    let mut main_func = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        },
        main_entry,
    );
    {
        let block = main_func
            .get_block_mut(main_entry)
            .expect("main entry block must exist");
        let out = ValueId::new(1);
        block.add_instruction(MirInstruction::Const {
            dst: out,
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Return { value: Some(out) });
    }
    module.add_function(main_func);

    let dead_entry = BasicBlockId::new(0);
    let then_bb = BasicBlockId::new(1);
    let else_bb = BasicBlockId::new(2);
    let mut dead_func = MirFunction::new(
        FunctionSignature {
            name: "dead_branch".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        },
        dead_entry,
    );
    {
        let block = dead_func
            .get_block_mut(dead_entry)
            .expect("dead helper entry block must exist");
        let cond = ValueId::new(1);
        block.add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Branch {
            condition: cond,
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        });
    }
    let mut then_block = BasicBlock::new(then_bb);
    let then_out = ValueId::new(2);
    then_block.add_instruction(MirInstruction::Const {
        dst: then_out,
        value: ConstValue::Integer(1),
    });
    then_block.add_instruction(MirInstruction::Return {
        value: Some(then_out),
    });
    dead_func.add_block(then_block);

    let mut else_block = BasicBlock::new(else_bb);
    let else_out = ValueId::new(3);
    else_block.add_instruction(MirInstruction::Const {
        dst: else_out,
        value: ConstValue::Integer(0),
    });
    else_block.add_instruction(MirInstruction::Return {
        value: Some(else_out),
    });
    dead_func.add_block(else_block);
    module.add_function(dead_func);

    let mut backend = WasmBackend::new();
    let wat = backend
        .compile_to_wat(module)
        .expect("WAT generation should succeed");
    assert!(wat.contains("(func $main"));
    assert!(!wat.contains("(func $dead_branch"));

    let wasm = backend
        .convert_wat_to_wasm(&wat)
        .expect("unused branching helper must not poison validation");
    assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
}

#[test]
fn test_wat_to_wasm_ascii_guard_fails_fast() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let backend = WasmBackend::new();
    let err = backend
        .convert_wat_to_wasm("(module (func (export \"main\") (result i32) i32.const 0 ;; あ))")
        .expect_err("non-ascii WAT must fail fast");
    let msg = err.to_string();
    assert!(msg.contains("WAT source contains non-ASCII characters"));
}

#[test]
fn test_wat_to_wasm_invalid_wat_fails_fast() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let backend = WasmBackend::new();
    let err = backend
        .convert_wat_to_wasm("(module (func")
        .expect_err("malformed WAT must fail fast");
    let msg = err.to_string();
    assert!(msg.contains("WAT to WASM conversion failed"));
}

#[test]
fn wasm_binary_writer_minimal_module_contract() {
    let backend = WasmBackend::new();
    let wasm = backend
        .build_minimal_i32_const_wasm(7)
        .expect("binary writer helper must succeed");
    assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
}

#[test]
fn wasm_binary_writer_loop_extern_skeleton_contract() {
    let backend = WasmBackend::new();
    let wasm = backend
        .build_loop_extern_call_skeleton_wasm(3)
        .expect("loop extern skeleton helper must succeed");
    assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    assert!(wasm.windows(4).any(|w| w == b"main"));
}

#[test]
fn wasm_binary_writer_extract_min_const_return_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let found = shape_table::match_native_shape(&module).expect("native shape should match");
    assert_eq!(found.value, 7);
    assert_eq!(found.shape.id(), "wsm.p4.main_return_i32_const.v0");
}

#[test]
fn wasm_hako_default_lane_plan_native_for_shape_table_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let backend = WasmBackend::new();
    let plan = backend.plan_hako_default_lane(&module);
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract() {
    let module = MirModule::new("test".to_string());
    let backend = WasmBackend::new();
    let plan = backend.plan_hako_default_lane(&module);
    assert_eq!(plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
}

#[test]
fn wasm_hako_default_lane_trace_includes_shape_id_for_native_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let backend = WasmBackend::new();
    let trace = backend.plan_hako_default_lane_trace(&module);
    assert_eq!(trace.plan, WasmHakoDefaultLanePlan::NativeShapeTable);
    assert_eq!(trace.shape_id, Some("wsm.p4.main_return_i32_const.v0"));
}

#[test]
fn wasm_hako_default_lane_trace_has_none_shape_id_for_bridge_contract() {
    let module = MirModule::new("test".to_string());
    let backend = WasmBackend::new();
    let trace = backend.plan_hako_default_lane_trace(&module);
    assert_eq!(trace.plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
    assert_eq!(trace.shape_id, None);
}

#[test]
fn wasm_hako_native_shape_bytes_emits_for_pilot_shape_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });

    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let bytes = compile_hako_native_shape_bytes(&module)
        .expect("native helper should succeed")
        .expect("pilot shape must emit bytes");
    assert!(bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
}

#[test]
fn wasm_hako_native_shape_bytes_rejects_non_pilot_contract() {
    let module = MirModule::new("test".to_string());
    let bytes = compile_hako_native_shape_bytes(&module)
        .expect("native helper should return Ok(None) for non-pilot");
    assert!(bytes.is_none());
}

#[test]
fn wasm_hako_native_shape_bytes_emits_for_const_copy_return_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    let const_dst = ValueId::new(1);
    let copy_dst = ValueId::new(2);
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

    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let bytes = compile_hako_native_shape_bytes(&module)
        .expect("native helper should succeed")
        .expect("const-copy-return shape must emit bytes");
    assert!(bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
}

#[test]
fn wasm_hako_native_shape_emit_reports_shape_id_for_const_copy_return_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    let const_dst = ValueId::new(1);
    let copy_dst = ValueId::new(2);
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
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let emitted = compile_hako_native_shape_emit(&module)
        .expect("native shape emit should succeed")
        .expect("shape should match");
    assert_eq!(emitted.shape_id, "wsm.p5.main_return_i32_const_via_copy.v0");
    assert!(emitted.bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
}

#[test]
fn wasm_hako_native_shape_emit_reports_shape_id_for_const_binop_return_contract() {
    let sig = FunctionSignature {
        name: "main".to_string(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: crate::mir::EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut func = MirFunction::new(sig, entry);
    let block = func.get_block_mut(entry).expect("entry block");
    let lhs = ValueId::new(1);
    let rhs = ValueId::new(2);
    let out = ValueId::new(3);
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
        op: crate::mir::BinaryOp::Add,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return { value: Some(out) });
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let emitted = compile_hako_native_shape_emit(&module)
        .expect("native shape emit should succeed")
        .expect("shape should match");
    assert_eq!(emitted.shape_id, "wsm.p9.main_return_i32_const_binop.v0");
    assert!(emitted.bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
}
