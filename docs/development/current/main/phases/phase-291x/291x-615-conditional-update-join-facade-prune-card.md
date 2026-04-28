---
Status: Landed
Date: 2026-04-28
Scope: prune conditional_update_join feature facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/features/mod.rs
  - src/mir/builder/control_flow/plan/features/conditional_update_join.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_body_helpers.rs
  - src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs
---

# 291x-615: Conditional Update Join Facade Prune

## Goal

Remove the `features::conditional_update_join` facade and point callers at the
parts SSOT, `parts::conditional_update`.

This is BoxShape-only cleanup. It does not change conditional-update lowering,
recipe-authority behavior, accepted loop shapes, or emitted MIR.

## Boundaries

- Keep implementation ownership in `plan::parts::conditional_update`.
- Do not change conditional-update helper bodies or visibility beyond existing
  plan-internal use.
- Do not alter loop-cond, generic-loop, or true-break-continue route behavior.

## Result

- Updated loop-cond, generic-loop, and true-break-continue callers to import or
  call `parts::conditional_update` directly.
- Removed `conditional_update_join` from `plan::features`.
- Deleted the now-empty facade file.

## Verification

```bash
! rg -n "features::conditional_update_join|conditional_update_join::|mod conditional_update_join" src/mir/builder/control_flow/plan -g'*.rs'
cargo test -q conditional_update
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
