---
Status: SSOT
Scope: CorePlan purity (Stage-1) - strict/dev fallback visibility
Related:
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/flowbox-fallback-observability-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# CorePlan purity Stage-1 (SSOT)

Purpose: define the minimum "purity" bar for CorePlan routing so fallback is
visible in strict/dev while release logs remain unchanged.

## Stage-1 Done (SSOT)

- JoinIR regression gate stays green:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- For gate-target shapes, strict/dev never silently falls back:
  - fallback is surfaced via `flowbox/freeze` (fail-fast tag with stable code).
- Fallback vocabulary is SSOT and does not depend on by-name dispatch.

## Ok(None) vs Freeze boundary

- Ok(None) is allowed only for non-candidates (shape clearly out of scope).
- If the shape looks like a gate-target candidate but composition fails:
  - strict/dev must emit a fallback tag or Freeze.
- Release behavior stays unchanged (no new tags/logs).

## Fallback vocabulary (SSOT)

FlowBox fallback observability の SSOT に収束する:

- `docs/development/current/main/design/flowbox-fallback-observability-ssot.md`

最小 code 語彙（strict/dev only）:

- `planner_none`
- `composer_reject`
- `unstructured`
- `unwind`

## Verification (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
