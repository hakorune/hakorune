---
Status: Landed
Date: 2026-04-27
Scope: loop-if-break-continue placeholder/fossil boundary review
Related:
  - src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs
  - src/mir/control_tree/normalized_shadow/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-433-normalization-default-path-wording-cleanup-card.md
---

# 291x-434: Loop-If-Break-Continue Placeholder Review

## Goal

Review loop-if-break-continue placeholder/fossil boundary under the closeout
cap.

This is a BoxShape review. No behavior changed.

## Findings

The module is not a fossil baseline like `entry/if_only.rs`. It is an active
normalized-shadow route.

The real issue is stale scope wording:

- module header says `Phase 143 P0/P1`
- top-level scope says else branch is not supported
- `mod.rs` still labels the module as `Phase 143 P0`
- implementation validates with `validate_for_p2()` and has explicit P2
  then/else match arms

The placeholder functions (`k_then`, `k_else`) are already explained as reserved
structural functions. They should stay unless a route-replacement card changes
the JoinModule shape.

## Decision

Clean only comments/docs in the active route:

```text
P0/P1 -> P0/P1/P2
Else not supported -> else branch is supported when represented by LoopIfExitShape
placeholder -> reserved structural function
```

Do not change:

- `LoopIfExitShape`
- `validate_for_p2()`
- branch emission
- function IDs
- `k_then` / `k_else` presence
- accepted route shapes
- generated JoinIR

## Next Cleanup

`291x-435`: loop-if-break-continue scope wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "Else branch: Not supported|Phase 143 P0: loop\\(true\\) \\+ if \\+ break|P0/P1:" \
  src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs \
  src/mir/control_tree/normalized_shadow/mod.rs
```

The final `rg` should produce no output.
