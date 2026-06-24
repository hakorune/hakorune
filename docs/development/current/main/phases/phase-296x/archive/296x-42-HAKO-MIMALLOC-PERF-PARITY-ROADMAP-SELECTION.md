---
Status: Landed
Date: 2026-05-27
Scope: select the `.hako` mimalloc performance-parity lane and keep hakozuna reference-only.
Blocker: HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/phases/phase-296x/296x-41-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT.md
---

# 296x-42 Hako Mimalloc Performance Parity Roadmap Selection

## Decision

Close:

```text
HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001
```

Select the `.hako` mimalloc performance-parity lane:

```text
.hako mimalloc parity
  -> make the `.hako` mimalloc port approach C mimalloc under identical
     workload contracts

hakozuna reference
  -> preserve hakozuna evidence as a comparison subject only

allocator product selection
  -> parked until a separate decision row opens it
```

This lane stays focused on parity evidence. It does not become a hakozuna
selection lane and it does not open host allocator replacement.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX-296X-001
```

The next row should define the workload matrix and the stable subject ids
before any additional measurements are compared.

## Stop Line

This row does not run benchmarks, activate providers, replace the process
allocator, install hooks, or claim a winner.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_roadmap_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
