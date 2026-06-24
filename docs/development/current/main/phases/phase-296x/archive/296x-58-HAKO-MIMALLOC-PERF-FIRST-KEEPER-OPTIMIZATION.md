---
Status: Landed
Date: 2026-05-27
Scope: apply the first evidence-backed optimization only if in-process diagnostics allow it.
Blocker: HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
---

# 296x-58 Hako Mimalloc First Keeper Optimization

## Purpose

Apply exactly one optimization only when owner diagnostics select this row.

## Required Input

```text
output_contract=hako-mimalloc-compiler-allocator-owner-split-v0
selected_next_row=HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001
next_optimization_allowed=1
selected_gap_owner=allocator_algorithm
selected_gap_confidence=high
winner_claim=0
```

## Optimization

Applied exactly one allocator-algorithm change:

```text
optimization_kind=page_model_reuse_via_reset_to_fresh
changed_surface=apps/hako-alloc-mimalloc-comparison-in-process-small-block-proof/main.hako
allocator_owner=allocator_algorithm
```

The in-process small-block fixture now reuses one `HakoAllocPageModel` across
inner repeats and calls `resetToFresh()` per repeat instead of constructing a
new page model each time.

## Evidence

Before:

```text
hako_external_elapsed_median_ms=330
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=326
```

After:

```text
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
operation_repeat=8192
process_repeat=3
hako_external_elapsed_median_ms=280
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=276
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The gap improved by 50ms under the same in-process measurement contract.
Winner/parity claims remain closed; this is a keeper optimization candidate,
not a performance victory claim.

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_perf_first_keeper_optimization_guard.sh
```

## Stop Line

Do not start from noisy, low-confidence, benchmark-harness, provider-wrapper, or
hako-runtime-baseline evidence. Do not combine compiler and allocator changes in
one row.
