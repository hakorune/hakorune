---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: move active pure-keep helper from `HAKO_CAPI_PURE` alias to canonical backend recipe spelling.
Related:
  - docs/development/current/main/phases/phase-29cv/P96-HAKO-CAPI-PURE-ALIAS-RETIREMENT-INVENTORY.md
  - tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh
  - tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
  - tools/smokes/v2/profiles/integration/proof/pure-legacy-cluster/README.md
---

# P97 HAKO_CAPI_PURE Active Helper Recipe Spelling

## Goal

Remove `HAKO_CAPI_PURE=1` from active pure-keep helper execution without
changing the keep canary payloads.

The active helper should prove the same boundary pure-first route through:

```text
HAKO_BACKEND_COMPILE_RECIPE=pure-first
HAKO_BACKEND_COMPAT_REPLAY=none
```

## Decision

- `phase2120_boundary_pure_helper.sh` exports canonical recipe/replay values.
- Active helper execution unsets `HAKO_CAPI_PURE` before invoking `ny-llvmc`.
- `run_pure_keep.sh` no longer exports `HAKO_CAPI_PURE`.
- The pure legacy cluster README no longer states that the alias is required.

## Non-goals

- no warning or fail-fast for `HAKO_CAPI_PURE`
- no historical alias probe change
- no archive script rewrite
- no backend acceptance widening

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
rg -n "HAKO_CAPI_PURE" tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh \
  tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh \
  tools/smokes/v2/profiles/integration/proof/pure-legacy-cluster/README.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
