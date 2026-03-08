---
Status: SSOT
Scope: Migrate selected strict smokes to FlowBox schema tags
Related:
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/phases/phase-29aw/README.md
---

# P2: Migrate strict smokes to FlowBox tags

## Goal

- Move a small subset of strict smokes from pattern-name tags to FlowBox schema tags.
- Keep existing coreplan/shadow_adopt tag checks (do not remove).
- Release output remains unchanged.

## Target smokes (minimal)

1) `tools/smokes/v2/profiles/integration/joinir/scan_with_init_strict_shadow_vm.sh`
- Require FlowBox tag: box_kind=Loop, via=shadow (features may include return/value_join).

2) `tools/smokes/v2/profiles/integration/joinir/phase29at_match_return_strict_shadow_vm.sh`
- Require FlowBox tag: box_kind=Seq, features includes return, via=shadow.

## SSOT updates

- `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md` (mark P2 migrated smokes).

## Verification (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
