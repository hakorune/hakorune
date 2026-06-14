---
Status: Active
Date: 2026-06-15
Task: PARAM-ALIAS-COPY-OWNER-REFRESH-001
Scope: Re-select the owner for param-origin copy chains after direct-consumer
  LocalSSA forwarding did not reduce the target candidate count.
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

This row reselects the actual owner for the `%param -> copy -> copy -> direct
consumer` chains.

```text
row_kind=owner_refresh
implementation_started=0
optimization_open=0
previous_attempt=local_ssa_param_direct_consumer_forwarding
previous_attempt_keeper=0
```

## Evidence

After the rejected local attempt:

```text
before_param_candidate_copy_count=7
after_param_candidate_copy_count=7
after_expression_materialization_copy_count=10
after_unsafe_forward_count=0
```

Representative current MIR shape:

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

This suggests the owner is upstream from direct consumer finalization:

```text
candidate_owner=param_alias_local_binding_copy_chain
candidate_owner=variable_or_local_binding_materialization
candidate_owner=copy_chain_cleanup_after_variable_access
```

## Required First Step

Add or run an owner-refresh probe that classifies each param candidate by:

```text
chain_origin_param_id
chain_first_copy_block
chain_first_copy_position
chain_second_copy_position
final_consumer_family
whether_first_copy_is_variable_binding
whether_second_copy_is_local_ssa_or_expression_materialization
```

Required output shape:

```text
output_contract=hako-mimalloc-param-alias-copy-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
param_candidate_copy_count=7
dominant_chain_shape=<shape>
selected_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<next>
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not reapply local_ssa_param_direct_consumer_forwarding
do not broaden Arg forwarding for all calls
do not remove variable/local copies without identifying their semantic owner
do not change source .hako
do not touch allocator provider activation
```

## Acceptance

```text
param_alias_copy_owner_refresh_001_active=1
previous_attempt_keeper=0
owner_refresh_probe_run=0
selected_owner=0
implementation_started=0
optimization_open=0
summary=pending
```
