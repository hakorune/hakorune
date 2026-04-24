---
Status: Landed
Date: 2026-04-25
Scope: Prune the MIR-call route-policy `indexOf` method-surface mirror after StringIndexOf metadata and indexOf-line boundary metadata coverage landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-217-string-indexof-carrier-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-218-indexof-line-boundary-route-fixture-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-219 IndexOf Route Surface Mirror Prune Card

## Goal

Remove the last `indexOf` method-name classifier row from MIR-call route policy:

```c
if (!strcmp(mname, "indexOf")) ...
```

`StringBox.indexOf(needle)` and String-origin `RuntimeDataBox.indexOf(needle)`
now route through `generic_method.indexOf` / `StringIndexOf` metadata. The
indexOf-line boundary seed is also explicit through
`array_text_state_residence_route`.

## Boundary

- Prune only the MIR-call method-surface `indexOf` string classifier and its
  allowlist row.
- Keep receiver-surface fallback rows.
- Keep `has` fallback rows.
- Do not change StringIndexOf carrier vocabulary.
- Preserve metadata-bearing array-string indexOf deferred/window lowering by
  consuming the existing observer metadata lookup instead of rediscovering
  `mname == "indexOf"`.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_indexof_line_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed the MIR-call route-policy `indexOf` method-name classifier row.
- Removed the corresponding no-growth allowlist row.
- Added a route-state bit for StringIndexOf so dispatch no longer needs
  `mname == "indexOf"` to select the StringIndexOf call path.
- Wired deferred array-string indexOf selection from the existing observer
  metadata lookup, not from method-surface classification.
- `core_method_contract_inc_no_growth_guard.sh` dropped from
  `classifiers=12 rows=12` to `classifiers=11 rows=11`.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_indexof_line_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Residual risk:

- The `phase29ck_boundary_pure_array_string_indexof_*` archive fixtures are
  still metadata-absent and fail with `unsupported pure shape for current
  backend recipe` on the baseline commit `c9c2f4869` as well. They are not a
  regression from this prune and should be handled by a separate
  array-text-observer metadata fixture card.
