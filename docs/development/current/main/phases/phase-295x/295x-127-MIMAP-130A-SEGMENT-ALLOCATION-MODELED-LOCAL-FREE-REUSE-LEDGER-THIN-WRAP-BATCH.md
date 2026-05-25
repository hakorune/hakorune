---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-130A local-free reuse ledger guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh
  - tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh
  - lang/src/hako_alloc/memory/README.md
---

# 295x-127 MIMAP-130A Local-Free Reuse Ledger Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-130A local-free reuse ledger guard root. The batch keeps
the same validation semantics, moves the real shell body into `tools/checks/impl/`,
and keeps the memory owner note in sync.

Selected root:

- `k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the MIMAP-130A local-free reuse ledger owner visible in
  `lang/src/hako_alloc/memory/README.md`.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-130A local-free reuse ledger guard is now easier to scan at the root
level, and the memory README explicitly documents the owner note that the
guard expects.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
