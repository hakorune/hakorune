---
Status: Landed
Date: 2026-04-25
Scope: Define the safe boundary for making `set` storage-route selection consume MIR metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-178-core-method-set-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-179-core-method-set-emit-kind-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-181 Set Storage-Route Metadata Preflight Card

## Goal

Prepare the next cleanup without changing lowering:

```text
generic_method.set + core_method.op=MapSet + route_kind=map_store_any
  -> storage route may select MAP_STORE_ANY from metadata

generic_method.set + core_method.op=ArraySet + route_kind=array_store_any
  -> storage route may select ARRAY_STORE_{I64,STRING,ANY}
     after preserving existing value-type discrimination
```

This is a BoxShape boundary card. It exists to keep the next `.inc` edit from
turning into a silent semantic shortcut.

## Boundary

- Do not remove `classify_generic_method_set_route(...)`.
- Do not remove the generic `set` emit-kind mirror row.
- Do not make `RuntimeDataBox.set` metadata-present in this card.
- Do not change helper symbols or storage behavior.
- Do not collapse ArrayBox value-shape selection:
  `ArraySet + route_kind=array_store_any` is only the family route; the C side
  must still choose I64, String, or Any using the existing value checks.
- Do not accept `ArraySet` with `map_store_any` or `MapSet` with
  `array_store_any`.
- Keep fallback active when route metadata is missing, invalid, or mismatched.

## Required Consumer Shape

The next implementation may add a metadata-first set-route reader if it obeys
this shape:

```text
site = block + instruction_index
route_id = generic_method.set
core_method.op in {ArraySet, MapSet}
proof = core_method_contract_manifest
lowering_tier = cold_fallback
route_kind in {array_store_any, map_store_any}
fallback = classify_generic_method_set_route(...)
```

The consumer must produce only a storage-route enum for the existing lowering.
It must not infer legality, mutate publication policy, or bypass the
metadata-absent RuntimeData fallback.

## Result

The next implementation target is a metadata-first storage-route consumer for
direct ArrayBox/MapBox set routes. Any prune of storage-route mirror logic
remains a later probe.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
