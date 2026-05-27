---
Status: Landed
Date: 2026-05-27
Scope: refresh confidence for a stable but low-confidence owner classification.
Blocker: HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-50-HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION.md
---

# 296x-51 Hako Mimalloc Owner Confidence Refresh

## Purpose

Refine the current stable but low-confidence `hako_runtime_baseline`
classification before any optimization work starts.

## Current Evidence

The latest refreshed taxonomy was:

```text
evidence_quality=stable
gap_owner=hako_runtime_baseline
gap_confidence=low
next_diagnostic=owner_confidence_refresh
next_optimization_allowed=0
```

## Required Next Diagnostic

The confidence refresh uses an empty workload repeated measurement as a fixed
runtime baseline probe:

```text
workload_id=representative-empty-v0
sample_count=5
warmup_count=1
operation_repeat=128
hako_runtime_config_profile=empty
```

The row 51 evidence was:

```text
output_contract=hako-mimalloc-owner-confidence-refresh-v0
confidence_refresh_kind=empty_workload_runtime_baseline
small_elapsed_median_gap_ms=10
empty_hako_elapsed_median_ms=80
empty_c_elapsed_median_ms=70
empty_elapsed_median_gap_ms=10
refreshed_gap_owner=hako_runtime_baseline
refreshed_gap_confidence=medium
next_diagnostic=repeat_scaling_runtime_diagnostic
next_optimization_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Because the empty workload keeps the same 10ms median gap, the next row must
separate fixed runtime/process cost from per-operation scaling before any
allocator or compiler optimization starts.

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_owner_confidence_refresh_guard.sh
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
