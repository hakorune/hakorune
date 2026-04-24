---
Status: Landed
Date: 2026-04-25
Scope: Remove dead MIR-call method-surface enum variants and branches after only `has` remains as a legacy method-surface fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-220-stringbox-receiver-surface-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
---

# 291x-221 Method Surface Dead Branch Cleanup Card

## Goal

After the `get` / `set` / `len` / `push` / `substring` / `indexOf`
method-surface rows were pruned, `classify_mir_call_method_surface()` only
returns:

```text
UNKNOWN
HAS
```

Remove the dead enum variants and unreachable route/need/prepass branches so the
remaining fallback surface is explicit.

## Boundary

- Keep the `has` method-surface fallback.
- Do not remove `MapBox`, `ArrayBox`, or `RuntimeDataBox` receiver rows.
- Do not change metadata-first CoreMethod consumers.
- Do not change generic method emit policy rows.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Reduced `MirCallMethodSurfaceKind` to `UNKNOWN` and `HAS`.
- Removed unreachable route-policy branches for pruned method surfaces.
- Removed unreachable need-policy branches for pruned method surfaces.
- Removed the prepass String fallback branch that still referenced
  `indexOf` / `substring` method-surface variants.
- No no-growth row count changed; guard remains `classifiers=10 rows=10`.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
