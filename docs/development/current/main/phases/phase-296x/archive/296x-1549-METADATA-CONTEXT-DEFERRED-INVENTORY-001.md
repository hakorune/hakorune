# 296x-1549 METADATA-CONTEXT-DEFERRED-INVENTORY-001

Status: landed
Date: 2026-06-22

## Purpose

Keep `hakorune_mir_builder::metadata_context` deferred as a consultation-only
row.

TypeContext has now been recorded as the current bounded map slice consultation
candidate. MetadataContext remains broader and is still parked because it
mixes generics, optional source hints, region-stack tracking, and diagnostic
origin tables.

## Scope

```text
BoxCount: one consultation inventory
owner: MirBuilder MetadataContext deferred decision
input: current MetadataContext source shape
output: one durable deferred inventory and guard
```

## Observed Source Shape

```text
MetadataContext<SpanT, RegionIdT>
current_span
source_file
hint_sink
current_region_stack
value_origin_spans
value_origin_callers

new
current_span
set_current_span
set_source_file
clear_source_file
current_source_file
hint_scope_enter
hint_scope_leave
hint_join_result
push_region
pop_region
current_region_stack
record_value_span
value_span
record_value_caller
value_caller
value_origin_callers
```

## Deferred Reason

```text
generics
Option<String>
Vec region stack
HashMap origin tables
diagnostic caller provenance
source-file cloning
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_metadata_context_deferred_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_metadata_context_deferred_guard.sh
```

## Acceptance

```text
the deferred decision is fixed in one machine-readable fixture
route selection remains unopened
nightly rustc adapter remains unopened
MetadataContext is explicitly parked until a smaller candidate lands
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
