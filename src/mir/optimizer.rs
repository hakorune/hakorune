/*!
 * MIR Optimizer - Phase 3 Implementation
 *
 * Implements Effect System based optimizations for the new 26-instruction MIR
 * - Pure instruction reordering and CSE (Common Subexpression Elimination)
 * - BoxFieldLoad/Store dependency analysis
 * - Intrinsic function optimization
 * - Dead code elimination
 */

use super::{MirFunction, MirInstruction, MirModule, MirType, ValueId};
use crate::mir::optimizer_stats::OptimizationStats;
use crate::runtime::get_global_ring0;

mod diagnostics;

/// MIR optimization passes
pub struct MirOptimizer {
    /// Enable debug output for optimization passes
    debug: bool,
}

/// Phase 29x X63: Optimization safe-set vocabulary lock (policy only).
///
/// Notes:
/// - This is a lane-level contract for runtime-core extension progression.
/// - Activation strategy is handled by phase gates/docs; this constant only pins names.
pub const PHASE29X_OPT_SAFESET: &[&str] = &["const_fold", "dce", "cfg_simplify"];

pub fn phase29x_opt_safeset() -> &'static [&'static str] {
    PHASE29X_OPT_SAFESET
}

impl MirOptimizer {
    /// Create new optimizer
    pub fn new() -> Self {
        Self { debug: false }
    }

    /// Enable debug output
    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// Run all optimization passes on a MIR module
    pub fn optimize_module(&mut self, module: &mut MirModule) -> OptimizationStats {
        let mut stats = OptimizationStats::new();

        // Dev/diagnostic: allow disabling optimizer entirely via env gate
        // Default OFF (no behavior change). When ON, return immediately with empty stats.
        // Accepted keys: NYASH_MIR_DISABLE_OPT=1 or HAKO_MIR_DISABLE_OPT=1
        let disable_opt = std::env::var("NYASH_MIR_DISABLE_OPT").ok().as_deref() == Some("1")
            || std::env::var("HAKO_MIR_DISABLE_OPT").ok().as_deref() == Some("1");
        if disable_opt {
            if self.debug {
                get_global_ring0()
                    .log
                    .debug("[mir-opt] disabled by env (returning without passes)");
            }
            return stats;
        }

        if self.debug {
            get_global_ring0()
                .log
                .debug("🚀 Starting MIR optimization passes");
        }

        // Env toggles for phased MIR cleanup
        let core13 = crate::config::env::mir_core13();
        let mut ref_to_boxcall = crate::config::env::mir_ref_boxcall();
        if core13 {
            ref_to_boxcall = true;
        }

        // Pass 0: Normalize legacy instructions to unified forms
        //  - Includes optional Array→BoxCall guarded by env (inside the pass)
        stats.merge(
            crate::mir::optimizer_passes::normalize::normalize_legacy_instructions(self, module),
        );
        // Pass 0.1: RefGet/RefSet → BoxCall(getField/setField) (guarded)
        if ref_to_boxcall {
            stats.merge(
                crate::mir::optimizer_passes::normalize::normalize_ref_field_access(self, module),
            );
        }

        // Normalize Python helper form: py.getattr(obj, name) → obj.getattr(name)
        stats.merge(
            crate::mir::optimizer_passes::normalize::normalize_python_helper_calls(self, module),
        );

        // Step 5: run the first generic placement/effect transform owner seam
        // before DCE so dead intermediate borrowed-string values can be removed
        // in the same optimize wave.
        let placement_effect_rewrites =
            crate::mir::passes::placement_effect_transform::apply_pre_dce_transforms(module);
        if placement_effect_rewrites > 0 {
            stats.intrinsic_optimizations += placement_effect_rewrites;
        }

        // Pass 1: semantic simplification bundle owner seam
        // Current cut keeps behavior identical by bundling the landed DCE and
        // CSE passes under one top-level owner.
        stats.merge(crate::mir::passes::semantic_simplification::apply(module));

        // Step 5.1: rerun the generic placement/effect transform owner seam
        // after DCE. The first sweep introduces `substring_len_hii`, but
        // complementary length-pair fusion may only become single-use once
        // dead substring temps are removed in the same optimization wave.
        let placement_effect_reruns =
            crate::mir::passes::placement_effect_transform::apply_post_dce_transforms(module);
        if placement_effect_reruns > 0 {
            stats.intrinsic_optimizations += placement_effect_reruns;
        }

        // Pass 2: Pure instruction reordering for better locality
        stats.merge(crate::mir::optimizer_passes::reorder::reorder_pure_instructions(self, module));

        // Pass 3: Intrinsic function optimization
        stats.merge(
            crate::mir::optimizer_passes::intrinsics::optimize_intrinsic_calls(self, module),
        );

        // Safety-net passesは削除（Phase 2: 変換の一本化）。診断のみ後段で実施。

        // Pass 4: BoxField dependency optimization
        stats.merge(
            crate::mir::optimizer_passes::boxfield::optimize_boxfield_operations(self, module),
        );

        // Pass 5: 受け手型ヒントの伝搬（callsite→callee）
        // 目的: helper(arr){ return arr.length() } のようなケースで、
        //       呼び出し元の引数型（String/Integer/Bool/Float）を callee の params に反映し、
        //       Lowererがより正確にBox種別を選べるようにする。
        let updates = crate::mir::passes::type_hints::propagate_param_type_hints(module);
        if updates > 0 {
            stats.intrinsic_optimizations += updates as usize;
        }

        // Pass 5.5: Call-site canonicalization lane entry (MCL-0 scaffold)
        let canonicalized =
            crate::mir::passes::callsite_canonicalize::canonicalize_callsites(module);
        if canonicalized > 0 {
            stats.intrinsic_optimizations += canonicalized;
        }

        // Pass 5.6 (opt-in): String concat chain canonicalization
        //   (a + b) + c / a + (b + c) -> call extern nyash.string.concat3_hhh(a, b, c)
        // NOTE: kept behind env gate while tuning perf parity with backend-local concat folding.
        if std::env::var("NYASH_MIR_CONCAT3_CANON").ok().as_deref() == Some("1") {
            let concat3 =
                crate::mir::passes::concat3_canonicalize::canonicalize_string_concat3(module);
            if concat3 > 0 {
                stats.intrinsic_optimizations += concat3;
            }
        }

        // Pass 6 (optional): Core-13 pure normalization
        if crate::config::env::mir_core13_pure() {
            stats.merge(
                crate::mir::optimizer_passes::normalize_core13_pure::normalize_pure_core13(
                    self, module,
                ),
            );
        }

        if self.debug {
            get_global_ring0()
                .log
                .debug(&format!("✅ Optimization complete: {}", stats));
        }
        // Diagnostics (informational): report unlowered patterns
        let diag1 =
            crate::mir::optimizer_passes::diagnostics::diagnose_unlowered_type_ops(self, module);
        stats.merge(diag1);
        // Diagnostics (policy): detect legacy (pre-unified) instructions when requested
        let diag2 =
            crate::mir::optimizer_passes::diagnostics::diagnose_legacy_instructions(self, module);
        stats.merge(diag2);

        stats
    }

    /// Convert instruction to string key for CSE
    #[allow(dead_code)]
    fn instruction_to_key(&self, instruction: &MirInstruction) -> String {
        match instruction {
            MirInstruction::Const { value, .. } => format!("const_{:?}", value),
            MirInstruction::BinOp { op, lhs, rhs, .. } => {
                format!("binop_{:?}_{}_{}", op, lhs.as_u32(), rhs.as_u32())
            }
            MirInstruction::Compare { op, lhs, rhs, .. } => {
                format!("cmp_{:?}_{}_{}", op, lhs.as_u32(), rhs.as_u32())
            }
            // MirInstruction::BoxFieldLoad { box_val, field, .. } => format!("boxload_{}_{}", box_val.as_u32(), field),
            MirInstruction::Call { func, args, .. } => {
                let args_str = args
                    .iter()
                    .map(|v| v.as_u32().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("call_{}_{}", func.as_u32(), args_str)
            }
            _ => format!("other_{:?}", instruction),
        }
    }
}

impl MirOptimizer {
    /// Expose debug flag for helper modules
    pub(crate) fn debug_enabled(&self) -> bool {
        self.debug
    }
}

impl MirOptimizer {
    /// Normalize Python helper calls that route via PyRuntimeBox into proper receiver form.
    ///
    /// Rewrites: BoxCall { box_val=py (PyRuntimeBox), method="getattr"|"call", args=[obj, rest...] }
    ///        →  BoxCall { box_val=obj, method, args=[rest...] }
    #[allow(dead_code)]
    fn normalize_python_helper_calls(&mut self, module: &mut MirModule) -> OptimizationStats {
        crate::mir::optimizer_passes::normalize::normalize_python_helper_calls(self, module)
    }
    /// Normalize legacy instructions into unified MIR26 forms.
    /// - WeakRef(Load) を canonical 表現として維持（WeakNew/WeakLoad は enum remove 済み）
    /// - Barrier は unified op として維持（BarrierRead/BarrierWrite は enum remove 済み）
    #[allow(dead_code)]
    fn normalize_legacy_instructions(&mut self, module: &mut MirModule) -> OptimizationStats {
        crate::mir::optimizer_passes::normalize::normalize_legacy_instructions(self, module)
    }

    /// Normalize RefGet/RefSet to BoxCall("getField"/"setField") with Const String field argument.
    #[allow(dead_code)]
    fn normalize_ref_field_access(&mut self, module: &mut MirModule) -> OptimizationStats {
        crate::mir::optimizer_passes::normalize::normalize_ref_field_access(self, module)
    }
}

impl Default for MirOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, FunctionSignature, MirFunction, MirModule, MirType,
        TypeOpKind, ValueId,
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
        assert_eq!(stats.total_optimizations(), 8);

        let other_stats = OptimizationStats {
            dead_code_eliminated: 2,
            cse_eliminated: 1,
            ..Default::default()
        };

        stats.merge(other_stats);
        assert_eq!(stats.dead_code_eliminated, 7);
        assert_eq!(stats.cse_eliminated, 4);
        assert_eq!(stats.total_optimizations(), 11);
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
                layout:
                    crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
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
}
