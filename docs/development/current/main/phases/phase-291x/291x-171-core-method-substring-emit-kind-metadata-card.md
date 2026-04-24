---
Status: Landed
Date: 2026-04-24
Scope: Make generic-method `substring` emit-kind selection prefer MIR CoreMethod metadata before legacy fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-170-core-method-substring-route-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-171 CoreMethod Substring Emit-Kind Metadata Card

## Goal

Move substring emit-kind selection to the metadata-first boundary:

```text
generic_method_routes[].route_id = generic_method.substring
core_method.op = StringSubstring
  -> dispatch selects HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING
  -> legacy substring classifier remains fallback only
```

This is a one-family consumer card. It does not delete allowlist rows and does
not change string corridor/window lowering.

## Boundary

- Do not remove the `substring` allowlist row.
- Do not change substring helper symbols.
- Do not add hot inline lowering.
- Do not infer string receiver legality from method names in the new selector;
  only consume MIR-owned CoreMethod metadata.
- Keep metadata-absent substring routes on the legacy fallback.

## Implementation

- Extend the generic emit-kind metadata selector to accept
  `route_id=generic_method.substring`.
- Accept only `core_method.op=StringSubstring` with
  `proof=core_method_contract_manifest` and
  `lowering_tier=warm_direct_abi`.

## Result

`emit_mir_call_dispatch(...)` can now select `EMIT_SUBSTRING` from valid MIR
substring CoreMethod metadata before the legacy method-name classifier. The
legacy fallback remains required until a separate prune probe proves
metadata-absent fixtures are covered.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_route
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_substring_concat_loop_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
