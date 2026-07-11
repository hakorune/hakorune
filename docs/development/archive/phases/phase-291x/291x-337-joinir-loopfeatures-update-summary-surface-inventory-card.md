---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures update_summary surface inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
---

# 291x-337: JoinIR LoopFeatures Update-summary Surface Inventory

## Goal

Inventory whether `LoopFeatures.update_summary` is still a live compiler
contract surface after the loop-update analyzer cleanup series.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

Production producers do not populate `LoopFeatures.update_summary`.

```text
loop_route_detection::features::extract_features(...)
  -> update_summary = None

builder/control_flow/facts/ast_feature_extractor::extract_features(...)
  -> LoopFeatures { ..Default::default() }

loop_view_builder.rs
  -> stub LoopFeatures { ..Default::default() }
```

The only real loop-update analyzer call site is independent of
`LoopFeatures.update_summary`.

```text
RoutePrepContext::is_if_phi_join_pattern(...)
  -> analyze_loop_updates_from_ast(...)
```

The only `update_summary` consumer is `CaseALoweringShape::detect_with_updates`,
and that entrypoint is used only by tests.

```text
CaseALoweringShape::detect_with_updates(...)
  -> reads features.update_summary
  -> no production caller
```

Test helpers construct `LoopFeatures.update_summary` directly to exercise the
dead `detect_with_updates(...)` path.

## Decision

`LoopFeatures.update_summary` is a dead reserved surface, not a live contract.

Keeping it would preserve a second path for update-kind metadata and make the
compiler boundary look broader than it is. The current live contract is:

```text
AST observation
  -> analyze_loop_updates_from_ast(...)
  -> direct route-local decision / tests
```

`LoopFeatures` should remain a route-shape feature vector, not a transport for
unused update-kind summaries.

## Next Cleanup

Prune the dead surface:

```text
LoopFeatures.update_summary
CaseALoweringShape::detect_with_updates(...)
tests that only exercise the dead surface
```

Preserve:

```text
LoopFeatures route classification fields
CaseALoweringShape::detect_from_features(...)
CaseALoweringShape::detect(...) compatibility wrapper
loop_update_summary analyzer and its focused tests
RoutePrepContext::is_if_phi_join_pattern(...) local analyzer use
```

## Non-Goals

- No route classification behavior change.
- No loop-update analyzer behavior change.
- No Case-A descriptor fallback change.
- No new accepted loop shape.

## Validation

```bash
rg -n "detect_with_updates\\(|update_summary" src -g '*.rs'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
