---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod MapHas metadata to the RuntimeDataBox.has Map-origin boundary fixture before any has mirror pruning.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-164-metadata-absent-has-fallback-contract-card.md
  - apps/tests/mir_shape_guard/runtime_data_map_has_missing_min_v1.mir.json
---

# 291x-209 RuntimeData Map Has Metadata Fixture Card

## Goal

Align the hand-authored Map-origin `RuntimeDataBox.has` boundary fixture with
the MIR-owned CoreMethod route contract.

The MIR planner already promotes this shape:

```text
RuntimeDataBox.has(receiver_origin_box=MapBox, key_route=i64_const)
  -> CoreMethodOp::MapHas
  -> route_kind=map_contains_i64
  -> helper_symbol=nyash.map.probe_hi
```

The boundary fixture should carry that metadata instead of exercising the
metadata-absent `runtime_data_contains_any` fallback.

## Boundary

- Update only the Map-origin `RuntimeDataBox.has` fixture.
- Do not update the Array-origin `RuntimeDataBox.has` fixture; there is no
  `ArrayHas` CoreMethod vocabulary yet.
- Do not prune `.inc` rows in this card.
- Do not change fixture instruction order or expected return behavior.

## Acceptance

```bash
cargo test -q promotes_runtime_data_mapbox_i64_has_to_map_contains_i64
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The Map-origin `RuntimeDataBox.has` boundary fixture now carries `MapHas`
CoreMethod metadata and routes through `map_contains_i64` /
`nyash.map.probe_hi`. The no-growth baseline remains unchanged at
`classifiers=14 rows=14`; `has` mirror pruning stays blocked until Array-origin
fallback coverage has a CoreMethod contract or an explicit keep decision.
