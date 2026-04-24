---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod MapHas metadata to the RuntimeData dispatch E2E fixture and lock the map-probe symbol in the smoke.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-209-runtime-data-map-has-metadata-fixture-card.md
  - apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json
  - tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
---

# 291x-210 RuntimeData Map Has Dispatch Fixture Card

## Goal

Close the RuntimeData dispatch E2E fixture gap for Map-origin `has`:

```text
RuntimeDataBox.has(receiver_origin_box=MapBox, key_route=i64_const)
  -> CoreMethodOp::MapHas
  -> map_contains_i64
  -> nyash.map.probe_hi
```

This keeps the app-level dispatch fixture aligned with the MIR planner route
contract already covered by the shape-guard fixture.

## Boundary

- Add only the Map-origin `RuntimeDataBox.has` route metadata.
- Keep the Array-origin `RuntimeDataBox.has` call metadata-absent; it still
  exercises the `runtime_data_contains_any` fallback until an `ArrayHas`
  contract or explicit keep decision exists.
- Do not prune `.inc` rows in this card.
- Do not change expected executable return code.

## Acceptance

```bash
python3 -m json.tool apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The RuntimeData dispatch E2E fixture now carries `MapHas` CoreMethod metadata
for the Map-origin `has` call. The smoke also checks both `nyash.runtime_data.has_hh`
for the Array-origin fallback and `nyash.map.probe_hi` for the Map-origin
CoreMethod route. The no-growth baseline remains `classifiers=14 rows=14`.
