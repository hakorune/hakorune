---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: lock `HAKO_CAPI_PURE` active ownership to the historical alias probe.
Related:
  - docs/development/current/main/phases/phase-29cv/P96-HAKO-CAPI-PURE-ALIAS-RETIREMENT-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P97-HAKO-CAPI-PURE-ACTIVE-HELPER-RECIPE-SPELLING.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/reference/environment-variables.md
  - tools/dev/phase29ck_boundary_historical_alias_probe.sh
  - tools/dev/phase29ck_boundary_explicit_compat_probe.sh
---

# P98 HAKO_CAPI_PURE Alias Probe Split

## Goal

After P97, active pure-keep helper execution no longer depends on
`HAKO_CAPI_PURE=1`. P98 fixes the remaining active ownership:

```text
HAKO_CAPI_PURE=1
  -> tools/dev/phase29ck_boundary_historical_alias_probe.sh only
```

Canonical active/compat proof should use:

```text
HAKO_BACKEND_COMPILE_RECIPE=pure-first
HAKO_BACKEND_COMPAT_REPLAY=none|harness
```

## Decision

- Keep `tools/dev/phase29ck_boundary_historical_alias_probe.sh` as the single
  active behavior probe for the old spelling.
- Keep `tools/dev/phase29ck_boundary_explicit_compat_probe.sh` as the canonical
  non-alias compat replay reference.
- Update route/env docs so active pure-keep is no longer described as an alias
  user.
- Do not change alias runtime behavior in this card.

## Current Split

| Surface | Owner | Alias behavior |
| --- | --- | --- |
| active pure-keep helper | `tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh` | unsets alias; uses canonical recipe |
| active pure-keep suite wrapper | `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh` | unsets alias; uses canonical recipe |
| historical alias behavior | `tools/dev/phase29ck_boundary_historical_alias_probe.sh` | intentionally sets alias |
| explicit compat replay | `tools/dev/phase29ck_boundary_explicit_compat_probe.sh` | clears alias; uses pure-first export and explicit replay |
| archive/historical scripts | `tools/archive/**`, historical docs | may mention alias as archived evidence only |

## Non-goals

- no warning or fail-fast for `HAKO_CAPI_PURE`
- no C/Rust alias branch deletion
- no active smoke suite expansion
- no `HAKO_BACKEND_COMPAT_REPLAY=harness` behavior change

## Acceptance

```bash
bash tools/dev/phase29ck_boundary_historical_alias_probe.sh
bash tools/dev/phase29ck_boundary_explicit_compat_probe.sh
rg -n "HAKO_CAPI_PURE" tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh \
  tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh \
  tools/dev/phase29ck_boundary_historical_alias_probe.sh \
  tools/dev/phase29ck_boundary_explicit_compat_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
