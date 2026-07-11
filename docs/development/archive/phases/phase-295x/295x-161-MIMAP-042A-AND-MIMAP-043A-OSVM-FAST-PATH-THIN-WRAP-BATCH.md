---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-042A and MIMAP-043A OSVM fast-path guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh
  - tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh
---

# 295x-161 MIMAP-042A and MIMAP-043A OSVM Fast-Path Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-042A OSVM-backed fast-path bounded purge guard root and
the MIMAP-043A OSVM-backed fast-path recommit/reuse guard root. The batch keeps
the validation semantics unchanged and moves both real shell bodies into
`tools/checks/impl/`.

Selected roots:

- `k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh`
- `k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh`

## Cleanup

- Keep the root scripts as thin wrappers that exec their impl bodies.
- Keep the OSVM fast-path purge/reuse routes unchanged.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The OSVM fast-path guard roots are easier to scan at the root level.

## Stop Line

This batch does not open real OSVM/page-source execution, unreserve,
recommit/provider activation, process replacement, hooks, worker/TLS, or
winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
