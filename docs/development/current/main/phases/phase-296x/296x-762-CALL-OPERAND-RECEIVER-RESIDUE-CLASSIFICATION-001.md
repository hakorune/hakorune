---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001
Scope: Classify the two receiver route-carrier samples from the receiver/arg
  split probe and select the next design row without opening implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-761-CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-696-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001.md
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001

## Result

```text
output_contract=hako-mimalloc-call-operand-receiver-residue-classification-v0
source_evidence=296x-761
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
call_operand_receiver_route_carrier_copy_count=2
call_operand_arg_route_carrier_copy_count=11
receiver_sample_0_block=block_596
receiver_sample_0_dst=62
receiver_sample_0_src=0
receiver_sample_0_callee=HakoAllocObjectLifecycleFacade.smallAcquireFailedReason/0
receiver_sample_0_class=same_block_self_receiver_materialization
receiver_sample_1_block=block_597
receiver_sample_1_dst=78
receiver_sample_1_src=0
receiver_sample_1_callee=HakoAllocObjectLifecycleFacade.recordSmallAllocSuccess/1
receiver_sample_1_class=same_block_self_receiver_materialization
same_block_self_receiver_materialization_count=2
prior_cfg_stable_receiver_keeper_reopen=0
selected_policy_family=same_block_self_receiver_materialization_rewrite
selected_next_action=call_operand_same_block_self_receiver_materialization_design
implementation_allowed=0
design_required=1
arg_forwarding_enabled=0
winner_claim=0
startup_lane_reopened=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
product_default_changed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

The receiver residue samples are both same-block calls whose receiver is copied
from value `0` before invoking methods on the same
`HakoAllocObjectLifecycleFacade` instance:

```text
block 596:
  copy dst=62 src=0
  call HakoAllocObjectLifecycleFacade.smallAcquireFailedReason/0 receiver=62

block 597:
  copy dst=78 src=0
  call HakoAllocObjectLifecycleFacade.recordSmallAllocSuccess/1 receiver=78
```

This is not the previous cross-block CFG-stable receiver family. It is a
smaller same-block self-receiver materialization surface.

## Decision

The prior CFG-stable receiver keeper remains closed:

```text
prior_cfg_stable_receiver_keeper_reopen=0
```

The next row may design a narrow same-block self-receiver rewrite. It must not
reuse the previous cross-block dominance machinery blindly and must not open
arg forwarding despite the larger arg residue.

## Stop Line

```text
do not implement from this classification row
do not patch LocalSSA::ensure_fallback_copy
do not reopen the previous CFG-stable receiver keeper
do not open arg forwarding
do not special-case source names, helper names, or benchmark names
do not change PHI lifecycle or freshness contracts
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001:
  design the narrow owner and guard surface for same-block self receiver
  materialization copies only
```
