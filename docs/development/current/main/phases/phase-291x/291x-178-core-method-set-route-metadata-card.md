---
Status: Landed
Date: 2026-04-25
Scope: Add MIR CoreMethod metadata carriers for direct `ArrayBox.set` / `MapBox.set` generic method routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-177-set-mutating-carrier-preflight-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_facts.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-178 CoreMethod Set Route Metadata Card

## Goal

Prepare the `set` emit-kind and storage-route mirror cleanup without changing
backend lowering:

```text
MethodCall(ArrayBox, "set", [index, value])
  -> generic_method_routes[].core_method.op = ArraySet
  -> route_id = generic_method.set
  -> route_kind = array_store_any

MethodCall(MapBox, "set", [key, value])
  -> generic_method_routes[].core_method.op = MapSet
  -> route_id = generic_method.set
  -> route_kind = map_store_any
```

This is a carrier card. It gives MIR typed `ArraySet` / `MapSet` identity for
direct surface calls while preserving the existing C shim fallback behavior.

## Boundary

- Do not remove the generic `set` emit-kind mirror row.
- Do not make `.inc` consume `generic_method.set` in this card.
- Do not change helper symbols or runtime behavior.
- Do not add hot inline lowering.
- Do not encode the stored value as key metadata.
- Keep `RuntimeDataBox.set` metadata-absent for now.
- Keep `return_shape` and `publication_policy` null; direct ArrayBox and MapBox
  set have different source-level return contracts.

## Implementation

- Add `GenericMethodRouteKind::ArrayStoreAny` and
  `GenericMethodRouteKind::MapStoreAny`.
- Add `GenericMethodRouteProof::SetSurfacePolicy`.
- Detect only direct `ArrayBox.set(index, value)` and
  `MapBox.set(key, value)`.
- Emit JSON with `route_id=generic_method.set`, `arity=2`, and first-argument
  key metadata.
- Pin focused tests for accepted direct set routes and rejected
  `RuntimeDataBox.set` fallback.

## Result

MIR JSON can now carry:

```text
generic_method.set + core_method.op=ArraySet
generic_method.set + core_method.op=MapSet
```

The next card may make generic emit-kind `SET` selection metadata-first while
preserving the legacy fallback path.

## Acceptance

```bash
cargo fmt --check
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q map_lookup_fusion
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
