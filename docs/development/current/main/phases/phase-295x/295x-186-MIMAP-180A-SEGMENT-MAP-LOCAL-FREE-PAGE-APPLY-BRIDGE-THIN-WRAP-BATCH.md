---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-180A segment-map local-free page-apply bridge guard root into an impl-backed wrapper while keeping the memory README owner note in place.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh
---

# 295x-186 MIMAP-180A Segment-Map Local-Free Page-Apply Bridge Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-180A segment-map local-free page-apply bridge guard root. The validation semantics stay the same while the real shell body moves into `tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the memory README owner note for the page-apply bridge in place.
- Keep the root proof manifest textually aligned with the include-owned MIMAP-180A id without duplicating the manifest row.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-180A page-apply bridge is easier to scan at the root level.

## Stop Line

This batch does not open real free-list mutation, raw pointer residence, segment-map execution, atomics, OSVM/page-source, worker/TLS, provider activation, or global allocator seams.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
