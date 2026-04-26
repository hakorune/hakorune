---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A carrier-count heuristic inventory
Related:
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - docs/development/current/main/phases/phase-291x/291x-320-joinir-casea-update-summary-name-only-prune-card.md
---

# 291x-321: JoinIR Case-A Carrier-count Heuristic Inventory

## Goal

Inventory the remaining Case-A shape-dispatch seam where carrier count alone
selects a specialized lowerer.

This card is audit-only. It does not change lowering behavior.

## Findings

`CaseALoweringShape::detect_from_features(...)` still treats:

```text
carrier_count >= 2 -> IterationWithAccumulation
```

as a recognized shape even when no update summary is present.

`CaseALoweringShape::detect_with_updates(...)` has the same fallback when the
summary is absent or does not contain accumulation evidence.

This means:

```text
number of carriers -> specialized stage1 lowerer
```

can happen without proving that one carrier is an accumulator.

There is also an unused legacy function:

```text
detect_with_carrier_name(...)
```

which keeps an older single-carrier name heuristic alive behind `dead_code`.

## Decision

The next implementation target is:

```text
JoinIR Case-A carrier-count heuristic prune
```

Implementation boundary:

```text
No observed update summary -> Generic
Observed summary without AccumulationLike -> Generic
Observed AccumulationLike + multiple carriers -> IterationWithAccumulation
Observed AccumulationLike + single carrier -> ArrayAccumulation
```

Known exact Case-A targets must continue through the descriptor fallback:

```text
find_case_a_minimal_target(...) -> lowerer kind
```

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No lowerer rewrite.
- No LoopScopeShape field change.
- No AST/MIR update analyzer expansion.

## Acceptance

```bash
cargo test -q case_a_carrier_count
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
