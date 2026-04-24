---
Status: Landed
Date: 2026-04-25
Scope: Extend MIR-call need-policy CoreMethod metadata consumption beyond direct set routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-190-mir-call-set-need-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-196-mir-call-route-state-metadata-consumer-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
---

# 291x-197 MIR-Call Need Metadata Consumer Card

## Goal

Make MIR-call helper need selection consume the same validated CoreMethod route
metadata as route-state selection:

```text
generic_method_routes[block + instruction_index]
  -> core_method.op + route_kind
  -> MirCallNeedKind
  -> legacy box/method surface classifier only when metadata is absent
```

`291x-190` already did this for direct `ArraySet`/`MapSet`. This card extends
the consumer to read the other landed CoreMethod route families.

## Boundary

- Do not remove route-policy allowlist rows in this card.
- Do not change RuntimeData fallback behavior.
- Do not change helper symbols or lowering.
- Accept only manifest-proven CoreMethod metadata with the expected
  lowering-tier and route-kind pair.
- Keep `StringIndexOf` metadata out of scope because no route carrier is
  currently emitted for it.

## Implementation

- Add a small token mapper from validated `route_id + core_method.op +
  route_kind + lowering_tier` to `MirCallNeedKind`.
- Keep the existing set mappings.
- Add mappings for:
  - `MapGet`
  - `MapHas`
  - `MapLen`
  - `ArrayLen`
  - `ArrayPush`
  - `StringLen`
  - `StringSubstring`

## Result

Metadata-bearing CoreMethod routes now select both route-state and need flags
before legacy box/method surface fallback. Remaining mirror rows are fallback
coverage for metadata-absent RuntimeData/direct boundaries or constructor
compatibility.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
