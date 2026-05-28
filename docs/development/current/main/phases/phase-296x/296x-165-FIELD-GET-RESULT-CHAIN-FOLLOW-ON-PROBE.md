---
Status: Current
Date: 2026-05-28
Scope: classify remaining field_get result-chain copy consumers.
Blocker: FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-164-POST-FIELD-GET-CLEANUP-OWNER-REFRESH.md
  - tools/allocator/mir_field_get_result_chain_follow_on_probe.py
---

# 296x-165 Field Get Result Chain Follow-On Probe

## Purpose

Classify the remaining field_get result-chain copies after row164 by consumer
kind. This row verifies whether the next owner should be another field_get
lowering tweak, a PHI incoming copy cleanup, or a LocalSSA same-block reuse
cleanup.

## Required Output

```text
output_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0
input_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0
selected_owner
owner_confidence
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0
input_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
field_get_result_chain_copy_count=30
same_block_origin_copy_count=30
selected_owner=local_ssa_same_block_field_get_reuse_probe
owner_confidence=medium
owner_reason=same_block_field_get_origins_and_internal_copy_chains_dominate
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
consumer_copy_source_count=15
consumer_field_get_receiver_count=2
consumer_phi_incoming_count=10
consumer_compare_operand_count=4
consumer_binop_operand_count=2
consumer_field_set_value_count=2
consumer_branch_condition_count=0
consumer_dead_or_chain_internal_count=0
origin_field_last_selected_index_copy_count=5
origin_field_last_selected_page_copy_count=5
origin_field_last_selected_kind_copy_count=5
origin_field_alloc_result_copy_count=3
origin_field_object_lifecycle_queue_copy_count=3
origin_field_page_count_copy_count=3
origin_field_attempt_count_copy_count=2
origin_field_request_count_copy_count=2
origin_field_last_selected_page_id_copy_count=2
top_block_0_id=block_552
top_block_0_field_get_chain_copy_count=11
top_block_1_id=block_560
top_block_1_field_get_chain_copy_count=6
top_block_2_id=block_555
top_block_2_field_get_chain_copy_count=5
top_block_3_id=block_557
top_block_3_field_get_chain_copy_count=5
top_block_4_id=block_553
top_block_4_field_get_chain_copy_count=2
top_block_5_id=block_575
top_block_5_field_get_chain_copy_count=1
summary=ok
```

Interpretation:

```text
All field_get result-chain copies have same-block field_get origins. The
dominant immediate consumer is internal copy-source chaining, with PHI incoming
uses second. The next implementation should test a narrow LocalSSA same-block
reuse rule before touching PHI construction or source-level .hako shape.
```

## Next

```text
row166:
  local_ssa_same_block_field_get_reuse_selection

Goal:
  select a narrow compiler owner for returning same-block values directly from
  LocalSSA ensure when dominance is already local, while keeping non-dominating
  and cross-block copies protected.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_result_chain_follow_on_probe_guard.sh
```
