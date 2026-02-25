---
Status: SSOT
Scope: Observability tags for CorePlan/FlowBox (strict/dev only)
Related:
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# FlowBox observability tags (SSOT)

## Goal

Make strict/dev observability tags stable and schema-based, so they do not
depend on pattern names as the planner/normalizer evolves.

Release output must remain unchanged.

## Principles

- Tags are **strict/dev only**.
- Tags are single-line and start with `[` so they can be filtered (`filter_noise`).
- Tags are emitted at SSOT boundaries (planner outcome, composer adopt, freeze).
- No by-name dispatch; tag contents are derived from existing SSOT state.

## Tag schema (Stage-1)

One of:

- Adopt:
  - `[flowbox/adopt box_kind=<Loop|If2|BranchN|Seq|Leaf> features=<csv> via=<shadow|release>]`
- Freeze:
  - `[flowbox/freeze code=<freeze_code> box_kind=<...> features=<csv>]`
- Fallback (deprecated):
  - Phase 29ba で `[plan/fallback:*]` は撤去し、strict/dev のフォールバック観測は
    `[flowbox/freeze ...]` のみを SSOT とする。

### `features` vocabulary (minimal)

Use the smallest set that is already available without re-analysis:

- `return`
- `break`
- `continue`
- `value_join`
- `cleanup`
- `nested_loop`

Missing/unknown features are omitted (empty string allowed).

## Non-goals

- No new env vars.
- No large JSON dumps in tags.
- No behavior changes in release.
