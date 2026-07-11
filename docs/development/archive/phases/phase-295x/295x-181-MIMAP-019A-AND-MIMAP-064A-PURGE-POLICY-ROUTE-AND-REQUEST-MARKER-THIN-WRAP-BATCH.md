---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-019A purge-policy route guard root and the MIMAP-064A reclaim scheduler request-marker guard root into impl-backed wrappers while keeping the memory README owner notes in sync.
Related:
  - tools/checks/k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh
  - tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh
---

# 295x-181 MIMAP-019A and MIMAP-064A Purge-Policy Route and Request-Marker Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-019A purge-policy route guard root and the MIMAP-064A
reclaim scheduler request-marker guard root. The validation semantics stay the
same while the real shell bodies move into `tools/checks/impl/`.

Selected roots:

- `k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh`
- `k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh`

## Cleanup

- Keep both root scripts as thin wrappers that exec their impl bodies.
- Keep the purge-policy owner note and request-marker owner note in the memory README aligned with the route.
- Keep the root proof manifest textually aligned with the MIMAP-064A include-owned id without duplicating the manifest row.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-019A purge-policy route guard and the MIMAP-064A reclaim scheduler
request-marker guard are easier to scan at the root level.

## Stop Line

This batch does not open direct page-source, OSVM, provider activation, hook,
or allocator replacement seams.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
