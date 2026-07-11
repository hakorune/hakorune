# 208x-91 Task Board

Status: Closed

## Docs / Facts

- [x] inventory the sum placement pilot surface
  - `sum_placement_facts`
  - `sum_placement_selections`
  - `sum_placement_layouts`
- [x] inventory the thin-entry pilot surface
  - `thin_entry_candidates`
  - `thin_entry_selections`
- [x] inventory the user-box local-body pilot surface
  - `field_decls + thin_entry_selections.inline_scalar`
- [x] inventory the ArrayBox typed-slot pilot
  - narrow typed-slot storage lane
- [x] inventory the tuple multi-payload compat transport
  - hidden payload boxes only
- [x] write the owner boundary notes for what belongs in agg_local scalarization vs domain-specific pilots
- [x] sync current pointers to `phase208x`
- [x] record the follow-on handoff to the actual `agg_local scalarization` layer

## Non-Goals

- no code changes
- no DCE widening
- no string / sum semantic widening
- no actual `agg_local scalarization` implementation yet

## Exit

- docs-only inventory is complete
- `phase208x` is landed and the actual `agg_local scalarization` layer is next
