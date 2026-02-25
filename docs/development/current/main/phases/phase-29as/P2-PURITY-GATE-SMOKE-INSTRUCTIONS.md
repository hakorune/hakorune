---
Status: Ready
Scope: CorePlan purity Stage-1 (purity gate smoke)
Related:
- docs/development/current/main/phases/phase-29as/README.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29as P2: purity gate smoke

Goal: add an integration smoke that asserts strict/dev does not emit any
`[plan/fallback:*]` tags for gate-target shapes, using raw output.

## Non-goals

- No changes to existing quick smoke expectations.
- No release-only logging changes.

## Implementation outline

1. Add a dedicated smoke script (VM) that:
   - runs a small subset of gate fixtures with strict/dev enabled
   - checks raw output does not contain `[plan/fallback:`
2. Wire it into `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`.

## Acceptance (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
