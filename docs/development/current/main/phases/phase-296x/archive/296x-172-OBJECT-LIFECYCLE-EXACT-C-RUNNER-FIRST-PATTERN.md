---
Status: Landed
Date: 2026-05-28
Scope: add the missing C mimalloc explicit object-lifecycle exact pair.
Blocker: OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-171-OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT.md
  - tools/allocator/c_mimalloc_explicit_runner.c
---

# 296x-172 Object Lifecycle Exact C Runner First Pattern

## Purpose

Add the missing C mimalloc explicit comparison subject for the `.hako`
object-lifecycle small-block workload. This closes the first half of the row171
measurement contract without touching `.hako` source timing or reopening
optimization.

## Required Output

```text
workload=representative-object-lifecycle-small-block-v0
operation_sequence_id=representative-object-lifecycle-small-block-v0-seq
free_order_id=even-odd-release-v0
allocation_count=524288
free_count=524288
requested_bytes=272416768
c_body_timing_available=1
summary=ok
```

## Evidence

The C runner now accepts:

```bash
tools/allocator/c_mimalloc_explicit_runner.sh \
  --out /tmp/c.out \
  --allow-ldconfig-discovery \
  --workload representative-object-lifecycle-small-block-v0 \
  --in-process-repeat 8192 \
  --operation-repeat 1
```

The expected exact-pair output includes:

```text
output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0
workload=representative-object-lifecycle-small-block-v0
operation_family=small-block
operation_sequence_id=representative-object-lifecycle-small-block-v0-seq
free_order_id=even-odd-release-v0
in_process_operation_repeat=8192
allocation_count=524288
free_count=524288
requested_bytes=272416768
c_body_timing_available=1
hako_body_timing_available=0
body_timing_repeat_kind=workload-body-monotonic-v0
body_timing_is_process_timing=0
process_replacement_executed=0
hook_installed=0
global_allocator_installed=0
summary=ok
```

Interpretation:

```text
The C subject now matches the .hako object-lifecycle workload identity and
operation counters. The remaining measurement gap is .hako body timing; exact
optimization work remains closed until that second half exists.
```

## Next

```text
row173:
  object_lifecycle_hako_body_timing_first_pattern

Goal:
  expose a comparable body_elapsed_ns for the .hako exact-EXE
  object-lifecycle workload without changing allocator semantics.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_exact_c_runner_first_pattern_guard.sh
```
