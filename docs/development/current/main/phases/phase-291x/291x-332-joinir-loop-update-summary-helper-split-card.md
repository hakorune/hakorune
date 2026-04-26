---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update summary helper split
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/join_ir/lowering/loop_update_summary/assignment_scan.rs
  - src/mir/join_ir/lowering/loop_update_summary/rhs_classification.rs
  - src/mir/join_ir/lowering/loop_update_summary/tests.rs
  - docs/development/current/main/phases/phase-291x/291x-331-joinir-loop-update-summary-helper-split-inventory-card.md
---

# 291x-332: JoinIR Loop-update Summary Helper Split

## Goal

Split loop-update traversal, RHS classification, and tests out of the public
summary module without behavior changes.

## Change

Added:

```text
loop_update_summary/assignment_scan.rs
loop_update_summary/rhs_classification.rs
loop_update_summary/tests.rs
```

Kept in `loop_update_summary.rs`:

```text
UpdateKind
CarrierUpdateInfo
LoopUpdateSummary
analyze_loop_updates_from_ast(...)
```

## Preserved Behavior

- No classification policy changed.
- No traversal policy changed.
- No test expectations changed.
- Public callers continue to use `analyze_loop_updates_from_ast(...)`.

## Non-Goals

- No new update operator support.
- No route/lowerer change.
- No branch-sensitive PHI modeling.
- No public API expansion.

## Validation

```bash
cargo test -q loop_update_
cargo test -q case_a_carrier_count
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
