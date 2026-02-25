---
Status: Complete
Scope: FlowBox tag coverage (Stage-2) - migrate gates away from pattern-name tags
Related:
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aw: FlowBox tag coverage Stage-2

Goal: make regression coverage rely on FlowBox schema tags (`[flowbox/*]`)
instead of pattern-name tags, while keeping release output unchanged.

## Gate / Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Plan

- P0: docs-first mapping (scenario → expected FlowBox tag fields). ✅
- P1: add a dedicated gate smoke that asserts FlowBox tags for core scenarios. ✅
- P2: optional: incrementally migrate existing tag smokes to FlowBox schema. ✅
- P3: closeout (docs-only). ✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29aw/P0-FLOWBOX-TAG-COVERAGE-MAP-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29aw/P1-FLOWBOX-TAGS-ONLY-GATE-SMOKE-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29aw/P2-FLOWBOX-TAGS-MIGRATION-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29aw/P3-CLOSEOUT-INSTRUCTIONS.md`
