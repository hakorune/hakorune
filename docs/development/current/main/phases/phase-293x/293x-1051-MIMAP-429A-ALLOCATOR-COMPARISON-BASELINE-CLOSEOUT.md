# 293x-1051 MIMAP-429A Allocator Comparison Baseline Closeout

Status: landed
Date: 2026-05-21

## Purpose

Close out the allocator comparison baseline pack after MIMAP-427A and
MIMAP-428A. The closeout should prove the inventory and diagnostics rows
compose into a representative baseline readiness package before any benchmark
execution or host allocator replacement is opened.

## Scope

- Re-run the MIMAP-427A inventory guard.
- Re-run the MIMAP-428A diagnostics guard.
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

Closeout validation should include the L2 row guards. L3 remains optional and
must stay representative-only unless a later row explicitly opens benchmark
execution.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the comparison baseline closeout SSOT and guard.
- Re-ran the MIMAP-427A inventory L2 guard.
- Re-ran the MIMAP-428A diagnostics L2 guard.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
