---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update docs/comment contract cleanup
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/join_ir/lowering/loop_update_summary/rhs_classification.rs
  - docs/development/current/main/phases/phase-291x/291x-333-joinir-loop-update-stale-docs-inventory-card.md
---

# 291x-334: JoinIR Loop-update Docs/Comment Contract Cleanup

## Goal

Make loop-update comments describe the current analyzer contract instead of
future or unsupported update forms.

This is documentation-only cleanup.

## Change

Updated comments for:

```text
UpdateKind::CounterLike
UpdateKind::AccumulationLike
CarrierUpdateInfo branch payload fields
analyze_loop_updates_from_ast(...)
rhs_classification helper contract
```

The documented current contract is now:

```text
RHS must be self-referential Add.
`x = x + 1` uses carrier name only as a tie-breaker.
Subtraction, compound assignment, push, and append are not recognized here.
```

## Preserved Behavior

- No code behavior changed.
- No test expectations changed.
- No public API changed.

## Non-Goals

- No new update operator support.
- No route/lowerer change.
- No branch-sensitive PHI modeling.

## Validation

```bash
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
