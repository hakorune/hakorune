---
Status: Landed
Date: 2026-06-15
Task: LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001
Scope: Design the next step after 296x-748 selected the
  local_ssa_block_entry_phi_edge_copy_family residue.
Related:
  - docs/development/current/main/phases/phase-296x/296x-748-POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001.md
  - docs/development/current/main/design/copy-emission-ssot.md
  - src/mir/builder/ssa/phi_input_materializer.rs
  - src/mir/builder/ssa/phi_input_contract.rs
---

# LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001

## Result

```text
output_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-design-v0
source_evidence=296x-748
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_next_owner=local_ssa_block_entry_phi_edge_copy_family
selected_owner_confidence=medium
copy_count=51
local_ssa_copy_materialization_copy_count=20
phi_edge_copy_count=18
block_entry_copy_count=10
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_position=phi_edge
dominant_local_like_position=block_entry
selected_design=split_inventory_before_implementation
phi_edge_optimization_allowed=0
block_entry_optimization_allowed=0
local_ssa_broad_copy_coalescing_allowed=0
phi_lifecycle_changed=0
cfg_changed=0
copy_emission_ssot_preserved=1
candidate_probe_required=1
next_task=local_ssa_block_entry_phi_edge_copy_candidate_probe
implementation_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

This family must be split before implementation:

```text
phi_edge:
  owned by PHI lifecycle / edge-value materialization
  not safe for allocator-lane peephole removal

block_entry:
  may contain ordinary freshness copies, route-carrier copies, and
  binding/PHI freshness copies
  not safe for broad LocalSSA coalescing
```

The next step is a candidate probe, not an implementation. The probe must
separate at least these buckets:

```text
phi_edge:
  closed unless a PHI lifecycle contract explicitly owns the change

block_entry_route_none:
  possible candidate only if producer dominates all uses and no variable_map /
  PHI freshness boundary is crossed

block_entry_route_carrier:
  closed unless the route owner provides a direct operand/field plan

block_entry_field_set_value:
  closed unless field-set value owner provides a direct policy
```

## Why Implementation Is Closed

The current evidence says the compare-operand family is gone:

```text
compare_operand_route_carrier_copy_count=0
mir_call_origin_copy_count=0
```

The remaining residue is structural:

```text
dominant_position=phi_edge
dominant_local_like_position=block_entry
```

Patching `LocalSSA::ensure_fallback_copy` broadly here would mix allocator
optimization with PHI lifecycle, variable freshness, and route-carrier policy.
That is exactly the local-patch failure mode this lane is avoiding.

## Stop Line

```text
do not remove PHI-edge copies in this row
do not change PHI input materialization or PHI lifecycle
do not change CFG, block structure, variable_map, or block sealing
do not add broad LocalSSA copy coalescing
do not add benchmark/helper-name branches
do not change product runtime/provider behavior
```

## Next

```text
LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-CANDIDATE-PROBE-001:
  inventory block-entry and PHI-edge copies by role
  report safe_candidate_count
  keep implementation closed unless safe_candidate_count > 0 and a narrow
  guard surface can be written
```
