---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures update_summary dead surface prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/tests.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/join_ir/lowering/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-337-joinir-loopfeatures-update-summary-surface-inventory-card.md
---

# 291x-338: JoinIR LoopFeatures Update-summary Prune

## Goal

Remove the dead `LoopFeatures.update_summary` surface identified by 291x-337.

This is behavior-preserving BoxShape cleanup. The live loop-update analyzer
entrypoint remains route-local.

## Change

Removed:

```text
LoopFeatures.update_summary
CaseALoweringShape::detect_with_updates(...)
tests that only exercised the dead update_summary transport path
```

Kept:

```text
LoopFeatures route-shape fields
CaseALoweringShape::detect_from_features(...)
CaseALoweringShape::detect(...) compatibility wrapper
loop_update_summary analyzer module
RoutePrepContext::is_if_phi_join_pattern(...) local analyzer use
```

## Boundary After Prune

`LoopFeatures` is now only a route-shape feature vector.

```text
LoopFeatures
  = break / continue / if / carrier_count / nesting / route-shape facts
```

Loop update-kind proof no longer has a reserved transport field on
`LoopFeatures`.

```text
AST observation
  -> analyze_loop_updates_from_ast(...)
  -> route-local decision
```

This avoids a misleading second metadata path.

## Preserved Behavior

- `extract_features(...)` route classification fields are unchanged.
- Case-A name-only/carrier-count-only detection still stays `Generic`.
- The loop-update analyzer and its tests remain intact.
- No new route shape is accepted.

## Non-Goals

- No IfPhiJoin route behavior change.
- No loop-update analyzer behavior change.
- No Case-A descriptor fallback change.
- No new update-kind metadata carrier.

## Validation

```bash
rg -n "update_summary|detect_with_updates" src -g '*.rs'
cargo test -q case_a_
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
