---
Status: Landed
Date: 2026-04-25
Scope: Clarify the remaining receiver-surface fallback blockers before any receiver row pruning.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
---

# 291x-216 Receiver Surface Fallback Sunset Design Card

## Goal

Prevent receiver-surface rows from being pruned one-by-one while method
fallbacks still depend on them.

The route-policy and need-policy layers now consume many CoreMethod metadata
families, but the remaining receiver-surface classifier still feeds the
metadata-absent fallback path for:

- `has`, especially Array-origin `RuntimeDataBox.has`
- `indexOf`, until `generic_method.indexOf` exists
- constructor / birth compatibility
- RuntimeData set and other mutating compat boundaries

## Decision

Do not prune receiver-surface rows until the corresponding method-surface and
compat rows are gone or exact non-use is proven by fixtures and smokes.

Required blockers:

- `MapBox` receiver row: keep while `has` fallback remains.
- `ArrayBox` receiver row: keep while Array-origin `has` and constructor/set
  compat remain.
- `StringBox` receiver row: keep while `indexOf` route-surface fallback remains.
- `RuntimeDataBox` receiver row: keep while RuntimeData set/has/String
  compatibility fallback remains.

## Next Implementable Work

The next cleanup should not target receiver rows directly. Prefer one of:

1. `ArrayHas` CoreMethod contract, if Array-origin `RuntimeDataBox.has` should
   become metadata-owned.
2. `StringIndexOf` route carrier and consumers.
3. Constructor/birth contract for zero-arg ArrayBox/MapBox birth.
4. RuntimeData set metadata-absent boundary retirement.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The no-growth allowlist now names the receiver-surface sunset blockers
explicitly. Receiver rows remain pinned at this point; the no-growth baseline
stays `classifiers=12 rows=12`.
