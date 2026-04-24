---
Status: Landed
Date: 2026-04-24
Scope: Make generic-method `len`/`length`/`size` emit-kind selection prefer MIR CoreMethod metadata before legacy alias fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-167-core-method-len-route-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-168 CoreMethod Len Emit-Kind Metadata Card

## Goal

Move the length-family emit-kind selector to the metadata-first boundary:

```text
generic_method_routes[].route_id = generic_method.len
core_method.op = ArrayLen | MapLen | StringLen
  -> dispatch selects HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN
  -> legacy len/length/size classifier remains fallback only
```

This is a one-family consumer card. It does not delete allowlist rows and does
not change helper symbols or lowering.

## Boundary

- Do not remove `len`/`length`/`size` allowlist rows.
- Do not change `nyash.array.slot_len_h`, `nyash.map.entry_count_i64`, or
  `nyash.string.len_h` selection.
- Do not add hot inline lowering.
- Do not infer receiver family from method names in the new selector; only
  consume MIR-owned CoreMethod metadata.
- Keep metadata-absent length routes on the legacy alias fallback.

## Implementation

- Extend the generic emit-kind metadata selector to accept
  `route_id=generic_method.len`.
- Accept only `core_method.op` values `ArrayLen`, `MapLen`, or `StringLen`
  with `proof=core_method_contract_manifest` and
  `lowering_tier=warm_direct_abi`.

## Result

`emit_mir_call_dispatch(...)` can now select `EMIT_LEN` from valid MIR
length-family CoreMethod metadata before the legacy alias classifier. The
legacy `len`/`length`/`size` fallback remains required for metadata-absent MIR
JSON fixtures until a separate prune probe proves otherwise.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_route
cargo test -q map_lookup_fusion
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
