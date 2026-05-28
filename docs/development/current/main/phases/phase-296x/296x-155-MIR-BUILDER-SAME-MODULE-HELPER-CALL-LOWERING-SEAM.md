---
Status: Landed
Date: 2026-05-28
Scope: lower the remaining same-module helper setter calls without reopening the nested-call wrapper path.
Blocker: MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-154-POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH.md
  - tools/allocator/hako_mimalloc_small_alloc_helper_copy_family_probe.py
---

# 296x-155 MIR Builder Same-Module Helper Call Lowering Seam

## Purpose

Reduce the remaining `helper_result_local_ssa` and receiver-copy pressure by
lowering the simple same-module result-setter helpers more directly while
keeping nested helper wrappers as calls.

## Required Output

```text
output_contract=same-module-helper-call-lowering-seam-v0
input_contract=post-page-acquire-usize-source-mir-refresh-v0
selected_owner
selected_next
summary=ok
```

## Acceptance Notes

```text
The candidate set is narrow:
  - `recordAttempt`
  - `recordSelectedPage`
  - `recordBlock`
  - `recordLastAllocPage`

Keep these as calls unless the lowering proof can show the body is a simple
setter with no nested call / field-access chain.

Keep nested wrappers out of the new inline path:
  - `resetSmallAllocResult`
  - `recordSmallAllocFailure`
  - `recordSmallAllocSuccess`

Do not reopen source-side wrapper inlining as a workaround.
```

## Evidence

```text
output_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0
input_contract=small-alloc-call-copy-shape-deep-dive-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
helper_call_count=5
helper_copy_count=16
receiver_copy_count=13
arg_copy_count=0
result_copy_count=3
local_ssa_copy_count=85
dominant_copy_family=helper_result_local_ssa
dominant_callee_family=page_hotpath_helpers
selected_next=same_module_helper_call_lowering_seam
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The remaining same-module setter wrappers in objectLifecycleSmallAlloc are
down to the alloc_result / page-hotpath helper surface. The nested wrapper
path itself is no longer the dominant cost; the next seam is the remaining
result-setter and return-block materialization work.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_builder_same_module_helper_call_lowering_seam_guard.sh
```
