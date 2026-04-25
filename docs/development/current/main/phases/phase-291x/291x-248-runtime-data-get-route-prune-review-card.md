---
Status: Done
Date: 2026-04-25
Scope: Review whether the remaining RuntimeDataBox fallback row in generic_method.get route policy can be removed after direct ArrayBox and MapBox branches became metadata-bearing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-166-metadata-absent-get-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-200-runtime-data-array-get-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-240-mapbox-get-route-policy-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-247-arraybox-get-route-policy-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-248 RuntimeDataBox Get Route Policy Review Card

## Goal

Determine whether the remaining `RuntimeDataBox` fallback branch in
`classify_generic_method_get_route(...)` can be removed after `291x-240` and
`291x-247` pruned the direct `MapBox` and `ArrayBox` mirror rows.

## Boundary

- Do not change helper symbols or lowering.
- Do not add fallback behavior.
- Do not add new contract rows.
- Keep the `RuntimeDataBox` branch pinned unless the metadata-absent get
  contract is proven complete.

## Analysis

`generic_method.get` is metadata-first, but `291x-166` explicitly pinned the
metadata-absent get fallback contract. The no-growth allowlist still requires:

```text
lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
classify_generic_method_get_route box RuntimeDataBox
replace-with-runtime-data-get-contract-and-metadata-absent-runtime-data-get-boundary-coverage
```

The active boundary fixtures still need the fallback layer for metadata-absent
`RuntimeDataBox.get` paths, so pruning the runtime-data branch would be unsafe.

## Decision

No prune.

## Result

The remaining `RuntimeDataBox` fallback row stays pinned. Direct `ArrayBox`
and `MapBox` get routes are already metadata-first, but metadata-absent get
remains a separate contract.

## Next Work

The next likely seam is the constructor / birth compatibility contract for
zero-arg `ArrayBox` / `MapBox` birth, not another get prune.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

This is a docs-only review card. No code changes were required.
