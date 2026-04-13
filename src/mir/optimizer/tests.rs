use super::*;
use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, TypeOpKind, ValueId,
};
use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

struct EnvVarRestore {
    entries: Vec<(&'static str, Option<String>)>,
}

impl EnvVarRestore {
    fn set(vars: &[(&'static str, &'static str)]) -> Self {
        let mut entries = Vec::with_capacity(vars.len());
        for (key, value) in vars {
            entries.push((*key, std::env::var(key).ok()));
            std::env::set_var(key, value);
        }
        Self { entries }
    }
}

impl Drop for EnvVarRestore {
    fn drop(&mut self) {
        for (key, old) in self.entries.drain(..) {
            if let Some(value) = old {
                std::env::set_var(key, value);
            } else {
                std::env::remove_var(key);
            }
        }
    }
}

#[test]
fn test_optimizer_creation() {
    let optimizer = MirOptimizer::new();
    assert!(!optimizer.debug);

    let debug_optimizer = MirOptimizer::new().with_debug();
    assert!(debug_optimizer.debug);
}

#[test]
fn test_optimization_stats() {
    let mut stats = OptimizationStats::new();
    assert_eq!(stats.total_optimizations(), 0);

    stats.dead_code_eliminated = 5;
    stats.cse_eliminated = 3;
    stats.cfg_simplified = 2;
    stats.memory_effect_optimizations = 4;
    assert_eq!(stats.total_optimizations(), 14);

    let other_stats = OptimizationStats {
        dead_code_eliminated: 2,
        cse_eliminated: 1,
        cfg_simplified: 4,
        memory_effect_optimizations: 1,
        ..Default::default()
    };

    stats.merge(other_stats);
    assert_eq!(stats.dead_code_eliminated, 7);
    assert_eq!(stats.cse_eliminated, 4);
    assert_eq!(stats.cfg_simplified, 6);
    assert_eq!(stats.memory_effect_optimizations, 5);
    assert_eq!(stats.total_optimizations(), 22);
}

#[test]
fn test_instruction_to_key() {
    let optimizer = MirOptimizer::new();

    let const_instr = MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(42),
    };

    let key = optimizer.instruction_to_key(&const_instr);
    assert!(key.contains("const"));
    assert!(key.contains("42"));
}

#[test]
fn mir_optimizer_phase29x_allowlist_lock() {
    assert_eq!(
        phase29x_opt_safeset(),
        &["const_fold", "dce", "cfg_simplify"]
    );
}

#[test]
fn test_dce_does_not_drop_typeop_used_by_console_log() {
    // Build: %v=TypeOp(check); extern_call env.console.log(%v); ensure TypeOp remains after optimize
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    let v1 = ValueId::new(1);
    b0.add_instruction(MirInstruction::NewBox {
        dst: v0,
        box_type: "IntegerBox".to_string(),
        args: vec![],
    });
    b0.add_instruction(MirInstruction::TypeOp {
        dst: v1,
        op: TypeOpKind::Check,
        value: v0,
        ty: MirType::Integer,
    });
    b0.add_instruction(crate::mir::ssot::extern_call::extern_call(
        None,
        "env.console".to_string(),
        "log".to_string(),
        vec![v1],
        super::super::effect::EffectMask::IO,
    ));
    b0.add_instruction(MirInstruction::Return { value: None });
    func.add_block(b0);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let mut opt = MirOptimizer::new();
    let _stats = opt.optimize_module(&mut module);

    // Ensure TypeOp remains in bb0
    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    let has_typeop = block
        .all_spanned_instructions()
        .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }));
    assert!(
        has_typeop,
        "TypeOp should not be dropped by DCE when used by console.log (ExternCall)"
    );
}

#[test]
fn test_method_call_not_rewritten_even_if_legacy_env_is_set() {
    let _guard = env_guard().lock().expect("env mutex poisoned");
    let _restore =
        EnvVarRestore::set(&[("NYASH_MIR_PLUGIN_INVOKE", "1"), ("NYASH_PLUGIN_ONLY", "1")]);

    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    let v1 = ValueId::new(1);
    b0.add_instruction(MirInstruction::NewBox {
        dst: v0,
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    b0.add_instruction(MirInstruction::Const {
        dst: v1,
        value: ConstValue::Integer(0),
    });
    // Use canonical Call with Callee::Method (replaces BoxCall)
    b0.add_instruction(super::super::ssot::method_call::runtime_method_call(
        None,
        v0,
        "ArrayBox",
        "set",
        vec![v1, v1],
        super::super::effect::EffectMask::WRITE,
        super::super::definitions::call_unified::TypeCertainty::Known,
    ));
    b0.set_terminator(MirInstruction::Return { value: None });
    func.add_block(b0);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let mut opt = MirOptimizer::new();
    let _stats = opt.optimize_module(&mut module);

    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    let mut has_method_call = false;
    for sp in block.all_spanned_instructions() {
        match sp.inst {
            MirInstruction::Call {
                callee: Some(super::super::Callee::Method { .. }),
                ..
            } => has_method_call = true,
            _ => {}
        }
    }
    assert!(has_method_call, "Call(Method) should remain Call(Method)");
}

#[test]
fn test_normalize_keeps_weakref_load_instruction() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    let v1 = ValueId::new(1);
    let v2 = ValueId::new(2);
    b0.add_instruction(MirInstruction::NewBox {
        dst: v0,
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    b0.add_instruction(MirInstruction::WeakRef {
        dst: v1,
        op: crate::mir::WeakRefOp::New,
        value: v0,
    });
    b0.add_instruction(MirInstruction::WeakRef {
        dst: v2,
        op: crate::mir::WeakRefOp::Load,
        value: v1,
    });
    b0.set_terminator(MirInstruction::Return { value: None });
    func.add_block(b0);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let mut opt = MirOptimizer::new();
    let _stats = opt.optimize_module(&mut module);

    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    let mut has_weakref_load = false;
    for sp in block.all_spanned_instructions() {
        match sp.inst {
            MirInstruction::WeakRef {
                op: crate::mir::WeakRefOp::Load,
                ..
            } => has_weakref_load = true,
            _ => {}
        }
    }
    assert!(
        has_weakref_load,
        "WeakRef(Load) should remain representable in normalized MIR"
    );
}

#[test]
fn test_normalize_keeps_barrier_read_instruction() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    b0.add_instruction(MirInstruction::Const {
        dst: v0,
        value: ConstValue::Integer(1),
    });
    b0.add_instruction(MirInstruction::Barrier {
        op: crate::mir::BarrierOp::Read,
        ptr: v0,
    });
    b0.set_terminator(MirInstruction::Return { value: None });
    func.add_block(b0);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let mut opt = MirOptimizer::new();
    let _stats = opt.optimize_module(&mut module);

    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    let mut has_barrier_read_unified = false;
    for sp in block.all_spanned_instructions() {
        match sp.inst {
            MirInstruction::Barrier {
                op: crate::mir::BarrierOp::Read,
                ..
            } => has_barrier_read_unified = true,
            _ => {}
        }
    }
    assert!(
        has_barrier_read_unified,
        "Barrier(Read) should remain representable in normalized MIR"
    );
}

#[test]
fn test_normalize_keeps_barrier_write_instruction() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    b0.add_instruction(MirInstruction::Const {
        dst: v0,
        value: ConstValue::Integer(1),
    });
    b0.add_instruction(MirInstruction::Barrier {
        op: crate::mir::BarrierOp::Write,
        ptr: v0,
    });
    b0.set_terminator(MirInstruction::Return { value: None });
    func.add_block(b0);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    let mut opt = MirOptimizer::new();
    let _stats = opt.optimize_module(&mut module);

    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    let mut has_barrier_write_unified = false;
    for sp in block.all_spanned_instructions() {
        match sp.inst {
            MirInstruction::Barrier {
                op: crate::mir::BarrierOp::Write,
                ..
            } => has_barrier_write_unified = true,
            _ => {}
        }
    }
    assert!(
        has_barrier_write_unified,
        "Barrier(Write) should remain representable in normalized MIR"
    );
}

#[test]
fn test_dce_keeps_edge_args_values() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let bb1 = BasicBlockId::new(1);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    let v1 = ValueId::new(1);
    b0.add_instruction(MirInstruction::Const {
        dst: v0,
        value: ConstValue::Integer(1),
    });
    crate::mir::builder::copy_emitter::emit_copy_into_detached_block(
        &mut b0,
        v1,
        v0,
        crate::mir::builder::copy_emitter::CopyEmitReason::TestMirOptimizerDceKeepsEdgeArgsValues,
    )
    .unwrap();
    b0.set_jump_with_edge_args(
        bb1,
        Some(crate::mir::EdgeArgs {
            layout: crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
            values: vec![v1],
        }),
    );
    func.add_block(b0);
    let mut exit_block = BasicBlock::new(bb1);
    exit_block.set_terminator(MirInstruction::Return { value: None });
    func.add_block(exit_block);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    crate::mir::passes::dce::eliminate_dead_code(&mut module);

    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    let has_copy = block
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Copy { .. }));
    assert!(
        has_copy,
        "Copy used only by edge args should not be eliminated"
    );
}

#[test]
fn test_dce_syncs_instruction_spans() {
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: super::super::effect::EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId::new(0));
    let bb0 = BasicBlockId::new(0);
    let mut b0 = BasicBlock::new(bb0);
    let v0 = ValueId::new(0);
    let v1 = ValueId::new(1);
    b0.add_instruction(MirInstruction::Const {
        dst: v0,
        value: ConstValue::Integer(1),
    });
    b0.add_instruction(MirInstruction::Const {
        dst: v1,
        value: ConstValue::Integer(2),
    });
    b0.add_instruction(MirInstruction::Return { value: Some(v0) });
    func.add_block(b0);
    let mut module = MirModule::new("test".to_string());
    module.add_function(func);

    crate::mir::passes::dce::eliminate_dead_code(&mut module);

    let f = module.get_function("main").unwrap();
    let block = f.get_block(bb0).unwrap();
    assert_eq!(
        block.instructions.len(),
        block.instruction_spans.len(),
        "Instruction spans must stay aligned after DCE"
    );
    let has_unused_const = block
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1));
    assert!(
        !has_unused_const,
        "Unused const should be eliminated by DCE"
    );
}
