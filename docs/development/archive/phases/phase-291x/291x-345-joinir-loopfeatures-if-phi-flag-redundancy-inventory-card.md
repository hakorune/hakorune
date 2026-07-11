---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures if-phi flag redundancy inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
---

# 291x-345: JoinIR LoopFeatures If-phi Flag Redundancy Inventory

## Goal

Inventory the remaining `LoopFeatures` if/phi flags before pruning duplicate
state.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

`LoopFeatures` carries two related flags:

```text
has_if
has_if_else_phi
```

The classifier reads only `has_if`.

```text
classify(...)
  -> IfPhiJoin when has_if && carrier_count >= 1 && !break && !continue
  -> LoopSimpleWhile when !break && !continue && !has_if
```

The AST feature extractor currently uses precise if/phi detection, then mirrors
that value into both fields.

```text
let has_if_else_phi = detect_if_else_phi_in_body(body);
let has_if = has_if_else_phi;
```

The LoopForm extractor emits both as `false`.

```text
has_if_else_phi = false
has_if = false
```

No live classifier path reads `has_if_else_phi`.

## Decision

`has_if_else_phi` is duplicate reserved state.

Keep `has_if` as the live route flag because it is the classifier input today.
Clarify that `has_if` means "if/phi route signal", not "any if statement".

## Next Cleanup

Prune:

```text
LoopFeatures.has_if_else_phi
producer assignments for has_if_else_phi
test initializers for has_if_else_phi
```

Preserve:

```text
LoopFeatures.has_if
classify(...) route behavior
detect_if_else_phi_in_body(body) as the AST producer for has_if
```

## Non-Goals

- No route behavior change.
- No `detect_if_else_phi_in_body(...)` behavior change.
- No IfPhiJoin acceptance expansion.

## Validation

```bash
rg -n "has_if_else_phi" src/mir -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
