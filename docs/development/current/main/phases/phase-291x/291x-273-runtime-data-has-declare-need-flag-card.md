---
Status: Landed
Date: 2026-04-26
Scope: Make nyash.runtime_data.has_hh declaration demand-driven.
Related:
  - docs/development/current/main/phases/phase-291x/291x-272-mir-call-maphas-surface-fallback-closeout-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# 291x-273 RuntimeData Has Declare Need-Flag Card

## Goal

Replace the unconditional LLVM declaration for `nyash.runtime_data.has_hh`
with a demand-driven `runtime_data_has` need flag.

## Boundary

- Add only the `runtime_data_has` declaration flag.
- Keep `nyash.runtime_data.get_hh`, `set_hhh`, and `push_hh` declarations
  unchanged in this card.
- Preserve both supported `has_hh` callers:
  - metadata-present `generic_method.has route_kind=runtime_data_contains_any`
  - metadata-absent direct `MapBox.has` fallback via `route.runtime_map_has`

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_has_facade_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
git diff --check
```

## Result

`nyash.runtime_data.has_hh` is now declared only when
`needs.runtime_data_has` is set.

The need flag is produced from both supported call paths:

- `generic_method.has route_kind=runtime_data_contains_any` metadata
- metadata-absent direct `MapBox.has` fallback via `route.runtime_map_has`

The card intentionally leaves `nyash.runtime_data.get_hh`, `set_hhh`, and
`push_hh` declarations unchanged.

## Validated

```bash
python3 -m json.tool apps/tests/mir_shape_guard/runtime_data_has_facade_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_has_facade_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
git diff --check
```
