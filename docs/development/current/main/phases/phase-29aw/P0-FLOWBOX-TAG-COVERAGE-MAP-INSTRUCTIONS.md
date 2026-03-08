---
Status: Ready
Scope: Define FlowBox tag coverage map (docs-first)
Related:
- docs/development/current/main/phases/phase-29aw/README.md
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aw P0: FlowBox tag coverage map (docs-first)

## Objective

Fix a stable mapping from “regression scenarios” to expected FlowBox schema tags,
so future refactors do not regress observability or silently re-introduce
pattern-name coupling.

## Tasks

1. Create an SSOT table (in a new design doc) that maps:
   - Scenario (smoke name)
   - Expected `box_kind` (Loop/If2/BranchN/Seq/Leaf)
   - Expected `features` subset (csv; empty allowed)
   - Expected `via` (shadow/release) for adopt tags
2. Decide the minimal set of scenarios to gate (keep fast):
   - `scan_with_init` (Loop, `via=shadow`; historical label 6)
   - `split_scan` (Loop, possibly `value_join`; historical label 7)
   - `is_integer` strict reject (negative coverage: no FlowBox tag, fail-fast marker required)
   - `match_return` (BranchN, features empty or `return`)
3. Wire a new gate smoke (Phase 29aw P1) into:
   - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
   - `docs/development/current/main/phases/phase-29ae/README.md`

## Acceptance

- docs-only (no tests required), or optionally:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
