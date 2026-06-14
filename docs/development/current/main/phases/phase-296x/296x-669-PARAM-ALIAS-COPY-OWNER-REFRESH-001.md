---
Status: Landed
Date: 2026-06-15
Task: PARAM-ALIAS-COPY-OWNER-REFRESH-001
Scope: Repair block-local copy-origin misclassification and re-select the
  owner after apparent param-origin copies resolved to function-wide field_get
  chains.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-668-PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# PARAM-ALIAS-COPY-OWNER-REFRESH-001

## Purpose

`PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001` rejected the narrow
consumer-side LocalSSA implementation because the target param candidate count
did not decrease.

This row reselects the actual owner for the apparent `%param -> copy -> copy ->
direct consumer` chains. The root cause was diagnostic, not product code:
`hako_mimalloc_expression_materialization_copy_origin_probe.py` used
block-local producers, so cross-block field_get roots were misclassified as
`param`.

```text
row_kind=owner_refresh
implementation_started=0
optimization_open=0
previous_attempt=local_ssa_param_direct_consumer_forwarding
previous_attempt_keeper=0
origin_probe_scope_repaired=function_wide_producers
```

## Evidence

After the rejected local attempt:

```text
before_param_candidate_copy_count=7
after_param_candidate_copy_count=7
after_expression_materialization_copy_count=10
after_unsafe_forward_count=0
```

Representative MIR shape before origin repair:

```text
field_set base chain:
  %51 = copy %3
  %50 = copy %51
  field_set box=%50 field=last_page_id

field_get base chain:
  %92 = copy %15
  %91 = copy %92
  field_get box=%91 field=last_selected_page_id

compare operand chain:
  %147 = copy %120
  %150 = copy %147
  %153 = copy %150
  compare lhs=%153
```

After repairing the probe to use function-wide producers, the selected owner
returned to field_get-origin chains:

```text
expression_materialization_copy_count=10
dominant_expression_origin=field_get
field_get_origin_copy_count=7
mir_call_origin_copy_count=2
param_origin_copy_count=0
selected_origin_policy=field_get_expression_value_copy_chain
next_diagnostic=field_get_expression_copy_chain_policy_selection
summary=ok
```

Refreshed field-get policy:

```text
field_get_origin_copy_count=7
expression_materialization_copy_count=10
field_get_origin_ratio_bp=7000
selected_chain_policy=field_get_direct_consumer_value_forwarding
selected_chain_policy_confidence=medium
summary=ok
```

Refreshed field-get candidate probe:

```text
field_get_expression_copy_count=7
consumer_reachable_copy_count=7
forwarding_candidate_copy_count=4
max_forwarding_chain_len=1
dominant_candidate_sink=field_get
dominant_candidate_field=object_lifecycle_queue
selected_optimization_owner=mir_builder_expression_materialization_forwarding
next_diagnostic=field_get_direct_consumer_forwarding_keeper_design
summary=ok
```

## Repaired Tools

```text
tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py:
  origin_label uses function-wide producers

tools/allocator/hako_mimalloc_field_get_direct_consumer_forwarding_candidate_probe.py:
  origin_label uses function-wide producers

tools/allocator/hako_mimalloc_param_direct_consumer_forwarding_candidate_probe.py:
  origin_label uses function-wide producers
```

## Stop Line

```text
do not reapply local_ssa_param_direct_consumer_forwarding
do not broaden Arg forwarding for all calls
do not use block-local producer origin attribution for cross-block chains
do not reuse historical row181/182 counts without current-MIR refresh
do not change source .hako
do not touch allocator provider activation
```

## Acceptance

```text
param_alias_copy_owner_refresh_001_landed=1
previous_attempt_keeper=0
origin_probe_scope_repaired=function_wide_producers
dominant_expression_origin=field_get
param_origin_copy_count=0
selected_origin_policy=field_get_expression_value_copy_chain
field_get_forwarding_candidate_copy_count=4
selected_owner=mir_builder_expression_materialization_forwarding
next_task=FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002
implementation_started=0
optimization_open=0
summary=ok
```
