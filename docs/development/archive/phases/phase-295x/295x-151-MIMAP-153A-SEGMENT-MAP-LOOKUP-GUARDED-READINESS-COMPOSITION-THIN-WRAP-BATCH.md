---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-153A segment-map lookup guarded readiness composition guard root into an impl-backed wrapper.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh
---

# 295x-151 MIMAP-153A Segment Map Lookup Guarded Readiness Composition Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-153A segment-map lookup guarded readiness composition
guard root. The batch keeps the same validation semantics and moves the real
shell body into `tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-153A guarded readiness composition guard root is now easier to scan
at the root level.

## Stop Line

This batch does not open real execution, raw pointer lookup, atomics,
page-source/OS release seams, provider activation, hooks, `#[global_allocator]`,
worker/TLS wider substrate work, remote-free stress, abandoned heap stress, or
winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
