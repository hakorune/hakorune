use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::MirModule;
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

/// Visible optimizer schedule groups.
///
/// These are facade groups only. They document the stable top-level ordering
/// while the existing subpasses remain the behavior-owning implementation.
pub const MIR_OPT_PIPELINE_GROUPS: &[&str] = &[
    "normalize_frontend_surface",
    "placement_effect_pre",
    "canonical_simplification",
    "memory_cleanup_wave",
    "placement_effect_post",
    "late_call_and_inline",
    "optional_and_diagnostics",
];

pub fn mir_opt_pipeline_groups() -> &'static [&'static str] {
    MIR_OPT_PIPELINE_GROUPS
}

impl MirOptimizer {
    /// Create new optimizer
    pub fn new() -> Self {
        Self { debug: false }
    }

    pub(in crate::mir) fn optimize_module_with_required_roots(
        &mut self,
        module: &mut MirModule,
        roots: &crate::mir::passes::dce::RequiredLivenessRoots,
    ) -> OptimizationStats {
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
                get_global_ring0().log.debug(
                    "[mir-opt] disabled for planner-required/env gate (returning without passes)",
                );
            }
            return stats;
        }

        if self.debug {
            get_global_ring0()
                .log
                .debug("🚀 Starting MIR optimization passes");
        }

        self.run_normalize_frontend_surface(module, &mut stats);
        self.run_placement_effect_pre(module, &mut stats);
        self.run_canonical_simplification(module, &mut stats, roots);
        self.run_memory_cleanup_wave(module, &mut stats, roots);
        self.run_placement_effect_post(module, &mut stats);
        self.run_late_call_and_inline(module, &mut stats);
        self.run_optional_and_diagnostics(module, &mut stats);

        stats
    }

    fn run_normalize_frontend_surface(
        &mut self,
        module: &mut MirModule,
        stats: &mut OptimizationStats,
    ) {
        let core13 = crate::config::env::mir_core13();
        let mut ref_to_boxcall = crate::config::env::mir_ref_boxcall();
        if core13 {
            ref_to_boxcall = true;
        }

        // Pass 0: Normalize legacy instructions to unified forms.
        // Includes optional Array→BoxCall guarded by env inside the pass.
        stats.merge(
            crate::mir::optimizer_passes::normalize::normalize_legacy_instructions(self, module),
        );
        // Pass 0.1: RefGet/RefSet → BoxCall(getField/setField) (guarded).
        if ref_to_boxcall {
            stats.merge(
                crate::mir::optimizer_passes::normalize::normalize_ref_field_access(self, module),
            );
        }

        // Normalize Python helper form: py.getattr(obj, name) → obj.getattr(name).
        stats.merge(
            crate::mir::optimizer_passes::normalize::normalize_python_helper_calls(self, module),
        );
    }

    fn run_placement_effect_pre(&mut self, module: &mut MirModule, stats: &mut OptimizationStats) {
        // Run the first generic placement/effect transform owner seam before
        // DCE so dead intermediate borrowed-string values can be removed in
        // the same optimize wave.
        let placement_effect_rewrites =
            crate::mir::passes::placement_effect_transform::apply_pre_dce_transforms(module);
        if placement_effect_rewrites > 0 {
            stats.intrinsic_optimizations += placement_effect_rewrites;
        }
    }

    fn run_canonical_simplification(
        &mut self,
        module: &mut MirModule,
        stats: &mut OptimizationStats,
        roots: &crate::mir::passes::dce::RequiredLivenessRoots,
    ) {
        // Semantic simplification bundle owner seam. Current cut keeps
        // behavior identical by bundling the landed DCE and CSE passes under
        // one top-level owner.
        stats.merge(
            crate::mir::passes::semantic_simplification::apply_with_required_roots(module, roots),
        );
    }

    fn run_memory_cleanup_wave(
        &mut self,
        module: &mut MirModule,
        stats: &mut OptimizationStats,
        roots: &crate::mir::passes::dce::RequiredLivenessRoots,
    ) {
        // Memory-effect layer owner seam. This remains separate from DCE so
        // future store/load widening can grow without re-burying memory logic
        // inside DCE.
        stats.merge(crate::mir::passes::memory_effect::apply_with_required_roots(module, roots));

        // Rerun pure DCE after memory effects; this can expose newly dead pure
        // defs after private-carrier Load/Store cleanup.
        stats.dead_code_eliminated +=
            crate::mir::passes::dce::eliminate_dead_code_with_required_roots(module, roots);
    }

    fn run_placement_effect_post(&mut self, module: &mut MirModule, stats: &mut OptimizationStats) {
        // Rerun placement/effect after the cleanup wave. Some string corridor
        // length-pair fusions only become single-use after dead temps and
        // memory cleanup have both run.
        let placement_effect_reruns =
            crate::mir::passes::placement_effect_transform::apply_post_dce_transforms(module);
        if placement_effect_reruns > 0 {
            stats.intrinsic_optimizations += placement_effect_reruns;
        }
    }

    fn run_late_call_and_inline(&mut self, module: &mut MirModule, stats: &mut OptimizationStats) {
        // Reserved hooks stay in their current order until a separate
        // classification card retires or hides them from the visible schedule.
        stats.merge(crate::mir::optimizer_passes::reorder::reorder_pure_instructions(self, module));
        stats.merge(
            crate::mir::optimizer_passes::intrinsics::optimize_intrinsic_calls(self, module),
        );

        stats.merge(
            crate::mir::optimizer_passes::boxfield::optimize_boxfield_operations(self, module),
        );

        let updates = crate::mir::passes::type_hints::propagate_param_type_hints(module);
        if updates > 0 {
            stats.intrinsic_optimizations += updates as usize;
        }

        // Inline consumes refreshed MIR InlinePlan metadata only.
        crate::mir::rune_plan_refresh::refresh_module_rune_plans(module);
        let inline_soft_leaf = crate::mir::passes::inline_soft_leaf::apply(module);
        if inline_soft_leaf > 0 {
            stats.intrinsic_optimizations += inline_soft_leaf;
        }
    }

    fn run_optional_and_diagnostics(
        &mut self,
        module: &mut MirModule,
        stats: &mut OptimizationStats,
    ) {
        if self.debug {
            get_global_ring0()
                .log
                .debug(&format!("✅ Optimization complete: {}", stats));
        }

        let diag1 =
            crate::mir::optimizer_passes::diagnostics::diagnose_unlowered_type_ops(self, module);
        stats.merge(diag1);

        let diag2 =
            crate::mir::optimizer_passes::diagnostics::diagnose_legacy_instructions(self, module);
        stats.merge(diag2);
    }

}

impl MirOptimizer {
    /// Expose debug flag for helper modules
    pub(crate) fn debug_enabled(&self) -> bool {
        self.debug
    }
}

impl Default for MirOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
