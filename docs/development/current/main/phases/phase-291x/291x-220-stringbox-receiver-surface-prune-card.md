---
Status: Landed
Date: 2026-04-25
Scope: Prune the MIR-call route-policy `StringBox` receiver-surface mirror after String method-surface fallbacks were removed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-208-len-route-surface-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-215-substring-route-surface-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-219-indexof-route-surface-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-220 StringBox Receiver Surface Prune Card

## Goal

Remove the remaining `StringBox` receiver-surface classifier row:

```c
if (!strcmp(bname, "StringBox")) ...
```

String len, substring, and indexOf now use CoreMethod / route metadata, so the
receiver-surface `StringBox` branch is dead compatibility surface.

## Boundary

- Prune only the MIR-call receiver-surface `StringBox` row and dead branches
  that become unreachable with that row gone.
- Keep `MapBox`, `ArrayBox`, and `RuntimeDataBox` receiver rows.
- Keep `has` method-surface fallback.
- Do not change CoreMethod carrier vocabulary.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed the MIR-call receiver-surface `StringBox` classifier row.
- Removed the now-unreachable StringBox receiver branches from route and need
  policy.
- Removed the corresponding no-growth allowlist row.
- `core_method_contract_inc_no_growth_guard.sh` dropped from
  `classifiers=11 rows=11` to `classifiers=10 rows=10`.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
