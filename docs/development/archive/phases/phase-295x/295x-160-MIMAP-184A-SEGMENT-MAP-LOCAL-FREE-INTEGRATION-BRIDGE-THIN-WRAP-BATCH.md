---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-184A segment-map local-free integration bridge guard root into an impl-backed wrapper.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh
---

# 295x-160 MIMAP-184A Segment-Map Local-Free Integration Bridge Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-184A segment-map local-free integration bridge guard root.
The batch keeps the validation semantics unchanged and moves the real shell
body into `tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the integration bridge route semantics unchanged.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The segment-map local-free integration bridge guard root is easier to scan at
the root level.

## Stop Line

This batch does not open real free-list mutation, raw pointer residence,
segment-map execution, atomics, OSVM/page-source, worker/TLS, provider
activation, host allocator replacement, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
