---
Status: Landed
Date: 2026-04-25
Scope: Make generic_method.len route selection prefer validated CoreMethod metadata for direct ArrayLen and MapLen boundaries.
Related:
  - docs/development/current/main/phases/phase-291x/291x-167-core-method-len-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-168-core-method-len-emit-kind-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-242 Len Route Metadata Consumer Card

## Goal

Make the generic_method.len route policy prefer CoreMethod metadata-derived flags over legacy box-name classification:

```text
classify_generic_method_len_route(bname, plan)
  -> check plan->runtime_map_size first (from MapLen metadata)
  -> check plan->runtime_array_len first (from ArrayLen metadata)
  -> check plan->runtime_string first (from StringLen metadata)
  -> fallback to bname check for metadata-absent compatibility
```

This is a policy consumer card. It does not remove allowlist rows but advances the route selector to metadata-first while keeping legacy fallback intact.

## Boundary

- Do not remove `MapBox` or `ArrayBox` allowlist rows from `core_method_contract_inc_no_growth_allowlist.tsv`.
- Keep metadata-absent fallback behavior (bname checks) functional.
- Do not change helper symbols (`nyash.array.slot_len_h`, `nyash.map.entry_count_i64`, `nyash.string.len_h`).
- Do not add new inline lowering.
- Keep RuntimeDataBox compatibility unchanged.

## Implementation

Reorder `classify_generic_method_len_route` in `hako_llvmc_ffi_generic_method_len_policy.inc`:

```c
// Before: (bname && !strcmp(bname, "MapBox")) || plan->runtime_map_size
// After:  plan->runtime_map_size first, then bname fallback

if (plan->runtime_map_size) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT;
}
if (plan->runtime_array_len || plan->runtime_array_string) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN;
}
if (plan->runtime_string) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN;
}
// Fallback for metadata-absent cases
if (bname && !strcmp(bname, "MapBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT;
}
if (bname && !strcmp(bname, "ArrayBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN;
}
```

The metadata flags (`runtime_map_size`, `runtime_array_len`, `runtime_string`) are populated by `classify_mir_call_generic_method_route_kind_from_core_method_metadata` in `hako_llvmc_ffi_mir_call_route_policy.inc` which reads `core_method.op` from MIR JSON.

## Result

The len route policy now prefers validated CoreMethod metadata (MapLen, ArrayLen, StringLen) when present, providing direct boundaries for metadata-carrying fixtures. Legacy box-name fallback remains for metadata-absent MIR.

This creates the seam for a future card to probe and potentially prune the MapBox/ArrayBox allowlist rows once metadata-absent len coverage is proven.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/quick/core/map/map_len_size_vm.sh
bash tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh
git diff --check
```
