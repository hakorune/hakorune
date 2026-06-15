---
Status: Landed
Date: 2026-06-15
Task: FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001
Scope: Select the next optimization action after the LocalSSA block-entry /
  PHI-edge copy family closed with no safe candidate.
Related:
  - docs/development/current/main/phases/phase-296x/296x-751-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/296x-747-MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001

## Result

```text
output_contract=hako-mimalloc-fresh-owner-selection-after-local-ssa-no-safe-candidate-v0
source_evidence=296x-747,296x-748,296x-751
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
last_measured_hako_body_elapsed_ns=365000000
last_measured_c_body_elapsed_ns=3727908
last_measured_body_elapsed_ratio=97.910
closed_family=local_ssa_block_entry_phi_edge_copy_family
closed_family_safe_candidate_count=0
closed_family_selected_policy=none
closed_family_implementation_allowed=0
fresh_high_confidence_owner_selected=0
selected_next_action=body_timing_rebaseline_after_local_ssa_nonkeeper
selected_next_action_reason=last_measurement_precedes_no_safe_candidate_closeout_and_current_owner_is_closed
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

Do not start another implementation row from the current copy-materialization
evidence. The latest implementation-adjacent owner was closed as a
no-safe-candidate family:

```text
closed_family=local_ssa_block_entry_phi_edge_copy_family
closed_family_safe_candidate_count=0
closed_family_selected_policy=none
```

The last body-timing measurement is still useful as context, but it predates
the no-safe-candidate closeout. The next aligned action is a fresh body-timing
rebaseline and owner selection, not another LocalSSA patch.

## Stop Line

```text
do not implement another LocalSSA copy rewrite from stale evidence
do not reopen PHI-edge or block-entry copy optimization without a new owner card
do not reopen startup optimization
do not move Box/Object management into MIRBuilder
do not change source .hako, product defaults, provider activation, replacement,
hooks, or global allocator
```

## Next

```text
MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001:
  rerun the product-route object-lifecycle body timing pair
  refresh MIR/runtime owner evidence from the new measurement
  keep implementation closed until a fresh high-confidence owner appears
```
