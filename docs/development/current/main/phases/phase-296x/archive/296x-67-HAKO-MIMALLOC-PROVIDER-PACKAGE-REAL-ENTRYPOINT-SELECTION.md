---
Status: Landed
Date: 2026-05-27
Scope: select the real `.hako` mimalloc explicit provider entrypoint after port feature inventory.
Blocker: HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
---

# 296x-67 Hako Mimalloc Provider Package Real Entrypoint Selection

## Purpose

Select which real `.hako` mimalloc surface should become the next explicit
provider package API evidence row.

## Required Input

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
primary_gap_kind=integration_surface_gap
next_port_feature=real_provider_explicit_entrypoint_selection
provider_entrypoint_selection_ready=1
ld_preload_shim_ready=0
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0
selected_entrypoint
selected_surface_owner
provider_call_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0
input_contract=hako-mimalloc-port-feature-gap-inventory-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
selected_surface_owner=HakoAllocObjectLifecycleFacade
selected_surface_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
selected_alloc_method=objectLifecycleSmallAlloc
selected_release_method=objectLifecycleReleaseBlock
selected_page_id_method=objectLifecycleAllocPageId
selected_block_id_method=objectLifecycleAllocBlockId
selected_surface_scope=small_block_object_lifecycle
provider_call_allowed=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
ld_preload_shim_ready=0
winner_claim=0
next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001
summary=ok
```

## Selection Rationale

```text
selected_surface_reason=real_hako_mimalloc_facade_with_page_selection_reuse_release_and_observers
rejected_0_entrypoint=production_facade_basic_alloc_release_v0
rejected_0_reason=Production facade is still mainly backed by the older HakoAllocHeap route, so it would hide the post-plateau integration gap.
rejected_1_entrypoint=ld_preload_malloc_free_v0
rejected_1_reason=LD_PRELOAD-compatible malloc/free replacement needs explicit provider call evidence first.
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_provider_real_entrypoint_selection_guard.sh
```

## Stop Line

Do not activate providers, replace the process allocator, install hooks,
select hakozuna, or build an LD_PRELOAD shim in this row.
