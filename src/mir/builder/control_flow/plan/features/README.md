## plan/features

Responsibility: apply minimal "feature" deltas to an existing skeleton/CorePlan.

Rules:
- feature code only; no AST re-parse or shape detection
- no CorePlan skeleton allocation here (done in skeletons/)
- strict/dev-only expansions stay behind planner-required gates
- ops/pipeline must not insert PHI directly or handcraft BranchStub; use feature helpers (loop_carriers/edgecfg_stubs/carrier_merge/if_join)
- ops/pipeline must attach loop phis via `loop_carriers::with_loop_carriers` (no direct `phis` assignment)
- ops/pipeline must set `step_mode` via `features::step_mode` (`plan::step_mode` SSOT adapter; no direct `LoopStepMode::*` literals)
- pipeline apply-order is SSOT; do not reorder without docs update
- helper boundary SSOT: `docs/development/current/main/design/feature-helper-boundary-ssot.md`

Naming / navigation:
- `plan/<kind>/facts.rs` is the conservative entry extraction for a specific plan kind.
- `plan/<kind>/normalizer/*` should be a thin adapter that calls a pipeline.
- reusable “lego parts” live here (`plan/features/*`) and must stay kind-agnostic.

Pipeline:
- generic_loop pipeline applies features in fixed order:
  - step/cond (`features/generic_loop_step.rs`)
  - body (`features/generic_loop_body.rs`)
  - carriers finalize (v1)
- entry: `features/generic_loop_pipeline.rs` (skeleton in → CorePlan out)
- loop pipelines:
  - loop_true_break_continue: `features/loop_true_break_continue_pipeline.rs`
  - loop_cond_break_continue: `features/loop_cond_break_continue_pipeline.rs`
- scan/split pipelines:
  - scan_with_init: `features/scan_with_init_pipeline.rs` + `features/scan_with_init_ops.rs`
  - split_scan: `features/split_scan_pipeline.rs` + `features/split_scan_ops.rs`
  - split-specific emit/match: `features/split_emit.rs`
- pattern5 pipeline:
  - pattern5_infinite_early_exit: `features/pattern5_infinite_early_exit_pipeline.rs` + `features/pattern5_infinite_early_exit_ops.rs`

Exit-branch helper:
- `features/exit_branch.rs`: shared “exit branch” extractor/lowerer (prelude + ExitKind) used by `exit_if_map` and BranchN/match (SSOT: `docs/development/current/main/design/exit-branch-feature-ssot.md`).
