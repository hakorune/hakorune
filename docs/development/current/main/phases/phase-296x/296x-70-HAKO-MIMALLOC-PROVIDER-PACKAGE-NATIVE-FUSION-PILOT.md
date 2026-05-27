---
Status: Current
Date: 2026-05-27
Scope: pilot a hako-derived provider semantic mode for the real `.hako` mimalloc entrypoint.
Blocker: HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-69-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION.md
---

# 296x-70 Hako Mimalloc Provider Package Native Fusion Pilot

## Purpose

Add and prove the first hako-derived provider semantic mode for the selected
object-lifecycle small alloc/release entrypoint.

## Required Input

```text
output_contract=hako-mimalloc-provider-package-native-fusion-selection-v0
native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0
required_codegen_mode=object-lifecycle-small-alloc-release-v0
required_fixture=apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako
provider_package_native_fusion_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-provider-package-native-fusion-pilot-v0
hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
hako_entrypoint_mir_call_chain_verified=1
provider_alloc_executed=1
provider_free_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Stop Line

Do not build an LD_PRELOAD shim, activate process allocator replacement,
install hooks, select hakozuna, or claim benchmark parity in this row.
