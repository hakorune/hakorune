Status: Done
Date: 2026-06-17
Scope: user-box method fastpath gap report cleanup
Previous:
  - docs/development/current/main/phases/phase-296x/296x-1068-USER-BOX-METHOD-DIRECT-CALL-PUBLICATION-REQUIREMENT-DESIGN-001.md

# USER-BOX-METHOD-THIN-ENTRY-COVERAGE-INVENTORY-001

## Purpose

Update `fastpath_gap_inventory` so product-compatible user-box direct-call
routes covered by thin-entry selection are not reported as truly uncovered
just because they do not have `LocalFastPathFact` rows.

## Change

`tools/hako_check/fastpath_gap_inventory.py` now reports:

```text
known_receiver_direct_method_without_fact_count
known_receiver_direct_method_thin_entry_covered_count
known_receiver_direct_method_uncovered_count
```

The first field remains a raw count of route-positive sites without
`LocalFastPathFact`.

The second field counts the subset covered by:

```text
thin_entry_selections.surface=user_box_method
thin_entry_selections.manifest_row=user_box_method.known_receiver
thin_entry_selections.selected_entry=thin_internal_entry
thin_entry_selections.state=candidate
same block/instruction site
```

The third field is the real remaining gap:

```text
without_fact - thin_entry_covered
```

## Active Target Result

For the active object-lifecycle front:

```text
known_receiver_direct_method_route_count=19
known_receiver_direct_method_without_fact_count=19
known_receiver_direct_method_thin_entry_covered_count=19
known_receiver_direct_method_uncovered_count=0
thin_entry_method_candidate_count=19
top_gap_count=0
```

This means the active user-box method direct-call route surface is covered by
thin-entry selection. It is not a missing `LocalFastPathFact` producer gap.

## Contract

```text
output_contract=hako-fastpath-gap-inventory-v0

thin_entry_coverage_count_enabled=1
known_receiver_direct_method_thin_entry_covered_count=19
known_receiver_direct_method_uncovered_count=0

local_fastpath_fact_count=0
local_fastpath_fact_user_box_method_required=0
backend_lowering_changed=0
route_priority_changed=0
publication_classifier_changed=0

next_task=FRESH-COMPILER-OWNER-SELECTION-007
summary=ok
```

## Stop Lines

```text
do not treat thin-entry coverage as LocalFastPathFact
do not remove publication classifier
do not change backend lowering
do not change route priority
do not claim body-time win from report cleanup
```

## Validation

```bash
python3 -m unittest tools.hako_check.tests.test_fastpath_gap_inventory

python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body
```
