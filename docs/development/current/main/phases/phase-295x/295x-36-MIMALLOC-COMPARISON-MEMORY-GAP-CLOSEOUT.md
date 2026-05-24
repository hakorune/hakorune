---
Status: Landed
Date: 2026-05-25
Scope: close RSS gap attribution pack.
Blocker: MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-35-MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL.md
  - tools/checks/k2_wide_phase295x_memory_gap_closeout_guard.sh
---

# 295x-36 Mimalloc Comparison Memory Gap Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT-295X-001
```

The attribution pack shows that the observed `.hako` vs C mimalloc RSS gap is
not primarily workload-incremental for the selected small/realloc/mixed rows.
The empty baseline already carries a large fixed process/runtime delta, while
the workload-incremental deltas are presentation evidence only and do not open
winner claims.

Phase-295x must therefore treat raw total RSS deltas as mixed evidence:

```text
total_delta = fixed_process_runtime_baseline_delta + workload_incremental_delta
```

Allocator-facing comparison should not claim memory wins or losses until the
fixed `.hako` exact-EXE baseline is understood.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-HAKO-BASELINE-BREAKDOWN-SELECTION-295X-001
```

The next row should choose a narrow baseline breakdown seam for the `.hako`
exact-EXE path. Candidate breakdowns include:

```text
empty app exact-EXE runtime footprint
linked runtime/static data footprint
route/preflight/build-time artifact effects
measurement collector differences
```

The row should select one diagnostic seam, not start broad optimization.

## Stop Line

This row does not reduce baseline RSS, change linker/runtime behavior, compute
performance or memory winners, require RSS parity, enable provider/DLL or host
replacement seams, install hooks, or open worker/TLS, atomics, remote-free
stress, abandoned heap stress, OSVM page-source parity, or native allocator
replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_memory_gap_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
