---
Status: Landed
Date: 2026-04-28
Scope: prune PlanNormalizer root cond-lowering re-export facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/normalizer/mod.rs
  - src/mir/builder/control_flow/plan/normalizer/cond_lowering_entry.rs
  - src/mir/builder/control_flow/plan/normalizer/cond_lowering_loop_header.rs
---

# 291x-634: Normalizer Cond Lowering Facade Prune

## Goal

Remove root-level condition-lowering re-exports from `plan::normalizer` so
callers use the cond-lowering owner modules directly.

This is BoxShape cleanup only. It does not change condition lowering,
short-circuit behavior, loop header wiring, or accepted loop shapes.

## Evidence

`plan::normalizer` re-exported:

- `lower_cond_branch`
- `lower_cond_value`
- `lower_bool_expr_value_id`
- `lower_loop_header_cond`

The implementation owners are already explicit:

- `normalizer::cond_lowering_entry`
- `normalizer::cond_lowering_loop_header`

Direct owner-path imports make the condition lowering boundary visible and
remove the backwards-compatibility facade from the normalizer root.

## Boundaries

- Repoint imports only.
- Remove only the root re-export lines.
- Do not modify lowering function bodies or call ordering.
- Keep `PlanNormalizer` methods unchanged.

## Acceptance

- `cargo fmt` completes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Repointed callsites to `cond_lowering_entry` and
  `cond_lowering_loop_header`.
- Removed the normalizer root cond-lowering compatibility re-exports.

## Verification

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
