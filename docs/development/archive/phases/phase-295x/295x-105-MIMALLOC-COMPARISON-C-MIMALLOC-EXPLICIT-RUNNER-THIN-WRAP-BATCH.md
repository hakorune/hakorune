---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-451A explicit C mimalloc runner execution pilot guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker.
Related:
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
  - tools/checks/impl/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
  - docs/development/current/main/phases/phase-293x/293x-1073-MIMAP-451A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT.md
---

# 295x-105 Mimalloc C Explicit Runner Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-451A explicit C mimalloc runner execution pilot guard.
The batch keeps the same validation semantics, but moves the real shell body
into `tools/checks/impl/` so the root is a thin wrapper.

Selected root:

- `k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the explicit runner pilot on the current MIMAP-451A proof app and taskboard contract.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-451A explicit C mimalloc runner execution pilot guard is now easier
to scan at the root level and its implementation lives under `tools/checks/impl/`.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
