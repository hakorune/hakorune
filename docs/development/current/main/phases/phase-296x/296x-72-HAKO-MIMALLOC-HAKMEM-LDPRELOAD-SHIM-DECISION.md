---
Status: Current
Date: 2026-05-27
Scope: decide whether to open a hakmem-compatible LD_PRELOAD shim lane.
Blocker: HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-71-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT.md
---

# 296x-72 Hako Mimalloc Hakmem LD_PRELOAD Shim Decision

## Purpose

Decide whether to build a hakmem-compatible malloc/free export shim after the
native-fusion provider package has explicit measurement evidence.

## Required Input

```text
output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0
provider_explicit_measurement_ready=1
ld_preload_decision_ready=1
provider_call_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0
ld_preload_shim_decision=accepted|parked
provider_call_evidence_ready=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Stop Line

Do not activate process allocator replacement, install hooks, select hakozuna,
or claim benchmark parity in this row.
