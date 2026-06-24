---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001
Scope: Decide whether the two same-block self-receiver materialization copies
  justify another implementation owner, given the already-landed same-block
  receiver forwarding keeper.
Related:
  - docs/development/current/main/phases/phase-296x/296x-762-CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-685-CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-686-CALL-OPERAND-MATERIALIZATION-FORWARDING-GUARD-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-687-CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001.md
  - src/mir/builder/ssa/local.rs
---

# CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001

## Result

```text
output_contract=hako-mimalloc-call-operand-same-block-self-receiver-materialization-design-v0
source_evidence=296x-762,296x-685,296x-686,296x-687
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
same_block_self_receiver_materialization_count=2
prior_keeper_shape=same_block_root_receiver_operand_forwarding
prior_keeper_landed=1
prior_keeper_owner=LocalSSA::ensure_fallback_copy
existing_code_seam=LocalKind::Recv::can_forward_same_block_copy_root_to_receiver
current_receiver_residue_interpretation=receiver_pin_copy_not_additional_forwarding_candidate
selected_owner=none
selected_owner_reason=existing_same_block_receiver_keeper_already_landed_and_current_residue_is_the_receiver_pin_copy_itself
receiver_lane_closed=1
arg_forwarding_enabled=0
selected_next_action=call_operand_arg_residue_policy_selection
implementation_allowed=0
design_opens_implementation=0
measurement_required=0
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

## Decision

The two receiver samples are same-block self-receiver materialization copies:

```text
same_block_self_receiver_materialization_count=2
```

However, the same family was already designed and implemented by the
same-block-root receiver keeper:

```text
prior_keeper_shape=same_block_root_receiver_operand_forwarding
prior_keeper_landed=1
prior_keeper_owner=LocalSSA::ensure_fallback_copy
```

The current residue is therefore not a new implementation owner. It is the
receiver pin copy itself, not evidence that another broad LocalSSA rewrite is
safe. Reopening this lane would duplicate an existing seam and risk turning a
tiny receiver-only rule into general copy coalescing.

## Boundary

```text
closed:
  receiver_lane_closed=1
  prior CFG-stable receiver keeper
  LocalSSA broad coalescing
  arg forwarding until explicit arg policy selection

open next:
  call_operand_arg_residue_policy_selection
```

## Stop Line

```text
do not implement from this design row
do not patch LocalSSA::ensure_fallback_copy
do not add another same-block receiver seam
do not reopen the previous CFG-stable receiver keeper
do not open arg forwarding without a dedicated arg policy-selection row
do not special-case source names, helper names, or benchmark names
do not change PHI lifecycle or freshness contracts
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001:
  decide whether the 11 arg route-carrier copies have any safe narrow owner, or
  whether call-operand optimization should close and return to fresh owner
  selection
```
