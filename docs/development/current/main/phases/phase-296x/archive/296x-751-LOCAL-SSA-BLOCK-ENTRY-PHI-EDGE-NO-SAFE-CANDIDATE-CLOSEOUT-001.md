---
Status: Landed
Date: 2026-06-15
Task: LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001
Scope: Close the LocalSSA block-entry / PHI-edge copy family as a no-safe-candidate
  non-keeper before selecting another optimization owner.
Related:
  - docs/development/current/main/phases/phase-296x/296x-750-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-CANDIDATE-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-749-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001.md
---

# LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001

## Result

```text
output_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-no-safe-candidate-closeout-v0
input_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-candidate-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-750
copy_count=51
phi_edge_copy_count=18
block_entry_copy_count=10
block_entry_route_none_count=7
block_entry_route_carrier_count=3
safe_candidate_count=0
selected_policy=none
family_closed_as_non_keeper=1
phi_edge_owner_required=phi_lifecycle
block_entry_route_carrier_owner_required=route_specific_operand_policy
block_entry_route_none_owner_required=freshness_proof
phi_edge_optimization_allowed=0
block_entry_route_carrier_optimization_allowed=0
block_entry_route_none_optimization_allowed=0
local_ssa_broad_copy_coalescing_allowed=0
freshness_proof_available=0
phi_lifecycle_changed=0
cfg_changed=0
copy_emission_ssot_preserved=1
next_task=fresh_owner_selection_after_local_ssa_no_safe_candidate
implementation_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The LocalSSA block-entry / PHI-edge copy family is closed as a non-keeper for
this optimization lane. 296x-750 found no safe implementation subset:

```text
safe_candidate_count=0
selected_policy=none
```

The remaining buckets are not owned by a narrow LocalSSA optimization:

```text
phi_edge:
  requires PHI lifecycle ownership.

block_entry_route_carrier:
  requires route-specific operand / field policy ownership.

block_entry_route_none:
  requires explicit freshness proof across variable_map / block-entry / PHI
  boundaries before it can be reopened.
```

## Stop Line

```text
do not patch LocalSSA::ensure_fallback_copy for this family
do not remove PHI-edge copies from this optimization lane
do not optimize block-entry route-none copies without freshness proof
do not introduce broad LocalSSA copy coalescing
do not change CFG, PHI lifecycle, variable_map, or product runtime behavior
```

## Next

```text
FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001:
  return to owner-first evidence
  select a fresh optimization owner or pause
  keep implementation closed until a new high-confidence owner is selected
```
