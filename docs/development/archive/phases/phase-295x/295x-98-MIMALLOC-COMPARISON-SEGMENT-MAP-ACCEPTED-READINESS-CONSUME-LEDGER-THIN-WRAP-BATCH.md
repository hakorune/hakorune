---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the segment-map accepted-readiness consume-ledger guard root and align the consume-ledger closeout guard with the included proof / guard manifests.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_closeout_guard.sh
---

# 295x-98 Mimalloc Comparison Segment-Map Accepted-Readiness Consume-Ledger Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the remaining segment-map accepted-readiness consume-ledger guard
root first, then align the consume-ledger closeout guard to the included proof
and guard manifests. The selected batch keeps the same guard paths and same
check semantics, but moves the real shell bodies into `tools/checks/impl/` and
points the manifest assertions at the included SSOT files.

Selected roots:

- `k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh`
- `k2_wide_hako_alloc_segment_map_modeled_consume_ledger_closeout_guard.sh`

## Cleanup

- Keep each selected root as a thin wrapper that execs its impl body.
- Preserve all check semantics, artifact paths, and stop lines.
- Leave the current mimalloc comparison blocker unchanged.
- Keep the proof-app and guard-manifest assertions pointed at the included
  manifest SSOT files.

## Result

The selected thick guards are now easier to scan at the root level, while the
real validation logic lives under `tools/checks/impl/`.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
