---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update assignment-value traversal prune
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-329-joinir-loop-update-assignment-value-traversal-inventory-card.md
---

# 291x-330: JoinIR Loop-update Assignment-value Traversal Prune

## Goal

Stop using assignments nested inside assignment values as loop-update proof.

This is BoxShape cleanup: loop-update summary now observes statement-level
carrier assignments in the current loop body and current-loop if branches.

## Change

Updated:

```text
collect_assignment_rhses(...)
```

New rule:

```text
ASTNode::Assignment target == carrier -> collect its RHS
ASTNode::Assignment target != carrier -> ignore its value for update proof
ASTNode::If branches -> recurse
ASTNode::Loop bodies -> do not recurse
```

## Preserved Behavior

- Direct current-loop carrier assignments are still observed.
- Current-loop if-branch assignments are still observed.
- Nested loop bodies remain excluded by 291x-326.
- All-RHS conflict handling from 291x-328 is unchanged.

## Non-Goals

- No assignment-expression semantics change.
- No new update operator support.
- No MIR update analyzer expansion.
- No Case-A route or lowerer change.

## Validation

```bash
cargo test -q loop_update_assignment_value
cargo test -q loop_update_multi_assignment
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
