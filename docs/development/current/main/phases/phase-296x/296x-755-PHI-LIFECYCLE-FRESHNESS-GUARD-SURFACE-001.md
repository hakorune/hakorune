---
Status: Landed
Date: 2026-06-15
Task: PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001
Scope: Freeze the guard/report surface required before PHI-edge or block-entry
  copy optimization can reopen after the LocalSSA no-safe-candidate closeout.
Related:
  - docs/development/current/main/phases/phase-296x/296x-754-PHI-LIFECYCLE-FRESHNESS-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/local-patch-prevention-ssot.md
---

# PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001

## Result

```text
output_contract=hako-mimalloc-phi-lifecycle-freshness-guard-surface-v0
input_contract=hako-mimalloc-phi-lifecycle-freshness-design-v0
source_evidence=296x-754
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
body_elapsed_ratio=79.586
dominant_position=phi_edge
dominant_local_like_position=block_entry
phi_edge_copy_count=18
block_entry_copy_count=10
block_entry_route_none_count=7
block_entry_route_carrier_count=3
closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family
closed_nonkeeper_safe_candidate_count=0
guard_surface_defined=1
phi_edge_reopen_gate=phi_lifecycle_proof
block_entry_route_none_reopen_gate=freshness_proof
block_entry_route_carrier_reopen_gate=route_specific_operand_policy
variable_map_defined_only_guard_required=1
phi_lifecycle_mutation_guard_required=1
local_ssa_broad_coalescing_guard_required=1
cfg_semantics_changed_allowed=0
freshness_proof_available=0
phi_lifecycle_rewrite_proof_available=0
route_specific_operand_policy_available=0
implementation_allowed=0
next_task=PHI-LIFECYCLE-FRESHNESS-INVENTORY-001
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

This row adds no optimizer behavior. It fixes the guard vocabulary that must be
present before the PHI-edge / block-entry family can become an implementation
candidate again.

The previous candidate probe and closeout proved:

```text
safe_candidate_count=0
phi_edge_copy_count=18
block_entry_copy_count=10
block_entry_route_none_count=7
block_entry_route_carrier_count=3
```

The active guard surface therefore has three independent gates:

```text
phi_edge:
  gate=phi_lifecycle_proof
  requires:
    PHI mutation is routed through phi_lifecycle
    PHI inputs are preserved
    cfg_semantics_changed=0

block_entry_route_none:
  gate=freshness_proof
  requires:
    variable_map remains Defined-only
    predecessor remap is explicit
    freshness owner is named before rewrite

block_entry_route_carrier:
  gate=route_specific_operand_policy
  requires:
    route-specific policy card
    no broad LocalSSA coalescing
    no helper-name or benchmark-name special case
```

## Guard Vocabulary

Future inventory/probe rows must emit these fields before implementation can
open:

```text
phi_edge_reopen_gate=phi_lifecycle_proof
phi_edge_rewrite_uses_phi_lifecycle=0|1
phi_edge_rewrite_preserves_phi_inputs=0|1
phi_lifecycle_mutation_guard_required=1

block_entry_route_none_reopen_gate=freshness_proof
block_entry_freshness_proof_available=0|1
variable_map_defined_only_invariant_preserved=0|1
phi_predecessor_remap_safe=0|1

block_entry_route_carrier_reopen_gate=route_specific_operand_policy
route_specific_operand_policy_available=0|1
arg_forwarding_enabled=0|1
helper_name_special_case=0
benchmark_name_branch_count=0

local_ssa_broad_coalescing_guard_required=1
broad_local_ssa_coalescing_allowed=0
implementation_allowed=0|1
```

## Open Conditions

Implementation remains closed unless a future inventory row proves exactly one
of these cases:

```text
case=phi_edge
phi_edge_rewrite_uses_phi_lifecycle=1
phi_edge_rewrite_preserves_phi_inputs=1
variable_map_defined_only_invariant_preserved=1
cfg_semantics_changed=0
implementation_owner=phi_lifecycle

case=block_entry_route_none
block_entry_freshness_proof_available=1
phi_predecessor_remap_safe=1
variable_map_defined_only_invariant_preserved=1
implementation_owner=<freshness_owner>

case=block_entry_route_carrier
route_specific_operand_policy_available=1
helper_name_special_case=0
benchmark_name_branch_count=0
arg_forwarding_enabled=0 unless explicitly proven
implementation_owner=<route_policy_owner>
```

## Stop Line

```text
do not implement from this guard-surface row
do not patch LocalSSA::ensure_fallback_copy
do not add broad LocalSSA copy coalescing
do not use variable_map as a freshness repair owner
do not mutate PHI inputs outside phi_lifecycle
do not claim a winner from guard vocabulary
```

## Next

```text
PHI-LIFECYCLE-FRESHNESS-INVENTORY-001:
  inventory the current PHI-edge / block-entry copies against the guard
  vocabulary above and select either:
    - one proven owner with implementation_allowed=1, or
    - no owner with implementation_allowed=0
```
