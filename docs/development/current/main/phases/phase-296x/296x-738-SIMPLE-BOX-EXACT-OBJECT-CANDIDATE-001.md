---
Status: Landed / Parked
Date: 2026-06-15
Task: SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001
Scope: Decide whether a simple non-escaping box may enter the shared
  aggregate/object backend substrate now.
Related:
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md
---

# SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001

## Result

```text
output_contract=hako-simple-box-exact-object-candidate-v0
source_evidence=296x-731
record_box_surface_model=two_surface_one_substrate
simple_box_exact_object_candidate_allowed=1
fresh_high_confidence_owner_evidence=0
implementation_allowed=0
record_semantics_used_as_box_proof=0
object_storage_plan_required=1
routeplan_required=1
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002
summary=parked
```

## Decision

Simple non-escaping boxes are allowed to use the same backend substrate as
records only after fresh owner evidence selects that boundary.

The previous exact-object pilot reached the ObjectStoragePlan route but did not
produce a keeper:

```text
body_elapsed_ratio_before=114.326
body_elapsed_ratio_after=117.038
winner_claim=0
keeper_claim=0
```

Therefore this row does not open another implementation.

## Stop Line

```text
do not use record semantics as proof for boxes
do not start another ObjectStoragePlan implementation without fresh owner
evidence
do not move Box management into MIRBuilder
do not claim global Arc / HostHandle retirement
```
