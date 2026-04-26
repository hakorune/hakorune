---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures has_if_else_phi prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/loop_route_detection/tests.rs
  - docs/development/current/main/phases/phase-291x/291x-345-joinir-loopfeatures-if-phi-flag-redundancy-inventory-card.md
---

# 291x-346: JoinIR LoopFeatures If-phi Flag Prune

## Goal

Remove the duplicate `LoopFeatures.has_if_else_phi` field identified by
291x-345.

This is behavior-preserving BoxShape cleanup.

## Change

Removed:

```text
LoopFeatures.has_if_else_phi
producer assignments for has_if_else_phi
test initializers for has_if_else_phi
```

Changed the AST producer from mirrored fields to one live route flag:

```text
has_if = detect_if_else_phi_in_body(body)
```

Clarified that `LoopFeatures.has_if` is an if/phi route signal, not an
"any if statement" flag.

## Boundary After Prune

`LoopFeatures` now has one IfPhiJoin route signal.

```text
AST if/phi recognizer
  -> LoopFeatures.has_if
  -> classify(...)
```

## Preserved Behavior

- `classify(...)` still reads `has_if`.
- AST producer still uses `detect_if_else_phi_in_body(body)`.
- LoopForm producer still emits `has_if = false`.
- No IfPhiJoin acceptance expansion.

## Non-Goals

- No route classifier behavior change.
- No if/phi recognizer behavior change.
- No route-kind enum change.

## Validation

```bash
rg -n "has_if_else_phi" src/mir -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
