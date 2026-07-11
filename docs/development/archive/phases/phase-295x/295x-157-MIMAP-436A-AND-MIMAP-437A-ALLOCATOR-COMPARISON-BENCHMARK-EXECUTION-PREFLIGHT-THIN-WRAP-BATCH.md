---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-436A and MIMAP-437A allocator comparison benchmark execution preflight guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_closeout_guard.sh
---

# 295x-157 MIMAP-436A and MIMAP-437A Allocator Comparison Benchmark Execution Preflight Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-436A allocator comparison benchmark execution preflight
inventory guard root and the MIMAP-437A allocator comparison benchmark
execution preflight diagnostics guard root. The batch keeps the validation
semantics unchanged and moves both real shell bodies into `tools/checks/impl/`.

Selected roots:

- `k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh`
- `k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh`
- `k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_closeout_guard.sh`

## Cleanup

- Keep the inventory and diagnostics root scripts as thin wrappers that exec
  their impl bodies.
- Keep the closeout root script as a thin wrapper that chains the two
  preflight guards.
- Keep the benchmark execution preflight routes unchanged.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The allocator-comparison benchmark execution preflight guard roots are now
easier to scan at the root level.

## Stop Line

This batch does not open real benchmark execution, process replacement, hook
installation, backend matcher wiring, global allocator installation, or winner
claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
