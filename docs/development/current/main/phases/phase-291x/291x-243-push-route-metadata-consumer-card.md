---
Status: Landed
Date: 2026-04-25
Scope: Make generic_method.push route selection prefer validated CoreMethod metadata for direct ArrayPush boundaries while keeping RuntimeDataBox fallback pinned.
Related:
  - docs/development/current/main/phases/phase-291x/291x-174-core-method-push-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-175-core-method-push-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-242-len-route-metadata-consumer-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-243 Push Route Metadata Consumer Card

## Goal

Make the generic_method.push route policy prefer CoreMethod metadata-derived flags over legacy box-name classification:

```text
classify_generic_method_push_route(bname, plan)
  -> check plan->runtime_array_push first (from ArrayPush metadata)
  -> check plan->runtime_array_string first (from ArrayPush metadata)
  -> fallback to bname check for metadata-absent compatibility
```

This is a policy consumer card. It does not remove allowlist rows but advances the route selector to metadata-first while keeping legacy fallback intact.

## Boundary

- Do not remove `ArrayBox` or `RuntimeDataBox` allowlist rows from `core_method_contract_inc_no_growth_allowlist.tsv`.
- Keep metadata-absent fallback behavior (bname checks) functional.
- RuntimeDataBox.push fallback is explicitly pinned for mutating boundary coverage; do NOT prune.
- Do not change helper symbols (`nyash.array.slot_append_hh`, `nyash.runtime_data.push_hh`).
- Do not add new inline lowering.
- Keep BoxCount separate from BoxShape work.

## Implementation

Reorder `classify_generic_method_push_route` in `hako_llvmc_ffi_generic_method_push_policy.inc`:

```c
// Before: (bname && !strcmp(bname, "ArrayBox")) || plan->runtime_array_push
// After:  plan metadata flags first, then bname fallback

if (plan && (plan->runtime_array_push || plan->runtime_array_string)) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY;
}
// Fallback for metadata-absent cases
if (bname && !strcmp(bname, "ArrayBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY;
}
if (bname && !strcmp(bname, "RuntimeDataBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_RUNTIME_DATA_APPEND_ANY;
}
```

The metadata flags (`runtime_array_push`, `runtime_array_string`) are populated by `classify_mir_call_generic_method_route_kind_from_core_method_metadata` in `hako_llvmc_ffi_mir_call_route_policy.inc` which reads `core_method.op=ArrayPush` from MIR JSON.

## Result

The push route policy now prefers validated CoreMethod metadata (ArrayPush) when present, providing direct boundaries for metadata-carrying ArrayBox fixtures. Legacy box-name fallback remains for metadata-absent MIR.

RuntimeDataBox.push continues to use metadata-absent fallback, which is explicitly pinned by mutating boundary coverage requirements.

This creates the seam for a future card to probe and potentially prune the ArrayBox allowlist row once metadata-absent push coverage is proven, while RuntimeDataBox.push remains a permanent compat sentinel.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh
bash tools/smokes/v2/profiles/quick/core/array/array_push_vm.sh
git diff --check
```
