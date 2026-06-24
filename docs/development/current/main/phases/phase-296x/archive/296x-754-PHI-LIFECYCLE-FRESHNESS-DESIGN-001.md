---
Status: Landed
Date: 2026-06-15
Task: PHI-LIFECYCLE-FRESHNESS-DESIGN-001
Scope: Fix the PHI lifecycle / block-entry freshness boundary before reopening
  any PHI-edge or block-entry copy optimization after the LocalSSA
  no-safe-candidate closeout.
Related:
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
  - docs/development/current/main/phases/phase-296x/296x-751-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/phi-input-strategy-ssot.md
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - src/mir/builder/emission/phi_lifecycle.rs
---

# PHI-LIFECYCLE-FRESHNESS-DESIGN-001

## Result

```text
output_contract=hako-mimalloc-phi-lifecycle-freshness-design-v0
source_evidence=296x-753
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
body_elapsed_ratio=79.586
gap_owner=compiler_lowering
gap_confidence=medium
selected_mir_body_owner=local_ssa_copy_materialization
dominant_position=phi_edge
dominant_local_like_position=block_entry
phi_edge_copy_count=18
block_entry_copy_count=10
block_entry_route_none_count=7
block_entry_route_carrier_count=3
closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family
closed_nonkeeper_safe_candidate_count=0
phi_lifecycle_truth_owner=src/mir/builder/emission/phi_lifecycle.rs
phi_lifecycle_contract=Reserve_Define_Populate_Finalize
variable_map_role=defined_value_emission_cache
local_ssa_role=block_local_operand_materialization
freshness_truth_owner_required=1
block_entry_copy_reopen_requires_freshness_proof=1
phi_edge_copy_reopen_requires_phi_lifecycle_proof=1
route_carrier_copy_reopen_requires_route_specific_operand_policy=1
broad_local_ssa_coalescing_allowed=0
implementation_allowed=0
design_required=1
next_task=PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001
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

The body gap is still compiler-lowering-visible, but the residue is no longer a
safe LocalSSA keeper:

```text
dominant_position=phi_edge
dominant_local_like_position=block_entry
phi_edge_copy_count=18
block_entry_copy_count=10
closed_nonkeeper_safe_candidate_count=0
```

This row therefore fixes the design boundary and keeps implementation closed.
The next optimization must prove the relevant owner before changing emission.

## Responsibility Boundary

```text
PHI lifecycle:
  owner:
    src/mir/builder/emission/phi_lifecycle.rs
  owns:
    Reserve -> Define -> Populate -> Finalize
    provisional PHI transaction boundaries
    PHI insert / patch fail-fast
  does not own:
    block-local operand materialization policy
    route-specific call/compare operand forwarding

variable_map:
  role:
    defined-value emission cache
  invariant:
    may only expose Defined ValueIds
  not:
    freshness repair owner
    PHI predecessor remap owner

LocalSSA:
  role:
    block-local operand materialization
  may:
    ensure an operand exists for the current block
  must not:
    repair PHI-edge freshness
    infer predecessor remap policy
    globally coalesce block-entry copies
```

## Reopen Conditions

PHI-edge copy optimization can reopen only after all of these are true:

```text
phi_lifecycle_truth_owner=src/mir/builder/emission/phi_lifecycle.rs
phi_edge_rewrite_uses_phi_lifecycle=1
phi_edge_rewrite_preserves_phi_inputs=1
cfg_semantics_changed=0
variable_map_defined_only_invariant_preserved=1
```

Block-entry route-none copy optimization can reopen only after all of these are
true:

```text
freshness_truth_owner_required=1
block_entry_freshness_proof_available=1
variable_map_defined_only_invariant_preserved=1
phi_predecessor_remap_safe=1
block_entry_copy_reopen_requires_freshness_proof=1
```

Route-carrier block-entry optimization can reopen only as a route-specific
operand policy:

```text
route_carrier_copy_reopen_requires_route_specific_operand_policy=1
arg_forwarding_enabled=0 unless explicitly proven
helper_name_special_case=0
benchmark_name_branch_count=0
```

## Stop Line

```text
do not patch LocalSSA::ensure_fallback_copy for PHI-edge copies
do not add broad LocalSSA copy coalescing
do not use variable_map as a freshness repair surface
do not mutate PHI inputs outside phi_lifecycle
do not change CFG semantics for this optimization row
do not reopen the closed nonkeeper family without freshness / PHI lifecycle proof
do not change source .hako, product defaults, provider activation, replacement,
hooks, or global allocator
```

## Next

```text
PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001:
  add a guard/report surface that checks the reopen conditions above before any
  PHI-edge or block-entry copy optimization is attempted
  keep implementation closed until the guard identifies a single safe owner
```
