---
Status: Landed
Date: 2026-05-27
Scope: implement one selectPage single-page fast path keeper.
Blocker: HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
  - lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
---

# 296x-81 Hako Mimalloc Perf SelectPage Single-Page Fast Path

## Purpose

Implement exactly one next keeper selected by row 80: avoid the full
`selectPage` scan on the object-lifecycle small-alloc path when the queue has a
single known usable page.

## Required Output

```text
output_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0
input_contract=hako-mimalloc-perf-next-keeper-selection-v0
keeper=select_page_single_page_fast_path
target_method=objectLifecycleSmallAlloc
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not combine this with result-capsule reduction or observer getter reduction.

## Landed Evidence

```text
output_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0
input_contract=hako-mimalloc-perf-next-keeper-selection-v0
keeper=select_page_single_page_fast_path
target_method=objectLifecycleSmallAlloc
queue_fast_path_method=selectSinglePageFastPath
proof_expected_select_page_single=524288,0
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_select_page_single_page_fast_path_guard.sh
```

## Planned Follow-on Stack

Keep the selected keeper implementation ahead of observation surface expansion.
The v0 hako_check report already selected this keeper.

### Row 82 - Post SelectPage Keeper Measurement

Measure the object-lifecycle facade exact-EXE path after the
`select_page_single_page_fast_path` keeper.

Required output:

```text
output_contract=hako-mimalloc-perf-post-select-page-keeper-measurement-v0
input_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0
operation_repeat=8192
sample_count=3
keeper=select_page_single_page_fast_path
winner_claim=0
replacement_active=0
summary=ok
```

### Row 83 - hako_check Perf-Surface v1 Minimal

Extend source-level hako_check observation without adding rewrite or MIR
responsibility.

Required output:

```text
output_contract=hako-check-perf-surface-v1
loop_field_get_count
loop_field_set_count
loop_array_get_count
loop_array_length_count
allocation_like_in_loop_count
suggested_next_kind=box_count|box_shape|mir_diagnostic|none
confidence=low|medium|high
summary=ok
```

### Row 84 - Keeper Before/After Diff Adapter

Add an adapter that compares before/after source perf-surface reports and
measurement evidence for one keeper. This is not hako_check core.

Required output:

```text
output_contract=hako-mimalloc-keeper-before-after-diff-v0
keeper_id
source_surface_delta_ready=1
measurement_delta_ready=1
keeper_effect=accepted|no_effect|regressed|inconclusive
winner_claim=0
summary=ok
```

### Row 85 - Python MIR Method Shape Adapter

Add the first MIR-level observation app as Python, consuming selected MIR JSON
and producing method shape counts. This remains outside hako_check core.

Required output:

```text
output_contract=hako-mir-method-shape-v0
input_kind=mir_json
selected_method
mir_instruction_count
call_count
field_get_count
field_set_count
array_get_call_count
array_length_call_count
phi_count
copy_count
branch_count
return_count
summary=ok
```

### Row 86 - Source/MIR Shape Join Adapter

Join hako_check source perf-surface evidence with MIR method shape evidence for
one selected method.

Required output:

```text
output_contract=hako-source-mir-shape-join-v0
source_contract=hako-check-perf-surface-v1
mir_contract=hako-mir-method-shape-v0
selected_method
source_risk_confirmed_in_mir=0|1
next_diagnostic
summary=ok
```

### Row 87 - .hako MIR Method Shape Migration Selection

Decide whether the Python MIR method shape adapter is stable enough to port a
minimal reader/checker to `.hako`.

Required output:

```text
output_contract=hako-mir-method-shape-hako-migration-selection-v0
python_contract_stable=0|1
hako_migration_decision=accepted|parked
selected_scope
summary=ok
```
