---
Status: Landed
Date: 2026-04-25
Scope: Add StringSubstring CoreMethod metadata and exact seed route metadata to the substring boundary fixture.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-172-metadata-absent-substring-fallback-contract-card.md
  - apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json
---

# 291x-213 Substring Boundary Metadata Fixtures Card

## Goal

Replace the representative metadata-absent substring boundary fixture with
manifest-backed `StringSubstring` route metadata, and restore the pure-boundary
entry metadata required by the archived `substring-concat-loop-v1` seed.

The target fixture has three `RuntimeDataBox.substring` calls in block `19`.
They now carry:

```text
generic_method.substring
core_method.op = StringSubstring
route_kind = string_substring
helper_symbol = nyash.string.substring_hii
```

## Boundary

- Update only `substring_concat_loop_pure_min_v1.mir.json`.
- Do not modify string corridor/window/direct-kernel routes.
- Do not prune `.inc` rows in this card.
- Do not change instruction order, CFG, or expected boundary behavior.

## Implementation Notes

The archive boundary smoke does not enter through generic method lowering. It
uses the exact-seed pure compiler route:

```text
exact_seed_backend_route.tag = substring_concat_loop_ascii
source_route = string_kernel_plans.loop_payload
```

Therefore the fixture must carry both layers:

- `generic_method.substring` metadata for future mirror pruning.
- `exact_seed_backend_route + string_kernel_plans` metadata for the archived
  pure-boundary default route.

## Acceptance

```bash
python3 -m json.tool apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json >/dev/null
cargo test -q records_runtime_data_substring_from_string_origin
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The `substring_concat_loop_pure_min_v1` boundary fixture now carries three
`generic_method.substring` / `StringSubstring` routes plus the exact
`substring_concat_loop_ascii` seed metadata required by the archive pure
compiler route. The archive boundary smoke now reaches the default object
emitter without `ny-llvmc` harness fallback. The no-growth baseline remains
`classifiers=14 rows=14`.
