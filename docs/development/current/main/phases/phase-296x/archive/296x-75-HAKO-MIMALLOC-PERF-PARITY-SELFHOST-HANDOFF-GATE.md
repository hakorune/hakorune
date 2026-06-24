---
Status: Landed
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
selfhost_handoff_decision=parked
park_reason=hako_mimalloc_small_block_gap_still_large
remaining_allocator_gap_classified=1
next_diagnostic=hako_check_perf_surface_inventory
winner_claim=0
replacement_active=0
summary=ok
```

## Planned Decision

Park the selfhost handoff for now. The provider package and probe-only
LD_PRELOAD lane are functional, but the `.hako` mimalloc small-block model still
has a large hot-path gap. The next work should make `hako_check` report
optimization surfaces before adding another keeper optimization.

Planned output:

```text
output_contract=hako-mimalloc-perf-parity-selfhost-handoff-gate-v0
selfhost_handoff_decision=parked
park_reason=hako_mimalloc_small_block_gap_still_large
remaining_allocator_gap_classified=1
next_diagnostic=hako_check_perf_surface_inventory
winner_claim=0
replacement_active=0
summary=ok
```

## Next Task Stack

### Row 76 - hako_check Perf Surface Contract

Define the first `hako_check perf-surface` report contract. This is an
observation feature, not an optimizer or rewrite pass.

Required output:

```text
output_contract=hako-check-perf-surface-contract-v0
target_file
target_box
target_method
method_call_count
loop_method_call_count
array_access_count
linear_search_candidate=0|1
result_capsule_churn=0|1
observer_call_count
hot_path_risk=low|medium|high
suggested_next
summary=ok
```

### Row 77 - hako_check Perf Surface Inventory

Apply the contract to `object_lifecycle_facade_box.hako` and inventory
`objectLifecycleSmallAlloc` plus `objectLifecycleReleaseBlock`.

Expected candidate:

```text
target_method=objectLifecycleReleaseBlock
linear_search_candidate=1
suggested_next=release_known_page_fast_path
```

### Row 78 - Keeper 1: Release Known-Page Fast Path

Add one `.hako` allocator-model keeper optimization that avoids the hot
`objectLifecycleKnownPageIndexById` path when releasing the page that was just
allocated. Keep the normal fail-fast release route intact.

### Row 79 - Post-Keeper Measurement

Rerun the 8192-repeat in-process small-block measurement and compare against
the current `.hako` small-block checkpoint. Keep winner claims closed.

### Row 80 - Next Keeper Selection

Select exactly one next keeper candidate from hako_check evidence:

```text
selectPage single-page fast path
result capsule hot-loop update reduction
observer getter reduction
ArrayBox get/length call reduction
```

## Stop Line

Do not claim benchmark winner status in this row. If evidence is insufficient,
park handoff and select a focused parity diagnostic.

## Landed Evidence

```text
output_contract=hako-mimalloc-perf-parity-selfhost-handoff-gate-v0
selfhost_handoff_decision=parked
park_reason=hako_mimalloc_small_block_gap_still_large
remaining_allocator_gap_classified=1
next_diagnostic=hako_check_perf_surface_inventory
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_selfhost_handoff_gate_guard.sh
```
