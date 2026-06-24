---
Status: Landed
Date: 2026-06-16
Task: MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001
Scope: Select the next owner after the local known-receiver direct-call lane
  closed.
Related:
  - docs/development/current/main/phases/phase-296x/296x-822-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001.md
  - tools/allocator/hako_mimalloc_next_owner_after_local_known_receiver_closeout.py
---

# MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001

## Purpose

The local known-receiver direct-call lane is closed with the current
object-lifecycle body no longer Hako-slower:

```text
body_elapsed_ratio=0.836
lane_closed=1
new_speedup_claim=0
```

This row selects the next owner.  It must not invent another implementation row
for the same front without fresh evidence.

## Result

```text
output_contract=hako-mimalloc-next-owner-after-local-known-receiver-closeout-v0
source_evidence=296x-822,296x-821
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
local_known_receiver_direct_call_lane_closed=1
current_body_elapsed_ratio=0.836
hako_slower_current_front=0
current_front_winner_from_previous=1
selected_next_owner=none_current_front_not_hako_slower
selected_owner_confidence=high
implementation_started=0
new_backend_lowering_code_added=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001
summary=ok
```

## Interpretation

There is no current-front Hako-slower body gap to assign to a new implementation
owner.  The correct next step is a checkpoint/pause for this front, or a fresh
front selection if optimization continues.

## Stop Line

```text
do not continue patching local known-receiver direct calls
do not reopen Array.length in this front
do not open storage direct lowering without a new front and owner
do not bypass HostHandle
do not retire Arc
do not change product default runtime behavior
do not select implementation owner while hako_slower_current_front=0
```

## Proof

```bash
python3 -m py_compile tools/allocator/hako_mimalloc_next_owner_after_local_known_receiver_closeout.py
bash tools/checks/k2_wide_phase296x_next_owner_after_local_known_receiver_closeout_guard.sh
```
