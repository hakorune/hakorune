---
Status: Landed
Date: 2026-05-25
Scope: add empty-baseline evidence for the external `malloc-large` attribution family.
Blocker: MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-190-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION.md
  - docs/development/current/main/phases/phase-295x/295x-192-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL.md
  - docs/development/current/main/phases/phase-295x/295x-34-MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE.md
  - docs/development/current/main/phases/phase-295x/295x-35-MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL.md
  - tools/checks/k2_wide_phase295x_malloc_large_memory_gap_baseline_guard.sh
---

# 295x-191 Malloc-Large Memory Gap Baseline

## Decision

Close:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001
```

Add an explicit empty workload to the repeated measurement runner:

```text
workload=representative-empty-v0
operation_family=empty-baseline
operation_sequence_id=representative-empty-v0-seq
free_order_id=no-release-v0
```

This row measures the fixed `.hako` exact-EXE process/runtime RSS baseline and
the fixed C mimalloc explicit-runner baseline through the same repeated
measurement profile:

```text
measurement_profile=phase295x-repeated-v0
warmup_count=1
sample_count=5
canonical_rss_collector=external-time
winner_claim=0
```

The default repeated workload pack remains the four selected non-empty
workloads. `representative-empty-v0` is opt-in so the existing pack evidence
does not drift.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001
```

The follow-on should subtract the empty baseline medians from each workload
median and emit presentation-only incremental evidence:

```text
hako_incremental_rss = hako_workload_rss - hako_empty_baseline_rss
c_incremental_rss    = c_workload_rss    - c_empty_baseline_rss
incremental_delta    = hako_incremental_rss - c_incremental_rss
```

## Stop Line

This row does not compute incremental deltas, make memory/performance winner
claims, require RSS parity, enable provider/DLL/replacement seams, install
hooks, or open worker/TLS, atomics, remote-free stress, abandoned heap stress,
OSVM page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_memory_gap_baseline_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
