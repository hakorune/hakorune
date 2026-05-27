---
Status: Landed
Date: 2026-05-28
Scope: refresh source/MIR observation after the accepted .hako reason bind keeper.
Blocker: HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-124-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-MEASUREMENT.md
---

# 296x-125 Hako Mimalloc Post Hako Reason Bind Source MIR Refresh

## Purpose

Row124 accepted the `.hako` reason-local bind keeper:

```text
after_hako_elapsed_median_ms=610
previous_checkpoint_hako_elapsed_median_ms=620
keeper_effect=accepted
```

Refresh source/MIR observation before selecting another keeper or MIR-builder
probe.

## Required Output

```text
output_contract=hako-mimalloc-post-hako-reason-bind-source-mir-refresh-v0
input_contract=hako-mimalloc-post-hako-reason-bind-measurement-v0
selected_owner
selected_next
selected_next_kind=box_count|box_shape|mir_diagnostic|measurement
winner_claim=0
summary=ok
```

## Stop Line

Do not apply another optimization in this row.

## Evidence

Report:

```text
output_contract=hako-alloc-facade-reason-duplicate-inventory-v0
input_contract=hako-mimalloc-post-hako-reason-bind-measurement-v0
method_0=objectLifecycleSmallAlloc
method_0_source_reason_call_count=5
method_0_mir_reason_call_count=5
method_0_duplicate_reason_call_count=0
method_0_unused_duplicate_reason_call_count=0
method_1=objectLifecycleRecordAlignmentRequest
method_1_source_reason_call_count=1
method_1_mir_reason_call_count=2
method_1_duplicate_reason_call_count=1
method_1_unused_duplicate_reason_call_count=1
method_2=objectLifecycleSmallAllocAligned
method_2_source_reason_call_count=1
method_2_mir_reason_call_count=2
method_2_duplicate_reason_call_count=1
method_2_unused_duplicate_reason_call_count=1
method_3=objectLifecycleReleaseDirectCachedPage
method_3_source_reason_call_count=1
method_3_mir_reason_call_count=2
method_3_duplicate_reason_call_count=1
method_3_unused_duplicate_reason_call_count=1
method_4=objectLifecycleReleaseBlock
method_4_source_reason_call_count=5
method_4_mir_reason_call_count=10
method_4_duplicate_reason_call_count=5
method_4_unused_duplicate_reason_call_count=5
method_5=objectLifecycleReallocGrowFromPage
method_5_source_reason_call_count=2
method_5_mir_reason_call_count=4
method_5_duplicate_reason_call_count=2
method_5_unused_duplicate_reason_call_count=2
method_6=objectLifecycleReallocShrink
method_6_source_reason_call_count=5
method_6_mir_reason_call_count=10
method_6_duplicate_reason_call_count=5
method_6_unused_duplicate_reason_call_count=5
method_7=objectLifecycleReallocGrow
method_7_source_reason_call_count=5
method_7_mir_reason_call_count=10
method_7_duplicate_reason_call_count=5
method_7_unused_duplicate_reason_call_count=5
method_count=8
total_source_reason_call_count=25
total_mir_reason_call_count=45
total_duplicate_reason_call_count=20
total_unused_duplicate_reason_call_count=20
failing_method_count=7
failing_methods=objectLifecycleRecordAlignmentRequest,objectLifecycleSmallAllocAligned,objectLifecycleReleaseDirectCachedPage,objectLifecycleReleaseBlock,objectLifecycleReallocGrowFromPage,objectLifecycleReallocShrink,objectLifecycleReallocGrow
selected_owner=mir_nested_argument_single_evaluation
selected_next=hako_alloc_facade_reason_duplicate_eval_guard
selected_next_kind=mir_diagnostic
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_hako_reason_bind_source_mir_refresh_guard.sh
```

## Decision

`objectLifecycleSmallAlloc/1` no longer duplicates reason calls after row123,
but the same nested-call evaluation shape remains in seven facade methods.
This is now a correctness lane, tracked by:

```text
docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
```

Next row: add the narrow hako-alloc facade reason duplicate-evaluation guard
before changing MIR builder code.
