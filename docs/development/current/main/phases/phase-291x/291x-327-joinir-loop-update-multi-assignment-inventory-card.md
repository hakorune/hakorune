---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update multi-assignment inventory
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - docs/development/current/main/phases/phase-291x/291x-326-joinir-loop-update-nested-scope-prune-card.md
---

# 291x-327: JoinIR Loop-update Multi-assignment Inventory

## Goal

Inventory the loop-update analyzer seam where only the first assignment RHS for
a carrier is used as update proof.

This card is audit-only. It does not change update classification behavior.

## Findings

`find_assignment_rhs(...)` returns the first RHS that assigns the requested
carrier.

That makes classification order-dependent:

```text
i = 0
i = i + 1
```

and:

```text
i = i + 1
i = 0
```

can produce different proof candidates even though both are not a simple
single-update carrier shape.

Same-body if branches can also contain more than one assignment candidate for
the same carrier. Using only the first branch hides conflicting updates.

## Decision

The next implementation target is:

```text
JoinIR loop-update all-RHS classification
```

Implementation boundary:

```text
Collect all current-loop RHS assignments for the carrier.
No RHS       -> not included in summary
One RHS      -> classify as before
Multiple RHS -> classify only if every RHS agrees after name tie-break
Any conflict/Other -> Other
```

Nested loop bodies remain excluded by 291x-326.

## Non-Goals

- No new update operator support.
- No MIR update analyzer expansion.
- No Case-A route or lowerer change.
- No branch-sensitive PHI modeling.

## Acceptance

```bash
cargo test -q loop_update_multi_assignment
cargo test -q loop_update_nested_scope
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
