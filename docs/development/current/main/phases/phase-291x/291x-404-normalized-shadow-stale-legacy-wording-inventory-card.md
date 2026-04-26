---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow stale legacy wording inventory
Related:
  - src/mir/control_tree/normalized_shadow/builder.rs
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs
  - src/mir/control_tree/normalized_shadow/anf/README.md
  - src/mir/control_tree/normalized_shadow/anf/execute_box.rs
  - docs/development/current/main/phases/phase-291x/291x-403-normalized-shadow-legacy-physical-storage-move-card.md
---

# 291x-404: Normalized-Shadow Stale Legacy Wording Inventory

## Goal

Inventory stale "fallback to legacy" wording after the normalized-shadow legacy
module was removed.

This card is inventory-only. No code behavior changed.

## Findings

The current code/docs still use old wording for route chaining:

```text
src/mir/control_tree/normalized_shadow/builder.rs
src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
src/mir/control_tree/normalized_shadow/post_if_post_k.rs
src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs
src/mir/control_tree/normalized_shadow/anf/README.md
src/mir/control_tree/normalized_shadow/anf/execute_box.rs
```

In these sites, the behavior is not a real legacy module fallback anymore.
The intended meaning is one of:

```text
out-of-scope route returns Ok(None)
route chaining tries the next normalized-shadow route
ANF route declines when it has no hoist targets
```

Historical phase cards may still mention `LegacyLowerer` or `pub mod legacy`
because they describe the state at that time. Do not rewrite landed history.

## Decision

Clean only the live source comments and current normalized-shadow docs. Replace
stale `legacy` wording with route-chaining terminology:

```text
fallback to legacy        -> out-of-scope route / route chaining
legacy routing            -> route chaining
Condition not pure...     -> out of scope for this route
No hoist targets...       -> out of scope for the ANF route
```

## Next Cleanup

Apply wording cleanup in the listed live files.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "fallback to legacy|fall back to legacy|legacy routing" src/mir/control_tree/normalized_shadow
```

The final `rg` should produce no output.
