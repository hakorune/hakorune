---
Status: Current
Date: 2026-05-27
Scope: inventory missing `.hako` mimalloc port features after the small-model optimization checkpoint.
Blocker: HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-65-HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH.md
---

# 296x-66 Hako Mimalloc Port Feature Gap Inventory

## Purpose

Inventory missing `.hako` mimalloc features separately from the completed
small-model hot-path optimization checkpoint.

## Required Input

```text
output_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0
optimization_checkpoint=small_model_fast_path_plateau
next_diagnostic=port_feature_gap_inventory
next_optimization_allowed=0
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
small_model_checkpoint_elapsed_median_ms=240
missing_feature_count
next_port_feature
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
