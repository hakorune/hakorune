---
Status: SSOT
Scope: FlowBox tag coverage gate (strict/non-strict)
Related:
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# P1: FlowBox tags-only gate smoke

## Goal

- Add a dedicated gate that checks FlowBox tags per the coverage map.
- Strict/dev must emit FlowBox tags; non-strict must remain silent.
- Keep release output/logs unchanged.

## Steps

1. Add gate script

- `tools/smokes/v2/profiles/integration/joinir/phase29aw_flowbox_tag_coverage_gate_vm.sh`
- Strict/dev:
  - require `[flowbox/adopt box_kind=...]` per coverage map
  - allow extra features (required subset only)
- Non-strict:
  - assert no `[flowbox/` tags

2. Wire to regression pack

- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `docs/development/current/main/phases/phase-29ae/README.md`

## Verification (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
