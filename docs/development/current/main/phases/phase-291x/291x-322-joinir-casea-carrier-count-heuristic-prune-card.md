---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A carrier-count heuristic prune
Related:
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/loop_route_detection/features.rs
  - docs/development/current/main/phases/phase-291x/291x-321-joinir-casea-carrier-count-heuristic-inventory-card.md
---

# 291x-322: JoinIR Case-A Carrier-count Heuristic Prune

## Goal

Stop selecting specialized Case-A lowerers from carrier count alone.

This is behavior-narrowing BoxShape cleanup. Unknown shapes remain generic
unless observed update metadata proves a more specific shape.

## Change

Updated:

```text
CaseALoweringShape::detect_from_features(...)
CaseALoweringShape::detect_with_updates(...)
```

New rule:

```text
No observed update summary -> Generic
Observed summary without AccumulationLike -> Generic
Observed AccumulationLike + one carrier -> ArrayAccumulation
Observed AccumulationLike + multiple carriers -> IterationWithAccumulation
Observed single CounterLike -> StringExamination
```

Removed the unused legacy `detect_with_carrier_name(...)` path and its
single-carrier name heuristic.

## Preserved Behavior

Known exact Case-A targets still use descriptor fallback:

```text
find_case_a_minimal_target(...) -> lowerer kind
```

This preserves the existing explicit target routes while preventing unknown
multi-carrier loops from being treated as Stage1 accumulation solely because
they have two or more carriers.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No lowerer rewrite.
- No AST/MIR update analyzer expansion.
- No cleanup of the remaining AST update analyzer index-name heuristic.

## Validation

```bash
cargo test -q case_a_carrier_count
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
