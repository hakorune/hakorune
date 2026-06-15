---
Status: Landed
Date: 2026-06-15
Task: POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001
Scope: Refresh the post-measurement owner after 296x-746 removed the
  MIR-call-result CompareOperand copy family and 296x-747 measured no body-time
  winner.
Related:
  - docs/development/current/main/phases/phase-296x/296x-747-MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001.md
  - tools/allocator/hako_mimalloc_post_mir_call_compare_operand_forwarding_owner_refresh.py
  - tools/allocator/mir_callsite_copy_attribution.py
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001

## Result

```text
output_contract=hako-mimalloc-post-mir-call-compare-operand-forwarding-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-747
probe_tmpdir=/tmp/hakorune_row748_owner.eIJCQY
hako_body_elapsed_ns=365000000
c_body_elapsed_ns=3727908
body_elapsed_ratio=97.910
copy_count=51
local_ssa_copy_materialization_copy_count=20
phi_edge_copy_count=18
block_entry_copy_count=10
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
expression_materialization_copy_count=1
mir_call_origin_copy_count=0
const_origin_copy_count=1
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_position=phi_edge
dominant_local_like_position=block_entry
dominant_expression_origin=const
selected_next_owner=local_ssa_block_entry_phi_edge_copy_family
selected_owner_confidence=medium
selected_reason=compare_operand_family_removed_and_residue_moved_to_block_entry_phi_edge_copies
next_task=local_ssa_block_entry_phi_edge_copy_design
implementation_allowed=0
design_required=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The CompareOperand family selected by 296x-745/746 is gone:

```text
compare_operand_route_carrier_copy_count=0
mir_call_origin_copy_count=0
```

The remaining visible family is no longer a direct compare-operand emission
seam. It is now a structural LocalSSA boundary:

```text
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_position=phi_edge
dominant_local_like_position=block_entry
phi_edge_copy_count=18
block_entry_copy_count=10
```

This is a design timing. Implementation remains closed until the next row
defines a narrow, guardable policy for block-entry / PHI-edge copies.

## Stop Line

```text
do not patch LocalSSA broadly
do not elide PHI-edge copies without a PHI lifecycle contract
do not change CFG, PHI placement, or block structure
do not use benchmark/helper-name branches
do not change product runtime/provider behavior
```

## Next

```text
LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001:
  classify block-entry vs PHI-edge copies
  decide whether any copy family is safe and narrow enough to optimize
  define post target and guard surface before code changes
```
