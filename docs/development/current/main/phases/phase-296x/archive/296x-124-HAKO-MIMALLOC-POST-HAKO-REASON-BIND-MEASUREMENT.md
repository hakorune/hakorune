---
Status: Landed
Date: 2026-05-28
Scope: measure exact-EXE after the small-alloc .hako reason bind keeper.
Blocker: HAKO-MIMALLOC-POST-HAKO-REASON-BIND-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-123-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER.md
---

# 296x-124 Hako Mimalloc Post Hako Reason Bind Measurement

## Purpose

Row123 landed the `.hako` reason-local bind keeper and removed duplicate reason
calls from `objectLifecycleSmallAlloc/1`:

```text
reason_call_count=5
duplicate_reason_call_count=0
semantic_summary=ok
```

Run the exact-EXE measurement and compare against the last comparable checkpoint
before selecting another keeper or probe.

## Required Output

```text
output_contract=hako-mimalloc-post-hako-reason-bind-measurement-v0
input_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_hako_reason_bind
after_hako_elapsed_median_ms
previous_checkpoint_hako_elapsed_median_ms=620
keeper_effect=accepted|neutral|regressed
winner_claim=0
summary=ok
```

## Stop Line

Do not add another keeper in this row.

## Evidence

Report:

```text
output_contract=hako-mimalloc-post-hako-reason-bind-measurement-v0
input_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_hako_reason_bind
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=3
keeper=small_alloc_hako_reason_bind
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
sample_0_hako_external_elapsed_ms=610
sample_1_hako_external_elapsed_ms=620
sample_2_hako_external_elapsed_ms=600
after_hako_elapsed_median_ms=610
after_hako_elapsed_min_ms=600
after_hako_elapsed_max_ms=620
after_hako_external_rss_median_bytes=3641344
previous_checkpoint_hako_elapsed_median_ms=620
previous_checkpoint_source=296x-118-single-pred-phi-elision
keeper_effect=accepted
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_post_hako_reason_bind_measurement_guard.sh
```
