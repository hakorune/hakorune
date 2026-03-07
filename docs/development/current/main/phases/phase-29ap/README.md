---
Status: Complete
Scope: Legacy extractor reduction (planner+composer SSOT)
Related:
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ap: Legacy extractor reduction (Step-E)

Goal: Reduce legacy extractor fallbacks while keeping planner+composer as the SSOT path.

## What changed

- JoinIR legacy loop table removed; router now delegates to plan/composer only.
- stdlib loop subsets migrated to plan facts (StringUtils.to_lower, StringUtils.join, trim_start/trim_end).
- legacy labels `2/4/8/9` routing removed; nested_loop_minimal lane（historical label `6`）adopted via CorePlan (strict+release).
- Dead-code cleanup after legacy removal (unused JoinIR lowerers removed).

## Gate (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Residuals / Next

- Next phase is TBD (choose between Plan/Composer subset expansion or CorePlan vocabulary expansion).
