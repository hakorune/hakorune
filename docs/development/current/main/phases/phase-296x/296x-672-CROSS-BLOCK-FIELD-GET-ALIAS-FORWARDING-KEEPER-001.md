---
Status: Landed
Date: 2026-06-15
Task: CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-KEEPER-001
Scope: Implement the narrow dominance-aware field_get alias keeper selected by
  296x-671, then remeasure the exact body-timing owner.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-670-FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002.md
  - docs/development/current/main/phases/phase-296x/296x-671-CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-DESIGN-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-KEEPER-001

## Purpose

Implement only the keeper shape selected by 296x-671:

```text
keeper_shape=dominance_alias
selected_owner=cross_block_field_get_alias_copy_chain
safe_alias_candidate_count=4
dominance_required=1
same_field_mutation_guard_required=1
same_receiver_alias_guard_required=1
arbitrary_copy_coalescing_allowed=0
```

## Allowed Implementation Shape

The keeper may forward a field_get-origin copy chain only when all conditions
are true:

```text
1. root value is produced by FieldGet
2. root FieldGet block dominates the consumer/materialization block
3. chain between root and candidate is Copy-only
4. no possible same receiver/field mutation is visible on the candidate path
5. target consumer is a direct expression consumer selected by 296x-670/671
6. no arbitrary copy coalescing is introduced
```

## Stop Line

```text
do not reopen param forwarding
do not forward arbitrary Copy -> Copy chains
do not forward without dominance proof
do not forward across possible same receiver/field mutation
do not touch .hako source
do not touch allocator provider activation
do not claim winner without body-timing remeasurement
```

## Required Proof

```text
pre_guard:
  bash tools/checks/k2_wide_phase296x_cross_block_field_get_alias_design_guard.sh

post_candidate_probe:
  forwarding_candidate_copy_count must decrease from 4

semantic:
  selected object-lifecycle app still emits MIR and runs selected proof path

remeasure:
  rerun product-route body timing pair before keeper claim
```

## Result

```text
implementation_started=1
cargo_build_release_bin_hakorune=ok

post_probe:
  output_contract=hako-mimalloc-field-get-alias-keeper-post-probe-v0
  copy_count=69
  expression_materialization_copy_count=3
  field_get_expression_copy_count=0
  forwarding_candidate_copy_count=0
  optimization_open=0
  winner_claim=0
  summary=ok

body_timing_remeasured=1
  hako_body_elapsed_ns=364000000
  c_body_elapsed_ns=3922424
  body_elapsed_ratio=92.800
  gap_owner=compiler_lowering
  gap_confidence=medium
```

Interpretation:

```text
keeper_removed_selected_candidate_family=1
forwarding_candidate_copy_count_before=4
forwarding_candidate_copy_count_after=0
body_timing_material_win=small_or_noise
winner_claim=0
next_task=post_field_get_alias_keeper_owner_refresh
```

After the keeper, the old local-SSA selection ladder no longer has the same
owner shape: `dominant_dynamic_owner` moved to `page_hotpath_helper_attribution`
and callsite attribution reports `dominant_copy_owner=result_materialization`.
The next row must refresh the owner instead of extending this keeper.

Guard:

```bash
bash tools/checks/k2_wide_phase296x_cross_block_field_get_alias_keeper_guard.sh
```

## Acceptance

```text
cross_block_field_get_alias_forwarding_keeper_active=1
implementation_started=1
pre_guard_green=1
post_candidate_probe_run=1
forwarding_candidate_copy_count_before=4
forwarding_candidate_copy_count_after=0
body_timing_remeasured=1
winner_claim=0
optimization_open=0
summary=ok
```
