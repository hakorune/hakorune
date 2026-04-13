use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::{MirInstruction, MirModule};
use crate::runtime::get_global_ring0;

/// MIR optimization passes
pub struct MirOptimizer {
    /// Enable debug output for optimization passes
    pub(crate) debug: bool,
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

        // Compiler-lane contract: strict+planner_required gates validate
        // planning/lowering acceptance, not optimizer behavior. Keep the
        // optimizer out of that lane so selfhost failure-driven work does not
        // get blocked by unrelated optimizer cost or drift.
        let planner_required_lane =
            crate::config::env::joinir_dev::strict_planner_required_enabled();

        // Dev/diagnostic: allow disabling optimizer entirely via env gate.
        // Accepted keys: NYASH_MIR_DISABLE_OPT=1 or HAKO_MIR_DISABLE_OPT=1.
        let disable_opt = std::env::var("NYASH_MIR_DISABLE_OPT").ok().as_deref() == Some("1")
            || std::env::var("HAKO_MIR_DISABLE_OPT").ok().as_deref() == Some("1")
            || planner_required_lane;
        if disable_opt {
            if self.debug {
                get_global_ring0()
                    .log
                    .debug("[mir-opt] disabled for planner-required/env gate (returning without passes)");
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

        // Pass 1.1: memory-effect layer owner seam
        // Current cut keeps the landed private-carrier Load/Store cleanup as a
        // separate owner so future store/load widening can grow without
        // re-burying the logic inside DCE.
        stats.merge(crate::mir::passes::memory_effect::apply(module));

        // Pass 1.2: rerun the landed pure DCE cleanup after memory effects.
        // The memory-effect lane can expose newly dead pure defs, so we keep
        // the pure DCE sweep available as a cleanup pass without re-owning the
        // memory-sensitive Load/Store logic.
        stats.dead_code_eliminated += crate::mir::passes::dce::eliminate_dead_code(module);

        // Step 5.1: rerun the generic placement/effect transform owner seam
        // after the cleanup wave. The first sweep introduces `substring_len_hii`,
        // but complementary length-pair fusion may only become single-use once
        // dead substring temps and memory cleanup have both run in the same
        // optimization wave.
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
    pub(crate) fn instruction_to_key(&self, instruction: &MirInstruction) -> String {
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
    pub(crate) fn normalize_python_helper_calls(
        &mut self,
        module: &mut MirModule,
    ) -> OptimizationStats {
        crate::mir::optimizer_passes::normalize::normalize_python_helper_calls(self, module)
    }
    /// Normalize legacy instructions into unified MIR26 forms.
    /// - WeakRef(Load) を canonical 表現として維持（WeakNew/WeakLoad は enum remove 済み）
    /// - Barrier は unified op として維持（BarrierRead/BarrierWrite は enum remove 済み）
    #[allow(dead_code)]
    pub(crate) fn normalize_legacy_instructions(
        &mut self,
        module: &mut MirModule,
    ) -> OptimizationStats {
        crate::mir::optimizer_passes::normalize::normalize_legacy_instructions(self, module)
    }

    /// Normalize RefGet/RefSet to BoxCall("getField"/"setField") with Const String field argument.
    #[allow(dead_code)]
    pub(crate) fn normalize_ref_field_access(
        &mut self,
        module: &mut MirModule,
    ) -> OptimizationStats {
        crate::mir::optimizer_passes::normalize::normalize_ref_field_access(self, module)
    }
}

impl Default for MirOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
