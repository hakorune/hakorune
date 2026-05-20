# 293x-1052 MIMAP-430A Allocator Comparison Workload Matrix Inventory

Status: landed
Date: 2026-05-21

## Purpose

Inventory the comparison workload matrix after the allocator comparison
baseline pack is closed out. This row should name the workload families needed
before `.hako` / `hako_alloc` can be compared against C mimalloc for throughput
and memory usage.

## Scope

- Add an explicit workload-matrix owner or planning row.
- Track throughput and memory-usage comparison workload families.
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the allocator comparison workload matrix inventory owner.
- Added the proof app, design SSOT, guard, manifest row, and module export.
- Inventoried small allocation, small free, realloc, huge allocation,
  throughput, memory-usage, and workload-family count inputs.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
