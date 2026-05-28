---
Status: Landed
Date: 2026-05-28
Scope: count field_get direct-consumer forwarding candidates before optimization.
Blocker: FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-180-FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION.md
  - tools/allocator/hako_mimalloc_field_get_direct_consumer_forwarding_candidate_probe.py
---

# 296x-181 Field Get Direct Consumer Forwarding Candidate Probe

## Purpose

Count the concrete field-get expression copy chains that could plausibly be
forwarded to direct consumers. This row is the final observe-only candidate
surface before deciding whether to open a narrow MIR builder keeper.

## Required Output

```text
output_contract=hako-mimalloc-field-get-direct-consumer-forwarding-candidate-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
field_get_expression_copy_count=23
consumer_reachable_copy_count=19
forwarding_candidate_copy_count=11
max_forwarding_chain_len=2
dominant_candidate_sink=compare_eq
selected_optimization_owner=mir_builder_expression_materialization_forwarding
next_diagnostic=field_get_direct_consumer_forwarding_keeper_design
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The next row may design a narrow MIR builder keeper for field_get direct
consumer forwarding. It should target only expression-materialization copy
chains with a real consumer, not broad LocalSSA copy coalescing.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_direct_consumer_forwarding_candidate_probe_guard.sh
```
