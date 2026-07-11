---
Status: Done
Scope: is_integer unsupported (docs-only)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29aq P8: StringUtils.is_integer unsupported (SSOT)

Goal: keep is_integer unsupported in current Plan/Composer subsets and document
handoff to a future return-in-loop minimal phase.

## Rationale (SSOT)

- is_integer contains return-heavy loops (return false inside the loop).
- Current Pattern2Break / ScanWithInit / SplitScan subsets do not model
  return-in-loop; forcing it would weaken subset boundaries.
- Address via a dedicated CorePlan vocabulary step (return-in-loop minimal).

## Inventory update

- Keep is_integer as Unsupported with the reason above.

## Verification

- ./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
