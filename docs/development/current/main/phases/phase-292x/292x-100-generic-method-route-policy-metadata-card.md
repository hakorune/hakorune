---
Status: Landed
Date: 2026-04-22
Scope: next cleanup card for moving generic method route-policy decisions out of `.inc` raw MIR inspection and into MIR-owned metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
---

# 292x-100: Generic Method Route Policy Metadata

## Problem

The migrated `array_rmw_window`, `array_string_len_window`, and string
direct-set source-window routes now demonstrate the intended boundary shape:
MIR owns legality, and `.inc` reads route metadata. The remaining generic
method policy layer still has C-side route ownership around method
classification, route selection, and helper-specific fallback paths.

## Decision

Move the next route-policy decision into MIR metadata only after identifying one
narrow method family. `.inc` may keep only:

- metadata reader / field validation
- selected helper emission
- skip marking
- fail-fast on malformed metadata

Selected first family:

- `has` generic method route policy
- MIR route id: `generic_method.has`
- route kinds:
  - `runtime_data_contains_any` -> `nyash.runtime_data.has_hh`
  - `map_contains_any` -> `nyash.map.probe_hh`

This is deliberately narrower than `get` / `set` / `len`: `has` has no
window/placement fast path and no publication-side objectization policy, so it
can prove the metadata-first boundary with minimal coupling.

## Acceptance

Pin the first route-policy leaf with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
```

## Result

- Added MIR-owned `GenericMethodRoute` metadata for the `has` family.
- JSON now emits `generic_method_routes` entries with `route_id`,
  `route_kind`, helper symbol, proof, receiver/key values, and effects.
- `hako_llvmc_ffi_generic_method_has_policy.inc` now consumes
  `generic_method.has` metadata first and fail-fasts on malformed matching
  entries; legacy classification remains only as compatibility fallback when
  metadata is absent.
- RuntimeDataBox has boundary fixtures now carry the route metadata, and the
  smokes require `stage=generic_method_has_route result=hit
  reason=mir_route_metadata`.
