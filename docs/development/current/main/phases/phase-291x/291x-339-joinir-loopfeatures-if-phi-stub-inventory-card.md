---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures if-phi signature stub inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/join_ir/lowering/loop_route_router.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
---

# 291x-339: JoinIR LoopFeatures If-phi Stub Inventory

## Goal

Inventory the `LoopForm` feature extractor's IfPhiJoin signature stub before
pruning unused reserved shape.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

`loop_route_detection::features::extract_features(...)` has one production
caller.

```text
loop_route_router.rs
  -> extract_features(loop_form, None)
```

The `scope` parameter is never used with real data in the live caller.

```text
extract_features(loop_form, scope)
  -> carrier_count = scope.map(...).unwrap_or(0)
  -> has_if_phi_join_signature(scope)
```

Because the live caller passes `None`, the LoopForm extractor currently emits:

```text
carrier_count = 0
has_if_else_phi = false
has_if = false
```

`has_if_phi_join_signature(...)` itself is a conservative stub that always
returns `false`.

The AST route-feature path is separate and still owns AST-level if/phi
recognition.

```text
builder/control_flow/facts/ast_feature_extractor::extract_features(...)
  -> detect_if_else_phi_in_body(body)
```

## Decision

The LoopForm extractor's if-phi signature helper is a dead reserved seam.

Keeping the helper and unused `scope` parameter makes the extractor look more
capable than the live contract. The live LoopForm extractor contract is:

```text
LoopForm
  -> break/continue counts
  -> no AST if-phi observation
```

AST if/phi detection remains in the AST feature extractor.

## Next Cleanup

Prune the LoopForm reserved seam:

```text
has_if_phi_join_signature(...)
extract_features(loop_form, scope)
  -> extract_features(loop_form)
```

Preserve the current LoopForm behavior:

```text
carrier_count = 0
has_if_else_phi = false
has_if = false
```

## Non-Goals

- No AST route-feature behavior change.
- No IfPhiJoin acceptance expansion.
- No LoopForm scope analysis implementation.
- No route classifier behavior change.

## Validation

```bash
rg -n "has_if_phi_join_signature|extract_features\\(loop_form" src/mir -g '*.rs'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
