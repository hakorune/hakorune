---
Status: Landed
Date: 2026-06-15
Task: FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002
Scope: Refresh the field_get direct-consumer forwarding owner on current MIR
  after repairing copy-origin attribution to function-wide producers.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-669-PARAM-ALIAS-COPY-OWNER-REFRESH-001.md
  - docs/development/current/main/phases/phase-296x/296x-181-FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE.md
  - docs/development/current/main/phases/phase-296x/296x-182-FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002

## Purpose

The repaired origin probe shows the current object-lifecycle MIR is dominated
by field_get-origin expression copies, not param-origin copies. Historical rows
181/182 are landed but their counts are stale, so this row refreshes the owner
before implementation.

```text
row_kind=selection
implementation_started=0
optimization_open=0
origin_probe_scope=function_wide_producers
selected_origin_policy=field_get_expression_value_copy_chain
```

## Current Evidence

```text
expression_materialization_copy_count=10
dominant_expression_origin=field_get
field_get_origin_copy_count=7
mir_call_origin_copy_count=2
param_origin_copy_count=0
selected_origin_policy=field_get_expression_value_copy_chain

field_get_expression_copy_count=7
consumer_reachable_copy_count=7
forwarding_candidate_copy_count=4
max_forwarding_chain_len=1
dominant_candidate_sink=field_get
dominant_candidate_field=object_lifecycle_queue
selected_optimization_owner=mir_builder_expression_materialization_forwarding
```

Historical row182 implemented a same-block field_get forwarding keeper. Current
candidate samples still include field_get-origin `copy -> copy -> consumer`
chains, so this row must identify whether the remaining owner is:

```text
same_block_field_get_forwarding_gap
cross_block_field_get_alias_copy_chain
variable/local materialization around field_get result
post-field-get copy cleanup
```

## Required First Step

Run or add a refreshed candidate probe that classifies the four current
forwarding candidates by:

```text
origin_field
origin_block
candidate_block
same_block_origin
copy_chain_len
consumer_family
already_covered_by_row182_same_block_rule
```

Required output:

```text
output_contract=hako-mimalloc-field-get-direct-consumer-refresh-v2
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
forwarding_candidate_copy_count=4
same_block_candidate_count=<n>
cross_block_candidate_count=<n>
covered_by_existing_rule_count=<n>
selected_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<next>
optimization_open=0
winner_claim=0
summary=ok
```

## Refreshed Result

```text
output_contract=hako-mimalloc-field-get-direct-consumer-refresh-v2
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
field_get_expression_copy_count=7
forwarding_candidate_copy_count=4
same_block_candidate_count=1
cross_block_candidate_count=3
covered_by_existing_rule_count=0
dominant_candidate_sink=field_get
dominant_candidate_field=object_lifecycle_queue
selected_owner=cross_block_field_get_alias_copy_chain
selected_owner_confidence=medium
next_task=cross_block_field_get_alias_forwarding_design
optimization_open=0
winner_claim=0
summary=ok
```

Interpretation:

```text
row182_existing_same_block_field_get_rule_coverage=0
same_block_gap_exists=1
cross_block_gap_dominates=1
param_forwarding_reopen=0
```

The remaining candidates are not direct `FieldGet -> consumer` cases. They are
`FieldGet -> Copy -> Copy -> consumer` chains, with most roots crossing from an
earlier block into a later consumer block. Therefore this row closes as a
selection refresh, not an implementation row.

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_direct_consumer_refresh_guard.sh
```

## Stop Line

```text
do not reopen param forwarding
do not broaden LocalSSA coalescing
do not reuse stale row181/182 counts
do not change source .hako
do not touch allocator provider activation
```

## Acceptance

```text
field_get_direct_consumer_forwarding_refresh_002_active=1
origin_probe_scope=function_wide_producers
refreshed_candidate_probe_run=1
selected_owner=cross_block_field_get_alias_copy_chain
selected_owner_confidence=medium
next_task=cross_block_field_get_alias_forwarding_design
implementation_started=0
optimization_open=0
summary=ok
```
