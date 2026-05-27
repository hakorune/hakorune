---
Status: Current
Date: 2026-05-27
Scope: refresh in-process taxonomy after the second keeper optimization.
Blocker: HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-61-HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION.md
---

# 296x-62 Hako Mimalloc Post Second Keeper Taxonomy Refresh

## Purpose

Refresh the in-process gap after the acquire-side keeper optimization before
choosing another optimization or switching to feature-port inventory.

## Required Input

```text
optimization_kind=acquire_usize_free_top_fast_path
target_phase=alloc
before_full_elapsed_median_ms=280
after_full_elapsed_median_ms=260
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-post-second-keeper-taxonomy-refresh-v0
current_hako_external_elapsed_median_ms=260
current_c_external_elapsed_median_ms
remaining_gap_ms
gap_owner
gap_confidence
next_diagnostic
next_optimization_allowed=0|1
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
