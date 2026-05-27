---
Status: Current
Date: 2026-05-27
Scope: decide whether the mimalloc parity lane can hand focus back toward selfhosting.
Blocker: HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-74-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT.md
---

# 296x-75 Hako Mimalloc Perf Parity Selfhost Handoff Gate

## Purpose

Decide whether the current `.hako` mimalloc/provider/LD_PRELOAD evidence is
strong enough to return focus toward selfhosting, or whether another parity
diagnostic is required first.

## Required Input

```text
output_contract=hako-mimalloc-hakmem-ldpreload-bench-pilot-v0
hakmem_script_compatible=probe-only
ld_preload_env_applied=1
benchmark_sample_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-perf-parity-selfhost-handoff-gate-v0
selfhost_handoff_decision=accepted|parked
remaining_allocator_gap_classified=1
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not claim benchmark winner status in this row. If evidence is insufficient,
park handoff and select a focused parity diagnostic.
