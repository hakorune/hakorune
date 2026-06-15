---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001
Scope: Close the call-operand route-carrier investigation after receiver and
  arg surfaces selected no new implementation owner, then choose the next
  evidence refresh step.
Related:
  - docs/development/current/main/phases/phase-296x/296x-764-CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-763-CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-761-CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
---

# CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001

## Result

```text
output_contract=hako-mimalloc-call-operand-route-carrier-closeout-and-fresh-owner-selection-v0
source_evidence=296x-764,296x-763,296x-761,296x-753
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
last_body_timing_source=296x-753
last_hako_body_elapsed_ns=374000000
last_c_body_elapsed_ns=4699328
last_body_elapsed_ratio=79.586
copy_count=51
call_operand_route_carrier_copy_count=13
call_operand_receiver_route_carrier_copy_count=2
call_operand_arg_route_carrier_copy_count=11
receiver_lane_closed=1
arg_lane_closed=1
call_operand_route_carrier_lane_closed=1
selected_owner=none
selected_owner_reason=receiver_surface_already_has_landed_seam_and_arg_surface_has_no_single_safe_owner
fresh_high_confidence_implementation_owner_selected=0
selected_next_action=mimalloc_body_timing_rebaseline_after_call_operand_closeout
implementation_allowed=0
measurement_required=1
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

The call-operand route-carrier lane is closed:

```text
receiver_lane_closed=1
arg_lane_closed=1
call_operand_route_carrier_lane_closed=1
selected_owner=none
```

No implementation row opens from this closeout. The latest body timing evidence
is still the pre-closeout rebaseline:

```text
last_body_timing_source=296x-753
last_body_elapsed_ratio=79.586
```

Because the last several rows were design/probe/closeout rows and did not
change runtime behavior, the next aligned step is a fresh body/MIR rebaseline.
That rebaseline must decide whether any new compiler-lowering owner remains or
whether the lane should return to runtime/object/generated-runtime boundary
selection.

## Stop Line

```text
do not implement from this closeout row
do not patch LocalSSA::ensure_fallback_copy
do not reopen receiver or arg forwarding
do not add broad copy coalescing
do not special-case source names, helper names, or benchmark names
do not change PHI lifecycle or freshness contracts
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001:
  rerun body timing and MIR copy/route classification after call-operand
  closeout, then select a fresh owner or return to boundary inventory
```
