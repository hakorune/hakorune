---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M100A segment allocation modeled ledger released-token recycle guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
  - tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
  - lang/src/hako_alloc/memory/README.md
  - lang/src/hako_alloc/hako_module.toml
---

# 295x-137 M100A Segment Allocation Modeled Ledger Released-Token Recycle Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M100A segment allocation modeled ledger released-token recycle
guard root. The batch keeps the same validation semantics, moves the real
shell body into `tools/checks/impl/`, and keeps the memory owner note in sync.

Selected root:

- `k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the M100A owner notes visible in `lang/src/hako_alloc/memory/README.md`.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M100A released-token recycle guard root is now easier to scan at the root
level, and the memory README explicitly documents the owner note that the guard
expects.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
