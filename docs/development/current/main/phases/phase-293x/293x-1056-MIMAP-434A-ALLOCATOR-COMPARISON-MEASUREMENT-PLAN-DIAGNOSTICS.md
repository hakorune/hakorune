# 293x-1056 MIMAP-434A Allocator Comparison Measurement Plan Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Consume the MIMAP-433A allocator comparison measurement plan inventory report
and publish observer-only diagnostics for missing or invalid measurement plan
inputs before benchmark execution opens.

## Scope

- Classify missing run-count, warmup, output-contract, throughput, and
  memory-usage measurement plan inputs.
- Classify invalid run-count and warmup-count inputs.
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

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the measurement plan diagnostics owner and report.
- Added a scalar proof app for missing/invalid measurement input diagnostics.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
