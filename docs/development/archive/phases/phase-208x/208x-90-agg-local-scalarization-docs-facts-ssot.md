# 208x-90 Agg Local Scalarization Docs-Facts SSOT

Status: SSOT

## Goal

- fix the first docs/facts cut for the agg_local scalarization layer
- keep sum placement / thin-entry / user-box local-body / ArrayBox pilot scaffolds under the roadmap layer, not separate top-level rows

## In Scope

- inventory existing pilot surfaces and the facts they already export:
  - sum placement: `sum_placement_facts`, `sum_placement_selections`, `sum_placement_layouts`
  - thin-entry: `thin_entry_candidates`, `thin_entry_selections`
  - user-box local bodies: `field_decls + thin_entry_selections.inline_scalar`
  - ArrayBox typed-slot pilot: narrow typed-slot storage lane
  - tuple multi-payload compat transport: hidden payload boxes only
- define what belongs to agg_local scalarization vs family-specific pilot layers
- define the first handoff into the actual agg_local scalarization layer after this docs cut

## Pilot Boundary Notes

- agg_local scalarization owns aggregate payload scalarization, local-body layout selection, and scalar SSA shaping
- sum / thin-entry / user-box / ArrayBox own family-specific pilot facts and candidate generation
- backend consumers stay backend consumers; they do not become semantic owners

## Inventory Matrix

| Family | Current exported surface | Owner boundary |
| --- | --- | --- |
| sum placement | `sum_placement_facts`, `sum_placement_selections`, `sum_placement_layouts` | sum keeps pilot facts/layouts, while agg_local scalarization owns the aggregate-to-scalar reading |
| thin-entry | `thin_entry_candidates`, `thin_entry_selections` | thin-entry inventory stays metadata-only until the later actual-consumer switch |
| user-box local bodies | `field_decls + thin_entry_selections.inline_scalar` | local body proof is still a pilot inference, not a separate semantic owner |
| ArrayBox typed-slot | narrow typed-slot storage pilot | ArrayBox remains runtime authority; typed slots are compiler/internal only |
| tuple multi-payload compat | hidden payload boxes | compat transport only, not semantic owner |

## Fixed Decisions

- the layer-order SSOT remains `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- no dedicated deeper design SSOT is required before this phase
- `phase208x` is docs/facts only; no code changes in this cut
- the actual `agg_local scalarization` layer is the next follow-on after this docs/facts phase
- the generic layer should fold the landed pilots instead of adding new family rows

## Out of Scope

- any code widening
- semantic changes to string / sum / user-box / array / map
- DCE / simplification-bundle changes

## Acceptance

- docs-only
- current pointers were updated during this docs/facts cut, and the next step is the actual `agg_local scalarization` layer
- `git diff --check`
