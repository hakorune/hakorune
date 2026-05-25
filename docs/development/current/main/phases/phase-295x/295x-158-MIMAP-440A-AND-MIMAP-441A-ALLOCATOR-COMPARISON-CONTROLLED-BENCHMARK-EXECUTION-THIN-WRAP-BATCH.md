---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-440A and MIMAP-441A allocator comparison controlled benchmark execution guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_inventory_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_diagnostics_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_closeout_guard.sh
---

# 295x-158 MIMAP-440A and MIMAP-441A Allocator Comparison Controlled Benchmark Execution Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-440A allocator comparison controlled benchmark execution
inventory guard root and the MIMAP-441A allocator comparison controlled
benchmark execution diagnostics guard root. The batch keeps the validation
semantics unchanged and moves both real shell bodies into `tools/checks/impl/`.

Selected roots:

- `k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_inventory_guard.sh`
- `k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_diagnostics_guard.sh`
- `k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_closeout_guard.sh`

## Cleanup

- Keep the inventory and diagnostics root scripts as thin wrappers that exec
  their impl bodies.
- Keep the closeout root script as a thin wrapper that chains the two
  controlled benchmark execution guards.
- Keep the controlled benchmark execution routes unchanged.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The allocator-comparison controlled benchmark execution guard roots are now
easier to scan at the root level.

## Stop Line

This batch does not open real benchmark execution, process replacement, hook
installation, backend matcher wiring, global allocator installation, or winner
claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_inventory_guard.sh
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
