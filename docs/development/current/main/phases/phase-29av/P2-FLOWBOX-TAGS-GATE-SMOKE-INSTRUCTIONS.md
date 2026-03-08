---
Status: Ready
Scope: Gate smoke for FlowBox schema tags (strict/dev only)
Related:
- docs/development/current/main/phases/phase-29av/README.md
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29av P2: FlowBox tags gate smoke

## Objective

Add an integration smoke that:
- in strict/dev: asserts schema tags are present for selected fixtures
- in non-strict: asserts schema tags are absent

This makes observability drift impossible to miss.

## Outline

1. New smoke script:
   - `tools/smokes/v2/profiles/integration/joinir/flowbox_tags_gate_vm.sh`
2. Run 2–3 existing fixtures (fast):
   - strict: `HAKO_JOINIR_STRICT=1` require `\[flowbox/adopt `
   - non-strict: require absence of `\[flowbox/`
3. Wire into:
   - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
   - `docs/development/current/main/phases/phase-29ae/README.md`

## Acceptance (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

