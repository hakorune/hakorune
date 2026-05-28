---
Status: Current
Date: 2026-05-28
Scope: join Hako exact-EXE and C mimalloc object-lifecycle body timing evidence.
Blocker: OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-172-OBJECT-LIFECYCLE-EXACT-C-RUNNER-FIRST-PATTERN.md
  - docs/development/current/main/phases/phase-296x/296x-173-OBJECT-LIFECYCLE-HAKO-BODY-TIMING-FIRST-PATTERN.md
  - tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py
---

# 296x-174 Object Lifecycle Body Timing Pair Adapter

## Purpose

Join the exact C object-lifecycle report and the `.hako` exact-EXE
object-lifecycle report into one comparison surface. This row still does not
select an optimization keeper; it only closes the measurement contract gap that
blocked row170.

## Required Output

```text
output_contract=hako-mimalloc-object-lifecycle-body-timing-pair-v0
workload_id=representative-object-lifecycle-small-block-v0
body_elapsed_comparable=1
hako_body_timing_available=1
c_body_timing_available=1
hako_body_elapsed_ns=<positive integer>
c_body_elapsed_ns=<positive integer>
body_elapsed_ratio=<float>
next_diagnostic=object_lifecycle_body_timing_gap_taxonomy
next_optimization_allowed=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

The adapter validates both inputs share:

```text
operation_sequence_id=representative-object-lifecycle-small-block-v0-seq
free_order_id=even-odd-release-v0
in_process_operation_repeat=8192
allocation_count=524288
free_count=524288
requested_bytes=272416768
```

Interpretation:

```text
The next row can classify the Hako/C body-timing gap using a shared workload
identity. External elapsed remains secondary process/runtime evidence.
```

## Next

```text
row175:
  object_lifecycle_body_timing_gap_taxonomy

Goal:
  classify whether the body-timing gap points first at compiler lowering,
  allocator algorithm, runtime baseline, or measurement harness before reopening
  optimization.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_body_timing_pair_adapter_guard.sh
```
