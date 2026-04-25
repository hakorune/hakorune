---
Status: Landed
Date: 2026-04-25
Scope: Repair runtime-data get boundary smokes that still used metadata-absent fixtures.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json
  - apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
---

# 291x-235 RuntimeData Get Boundary Smoke Repair Card

## Goal

Make the phase29ck pure-first `RuntimeDataBox.get` boundary smokes runnable
again.

The scripts failed with:

```text
unsupported pure shape for current backend recipe
```

because their fixtures were still metadata-absent while the current
pure-first reader expects explicit CoreMethod route metadata for `get`.

## Boundary

- Do not re-enable harness fallback.
- Do not change runtime helper symbols.
- Do not broaden source-level language behavior.
- Keep the fixtures as narrow boundary-shape canaries.

## Repair

- Add `generic_method.get` route metadata to the ArrayBox and MapBox
  runtime-data get fixtures.
- Enable route tracing in the smokes.
- Check the route-state evidence in the build log before accepting the object.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_get_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 smoke repair slice.

- Added `generic_method.get` route metadata to the ArrayBox and MapBox
  runtime-data get boundary fixtures.
- Kept ArrayBox get on the existing `nyash.array.slot_load_hi` route.
- Kept MapBox get on the existing `nyash.runtime_data.get_hh` facade route.
- Added route-trace checks so the smokes fail if the expected get route-state
  is not selected.
