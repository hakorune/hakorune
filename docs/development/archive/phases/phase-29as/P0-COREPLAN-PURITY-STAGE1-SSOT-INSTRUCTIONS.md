---
Status: Ready
Scope: Stage-1 purity SSOT (docs-first)
Related:
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
---

# Phase 29as P0: CorePlan purity Stage-1 SSOT (docs-first)

Goal: define Stage-1 purity and align pointers before any code changes.

## Tasks

1. Add SSOT doc:
   - `docs/development/current/main/design/coreplan-purity-stage1-ssot.md`
2. Update done criteria:
   - `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
3. Update roadmap current/next:
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
4. Align Now/Backlog pointers:
   - `docs/development/current/main/10-Now.md`
   - `docs/development/current/main/30-Backlog.md`

## Verification (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
