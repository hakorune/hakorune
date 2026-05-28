---
Status: Current
Date: 2026-05-28
Scope: attribute objectLifecycleSmallAlloc copy pressure to callsites before another keeper row.
Blocker: OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-155-MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM.md
  - tools/allocator/mir_callsite_copy_attribution.py
---

# 296x-156 Object Lifecycle Small Hotpath Callsite Copy Attribution

## Purpose

Stop the source-level keeper loop and return to observation. The previous
rows reduced helper-setter calls, but the exact-EXE gap is still large and
source expansion produced non-keepers. This row attributes lowered MIR `copy`
instructions in `objectLifecycleSmallAlloc/1` to callsite receiver, argument,
result, local-SSA, phi-edge, and unknown owners.

This row does not optimize.

## Required Output

```text
output_contract=hako-mimalloc-callsite-copy-attribution-v0
input_contract=same-module-helper-call-lowering-seam-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
dominant_callee_family
dominant_copy_owner
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-callsite-copy-attribution-v0
input_contract=same-module-helper-call-lowering-seam-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
block_count=20
instruction_count=190
call_count=12
copy_count=98
phi_count=18
helper_call_count=6
helper_copy_count=25
receiver_copy_count=27
arg_copy_count=7
result_copy_count=9
local_ssa_copy_count=48
phi_edge_copy_count=10
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
callsite_0_callee=acquire_usize
callsite_0_callee_family=page_hotpath_helpers
callsite_0_receiver_copy_chain_len=2
callsite_0_arg_copy_count=1
callsite_0_result_copy_count=6
callsite_0_attributed_copy_count=9
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The current hotpath owner should not be selected from source shape alone.
The largest owner is local SSA copy materialization, while page-hotpath
helpers still dominate attributed helper callsites. The strongest single
callsite signal is acquire_usize, mostly through result movement.
```

## Next

```text
row157:
  per-method before/after MIR shape diff harness

row158:
  owner selection from callsite attribution and MIR shape diff evidence

row159:
  owner-specific optimization only after the owner is selected
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_small_hotpath_callsite_copy_attribution_guard.sh
```
