---
Status: Landed
Date: 2026-06-15
Task: LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-CANDIDATE-PROBE-001
Scope: Inventory block-entry and PHI-edge copies after 296x-749 and decide
  whether any narrow implementation candidate exists.
Related:
  - docs/development/current/main/phases/phase-296x/296x-749-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001.md
  - tools/allocator/hako_mimalloc_local_ssa_block_entry_phi_edge_copy_candidate_probe.py
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-CANDIDATE-PROBE-001

## Result

```text
output_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-candidate-probe-v0
input_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-design-v0+hako-mimalloc-local-ssa-copy-position-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-749
probe_tmpdir=/tmp/hakorune_row750_probe.yOU6vK
copy_count=51
phi_edge_copy_count=18
block_entry_copy_count=10
block_entry_route_none_count=7
block_entry_route_carrier_count=3
block_entry_field_set_value_count=2
block_entry_field_base_count=1
block_entry_call_operand_count=0
phi_edge_route_none_count=10
phi_edge_route_carrier_count=8
safe_candidate_count=0
selected_policy=none
phi_edge_optimization_allowed=0
block_entry_route_carrier_optimization_allowed=0
block_entry_route_none_optimization_allowed=0
freshness_proof_available=0
phi_lifecycle_changed=0
cfg_changed=0
copy_emission_ssot_preserved=1
next_task=local_ssa_block_entry_phi_edge_no_safe_candidate_closeout
implementation_allowed=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

No local implementation candidate is open:

```text
safe_candidate_count=0
selected_policy=none
```

Reasons:

```text
phi_edge:
  closed. Owned by PHI lifecycle / edge-value materialization.

block_entry_route_carrier:
  closed. Owned by route-specific operand/field policy.

block_entry_route_none:
  closed for this row. It still lacks freshness proof across variable_map /
  block-entry / PHI boundaries.
```

This means the current LocalSSA block-entry / PHI-edge residue should be
closed as a non-keeper for this optimization lane. Do not patch
`LocalSSA::ensure_fallback_copy` for this family.

## Stop Line

```text
do not implement a LocalSSA patch from this probe
do not remove PHI-edge copies
do not optimize block-entry route-none copies without freshness proof
do not add broad LocalSSA coalescing
do not change CFG, PHI lifecycle, variable_map, or product runtime behavior
```

## Next

```text
LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001:
  close this owner family as no-safe-candidate
  select the next owner from current evidence before any implementation
```
