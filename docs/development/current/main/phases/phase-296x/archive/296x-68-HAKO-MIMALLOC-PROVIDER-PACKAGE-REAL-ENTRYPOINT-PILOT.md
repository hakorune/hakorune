---
Status: Landed
Date: 2026-05-27
Scope: pilot explicit provider-package calls through the selected real `.hako` mimalloc entrypoint.
Blocker: HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-67-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION.md
---

# 296x-68 Hako Mimalloc Provider Package Real Entrypoint Pilot

## Purpose

Pilot the selected real `.hako` mimalloc surface through explicit provider
package calls before any LD_PRELOAD or replacement lane opens.

## Required Input

```text
output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
selected_surface_owner=HakoAllocObjectLifecycleFacade
provider_call_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
provider_call_executed=1
alloc_method_called=objectLifecycleSmallAlloc
release_method_called=objectLifecycleReleaseBlock
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0
input_contract=hako-mimalloc-provider-real-entrypoint-selection-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
selected_surface_owner=HakoAllocObjectLifecycleFacade
selected_surface_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
pilot_app=apps/mimalloc-facade-release-one-block-proof/main.hako
provider_call_kind=hako_exact_exe_selected_entrypoint_pilot
provider_call_executed=1
hako_selected_entrypoint_executed=1
alloc_method_called=objectLifecycleSmallAlloc
release_method_called=objectLifecycleReleaseBlock
alloc_observer_result=90,0
release_observer_result=90,0,0
release_counts=1,0
mir_call_chain_verified=1
exact_exe_run_verified=1
provider_package_native_artifact_generated=0
provider_package_native_fused_to_hako_entrypoint=0
provider_package_native_fusion_required=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
ld_preload_shim_ready=0
winner_claim=0
next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_provider_real_entrypoint_pilot_guard.sh
```

## Stop Line

Do not build an LD_PRELOAD shim, activate process allocator replacement,
install hooks, select hakozuna, or claim benchmark parity in this row.
