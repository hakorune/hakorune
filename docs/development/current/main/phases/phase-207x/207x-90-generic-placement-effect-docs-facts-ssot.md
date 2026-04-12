# 207x-90 Generic Placement / Effect Docs-Facts SSOT

Status: SSOT

## Goal

- fix the first docs/facts cut for the generic placement / effect layer
- keep string corridor / sum placement / thin-entry pilots as pilot scaffolds under the roadmap layer, not separate top-level rows

## In Scope

- inventory existing pilot surfaces and the facts they already export:
  - string corridor: `string_corridor_facts`, `string_corridor_candidates`, `metadata.string_kernel_plans`
  - sum placement: `sum_placement_facts`, `sum_placement_selections`, `sum_placement_layouts`
  - thin-entry: `thin_entry_candidates`, `thin_entry_selections`
- define what belongs to generic placement / effect vs family-specific pilot layers
- define the first handoff into `agg_local scalarization` after this docs cut

## Pilot Boundary Notes

- generic placement / effect owns placement, publish, materialize, and direct-kernel legality
- string / sum / thin-entry own family-specific pilot facts and candidate generation
- backend consumers stay backend consumers; they do not become semantic owners

## Fixed Decisions

- the layer-order SSOT remains `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- no dedicated deeper design SSOT is required before this phase
- `phase207x` is docs/facts only; no code changes in this cut
- `agg_local scalarization` is the next follow-on after this docs/facts phase
- the generic layer should fold the landed pilots instead of adding new family rows

## Out of Scope

- any code widening
- semantic changes to string / sum / user-box / array / map
- DCE / simplification-bundle changes

## Acceptance

- docs-only
- current pointers are updated to reference `phase207x`
- `git diff --check`
