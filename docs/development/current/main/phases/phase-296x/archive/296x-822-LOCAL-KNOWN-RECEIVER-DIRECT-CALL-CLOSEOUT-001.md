---
Status: Landed
Date: 2026-06-16
Task: LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001
Scope: Close the local known-receiver direct-call lane after measurement.
Related:
  - docs/development/current/main/phases/phase-296x/296x-821-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001.md
  - tools/allocator/hako_local_known_receiver_direct_call_closeout.py
---

# LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001

## Purpose

`LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001` measured the current
direct-exact object-lifecycle body with `in_process_repeat=65536` and found the
Hako body no longer slower than the C body.

This closeout fixes the interpretation:

```text
The lane is closed because the current front is no longer Hako-slower.
The lane did not add new backend lowering code.
The result must not be reinterpreted as a new implementation speedup.
```

## Result

```text
output_contract=hako-local-known-receiver-direct-call-closeout-v0
source_evidence=296x-821,296x-820,296x-819
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
closed_lane=local_known_receiver_direct_call
lane_closed=1
closeout_reason=current_front_no_longer_hako_slower_and_no_new_lowering_needed
hako_body_elapsed_ns=24000000
c_body_elapsed_ns=28710275
body_elapsed_ratio=0.836
winner_claim=1
winner_claim_source=current_front_measurement
new_speedup_claim=0
new_backend_lowering_code_added=0
page_specific_rule_enabled=0
method_name_special_case_enabled=0
helper_symbol_inference_enabled=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
next_owner_selection_required=1
selected_next=MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001
summary=ok
```

## Interpretation

The current front reached a good body-timing outcome after the previous
RoutePlan/object-boundary rows.  The local known-receiver direct-call row did
not introduce a new page-specific or method-specific lowering path; it classified
the existing generic `user_box_method_routes` C shim consumer as the relevant
RoutePlan backend seam.

The next row must therefore return to owner selection instead of extending this
lane.

## Stop Line

```text
do not add lowering after closeout
do not attribute the measurement to a new code change
do not reopen page/method/helper-specific branches
do not open storage direct lowering from this lane
do not bypass HostHandle from this lane
do not retire Arc from this lane
do not change product default runtime behavior
```

## Proof

```bash
python3 -m py_compile tools/allocator/hako_local_known_receiver_direct_call_closeout.py
bash tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_closeout_guard.sh
```
