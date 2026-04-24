---
Status: Landed
Date: 2026-04-25
Scope: Prune the MIR-call route-policy substring method-surface mirror after StringSubstring metadata coverage landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-214-substring-emit-kind-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-215 Substring Route-Surface Mirror Prune Card

## Goal

Remove the MIR-call method-surface classifier row:

```c
if (!strcmp(mname, "substring")) {
  return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SUBSTRING;
}
```

Route-state and helper-need selection should consume `StringSubstring`
CoreMethod metadata instead.

## Boundary

- Prune only MIR-call method-surface `substring`.
- Keep receiver-surface fallback rows.
- Do not change generic-method substring lowering.
- Do not touch `indexOf` or `has`.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_substring_concat_loop_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The MIR-call route-policy `substring` method-surface mirror row is gone.
Route-state and helper-need selection remain covered by `StringSubstring`
CoreMethod metadata. Receiver-surface fallback rows stay pinned for remaining
`has` / `indexOf` / constructor and compat cases. The no-growth baseline
dropped from `classifiers=13 rows=13` to `classifiers=12 rows=12`.
