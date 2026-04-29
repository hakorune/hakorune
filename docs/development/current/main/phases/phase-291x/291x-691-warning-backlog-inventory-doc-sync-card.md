---
Status: Landed
Date: 2026-04-29
Scope: docs + inventory
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-29ai/tier3-completion-summary.md
  - docs/development/current/main/phases/phase-29an/P2-PLANNER-SKELETON-FEATURE-STAGING-INSTRUCTIONS.md
---

# 291x-691: Warning Backlog Inventory and Current-Docs Sync

## Why

The phase-291x cleanup burst kept landing code cards after the current mirrors
stopped at `291x-666`. That left the current-state docs with two kinds of drift:

1. stale hard-coded pointers to `291x-666`
2. no compact inventory of the remaining low-risk cleanup backlog

This card fixes the docs first before reopening the next compiler-cleanliness
code lane.

## Inventory Snapshot

- Current baseline: `cargo build --release` reports **48** `nyash-rust (lib)`
  warnings.
- Current worktree target: keep the tree clean and continue only through
  focused BoxShape cards.
- False-positive wall (do not delete without owner-path proof):
  - `normalizer/simple_while_coreloop_builder.rs::build_simple_while_coreloop`
  - `features/edgecfg_stubs.rs::build_loop_cond_branch`
  - `generic_loop/body_check_shape_detectors/utils.rs::is_loop_var_minus_one`
  - `loop_cond_continue_with_return_phi_materializer.rs::current_bindings_mut`
- Highest-confidence next code cleanup clusters:
  - `features/loop_cond_bc_else_patterns/guard_break.rs`
  - `parts/conditional_update.rs`
  - `planner/helpers.rs` (`infer_skeleton_kind`, `infer_exit_usage`) audit
  - `generic_loop/body_check_shape_detectors/nested_loop_program2.rs` matcher
    audit
- Hold / annotate instead of deleting:
  - `trim_lowerer.rs`
  - `trim_validator.rs`
  - `condition_env_builder.rs`

## Docs Sync Done

- `CURRENT_STATE.toml` latest-card pointer now targets this inventory card.
- `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, `10-Now.md`, and the
  phase-291x README no longer hard-pin `291x-666` as the latest checkpoint.
- Historical phase docs (`phase-29ai`, `phase-29an`) now note that
  `infer_skeleton_kind` / `infer_exit_usage` are historical staging vocabulary
  and are currently under compiler-cleanliness audit.

## Next Cards

1. Prune true-dead loop-cond conditional-update residue.
2. Audit planner helper residues versus direct `CanonicalLoopFacts` access.
3. Audit the remaining generic-loop matcher shelf before deleting exported
   helpers.

## Proof

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
cargo build --release
```
