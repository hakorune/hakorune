---
Status: Active
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002
Scope: Refresh the object-lifecycle body timing owner after the exact-object
  pilot closeout and decide whether implementation may reopen.
Related:
  - docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/296x-738-SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002

## Result

```text
output_contract=hako-mimalloc-body-timing-fresh-owner-selection-002-v0
source_evidence=296x-731,296x-738,296x-741
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
measurement_tmpdir=/tmp/hakorune_row742_measure.4TIRms
measurement_pair_report=/tmp/hakorune_row742_measure.4TIRms/pair.out
taxonomy_report=/tmp/hakorune_row742_measure.4TIRms/taxonomy.out
attribution_report=/tmp/hakorune_row742_measure.4TIRms/attribution.out
mir_owner_report=/tmp/hakorune_row742_measure.4TIRms/mir-owner.out
dynamic_weight_report=/tmp/hakorune_row742_measure.4TIRms/dynamic-weight.out
position_report=/tmp/hakorune_row742_measure.4TIRms/position.out
copy_kind_policy_report=/tmp/hakorune_row742_measure.4TIRms/copy-kind-policy.out
expression_origin_report=/tmp/hakorune_row742_measure.4TIRms/expression-origin.out
hako_body_elapsed_ns=368000000
c_body_elapsed_ns=4056252
body_elapsed_ratio=90.724
gap_owner=compiler_lowering
gap_confidence=medium
selected_mir_body_owner=local_ssa_copy_materialization
selected_owner_confidence=high
dominant_dynamic_owner=local_ssa_copy_materialization
selected_copy_kind_policy=expression_materialization_copy_policy
dominant_expression_origin=mir_call
dominant_expression_sink=compare_eq
selected_origin_policy=mir_call_expression_value_copy_chain
fresh_narrow_owner=mir_call_selectPage_compare_eq_expression_copy_chain
fresh_narrow_owner_confidence=medium
implementation_allowed=0
selected_next=MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001
winner_claim=0
product_default_changed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
mirbuilder_object_management_enabled=0
object_storage_plan_implementation_allowed=0
summary=ok
```

## Decision

The object-lifecycle body gap is again large enough to require compiler-owner
classification before any implementation. The broad owner is
`local_ssa_copy_materialization`, but the fresh narrow seam is not the previous
field-get or parameter forwarding family.

The current narrow evidence points at a MIR call expression value chain:

```text
selectPage call result
  -> expression materialization copy
  -> compare_eq operand
```

This row does not open implementation. The next row must pin the policy for
MIR-call expression copy chains before code changes.

## Stop Line

```text
do not reopen ObjectStoragePlan from the previous exact-object pilot
do not move Box management into MIRBuilder
do not retry same_block_field_get_only forwarding
do not implement broad LocalSSA copy suppression from this card
do not special-case benchmark names, helper names, or selectPage by name
do not claim a winner from this measurement
```

## Next

```text
MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001:
  classify the mir_call -> compare_eq expression copy chain
  select a narrow policy or keep implementation closed
  keep source, product default, provider activation, and object substrate
  unchanged
```
