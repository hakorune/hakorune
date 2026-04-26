---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update summary helper split inventory
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-330-joinir-loop-update-assignment-value-traversal-prune-card.md
---

# 291x-331: JoinIR Loop-update Summary Helper Split Inventory

## Goal

Inventory the remaining shape debt inside `loop_update_summary.rs` before
moving helpers.

This card is audit-only. It does not change loop-update behavior.

## Findings

`loop_update_summary.rs` currently contains all of these responsibilities in
one 600+ line file:

```text
public summary data types
public summary query API
RHS classification
current-loop assignment traversal
public AST analyzer entrypoint
unit-test AST builders and cases
```

Recent cleanup made traversal/classification contracts stricter, but the file
still hides those contracts behind private helpers in the same module.

This makes future work harder because behavior-changing cleanup and physical
module cleanup both touch the same large file.

## Decision

The next implementation target is:

```text
JoinIR loop-update summary helper split
```

Implementation boundary:

```text
loop_update_summary.rs
  owns public types and analyze_loop_updates_from_ast(...)

loop_update_summary/assignment_scan.rs
  owns current-loop RHS collection

loop_update_summary/rhs_classification.rs
  owns RHS/self-reference classification and name tie-break

loop_update_summary/tests.rs
  owns test-only AST builders and cases
```

No behavior changes are allowed in this split.

## Non-Goals

- No new update operator support.
- No route/lowerer change.
- No classification policy change.
- No test expectation change.

## Acceptance

```bash
cargo test -q loop_update_
cargo test -q case_a_carrier_count
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
