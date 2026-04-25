---
Status: Landed
Date: 2026-04-26
Scope: Prune the direct ArrayBox.has route fallback after ArrayHas metadata coverage.
Related:
  - docs/development/current/main/phases/phase-291x/291x-268-arrayhas-core-method-carrier-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-269 ArrayBox Has Route Fallback Prune Card

## Goal

Remove the direct `ArrayBox` branch from
`classify_generic_method_has_route(...)`.

After `291x-268`, direct `ArrayBox.has(index)` carries
`core_method.op=ArrayHas` and `route_kind=array_contains_any`. The backend no
longer needs to rediscover this direct ArrayBox route by box name.

## Boundary

- Prune only `classify_generic_method_has_route box ArrayBox`.
- Keep `classify_generic_method_has_route box RuntimeDataBox`.
- Keep generic emit-kind `method has`.
- Keep MIR-call `MapBox + has` surface sentinels.
- Add a direct ArrayBox.has fixture/smoke before deleting the fallback row.

## Acceptance

```bash
python3 -m json.tool apps/tests/mir_shape_guard/array_has_missing_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/array/phase29ck_boundary_pure_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

## Result

`classify_generic_method_has_route box ArrayBox` is removed from the
no-growth allowlist and the backend route classifier. Direct `ArrayBox.has`
now relies on `ArrayHas` CoreMethod route metadata (`array_contains_any` /
`nyash.array.has_hh`) instead of rediscovering the receiver by box name.

Remaining no-growth rows after this card:

- `classify_generic_method_emit_kind method has`
- `classify_generic_method_has_route box RuntimeDataBox`
- `classify_mir_call_receiver_surface box MapBox`
- `classify_mir_call_method_surface method has`

## Validated

```bash
python3 -m json.tool apps/tests/mir_shape_guard/array_has_missing_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/array/phase29ck_boundary_pure_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/core_method_contract_manifest_guard.sh
cargo test -q arrayhas
cargo check -q
git diff --check
```

Guard result:

```text
[core-method-contract-inc-no-growth-guard] ok classifiers=4 rows=4
```

## Next

Do not prune the remaining RuntimeDataBox/MIR-surface rows blindly. The next
card must first pin metadata-absent `RuntimeDataBox.has` and MIR-call `has`
boundary evidence, then remove only rows proven unused.
