---
Status: Landed
Date: 2026-06-16
Task: MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001
Scope: Pause the current object-lifecycle body-timing front after it stopped
  being Hako-slower.
Related:
  - docs/development/current/main/phases/phase-296x/296x-823-MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001.md
  - tools/allocator/hako_mimalloc_current_front_pause_checkpoint.py
---

# MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001

## Purpose

The current `objectLifecycleSmallAlloc/1` body-timing front no longer has a
Hako-slower body gap:

```text
body_elapsed_ratio=0.836
hako_slower_current_front=0
selected_next_owner=none_current_front_not_hako_slower
```

This checkpoint pauses optimization for this front.  Further work should either
select a fresh front or remeasure because the environment changed; it should not
continue patching the current front without new evidence.

## Result

```text
output_contract=hako-mimalloc-current-front-optimization-pause-checkpoint-v0
source_evidence=296x-823,296x-822,296x-821
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
body_elapsed_ratio=0.836
current_front_paused=1
pause_reason=current_front_not_hako_slower
local_known_receiver_direct_call_lane_closed=1
implementation_owner_selected=0
implementation_started=0
new_backend_lowering_code_added=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
fresh_front_selection_allowed=1
remeasure_if_environment_changes=1
no_current_front_patch_without_new_evidence=1
selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001
summary=ok
```

## Stop Line

```text
do not patch the current front without new Hako-slower evidence
do not infer a new owner from old helper symbols
do not reopen local known-receiver direct-call implementation
do not open storage direct lowering from this front
do not bypass HostHandle
do not retire Arc
do not change product default runtime behavior
```

## Next

If optimization continues, start with a fresh front selection row:

```text
MIMALLOC-FRESH-FRONT-SELECTION-001
```

That row must choose a Hako-slower front before implementation work resumes.

## Proof

```bash
python3 -m py_compile tools/allocator/hako_mimalloc_current_front_pause_checkpoint.py
bash tools/checks/k2_wide_phase296x_current_front_pause_checkpoint_guard.sh
```
