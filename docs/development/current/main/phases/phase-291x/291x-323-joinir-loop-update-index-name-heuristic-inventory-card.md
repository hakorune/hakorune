---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update index-name heuristic inventory
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-322-joinir-casea-carrier-count-heuristic-prune-card.md
---

# 291x-323: JoinIR Loop-update Index-name Heuristic Inventory

## Goal

Inventory the remaining loop-update analyzer seam where variable names decide
`CounterLike` before RHS shape is validated.

This card is audit-only. It does not change update classification behavior.

## Findings

`analyze_loop_updates_from_ast(...)` currently checks:

```text
is_likely_loop_index(name) -> CounterLike
```

before looking at the assignment RHS.

That means an index-like carrier name can become `CounterLike` even if the
assignment is not a self increment.

`classify_update_kind_from_rhs(...)` also accepts any variable on the left side
of the RHS binary expression:

```text
i = j + 1
```

can look like a counter update because the classifier does not know the LHS
carrier name.

## Decision

The next implementation target is:

```text
JoinIR loop-update RHS-first classification
```

Implementation boundary:

```text
RHS must be self-referential before it can classify as CounterLike or AccumulationLike.
Name can only disambiguate self + 1:
  likely index name -> CounterLike
  non-index name    -> AccumulationLike
```

## Non-Goals

- No new supported update operator.
- No MIR update analyzer expansion.
- No Case-A route or lowerer change.
- No broader IfPhiJoin policy change.

## Acceptance

```bash
cargo test -q loop_update_rhs_first
cargo test -q case_a_carrier_count
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
