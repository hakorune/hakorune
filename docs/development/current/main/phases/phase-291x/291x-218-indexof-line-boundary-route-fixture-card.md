---
Status: Landed
Date: 2026-04-25
Scope: Add the missing indexOf-line boundary route metadata fixture without changing codegen or pruning legacy indexOf fallback rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-217-string-indexof-carrier-consumer-card.md
  - apps/tests/mir_shape_guard/indexof_line_pure_min_v1.mir.json
  - lang/c-abi/shims/hako_llvmc_ffi_indexof_text_state_residence.inc
  - tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh
---

# 291x-218 IndexOf Line Boundary Route Fixture Card

## Goal

Make the archive boundary seed explicit for the existing indexOf-line
text-state residence matcher:

```text
indexof_line_pure_min_v1
  -> metadata.array_text_state_residence_route
  -> direct_array_text_state_residence
  -> hako_llvmc_match_indexof_line_text_state_residence_fn(...)
```

This is a fixture metadata card only. It removes the harness-fallback
dependency from the boundary smoke but does not change the matcher or prune the
legacy `indexOf` method-surface fallback row.

## Boundary

- Add only the metadata contract consumed by the existing
  `hako_llvmc_match_indexof_line_text_state_residence_fn` matcher.
- Do not add new `.inc` method-name classifiers.
- Do not change runtime behavior or helper symbols.
- Do not prune `indexOf` fallback rows in this card.

## Acceptance

```bash
python3 -m json.tool apps/tests/mir_shape_guard/indexof_line_pure_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_indexof_line_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Added `metadata.array_text_state_residence_route` to
  `indexof_line_pure_min_v1.mir.json`.
- The fixture now satisfies the existing
  `hako_llvmc_match_indexof_line_text_state_residence_fn` metadata contract.
- No `.inc` classifier rows or fallback rows were changed.
- `core_method_contract_inc_no_growth_guard.sh` remains at
  `classifiers=12 rows=12`.

Validated with:

```bash
python3 -m json.tool apps/tests/mir_shape_guard/indexof_line_pure_min_v1.mir.json >/dev/null
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_indexof_line_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
