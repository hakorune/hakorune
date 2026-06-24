---
Status: Landed
Date: 2026-05-27
Scope: select the next native provider-package fusion step for the real `.hako` mimalloc entrypoint.
Blocker: HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-68-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT.md
---

# 296x-69 Hako Mimalloc Provider Package Native Fusion Selection

## Purpose

Select how to fuse the verified real `.hako` mimalloc entrypoint into the
native provider-package artifact without opening process allocator replacement.

## Required Input

```text
output_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
hako_selected_entrypoint_executed=1
provider_package_native_fused_to_hako_entrypoint=0
provider_package_native_fusion_required=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-provider-package-native-fusion-selection-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
native_fusion_strategy
provider_package_native_fusion_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-provider-package-native-fusion-selection-v0
input_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0
strategy_owner=src/cli/provider_package_hako_derived_build.rs
strategy_args_owner=src/cli/args.rs
required_codegen_mode=object-lifecycle-small-alloc-release-v0
required_fixture=apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako
required_surface_owner=HakoAllocObjectLifecycleFacade
required_alloc_method=objectLifecycleSmallAlloc
required_release_method=objectLifecycleReleaseBlock
required_mir_call_chain_check=1
required_provider_alloc_free_smoke=1
provider_package_native_fusion_allowed=1
provider_call_allowed=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
ld_preload_shim_ready=0
winner_claim=0
next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_provider_package_native_fusion_selection_guard.sh
```

## Stop Line

Do not build an LD_PRELOAD shim, activate process allocator replacement,
install hooks, select hakozuna, or claim benchmark parity in this row.
