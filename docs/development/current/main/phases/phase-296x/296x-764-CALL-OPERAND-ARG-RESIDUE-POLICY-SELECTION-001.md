---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001
Scope: Decide whether the remaining 11 arg route-carrier copies have any safe
  narrow owner, after receiver residue was closed.
Related:
  - docs/development/current/main/phases/phase-296x/296x-763-CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-761-CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-685-CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md
---

# CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001

## Result

```text
output_contract=hako-mimalloc-call-operand-arg-residue-policy-selection-v0
source_evidence=296x-763,296x-761,296x-685,296x-691
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
call_operand_arg_route_carrier_copy_count=11
arg_acquire_usize_copy_count=3
arg_record_failure_after_selected_page_copy_count=5
arg_record_failure_no_selection_copy_count=2
arg_record_small_alloc_success_copy_count=1
prior_arg_same_block_root_candidate_count=7
prior_safe_arg_candidate_count=1
prior_rejected_arg_forwarding_count=9
arg_forwarding_enabled=0
selected_owner=none
selected_owner_reason=arg_residue_spans_size_and_result_value_arguments_without_a_single_safe_receiver_like_owner
call_operand_lane_closed=1
selected_next_action=call_operand_route_carrier_closeout_and_fresh_owner_selection
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

## Evidence

The 11 arg route-carrier copies are not one uniform owner surface:

```text
arg_acquire_usize_copy_count=3
arg_record_failure_after_selected_page_copy_count=5
arg_record_failure_no_selection_copy_count=2
arg_record_small_alloc_success_copy_count=1
```

They include request-size arguments and result-recording arguments. Earlier
rows intentionally rejected arg forwarding:

```text
prior_arg_same_block_root_candidate_count=7
prior_safe_arg_candidate_count=1
prior_rejected_arg_forwarding_count=9
arg_forwarding_enabled=0
```

## Decision

No safe narrow arg owner is selected. Unlike receiver materialization, arg
forwarding would cross value-passing semantics and result object recording
surfaces. Opening it here would broaden LocalSSA copy coalescing rather than
fix a single owner.

This closes the call-operand route-carrier lane and returns to fresh owner
selection.

## Stop Line

```text
do not implement from this policy row
do not patch LocalSSA::ensure_fallback_copy
do not open arg forwarding
do not add helper-name / source-name / benchmark-name special cases
do not reopen receiver lanes
do not change PHI lifecycle or freshness contracts
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001:
  close the call-operand route-carrier lane and select the next owner from fresh
  body/MIR evidence
```
