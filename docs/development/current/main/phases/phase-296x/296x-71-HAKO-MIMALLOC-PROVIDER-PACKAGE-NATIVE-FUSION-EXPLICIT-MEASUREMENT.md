---
Status: Landed
Date: 2026-05-27
Scope: measure the native-fusion explicit provider package before LD_PRELOAD.
Blocker: HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-70-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT.md
---

# 296x-71 Hako Mimalloc Provider Package Native Fusion Explicit Measurement

## Purpose

Run one light explicit provider measurement for the native-fusion provider
package before deciding whether to open the LD_PRELOAD shim lane.

## Required Input

```text
output_contract=hako-mimalloc-provider-package-native-fusion-pilot-v0
hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0
hako_entrypoint_mir_call_chain_verified=1
provider_alloc_executed=1
provider_free_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0
input_contract=hako-mimalloc-provider-package-native-fusion-pilot-v0
selected_entrypoint=object_lifecycle_small_alloc_release_v0
hako_semantic_provider_codegen=object-lifecycle-small-alloc-release-v0
measurement_profile=provider-native-fusion-explicit-repeated-v0
sample_count=3
warmup_count=1
operation_repeat=8192
request_size=32
request_align=8
provider_explicit_measurement_ready=1
ld_preload_decision_ready=1
provider_call_executed=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
ld_preload_shim_ready=0
winner_claim=0
next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_provider_package_native_fusion_explicit_measurement_guard.sh
```

## Stop Line

Do not build an LD_PRELOAD shim, activate process allocator replacement,
install hooks, select hakozuna, or claim benchmark parity in this row.
