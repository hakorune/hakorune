---
Status: Landed
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

## Evidence

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
small_model_checkpoint_elapsed_median_ms=240
small_model_remaining_gap_ms=236
optimization_checkpoint=small_model_fast_path_plateau
implemented_surface_count=12
missing_feature_count=7
primary_gap_kind=integration_surface_gap
next_port_feature=real_provider_explicit_entrypoint_selection
next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001
ld_preload_shim_ready=0
provider_entrypoint_selection_ready=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Implemented Surface Inventory

```text
size_class_policy
page_model
page_queue
legacy_page_heap
production_facade_basic_alloc_realloc_release
page_map_release_realloc
aligned_small_policy_path
huge_page_model_and_routes
remote_free_policy_and_page_port
page_source_purge_recommit_routes
secure_free_list_policy
stats_surface
```

## Missing / Not Integrated

```text
unified_production_allocator_api: high
real_provider_explicit_entrypoint_selection: high
page_map_aligned_huge_osvm_facade_integration: high
segment_arena_reclaim_tls_unification: medium
secure_entropy_backed_free_list: medium
mutable_runtime_options: low
ld_preload_compatible_shim: later
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_port_feature_gap_inventory_guard.sh
```

## Stop Line

Do not optimize in this row. Do not claim parity, activate providers, replace
the process allocator, install hooks, or select hakozuna.
