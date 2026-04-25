---
Status: Landed
Date: 2026-04-26
Scope: Prune the generic method-name `has` emit-kind fallback.
Related:
  - docs/development/current/main/phases/phase-291x/291x-270-runtime-data-has-route-fallback-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-271 Generic Has Emit-Kind Fallback Prune Card

## Goal

Remove the generic `mname == "has"` branch from
`classify_generic_method_emit_kind(...)`.

After 291x-268..270, normal `has` lowering should enter through either:

- MIR `generic_method.has` metadata (`MapHas`, `ArrayHas`, or
  `runtime_data_contains_any`), or
- the still-pinned MIR-call `MapBox + has` surface fallback, represented as
  `route.runtime_map_has`.

The generic emit-kind classifier should not rediscover `has` from the method
name directly.

## Boundary

- Prune only `classify_generic_method_emit_kind method has`.
- Keep MIR-call `MapBox` receiver-surface and `has` method-surface rows.
- Add a metadata-absent direct `MapBox.has` boundary fixture/smoke before
  deleting the generic method-name branch.
- Keep `runtime_data_contains_any` metadata as a valid emit-kind source even
  though it has no `core_method` carrier.

## Acceptance

```bash
python3 -m json.tool apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo test -q runtime_data_has
cargo test -q arrayhas
cargo check -q
git diff --check
```

## Result

`classify_generic_method_emit_kind method has` is removed from the no-growth
allowlist and from the generic method-name classifier.

`has` lowering now enters through:

- `generic_method.has` route metadata, including `runtime_data_contains_any`
  metadata without a `core_method` carrier
- `route.runtime_map_has`, which is produced only by the still-pinned
  MIR-call `MapBox + has` surface fallback

The new metadata-absent direct `MapBox.has` fixture/smoke pins the remaining
fallback pair before any future prune attempt.

Remaining no-growth rows after this card:

- `classify_mir_call_receiver_surface box MapBox`
- `classify_mir_call_method_surface method has`

## Validated

```bash
python3 -m json.tool apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/core_method_contract_manifest_guard.sh
cargo test -q runtime_data_has
cargo test -q arrayhas
cargo check -q
git diff --check
```

Guard result:

```text
[core-method-contract-inc-no-growth-guard] ok classifiers=2 rows=2
```

## Next

The remaining rows are the paired MIR-surface fallback for metadata-absent
direct `MapBox.has`. Treat them as one review unit; pruning either side alone
breaks the newly pinned boundary.
