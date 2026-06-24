# 296x-1521 POST-VARIABLE-CONTEXT-SIMPLE-MAP-ROUTE-NEXT-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next bounded rust-to-Hako behavioral conversion owner after
BindingContext and VariableContext simple-map have both been selected as
`derived_hako` execution routes.

This row exists to keep the next work implementable by small workers. It must
not jump from the current two selected routes to whole MirBuilder conversion.

## Inputs

```text
296x-1517 BindingContext derived route selection
296x-1520 VariableContext simple-map derived route selection
docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
docs/development/current/main/design/rust-to-hako-ownership-converter-reference.md
tools/checks/rust_lifecycle_variable_context_immutable_borrow_guard.sh
tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh
tools/checks/rust_lifecycle_variable_context_mutable_map_deny_guard.sh
tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh
tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_guard.sh
```

## Worker Findings

VariableContext worker:

```text
next bounded order:
  1. immutable variable_map BorrowView
  2. snapshot/restore ownership
  3. variable_map_mut deny closeout
  4. carrier/PHI lifecycle inventory
  5. CarrierInfo::from_variable_map snapshot
  6. CarrierInfo::with_explicit_carriers snapshot
```

MirBuilder family worker:

```text
BindingContext:
  already selected as DerivedMainline

VariableContext simple-map:
  already selected as derived_hako pilot_scope=VariableContext_simple_map_only

context / core_context / type_context / metadata_context:
  generated-skeleton transport is green, but behavioral conversion is not
  ready. Each lacks lifecycle facts, plan, behavior recipe, oracle vectors,
  derived artifact manifest, and route entry.
```

## Decision

```text
selected_next_owner=VariableContext::variable_map immutable BorrowView
selected_next_card=296x-1522-VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ARTIFACT-PILOT-001
reason=smallest already-fixture-guarded post-simple-map behavior slice
```

The next row must create a generated derived artifact for immutable
`variable_map()` BorrowView only. It must not route-select it in the same row.

## Mini-Model Task Ladder

Use this ladder for small-worker implementation. One row means one commit-sized
slice.

```text
1522:
  VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ARTIFACT-PILOT-001
  Generate checked-in artifact + manifest for variable_map() immutable
  BorrowView only. Keep mainline_selected=0.

1523:
  VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ROUTE-SELECTION-001
  Add route manifest entry for pilot_scope=VariableContext_immutable_borrow_only
  if 1522 guard is green.

1524:
  VARIABLE-CONTEXT-SNAPSHOT-RESTORE-DERIVED-ARTIFACT-PILOT-001
  Generate checked-in artifact + manifest for snapshot()/restore() only.
  Keep ReplaceOwned/CloneOwnedMap scope explicit.

1525:
  VARIABLE-CONTEXT-SNAPSHOT-RESTORE-DERIVED-ROUTE-SELECTION-001
  Route-select snapshot/restore if artifact guard is green.

1526:
  VARIABLE-CONTEXT-MUTABLE-MAP-DERIVED-DENY-LOCK-001
  Lock variable_map_mut() as Deny(ReturnedMutableBorrow) across all selected
  VariableContext derived routes. No generated behavior.

1527:
  VARIABLE-CONTEXT-CARRIER-DERIVED-ARTIFACT-READINESS-INVENTORY-001
  Inventory carrier-sensitive derived artifact readiness after BorrowView and
  snapshot/restore are selected. No route selection.

1528:
  VARIABLE-CONTEXT-CARRIER-SNAPSHOT-DERIVED-ARTIFACT-PILOT-001
  Generate artifact for CarrierInfo::from_variable_map only.

1529:
  VARIABLE-CONTEXT-CARRIER-SNAPSHOT-DERIVED-ROUTE-SELECTION-001
  Route-select CarrierSnapshotFromBorrowView if guard is green.

1530:
  VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-DERIVED-ARTIFACT-PILOT-001
  Generate artifact for CarrierInfo::with_explicit_carriers only.

1531:
  VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-DERIVED-ROUTE-SELECTION-001
  Route-select ExplicitCarrierSnapshotFromBorrowView if guard is green.

1532:
  MIRBUILDER-NEXT-FAMILY-LIFECYCLE-READINESS-INVENTORY-001
  Re-evaluate context/core_context/type_context/metadata_context as behavior
  candidates. Skeleton transport alone remains insufficient.

1533:
  MIRBUILDER-NEXT-FAMILY-LIFECYCLE-FACTS-PILOT-SELECTION-001
  Select exactly one non-VariableContext family for facts extraction, or keep
  VariableContext as the only active behavioral family if readiness is absent.
```

## Stop Lines

```text
do_not_claim_full_VariableContext=1
do_not_claim_MirBuilder_wide_conversion=1
do_not_promote_skeleton_transport_as_behavior_conversion=1
do_not_mix_artifact_generation_and_route_selection_in_one_row=1
do_not_generate_variable_map_mut_behavior=1
do_not_runtime_fallback_from_Hako_to_Rust=1
```

## Closeout

```text
output_contract=rust-lifecycle-post-variable-context-simple-map-route-next-owner-selection-v0
selected_next_owner=VariableContext::variable_map immutable BorrowView
selected_next_card=296x-1522-VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ARTIFACT-PILOT-001
full_variable_context_claim=0
mirbuilder_wide_claim=0
skeleton_transport_as_behavior_claim=0
summary=ok
```
