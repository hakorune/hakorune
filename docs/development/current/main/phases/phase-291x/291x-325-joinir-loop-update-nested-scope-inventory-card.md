---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update nested-scope assignment inventory
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-324-joinir-loop-update-rhs-first-classification-card.md
---

# 291x-325: JoinIR Loop-update Nested-scope Assignment Inventory

## Goal

Inventory the loop-update analyzer seam where nested loop assignments are
treated as if they belonged to the current loop body.

This card is audit-only. It does not change update classification behavior.

## Findings

`extract_assigned_variables(...)` recurses into:

```text
ASTNode::Loop { body, .. }
```

`find_assignment_rhs(...)` does the same.

That means an outer loop update summary can be populated from an assignment
inside a nested loop:

```text
outer loop body:
  loop (...) {
    i = i + 1
  }
```

For current-loop classification, nested loop bodies are separate scopes. The
outer analyzer should observe statements in the current loop body and same-body
if branches, but should not use nested loop assignments as outer carrier
updates.

## Decision

The next implementation target is:

```text
JoinIR loop-update nested-scope prune
```

Implementation boundary:

```text
If branches: recurse, because they are current-loop body branches.
Nested loop bodies: do not recurse, because they are nested loop scopes.
```

## Non-Goals

- No new update operator support.
- No MIR update analyzer expansion.
- No Case-A route or lowerer change.
- No nested-loop lowering change.

## Acceptance

```bash
cargo test -q loop_update_nested_scope
cargo test -q loop_update_rhs_first
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
