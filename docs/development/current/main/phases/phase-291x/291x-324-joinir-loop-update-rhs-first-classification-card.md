---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update RHS-first classification
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-323-joinir-loop-update-index-name-heuristic-inventory-card.md
---

# 291x-324: JoinIR Loop-update RHS-first Classification

## Goal

Make loop-update classification require RHS self-reference before variable
names can influence `CounterLike` selection.

This is BoxShape cleanup: it removes a name-first shortcut without adding new
update forms.

## Change

Updated:

```text
classify_update_kind_from_rhs(var_name, rhs)
analyze_loop_updates_from_ast(...)
```

New rule:

```text
RHS must be `carrier + ...` before it can classify as an update.
`carrier + 1` uses the carrier name only as a tie-breaker:
  likely index name -> CounterLike
  non-index name    -> AccumulationLike
```

Rejected now:

```text
i = 0
i = j + 1
```

These no longer become `CounterLike` solely because the assigned variable is
named `i`.

## Preserved Behavior

- `i = i + 1` remains `CounterLike`.
- `sum = sum + 1` remains `AccumulationLike`.
- `sum = sum + expr` remains `AccumulationLike`.
- Case-A route and lowerer selection are unchanged in this card.

## Non-Goals

- No new update operator support.
- No MIR update analyzer expansion.
- No nested-scope assignment filtering.
- No IfPhiJoin policy change.

## Validation

```bash
cargo test -q loop_update_rhs_first
cargo test -q case_a_carrier_count
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
