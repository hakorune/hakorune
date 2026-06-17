# 296x-1042 FASTPATH-GAP-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: fastpath gap inventory / known receiver direct route visibility

## Contract

```text
output_contract=hako-fastpath-gap-inventory-v0
row_kind=report_only
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

known_receiver_direct_method_routes_visible=1
local_fastpath_fact_gap_visible=1
thin_entry_method_candidate_count_visible=1

fallback_evidence_fact_enabled=0
backend_lowering_changed=0
route_priority_changed=0
winner_claim_allowed=0

next_task=FRESH-COMPILER-OWNER-SELECTION-006
summary=ok
```

## Purpose

After `FASTPATH-VOCAB-SLIM-CLOSEOUT-001`, the next question is not whether a
new backend fastpath can be guessed from source shape. The next question is:

```text
Which already-known direct route has not become a positive LocalFastPathFact?
```

This row adds a read-only inventory tool for that gap. It reports
`user_box.method` routes that are already same-module direct calls, then counts
which sites lack a matching `metadata.local_fastpath_facts` entry.

## Observation

For the current mimalloc object-lifecycle front, generated MIR already shows
known receiver method routes such as:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1:
  user_box_method_routes are present
  local_fastpath_facts are not present

Main.runOne/2:
  direct user-box method routes are present
  local_fastpath_facts are not present
```

This means the next owner is an eligibility/producer gap, not a backend
consumer reachability gap.

## Stop Line

```text
do not emit backend code from this inventory
do not treat direct user_box_method_routes as LocalFastPathFact
do not create fallback facts
do not change route priority
do not claim winner from gap count
```

## Verification

```bash
python3 -m unittest tools.hako_check.tests.test_fastpath_gap_inventory
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body
bash tools/checks/k2_wide_phase296x_fastpath_gap_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
