---
Status: Landed
Date: 2026-06-15
Task: ROUTE-CARRIER-RESIDUAL-INVENTORY-001
Scope: Classify the residual route-carrier copies after the PHI freshness
  family remained closed, and select the next route-specific policy row without
  opening implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-757-MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-AFTER-PHI-FRESHNESS-NO-OWNER-001.md
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# ROUTE-CARRIER-RESIDUAL-INVENTORY-001

## Result

```text
output_contract=hako-mimalloc-route-carrier-residual-inventory-v0
source_evidence=296x-757,296x-753
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
body_elapsed_ratio=79.586
copy_count=51
local_ssa_copy_materialization_copy_count=20
closed_phi_freshness_family=local_ssa_block_entry_phi_edge_copy_family
closed_phi_freshness_implementation_allowed=0
route_carrier_residual_copy_count=13
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
block_entry_route_carrier_count=3
phi_edge_route_carrier_count=8
dominant_route_carrier_role=call_operand
selected_role=call_operand
selected_role_confidence=medium
fresh_high_confidence_implementation_owner_selected=0
selected_next_action=call_operand_route_carrier_policy_selection
selected_next_action_reason=call_operand_route_carrier_is_the_only_nonzero_route_carrier_role_after_compare_operand_and_phi_freshness_are_closed
implementation_allowed=0
policy_selection_required=1
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

The route-carrier residue is not a broad LocalSSA optimization target. The
remaining non-zero route-carrier role is call operands:

```text
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
dominant_route_carrier_role=call_operand
```

PHI-edge and block-entry freshness buckets remain closed by the previous
design/inventory rows. This row therefore selects a policy-selection row for
call-operand route carriers, not an implementation row.

## Boundaries

```text
allowed next:
  call_operand_route_carrier_policy_selection

not allowed:
  broad LocalSSA copy coalescing
  PHI freshness family reopen
  compare operand forwarding retry from zero candidates
  helper-name / benchmark-name special case
```

## Stop Line

```text
do not implement from this inventory row
do not patch LocalSSA::ensure_fallback_copy
do not reopen PHI-edge / block-entry freshness copies
do not alter CFG or PHI lifecycle
do not change source .hako, product defaults, provider activation, replacement,
hooks, or global allocator
```

## Next

```text
CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001:
  choose whether any narrow call-operand route-carrier policy is safe
  keep implementation closed until a specific policy and post target are fixed
```
