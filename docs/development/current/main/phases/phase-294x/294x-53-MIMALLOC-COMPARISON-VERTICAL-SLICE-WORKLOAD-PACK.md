---
Status: Landed
Date: 2026-05-23
Scope: V0 workload-pack selection for the mimalloc `.hako` comparison vertical slice.
Related:
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/phases/phase-293x/293x-1052-MIMAP-430A-ALLOCATOR-COMPARISON-WORKLOAD-MATRIX-INVENTORY.md
  - lang/src/hako_alloc/memory/allocator_comparison_workload_matrix_inventory_box.hako
  - tools/allocator/c_mimalloc_explicit_runner.c
  - tools/checks/k2_wide_mimalloc_comparison_vertical_slice_workload_pack_guard.sh
---

# 294x-53 Mimalloc Comparison Vertical Slice Workload Pack

## Decision

Select the first comparison-quality workload pack for the mimalloc `.hako`
port acceleration path.

This row does not attempt a full native mimalloc-compatible allocator. It
selects the minimum workload family needed to compare `.hako` / `hako_alloc`
evidence against the existing explicit C mimalloc runner lane.

## Selected Workload Pack

The first pack is:

```text
small_fixed_alloc_free_reuse:
  fixed-size small allocation, free, and same-page reuse

mixed_small_sizes:
  multiple small requested sizes through size-class and page-queue selection

realloc_same_class_and_grow:
  same-class no-move plus alloc-copy-release fallback

aligned_small:
  aligned small allocation through the normal page-map-backed path

huge_osvm_backed:
  huge threshold route plus OSVM-backed huge/page-source evidence
```

The selected pack maps onto the already-landed MIMAP-430A workload matrix
families:

```text
small allocation
small free
realloc
huge allocation
throughput
memory usage
```

## Output Schema Anchor

The comparison closeout should preserve these field families:

```text
allocator_id
runner_kind
workload_id
requested_bytes
live_bytes_or_handles
operation_count
failure_reason
rss_or_memory_evidence
```

The first `.hako` rows may expose scalar/report fields rather than a final JSON
schema, but the names above are the stable vocabulary for the V0-V5 vertical
slice.

## Stop Line

This row does not:

- run new benchmarks;
- replace the process allocator;
- install hooks;
- install `#[global_allocator]`;
- activate allocator providers;
- add backend owner-name matchers;
- open worker/TLS, abandoned heap, atomic bitmap, or true remote-free stress;
- require migration of every remaining `i64` allocator field.

The next row should close only the `usize` field/path gaps that this workload
pack actually consumes.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_comparison_vertical_slice_workload_pack_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
