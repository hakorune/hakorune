---
Status: Current
Date: 2026-05-28
Scope: refresh gap taxonomy after the LocalSSA same-block reuse rollback.
Blocker: POST-ROLLBACK-GAP-TAXONOMY-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-169-ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE.md
  - tools/allocator/hako_mimalloc_post_rollback_gap_taxonomy_refresh.py
---

# 296x-170 Post Rollback Gap Taxonomy Refresh

## Purpose

Stop optimization after the row167/168 structural non-keeper and reclassify the
remaining gap. This row records that current keeper decisions are too dependent
on external elapsed and MIR shape deltas without an exact C object-lifecycle
pair or Hako body timing.

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0
selected_gap_owner=measurement_contract_gap
next_optimization_allowed=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0
input_contract=rollback-local-ssa-same-block-reuse-v0
workload_id=representative-object-lifecycle-small-block-v0
current_hako_external_elapsed_median_ms=550
current_hako_external_elapsed_source=row163_checkpoint_restored_by_row169
current_c_exact_pair_available=0
current_c_exact_pair_reason=object_lifecycle_c_runner_missing
hako_body_elapsed_available=0
c_body_elapsed_available=0
body_elapsed_comparable=0
body_elapsed_primary=0
mir_shape_timing_correlation=weak
mir_shape_timing_evidence=row167_structural_win_row168_timing_regression
hako_source_suspicion=possible
hako_source_suspicion_reason=facade_result_capsules_and_page_hotpath_helpers_remain_but_not_isolated
compiler_lowering_suspicion=possible
compiler_lowering_suspicion_reason=copy_call_field_surface_remains_but_copy_reduction_was_not_sufficient
runtime_baseline_suspicion=possible
runtime_baseline_suspicion_reason=external_elapsed_only_currently_drives_keeper_decisions
selected_gap_owner=measurement_contract_gap
gap_confidence=high
owner_reason=missing_exact_c_object_lifecycle_pair_and_missing_hako_body_timing
next_diagnostic=object_lifecycle_body_timing_and_exact_c_pair_contract
next_optimization_allowed=0
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The lane should not continue guessing between .hako source shape and MIR builder
shape. Row167 showed that a large MIR structural cleanup can regress exact-EXE
timing, so MIR instruction/copy deltas are not sufficient keeper evidence. The
next row must build measurement separation: Hako body timing plus an exact C
object-lifecycle comparison subject.
```

## Next

```text
row171:
  object_lifecycle_body_timing_and_exact_c_pair_contract

Goal:
  define the smallest exact C object-lifecycle pair and body-timing contract
  before allowing another optimization row.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_rollback_gap_taxonomy_refresh_guard.sh
```
