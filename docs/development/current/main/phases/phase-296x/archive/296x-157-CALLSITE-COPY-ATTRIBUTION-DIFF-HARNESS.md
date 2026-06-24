---
Status: Landed
Date: 2026-05-28
Scope: compare before/after callsite copy attribution reports before running exact-EXE.
Blocker: CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-156-OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION.md
  - tools/allocator/mir_callsite_copy_attribution.py
  - tools/allocator/mir_callsite_copy_attribution_diff.py
---

# 296x-157 Callsite Copy Attribution Diff Harness

## Purpose

Add a thin before/after diff adapter over row156 attribution reports. Candidate
patches should first prove that the intended MIR owner changed before spending
time on exact-EXE measurement.

This row does not optimize.

## Required Output

```text
output_contract=hako-mimalloc-callsite-copy-attribution-diff-v0
input_contract=hako-mimalloc-callsite-copy-attribution-v0
candidate_id
target_method
selected_delta_owner
structural_effect=improved|regressed|mixed|no_effect
exact_exe_required=0|1
summary=ok
```

## Smoke Evidence

The guard uses the same row156 attribution report as both before and after.
That fixes the no-op baseline behavior:

```text
output_contract=hako-mimalloc-callsite-copy-attribution-diff-v0
input_contract=hako-mimalloc-callsite-copy-attribution-v0
candidate_id=self_smoke
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
before_dominant_callee_family=page_hotpath_helpers
after_dominant_callee_family=page_hotpath_helpers
before_dominant_copy_owner=local_ssa_copy_materialization
after_dominant_copy_owner=local_ssa_copy_materialization
selected_delta_owner=local_ssa_copy_materialization
structural_effect=no_effect
exact_exe_required=0
delta_instruction_count=0
delta_call_count=0
delta_copy_count=0
delta_local_ssa_copy_count=0
delta_receiver_copy_count=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row158:
  use row156 attribution plus row157 diff output to select one owner:
    - local_ssa_copy_materialization
    - receiver_materialization
    - method_call_route_lowering
    - verified_helper_inline
    - runtime_baseline
    - measurement_harness
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_callsite_copy_attribution_diff_harness_guard.sh
```
