Status: Done
Date: 2026-06-17
Scope: fresh compiler owner selection after user-box direct-call coverage cleanup
Previous:
  - docs/development/current/main/phases/phase-296x/296x-1069-USER-BOX-METHOD-THIN-ENTRY-COVERAGE-INVENTORY-001.md

# FRESH-COMPILER-OWNER-SELECTION-007

## Purpose

Re-evaluate the selected `local_fastpath_fact_producer_gap` owner after
`fastpath_gap_inventory` learned to distinguish raw missing
`LocalFastPathFact` rows from user-box method routes covered by thin-entry
selection.

## Evidence

Focused active target:

```bash
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body
```

Result:

```text
known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
known_receiver_direct_method_thin_entry_covered_count=19
known_receiver_direct_method_uncovered_count=0
thin_entry_method_candidate_count=19
top_gap_count=0
```

Whole artifact:

```bash
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --front object_lifecycle_body
```
Result:

```text
function_count=255
known_receiver_direct_method_route_count=184
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=184
known_receiver_direct_method_thin_entry_covered_count=184
known_receiver_direct_method_uncovered_count=0
thin_entry_method_candidate_count=205
top_gap_count=0
```

## Decision

The previous selected owner is closed:

```text
selected_owner=local_fastpath_fact_producer_gap
selected_owner_status=closed_by_thin_entry_coverage_inventory
```

There is no remaining user-box method direct-call metadata producer gap in the
current MIR artifact.

Do not implement a user-box `LocalFastPathFact` producer for these call-route
sites. The direct-call route is already represented by:

```text
user_box_method_routes
thin_entry_selections(surface=user_box_method, manifest_row=user_box_method.known_receiver)
```

## Next Step

The next owner cannot be selected from this metadata gap report. Return to a
fresh exact-AOT perf sweep:

```text
next_task=FRESH-EXACT-AOT-PERF-SWEEP-AFTER-FASTPATH-GAP-CLOSEOUT-001
```

That sweep should use measured active fronts to select the next owner, rather
than continuing the user-box method fact-producer lane.

## Contract

```text
output_contract=fresh-compiler-owner-selection-v7

previous_selected_owner=local_fastpath_fact_producer_gap
previous_selected_owner_closed=1

known_receiver_direct_method_route_count=19
known_receiver_direct_method_without_fact_count=19
known_receiver_direct_method_thin_entry_covered_count=19
known_receiver_direct_method_uncovered_count=0

whole_known_receiver_direct_method_route_count=184
whole_known_receiver_direct_method_without_fact_count=184
whole_known_receiver_direct_method_thin_entry_covered_count=184
whole_known_receiver_direct_method_uncovered_count=0

fresh_compiler_optimization_owner_selected=0
selected_perf_owner=none
implementation_started=0
backend_lowering_changed=0
route_priority_changed=0

next_task=FRESH-EXACT-AOT-PERF-SWEEP-AFTER-FASTPATH-GAP-CLOSEOUT-001
summary=ok
```

## Stop Lines

```text
do not reopen user-box LocalFastPathFact producer from raw without_fact_count
do not treat thin-entry coverage as LocalFastPathFact
do not claim a performance win from report cleanup
do not select the next compiler owner without fresh perf evidence
```

## Validation

```bash
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body

python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --front object_lifecycle_body
```
