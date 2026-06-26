---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001
---

# MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001

## Closeout

The semantic-closure report and guard now decompose the composite
`finalize_module.record_packed_layout_refresh` edge into ordered child owners.
`finalize_module.typed_object_plan_refresh` is exposed as the first leaf owner,
and the next selected card is the typed-object derived artifact slice.

## Summary

`finalize_module.record_packed_layout_refresh` is the first executable
materialization gap, but it is a composite owner. Do not materialize the full
composite as one large artifact. Decompose it into leaf child owners first,
then let the analyzer derive the first leaf owner that can be materialized.

## Authority

Semantic source:

```text
MirBuilder::finalize_module
src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
```

Composite boundary:

```text
RecordAndPackedLayoutRefresh
  -> refresh_module_record_layout_plans
  -> refresh_module_array_record_storage_plans
  -> refresh_module_array_record_autouse_eligibility_plans
  -> refresh_module_array_record_materialization_boundary_plans
  -> refresh_module_array_record_packed_autouse_pilot_plans
  -> refresh_module_source_packed_array_autouse_pilot_plans
  -> refresh_module_source_packed_array_direct_read_consumption_plans
  -> refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans
  -> refresh_module_hako_alloc_huge_page_packed_store_pilot_plans
```

The decomposition must keep the composite boundary explicit. The full edge may
not be turned into one artifact before the child owners are classified.

## Decomposition Rule

```text
AllowLeafArtifact:
  only after a child owner is classified as leaf

DenyCompositeNeedsDecomposition:
  while the analyzer still points at the composite owner

DenyMissingChildAuthority:
  when a child owner has no directability evidence yet
```

The first leaf owner must be derived from the analyzer, not hand-pinned in
task-order.

## Acceptance

```text
semantic closure report still derives finalize_module.record_packed_layout_refresh
child ownership is explicit and ordered
leaf/composite separation is preserved
no single Hako artifact is emitted for the full composite edge
no new ABI = 0
no new backend route = 0
runtime fallback = 0
source selfhost claim = 0
```

## Non-Claims

```text
full record/packed layout artifact landing = 0
full finalize_module = 0
new ABI = 0
new backend route = 0
runtime fallback = 0
source selfhost claim = 0
```

## Next

```text
analyzer-derived first leaf owner
```
