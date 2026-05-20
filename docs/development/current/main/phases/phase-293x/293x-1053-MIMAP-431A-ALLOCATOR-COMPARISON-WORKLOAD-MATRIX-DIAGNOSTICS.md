# 293x-1053 MIMAP-431A Allocator Comparison Workload Matrix Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Add diagnostics for missing allocator comparison workload matrix inputs after
MIMAP-430A. This row should observe missing small allocation, small free,
realloc, huge allocation, throughput, memory-usage, and invalid workload-count
families.

## Scope

- Consume the MIMAP-430A workload matrix inventory report.
- Summarize missing workload matrix inputs.
- Keep benchmark execution and process replacement closed.

## Stop Lines

- No benchmark execution.
- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Daily validation is L2:

```text
VM proof
MIR JSON emit
route preflight
```

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the allocator comparison workload matrix diagnostic owner.
- Added the proof app, design SSOT, guard, manifest row, and module export.
- Classified missing small allocation, small free, realloc, huge allocation,
  throughput, memory-usage, invalid workload-family count, and closed seam
  leaks.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
