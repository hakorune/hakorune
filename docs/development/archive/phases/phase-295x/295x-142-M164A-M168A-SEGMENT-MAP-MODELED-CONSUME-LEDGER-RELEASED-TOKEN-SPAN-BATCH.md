---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M164A and M168A segment-map modeled consume-ledger released-token recycle and released-span observation guard roots into impl-backed wrappers and keep the memory README owner notes in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh
  - lang/src/hako_alloc/memory/README.md
  - lang/src/hako_alloc/hako_module.toml
---

# 295x-142 M164A M168A Segment-Map Modeled Consume-Ledger Released-Token Span Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M164A and M168A segment-map modeled consume-ledger released-token recycle and released-span observation guard roots. The batch keeps the same validation semantics, moves the real shell bodies into `tools/checks/impl/`, and keeps the memory owner notes in sync.

Selected roots:

- `k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh`
- `k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh`

## Cleanup

- Keep the root scripts as thin wrappers that exec their impl bodies.
- Keep the M164A/M168A owner notes visible in `lang/src/hako_alloc/memory/README.md`.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The two consume-ledger release route guards are now easier to scan at the root
level, and the memory README explicitly documents the owner notes that the
guards expect.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
