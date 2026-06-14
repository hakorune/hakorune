---
Status: Landed
Date: 2026-06-15
Task: PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001
Scope: Probe whether param-origin expression copies can be forwarded directly
  to consumers before implementing a MIRBuilder policy.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-665-PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/allocator/hako_mimalloc_param_direct_consumer_forwarding_candidate_probe.py
---

# PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001

## Purpose

`PARAM-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001` selected
`param_direct_consumer_value_forwarding`. This row must inspect exact consumer
sites before implementation.

```text
row_kind=diagnostic
implementation_started=0
optimization_open=0
selected_chain_policy=param_direct_consumer_value_forwarding
```

## Input Evidence

```text
param_origin_copy_count=7
expression_materialization_copy_count=11
param_origin_ratio_bp=6363
param_field_get_sink_copy_count=2
param_field_set_sink_copy_count=2
param_compare_sink_copy_count=3
param_direct_sink_copy_count=7
selected_chain_policy_confidence=medium
```

## Required First Step

Build and run a candidate probe that enumerates the param-origin expression
copies and classifies whether direct forwarding is safe per sink family.

Command shape:

```bash
python3 tools/allocator/hako_mimalloc_param_direct_consumer_forwarding_candidate_probe.py \
  --mir-json "$mir_json" \
  --chain-policy "$param_policy_report" \
  --out "$candidate_report"
```

Output shape:

```text
output_contract=hako-mimalloc-param-direct-consumer-forwarding-candidate-probe-v0
input_contract=hako-mimalloc-param-expression-copy-chain-policy-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
param_candidate_copy_count=7
safe_forward_field_get_count=<n>
safe_forward_field_set_count=<n>
safe_forward_compare_count=<n>
unsafe_forward_count=<n>
selected_optimization_owner=<owner|0>
selected_owner_confidence=<low|medium|high>
next_task=<next>
optimization_open=0
summary=ok
```

The probe must preserve these boundaries:

```text
param value identity preserved
no source-level .hako rewrite
no broad LocalSSA coalescing
no field_get historical policy reuse without current-MIR proof
```

## Stop Line

```text
do not implement forwarding before candidate safety classification
do not change MIRBuilder param semantics
do not change allocator provider activation
do not install hooks or global allocator
do not claim keeper from selection-only output
```

## Evidence

Result:

```text
output_contract=hako-mimalloc-param-direct-consumer-forwarding-candidate-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
param_candidate_copy_count=7
safe_forward_total_count=7
safe_forward_field_get_count=2
safe_forward_field_set_count=2
safe_forward_compare_count=3
unsafe_forward_count=0
dominant_param_sink=compare_eq
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
selected_owner_confidence=medium
next_task=param_direct_consumer_forwarding_guard_surface
optimization_open=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The probe confirms that every current param-origin expression copy is consumed
by a direct sink family:

```text
safe_forward_field_get_count=2
safe_forward_field_set_count=2
safe_forward_compare_count=3
unsafe_forward_count=0
```

## Selected Next Row

```text
next_task=PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001
next_card=docs/development/current/main/phases/phase-296x/296x-667-PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001.md
implementation_open=0
```

## Acceptance

```text
param_direct_consumer_forwarding_candidate_probe_001_landed=1
selected_chain_policy=param_direct_consumer_value_forwarding
candidate_probe_run=1
param_candidate_copy_count=7
safe_forward_total_count=7
unsafe_forward_count=0
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
next_task=PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001
implementation_started=0
optimization_open=0
summary=ok
```
