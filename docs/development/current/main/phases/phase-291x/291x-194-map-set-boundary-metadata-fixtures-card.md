---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod metadata to remaining direct Map set pure-boundary MIR JSON fixtures.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-184-set-map-storage-route-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-192-direct-set-boundary-metadata-fixtures-card.md
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
---

# 291x-194 Map Set Boundary Metadata Fixtures Card

## Goal

Finish the direct MapBox `set` pure-boundary fixture coverage needed before
retrying the generic set storage-route `MapBox` fallback row prune.

The remaining metadata-absent direct Map set payloads were:

```text
MapBox.set -> get -> ret
MapBox.set -> size -> ret
```

## Boundary

- Update only inline MIR JSON fixture metadata.
- Do not prune `classify_generic_method_set_route(... "MapBox")` in this card.
- Do not change helper symbols, lowering, or expected return codes.
- Keep RuntimeDataBox.set fallback out of scope.

## Implementation

- Add `metadata.generic_method_routes` for the direct Map set instruction in
  the map get/unbox pure canary.
- Add `metadata.generic_method_routes` for the direct Map set instruction in
  the map set/size pure canary.
- Use `core_method.op=MapSet`, `route_kind=map_store_any`, and
  `lowering_tier=cold_fallback`.

## Result

All known direct pure-boundary MapBox set payloads now carry MapSet metadata.
The next card may retry pruning the legacy `MapBox` storage-route branch and
adjust the set policy guard to accept metadata-owned MapSet route consumption.

## Acceptance

```bash
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
