---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M212A, M220A, and M216A segment-map local-free reuse ledger lifecycle-token guard roots into impl-backed wrappers and keep the memory README owner notes in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh
  - lang/src/hako_alloc/memory/README.md
  - lang/src/hako_alloc/hako_module.toml
---

# 295x-140 M212A M220A M216A Segment-Map Local-Free Reuse Ledger Lifecycle-Token Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M212A, M220A, and M216A segment-map local-free reuse ledger
lifecycle-token guard roots. The batch keeps the same validation semantics,
moves the real shell bodies into `tools/checks/impl/`, and keeps the memory
owner notes in sync.

Selected roots:

- `k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh`
- `k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh`
- `k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh`

## Cleanup

- Keep the root scripts as thin wrappers that exec their impl bodies.
- Keep the M212A/M220A/M216A owner notes visible in `lang/src/hako_alloc/memory/README.md`.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The three lifecycle-token release guards are now easier to scan at the root
level, and the memory README explicitly documents the owner notes that the
guards expect.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
