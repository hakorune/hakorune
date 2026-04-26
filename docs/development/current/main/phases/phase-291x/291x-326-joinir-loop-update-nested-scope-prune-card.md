---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update nested-scope assignment prune
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-325-joinir-loop-update-nested-scope-inventory-card.md
---

# 291x-326: JoinIR Loop-update Nested-scope Assignment Prune

## Goal

Prevent nested loop assignments from becoming proof for the current loop's
update summary.

This is BoxShape cleanup: current-loop update observation stays local to the
current loop body and its if branches.

## Change

Updated:

```text
extract_assigned_variables(...)
find_assignment_rhs(...)
```

New rule:

```text
ASTNode::If branches: recurse
ASTNode::Loop bodies: do not recurse
```

## Preserved Behavior

- Current-loop direct assignments are still observed.
- Current-loop if-branch assignments are still observed.
- RHS-first self-reference classification from 291x-324 is unchanged.
- Case-A route and lowerer selection are unchanged in this card.

## Non-Goals

- No nested-loop lowering change.
- No new update operator support.
- No MIR update analyzer expansion.
- No multi-assignment merge policy change.

## Validation

```bash
cargo test -q loop_update_nested_scope
cargo test -q loop_update_rhs_first
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
