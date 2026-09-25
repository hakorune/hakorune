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
- Oracle `features/*_pipeline.rs` entries were retired with the ordered
  scheduler / oracle chain (R0 + M12-R2B). Live loop lowering is source-backed
  via `loop_true_break_continue_source.rs`, `loop_cond_bc_source.rs`, and the
  `loop_break_*_source.rs` lowerers feeding the retained
  `loop_true_break_continue_*` / `loop_cond_bc_*` helpers.
- `if_join::apply_if_joins` preserves every reaching `CoreIfJoin` incoming and
  delegates edge repair only to `ssa::phi_input_materializer::for_pred`;
  `pre_val` substitution and branch dropping are forbidden.
- LoopTrueBreakContinue carrier preparation rejects a carrier missing from
  `variable_ctx.variable_map` before allocating any carrier PHI destination.
- loop_true_early_exit route:
  - semantic route: `loop_true_early_exit`
  - current implementation: `recipe_tree/loop_true_early_exit_builder.rs`
  - note: no dedicated `features/*` pipeline file remains

Exit-branch helper:
- `plan/parts/exit_branch.rs` (moved out of features): shared “exit branch” extractor/lowerer (prelude + ExitKind) used by `exit_if_map` and BranchN/match (SSOT: `docs/development/current/main/design/exit-branch-feature-ssot.md`).
