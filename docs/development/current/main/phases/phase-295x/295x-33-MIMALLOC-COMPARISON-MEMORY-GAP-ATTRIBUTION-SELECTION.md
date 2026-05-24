---
Status: Landed
Date: 2026-05-25
Scope: select RSS gap attribution plan.
Blocker: MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-32-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - tools/checks/k2_wide_phase295x_memory_gap_attribution_selection_guard.sh
---

# 295x-33 Mimalloc Comparison Memory Gap Attribution Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001
```

Select baseline attribution as the next comparison seam.

The repeated presentation shows small/realloc/mixed workloads with a largely
constant RSS median gap, while the huge-ish gap is smaller. Before any winner
claim, phase-295x should attribute the gap into:

```text
fixed_process_runtime_baseline
workload_incremental_rss
unattributed_residual
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE-295X-001
```

The row should add explicit empty/baseline evidence for `.hako` and C mimalloc
under `measurement_profile=phase295x-repeated-v0`, then compute:

```text
hako_incremental_rss = hako_workload_rss - hako_baseline_rss
c_incremental_rss    = c_workload_rss    - c_baseline_rss
incremental_delta    = hako_incremental_rss - c_incremental_rss
```

## Stop Line

This row does not implement baseline runners, compute winners, require RSS
parity, enable provider/DLL/replacement seams, install hooks, or open
worker/TLS, atomics, remote-free stress, abandoned heap stress, OSVM
page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_memory_gap_attribution_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
