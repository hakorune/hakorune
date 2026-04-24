---
Status: Landed
Date: 2026-04-25
Scope: Add MIR CoreMethod metadata carriers for direct `ArrayBox.push` generic method routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-173-push-mutating-carrier-preflight-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_facts.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-174 CoreMethod Push Route Metadata Card

## Goal

Prepare the `push` emit-kind mirror cleanup without changing backend lowering:

```text
MethodCall(ArrayBox, "push", [value])
  -> generic_method_routes[].core_method.op = ArrayPush
  -> route_id = generic_method.push
  -> route_kind = array_append_any
  -> return_shape = scalar_i64
  -> value_demand = write_any
  -> publication_policy = no_publication
  -> .inc still falls back to the legacy push classifier
```

This is a carrier card. It gives MIR a typed `ArrayPush` identity for direct
ArrayBox surface calls while preserving the existing C shim fallback behavior.

## Boundary

- Do not remove the generic `push` emit-kind mirror row.
- Do not make `.inc` consume `generic_method.push` in this card.
- Do not change helper symbols or runtime behavior.
- Do not add hot inline lowering.
- Do not encode the pushed value as key metadata.
- Keep `RuntimeDataBox.push` metadata-absent for now, even when the receiver
  origin can be observed as `ArrayBox`; that facade boundary needs its own
  metadata-absent mutating contract before pruning.

## Implementation

- Add `GenericMethodRouteKind::ArrayAppendAny` with helper symbol
  `nyash.array.slot_append_hh`.
- Add `GenericMethodRouteProof::PushSurfacePolicy`.
- Add `GenericMethodValueDemand::WriteAny`.
- Detect only direct `ArrayBox.push(value)` in `generic_method_routes`.
- Emit JSON with `route_id=generic_method.push`, `key_route=null`, and
  `key_value=null`.
- Pin focused tests for both accepted direct `ArrayBox.push` and rejected
  `RuntimeDataBox.push` fallback.

## Result

MIR JSON can now carry:

```text
generic_method.push + core_method.op=ArrayPush
```

The next card may make generic emit-kind `PUSH` selection metadata-first while
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
