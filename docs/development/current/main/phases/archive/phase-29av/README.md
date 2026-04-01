---
Status: Complete
Scope: FlowBox observability tags (strict/dev only)
Related:
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29av: FlowBox observability tags (strict/dev only)

Goal: replace “pattern-name dependent” observability with FlowBox schema tags,
while keeping release output unchanged.

## Gate / Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Plan

- P0: docs-first SSOT + pointers. ✅
- P1: implement schema tags (strict/dev only) at adopt/freeze boundaries. ✅
- P2: add a dedicated integration smoke to assert schema tags appear only in strict/dev. ✅
- P3: closeout (docs-only). ✅

## Instructions

- P1: `docs/development/current/main/phases/phase-29av/P1-FLOWBOX-TAGS-IMPLEMENTATION-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29av/P2-FLOWBOX-TAGS-GATE-SMOKE-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29av/P3-CLOSEOUT-INSTRUCTIONS.md`
