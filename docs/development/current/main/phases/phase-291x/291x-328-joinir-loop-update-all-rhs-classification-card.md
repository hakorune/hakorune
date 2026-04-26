---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update all-RHS classification
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-327-joinir-loop-update-multi-assignment-inventory-card.md
---

# 291x-328: JoinIR Loop-update All-RHS Classification

## Goal

Make loop-update classification inspect every current-loop RHS assignment for a
carrier instead of trusting the first one.

This is BoxShape cleanup: update proof is no longer order-dependent.

## Change

Replaced:

```text
find_assignment_rhs(...)
```

with:

```text
collect_assignment_rhses(...)
classify_update_kind_from_rhses(...)
```

New rule:

```text
No RHS       -> carrier omitted from summary
One RHS      -> classified as before
Multiple RHS -> accepted only if every RHS agrees after name tie-break
Any conflict/Other -> Other
```

Nested loop bodies remain excluded. Current-loop if branches remain included.

## Preserved Behavior

- `i = i + 1` remains `CounterLike`.
- `sum = sum + 1` remains `AccumulationLike`.
- Current-loop if-branch assignments are still observed.
- Nested loop assignments remain excluded by 291x-326.

## Non-Goals

- No new update operator support.
- No branch-sensitive PHI modeling.
- No MIR update analyzer expansion.
- No Case-A route or lowerer change.

## Validation

```bash
cargo test -q loop_update_multi_assignment
cargo test -q loop_update_nested_scope
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
