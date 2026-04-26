---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow stale legacy wording cleanup
Related:
  - src/mir/control_tree/normalized_shadow/builder.rs
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs
  - src/mir/control_tree/normalized_shadow/anf/README.md
  - src/mir/control_tree/normalized_shadow/anf/execute_box.rs
  - docs/development/current/main/phases/phase-291x/291x-404-normalized-shadow-stale-legacy-wording-inventory-card.md
---

# 291x-405: Normalized-Shadow Stale Legacy Wording Cleanup

## Goal

Remove stale "fallback to legacy" wording from live normalized-shadow source
comments and docs.

This is a BoxShape cleanup. No behavior changed.

## Change

Updated live normalized-shadow wording from legacy fallback terminology to the
current contract:

```text
out-of-scope route returns Ok(None)
route chaining tries the next route
ANF route declines when it has no hoist targets
```

## Preserved Behavior

- No route order changed.
- No accepted StepTree shape changed.
- No fail-fast tag changed.
- Historical phase cards were not rewritten.

## Next Cleanup

Inventory the next compiler-cleanliness seam after normalized-shadow legacy
storage and wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "fallback to legacy|fall back to legacy|legacy routing" src/mir/control_tree/normalized_shadow
```
