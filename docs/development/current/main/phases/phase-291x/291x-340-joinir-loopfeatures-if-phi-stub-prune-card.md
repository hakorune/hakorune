---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures if-phi signature stub prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/join_ir/lowering/loop_route_router.rs
  - docs/development/current/main/phases/phase-291x/291x-339-joinir-loopfeatures-if-phi-stub-inventory-card.md
---

# 291x-340: JoinIR LoopFeatures If-phi Stub Prune

## Goal

Remove the dead LoopForm if-phi signature helper and unused `scope` parameter
identified by 291x-339.

This is behavior-preserving BoxShape cleanup.

## Change

Removed:

```text
has_if_phi_join_signature(...)
extract_features(loop_form, scope)
```

Replaced with the explicit current LoopForm contract:

```text
extract_features(loop_form)
  -> carrier_count = 0
  -> has_if_else_phi = false
  -> has_if = false
```

Updated the only production caller:

```text
loop_route_router.rs
  -> extract_features(loop_form)
```

## Boundary After Prune

LoopForm feature extraction owns only the route facts that `LoopForm` currently
contains.

```text
LoopForm
  -> break targets
  -> continue targets
```

AST if/phi observation remains in the AST feature extractor.

```text
AST body
  -> ast_feature_extractor::extract_features(...)
  -> detect_if_else_phi_in_body(body)
```

## Preserved Behavior

- LoopForm route extraction still reports no IfPhiJoin signature.
- AST route-feature behavior is unchanged.
- The route classifier is unchanged.
- No new route shape is accepted.

## Non-Goals

- No LoopForm scope analysis implementation.
- No IfPhiJoin acceptance expansion.
- No AST feature extractor change.
- No route classifier change.

## Validation

```bash
rg -n "has_if_phi_join_signature|extract_features\\(loop_form, None" src/mir -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
