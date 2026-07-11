# 216x-90 Sum Seed Metadata Helper Consumer Fold SSOT

Status: SSOT

## Goal

- move the current sum local seed metadata helper to folded `placement_effect_routes` first
- keep the cut behavior-preserving on the boundary pure-first lane

## In Scope

- add the minimal folded route field required by current sum routes:
  - `source_value`
- read `placement_effect_routes` first for:
  - `thin_entry` rows used by current sum seed matchers
  - `sum_placement` local-aggregate rows
  - `agg_local_scalarization` `sum_local_layout(...)` rows
- preserve family-specific metadata as compatibility fallback
- pin the folded route path with metadata-bearing sum fixtures

## Fixed Decisions

- `source_value` is generic folded route transport, not a sum-only exporter side channel
- current sum seed helper remains a backend consumer, not a new semantic owner
- `placement_effect_routes` is primary; legacy `thin_entry_selections` / `sum_placement_*` stay fallback
- this cut only proves the current sum local seed metadata helper surface

## Out of Scope

- changing current sum aggregate IR shape
- deleting legacy metadata lanes
- widening folded route consumers to unrelated families in the same cut

## Acceptance

- the current sum local seed metadata helper can prove the selected local route from `placement_effect_routes`
- metadata-bearing sum boundary smoke stays green
- `git diff --check`

