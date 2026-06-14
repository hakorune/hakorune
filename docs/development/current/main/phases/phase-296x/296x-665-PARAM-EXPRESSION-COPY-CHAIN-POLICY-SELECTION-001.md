---
Status: Landed
Date: 2026-06-15
Task: PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001
Scope: Select the param-origin expression copy-chain policy before any
  implementation against the object-lifecycle body-timing front.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-664-EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002.md
  - docs/development/current/main/phases/phase-296x/296x-180-FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/allocator/hako_mimalloc_param_expression_copy_chain_policy_selection.py
---

# PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001

## Purpose

The refreshed expression-materialization origin probe selected
`param_expression_value_copy_chain`, not the historical `field_get` chain. This
row chooses the next policy owner for param-origin expression materialization
copies before implementation.

```text
row_kind=selection
implementation_started=0
optimization_open=0
input_origin_policy=param_expression_value_copy_chain
```

## Input Evidence

From `EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002`:

```text
expression_materialization_copy_count=11
dominant_expression_origin=param
param_origin_copy_count=7
field_get_origin_copy_count=3
const_origin_copy_count=1
dominant_expression_sink=field_get
selected_origin_policy=param_expression_value_copy_chain
next_diagnostic=param_expression_copy_chain_policy_selection
```

Param-origin sinks are mixed:

```text
pair_param__field_get_copy_count=2
pair_param__field_set_last_page_id_copy_count=2
pair_param__compare_eq_copy_count=2
pair_param__compare_ne_copy_count=1
```

That means the next policy must decide whether this belongs to:

```text
param_direct_consumer_value_forwarding
block_entry_param_copy_policy
expression_materialization_param_copy_policy
later_copy_cleanup_pass
```

## Required First Step

Add and run a narrow selector over the origin report. It must be selection-only
and must not edit MIRBuilder.

Command shape:

```bash
python3 tools/allocator/hako_mimalloc_param_expression_copy_chain_policy_selection.py \
  --origin "$origin_report" \
  --out "$param_policy_report"
```

Output contract:

```text
output_contract=hako-mimalloc-param-expression-copy-chain-policy-selection-v0
input_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
param_origin_copy_count=7
expression_materialization_copy_count=11
param_origin_ratio_bp=<computed>
field_get_sink_copy_count=2
field_set_sink_copy_count=2
compare_sink_copy_count=3
selected_chain_policy=<selected>
selected_chain_policy_confidence=<low|medium|high>
next_diagnostic=<next>
optimization_open=0
winner_claim=0
summary=ok
```

## Evidence

Result:

```text
output_contract=hako-mimalloc-param-expression-copy-chain-policy-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
param_origin_copy_count=7
expression_materialization_copy_count=11
param_origin_ratio_bp=6363
param_field_get_sink_copy_count=2
param_field_set_sink_copy_count=2
param_compare_sink_copy_count=3
param_direct_sink_copy_count=7
selected_chain_policy=param_direct_consumer_value_forwarding
selected_chain_policy_confidence=medium
selected_chain_policy_reason=param_origin_dominates_expression_materialization_with_mixed_direct_sinks
rejected_chain_policy=field_get_expression_value_copy_chain
rejected_reason=current_expression_origin_is_param_not_field_get
rejected_chain_policy_2=local_ssa_broad_copy_coalescing
rejected_reason_2=recent_local_ssa_same_block_reuse_nonkeeper
next_diagnostic=param_direct_consumer_forwarding_candidate_probe
optimization_open=0
winner_claim=0
summary=ok
```

## Selected Next Diagnostic

```text
next_task=PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001
next_card=docs/development/current/main/phases/phase-296x/296x-666-PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001.md
implementation_open=0
```

## Stop Line

```text
do not implement param forwarding before selection
do not reopen broad LocalSSA copy coalescing
do not reuse field_get chain policy from row180 without proof
do not change product NyRT startup
do not install allocator hooks or global allocator
```

## Acceptance

```text
param_expression_copy_chain_policy_selection_001_landed=1
input_origin_policy=param_expression_value_copy_chain
selector_output_contract_defined=1
selector_run=1
param_origin_copy_count=7
param_origin_ratio_bp=6363
param_direct_sink_copy_count=7
selected_chain_policy=param_direct_consumer_value_forwarding
selected_chain_policy_confidence=medium
next_task=PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001
implementation_started=0
optimization_open=0
summary=ok
```
