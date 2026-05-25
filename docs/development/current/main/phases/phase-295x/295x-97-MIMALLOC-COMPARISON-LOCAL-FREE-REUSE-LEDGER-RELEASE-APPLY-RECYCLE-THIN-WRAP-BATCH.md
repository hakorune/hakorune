---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the remaining hako_alloc local-free reuse ledger release-apply / release-applied-recycle guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_applied_recycle_guard.sh
---

# 295x-97 Mimalloc Comparison Local-Free Reuse Ledger Release-Apply Recycle Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-RECYCLE-THIN-WRAP-BATCH-295X-001
```

## Decision

Thin-wrap the remaining hako_alloc local-free reuse ledger release-apply and
release-applied-recycle guard roots first. The selected batch keeps the same
guard paths and same check semantics, but moves the real shell bodies into
`tools/checks/impl/` so the root entrypoints stay stable and small.

Selected roots:

- `k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh`
- `k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_applied_recycle_guard.sh`

## Cleanup

- Keep each selected root as a thin wrapper that execs its impl body.
- Preserve all check semantics, artifact paths, and stop lines.
- Leave the current mimalloc comparison blocker unchanged.
- Keep the allocator-family inventories aligned with the root guard names.

## Result

The selected thick guards are now easier to scan at the root level, while the
real validation logic lives under `tools/checks/impl/`.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_applied_recycle_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
