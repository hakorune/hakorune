---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001
Scope: Rerun object-lifecycle body timing after the LocalSSA block-entry /
  PHI-edge family closed as a non-keeper, then select the next owner without
  opening implementation prematurely.
Related:
  - docs/development/current/main/phases/phase-296x/296x-752-FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001.md
  - docs/development/current/main/phases/phase-296x/296x-751-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001

## Result

```text
output_contract=hako-mimalloc-body-timing-rebaseline-after-local-ssa-nonkeeper-v0
source_evidence=296x-751,296x-752
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
measurement_tmpdir=/tmp/hakorune_row753_rebaseline.52TJFW
measurement_pair_report=/tmp/hakorune_row753_rebaseline.52TJFW/pair.out
taxonomy_report=/tmp/hakorune_row753_rebaseline.52TJFW/taxonomy.out
attribution_report=/tmp/hakorune_row753_rebaseline.52TJFW/attribution.out
mir_owner_report=/tmp/hakorune_row753_rebaseline.52TJFW/mir-owner.out
dynamic_weight_report=/tmp/hakorune_row753_rebaseline.52TJFW/dynamic-weight.out
position_report=/tmp/hakorune_row753_rebaseline.52TJFW/position.out
copy_kind_policy_report=/tmp/hakorune_row753_rebaseline.52TJFW/copy-kind-policy.out
expression_origin_report=/tmp/hakorune_row753_rebaseline.52TJFW/expression-origin.out
hako_body_elapsed_ns=374000000
c_body_elapsed_ns=4699328
body_elapsed_ratio=79.586
gap_owner=compiler_lowering
gap_confidence=medium
selected_mir_body_owner=local_ssa_copy_materialization
selected_owner_confidence=high
dominant_dynamic_owner=local_ssa_copy_materialization
copy_count=51
local_ssa_copy_materialization_copy_count=20
phi_edge_copy_count=18
block_entry_copy_count=10
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
expression_materialization_copy_count=1
dominant_position=phi_edge
dominant_local_like_position=block_entry
dominant_expression_origin=const
mir_call_origin_copy_count=0
const_origin_copy_count=1
closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family
closed_nonkeeper_safe_candidate_count=0
fresh_high_confidence_implementation_owner_selected=0
selected_next=PHI-LIFECYCLE-FRESHNESS-DESIGN-001
selected_next_reason=body_gap_large_but_dominant_residue_requires_phi_lifecycle_and_block_entry_freshness_design
implementation_allowed=0
design_required=1
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

The rebaseline still shows a large product-route body gap:

```text
body_elapsed_ratio=79.586
gap_owner=compiler_lowering
gap_confidence=medium
```

The fresh MIR evidence does not open a narrow implementation seam. The dominant
residue points back to the family closed in 296x-751:

```text
dominant_position=phi_edge
dominant_local_like_position=block_entry
phi_edge_copy_count=18
block_entry_copy_count=10
closed_nonkeeper_safe_candidate_count=0
```

The only expression-materialization copy is a const value with no MIR-call
origin:

```text
expression_materialization_copy_count=1
dominant_expression_origin=const
mir_call_origin_copy_count=0
```

So the correct next move is not another LocalSSA patch. The optimization lane
has reached a design boundary: PHI lifecycle and block-entry freshness must be
specified before any further copy reduction is safe.

## Stop Line

```text
do not patch LocalSSA::ensure_fallback_copy from this measurement
do not reopen PHI-edge copy removal without PHI lifecycle ownership
do not optimize block-entry copies without freshness proof
do not implement const expression cleanup as a body-time keeper
do not reopen startup optimization
do not move Box/Object management into MIRBuilder
do not change source .hako, product defaults, provider activation, replacement,
hooks, or global allocator
```

## Next

```text
PHI-LIFECYCLE-FRESHNESS-DESIGN-001:
  define the PHI lifecycle / block-entry freshness boundary
  decide what evidence is required before any PHI-edge or block-entry copy
  optimization can reopen
  keep implementation closed until the design owner is explicit
```
