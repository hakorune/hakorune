---
Status: Landed
Date: 2026-04-27
Scope: loop-if-break-continue scope wording cleanup
Related:
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-434-loop-if-break-continue-placeholder-review-card.md
---

# 291x-435: Loop-If-Break-Continue Scope Wording Cleanup

## Goal

Sync loop-if-break-continue route comments with the current P0/P1/P2 surface.

This is a BoxShape cleanup. No behavior changed.

## Change

Updated comments/docs to describe the current active route:

```text
P0: break-only
P1: continue-only
P2: optional else break/continue combinations
```

Also clarified that `k_then` / `k_else` are reserved structural continuations,
not accidental placeholders to remove in cleanup.

## Preserved Behavior

- function IDs are unchanged.
- `k_then` / `k_else` are unchanged.
- branch emission is unchanged.
- route acceptance is unchanged.
- generated JoinIR is unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "Else branch: Not supported|Phase 143 P0: loop\\(true\\) \\+ if \\+ break|P0/P1:" \
  src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs \
  src/mir/control_tree/normalized_shadow/mod.rs
```

## Next Cleanup

Run the closeout review for this cleanup burst.
