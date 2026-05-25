---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-161A release guard root and align the release closeout guard to the included proof and guard manifests that actually own the row definitions.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_closeout_guard.sh
---

# 295x-100 Mimalloc Comparison Segment-Map Consume-Ledger Release Guard Manifest-Alignment Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the remaining MIMAP-161A release guard root, and align the
MIMAP-162A release closeout guard to the included proof and guard manifests
that actually carry the row definitions.

Selected roots:

- `k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh`
- `k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_closeout_guard.sh`

## Cleanup

- Keep each selected root as a thin wrapper that execs its impl body.
- Point the release guard's proof-app checks at
  `tools/checks/manifests/proof_apps/hako_alloc_segment_map_release_lifecycle.toml`.
- Point the release closeout guard's proof and guard-row checks at the included
  proof and guard manifests rather than the aggregate root manifests.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The release guard and its closeout pack now read the actual included manifest
files that own MIMAP-161A / MIMAP-162A, while the root entrypoints stay thin.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
