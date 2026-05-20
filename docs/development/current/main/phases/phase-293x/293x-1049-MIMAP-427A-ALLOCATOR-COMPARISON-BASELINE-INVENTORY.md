# 293x-1049 MIMAP-427A Allocator Comparison Baseline Inventory

Status: landed
Date: 2026-05-21

## Purpose

Inventory the comparison baseline needed to judge `.hako` / `hako_alloc`
against C mimalloc. This row should define the measurement inputs without
changing allocator behavior or replacing the process allocator.

## Scope

- Name throughput and memory-usage baseline inputs.
- Keep process replacement parked.
- Keep optional replacement execution parked.
- Keep the comparison target explicit: C mimalloc performance and memory
  usage.

## Stop Lines

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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the allocator comparison baseline inventory owner.
- Added the proof app, design SSOT, guard, manifest row, and module export.
- Inventoried C mimalloc baseline, hako_alloc baseline, throughput target,
  memory-usage target, workload matrix, and repeat count inputs.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
