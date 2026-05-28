---
Status: Landed
Date: 2026-05-28
Scope: expose body timing from the .hako exact-EXE object-lifecycle workload.
Blocker: OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-172-OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN.md
  - apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
  - tools/allocator/hako_exe_memory_runner.sh
---

# 296x-173 Object Lifecycle Hako Body Timing First Pattern

## Purpose

Expose a comparable `.hako` exact-EXE body timing field for the object-lifecycle
small-block workload. This uses the existing `env.now_ms()` source-level host
time seam around the allocator workload loop, not `TimeBox`, because `TimeBox`
does not currently compile through the selected exact-EXE pure-first route.

## Required Output

```text
hako_body_timing_available=1
body_timing_repeat_kind=workload-body-env-now-ms-v0
body_timing_scope=allocator-workload-body
body_timing_is_process_timing=0
body_elapsed_ns=<positive integer>
summary=ok
```

## Evidence

The exact-EXE memory runner now forwards the app body timing fields:

```text
output_contract=hako-exe-memory-evidence-v0
workload=representative-object-lifecycle-small-block-v0
operation_family=small-block
operation_sequence_id=representative-object-lifecycle-small-block-v0-seq
free_order_id=even-odd-release-v0
in_process_operation_repeat=8192
hako_body_timing_available=1
body_timing_repeat_kind=workload-body-env-now-ms-v0
body_timing_scope=allocator-workload-body
body_timing_is_process_timing=0
body_elapsed_ns=<positive integer>
allocation_count=524288
free_count=524288
requested_bytes=272416768
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
summary=ok
```

Interpretation:

```text
The .hako subject now has body timing available without changing allocator
semantics. The timing source is millisecond env time converted to ns, so it is
good enough for row-level hot-loop separation but not a final nanosecond
precision claim.
```

## Next

```text
row174:
  object_lifecycle_body_timing_pair_adapter

Goal:
  join the row172 C body timing report and row173 .hako body timing report into
  one comparison surface before selecting the next optimization owner.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_hako_body_timing_first_pattern_guard.sh
```
