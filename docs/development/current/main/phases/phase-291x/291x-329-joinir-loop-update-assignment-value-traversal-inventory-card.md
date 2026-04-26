---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update assignment-value traversal inventory
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-328-joinir-loop-update-all-rhs-classification-card.md
---

# 291x-329: JoinIR Loop-update Assignment-value Traversal Inventory

## Goal

Inventory the loop-update analyzer seam where assignment values are recursively
searched for carrier assignments.

This card is audit-only. It does not change update classification behavior.

## Findings

`collect_assignment_rhses(...)` recurses into the value expression of an
assignment when the assignment target is not the requested carrier:

```text
other = <value containing i = i + 1>
```

This can turn a nested assignment expression into current-loop update proof for
`i`, even though the loop-update summary should observe statement-level carrier
assignments in the current loop body and current-loop if branches.

After 291x-328, all RHS candidates are considered, but this traversal still
controls which candidates enter the set.

## Decision

The next implementation target is:

```text
JoinIR loop-update assignment-value traversal prune
```

Implementation boundary:

```text
ASTNode::Assignment target == carrier -> collect its RHS
ASTNode::Assignment target != carrier -> do not recurse into value
ASTNode::If branches -> recurse
ASTNode::Loop bodies -> do not recurse
```

## Non-Goals

- No new update operator support.
- No assignment-expression semantics change.
- No MIR update analyzer expansion.
- No Case-A route or lowerer change.

## Acceptance

```bash
cargo test -q loop_update_assignment_value
cargo test -q loop_update_multi_assignment
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
