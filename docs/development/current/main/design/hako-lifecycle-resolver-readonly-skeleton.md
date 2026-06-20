# Hako Lifecycle Resolver Read-Only Skeleton

Status: SSOT
Scope: diagnostic-only lifecycle resolver skeleton for existing fixtures.

## Purpose

Start the resolver lane without making the resolver a compiler selection
owner, converter owner, or backend owner.

The skeleton reads frozen lifecycle fixture evidence and reports
`Allow(plan)` / `Deny(reason)` decisions for the fixture family only.

## Inputs

```text
RustLifecycleFacts-v0 fixtures
HakoLifecyclePlan-v0 fixtures
known unresolved lifecycle boundaries
```

Initial fixture family:

```text
BindingContext
VariableContext simple map
VariableContext immutable BorrowView
VariableContext snapshot/restore
CarrierInfo::from_variable_map
CarrierInfo::with_explicit_carriers
CarrierInfo::merge_from
```

## Outputs

```text
diagnostic decisions only
no MIR changes
no .hako emission
no backend route selection
no verifier promotion
```

Decision vocabulary:

```text
AllowPlan:
  source facts exist
  plan exists
  plan behavior claims stay passive
  required stop-line flags are not violated

DenyUnresolvedBoundary:
  boundary is known unresolved

DenyMissingFixture:
  source facts or plan fixture missing

DenyUnsafeClaim:
  plan claims resolver/emitter/PHI/whole-MirBuilder behavior too early
```

## Initial Allow Set

```text
BindingContext:
  OrderedMapBox plan fixtures exist

VariableContext simple map:
  simple map plan fixtures exist

VariableContext immutable BorrowView:
  BorrowView plan fixtures exist

VariableContext snapshot/restore:
  CloneOwnedMap / ReplaceOwned fixtures exist

CarrierInfo snapshots:
  CarrierSnapshotFromBorrowView and ExplicitCarrierSnapshotFromBorrowView

CarrierInfo merge_from:
  OwnedCarrierInfoMerge
```

## Initial Deny Set

```text
CarrierVar.join_id production lifecycle:
  DenyUnresolvedBoundary
  reason=no production Some(ValueId) producer

trim_helper lifecycle owner:
  DenyUnresolvedBoundary
  reason=route-specific metadata owner not selected

promoted_body_locals lifecycle owner:
  DenyUnresolvedBoundary
  reason=promotion owner not selected
```

## Stop Lines

```text
resolver_selection_owner=0
converter_emission_added=0
backend_behavior_changed=0
verifier_promotion=0
join_id_dependent_paths_allowed=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```
