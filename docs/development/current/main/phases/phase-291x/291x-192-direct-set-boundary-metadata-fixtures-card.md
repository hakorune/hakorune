---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod metadata to direct Array/Map set pure-boundary MIR JSON fixtures.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-190-mir-call-set-need-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-191-mir-call-set-surface-prune-retry-card.md
  - tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
---

# 291x-192 Direct Set Boundary Metadata Fixtures Card

## Goal

Make the direct pure-boundary Array/Map `set` fixtures represent the current
compiler contract:

```text
ArrayBox.set(index, value)
  -> generic_method.set
  -> core_method.op=ArraySet
  -> route_kind=array_store_any

MapBox.set(key, value)
  -> generic_method.set
  -> core_method.op=MapSet
  -> route_kind=map_store_any
```

This is fixture contract work. It does not remove the legacy MIR-call `set`
surface classifier yet.

## Boundary

- Update only the metadata carried by existing inline MIR JSON boundary
  payloads.
- Do not change helper symbols, lowering, or expected return codes.
- Do not add C-side method-name classification.
- Do not prune `classify_mir_call_method_surface(... "set")` in this card.
- Keep RuntimeDataBox.set metadata-absent fallback out of scope.

## Implementation

- Add `metadata.generic_method_routes` to the direct Array set boundary payload.
- Add `metadata.generic_method_routes` to the direct Map set boundary payload.
- Pin each route by `block + instruction_index` so the MIR-call need-policy
  metadata consumer can resolve the same instruction without reclassifying the
  method surface.

## Result

The two direct Array/Map pure-boundary set canaries are now metadata-bearing
fixtures. A later card may retry the MIR-call `set` surface prune against these
fixtures and the RuntimeData set fallback boundary separately.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
