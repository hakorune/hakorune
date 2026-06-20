# Trim Helper Carrier Lifecycle Inventory

Status: inventory
Scope: `CarrierInfo.trim_helper` lifecycle ownership in the Rust-to-Hako
lifecycle lane.

## Purpose

`trim_helper` is route-specific metadata carried inside `CarrierInfo`.

It is not owned by generic `VariableContext` snapshot plans and must not become
a generic resolver or emitter dependency until its producer and consumer
boundary is explicit.

## Source Entrypoints

```text
src/mir/join_ir/lowering/carrier_info/types.rs
src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
src/mir/loop_route_detection/support/body_local/carrier.rs
src/mir/loop_route_detection/support/body_local/condition.rs
src/mir/loop_route_detection/support/trim.rs
```

## Data Owner

Definition:

```rust
pub trim_helper: Option<TrimLoopHelper>
```

Payload:

```text
TrimLoopHelper.original_var
TrimLoopHelper.carrier_name
TrimLoopHelper.whitespace_chars
```

Meaning:

```text
route-specific trim promotion metadata
```

It is not a generic carrier lifecycle fact.

## Producers

Generic carrier constructors:

```text
CarrierInfo::from_variable_map:
  trim_helper=None

CarrierInfo::with_explicit_carriers:
  trim_helper=None

CarrierInfo::with_carriers:
  trim_helper=None
```

Trim route producer:

```text
TrimRouteInfo::to_carrier_info:
  carrier_info.trim_helper = Some(TrimLoopHelper::from_route_info(self))
  promoted_body_locals.push(original_var)
```

Condition promotion test evidence:

```text
LoopBodyCondPromoter::try_promote_for_condition:
  can produce CarrierInfo with trim_helper=Some(...)
```

## Merge Behavior

`CarrierInfo::merge_from` copies trim metadata from `other` when present:

```text
if other.trim_helper.is_some():
  self.trim_helper = other.trim_helper.clone()
```

The existing `CarrierInfo::merge_from` lifecycle plan may clone existing
route metadata, but it does not claim the trim route lifecycle owner.

## Consumers

Read-only consumer:

```text
CarrierInfo::trim_helper() -> Option<&TrimLoopHelper>
```

Route support consumer:

```text
body-local condition / trim route support
```

These consumers may observe existing trim metadata. They do not prove the
metadata was produced by a lifecycle-safe route.

## Current Fixture Boundary

Current resolver / verifier / emitter fixtures deny trim ownership:

```text
CarrierInfo.trim_helper.lifecycle_owner:
  DenyUnresolvedBoundary(route_specific_metadata_owner_not_selected)

CarrierInfo::merge_from verifier:
  denied_boundaries includes "trim_helper lifecycle owner"

CarrierInfo::merge_from emitter probe:
  may render clone-as-existing-route-metadata comments only
  must not claim trim_helper lifecycle owner
```

## Decision

```text
trim_helper_lifecycle_owner_selected=0
trim_helper_inventory_only=1
generic_carrier_snapshots_claim_trim=0
merge_from_claims_trim_owner=0
resolver_allows_trim_owner=0
emitter_claims_trim_owner=0
```

## Next Candidate Rows

```text
TRIM-HELPER-CARRIER-LIFECYCLE-PROBE-001:
  fixture-guard TrimRouteInfo::to_carrier_info as the trim metadata producer

PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001:
  separate because trim helper production also pushes promoted_body_locals

EMITTER-PARSER-MIR-CHECKABLE-SURFACE-001:
  separate because emitter acceptance must not imply trim ownership
```

## Stop Lines

```text
do not merge trim_helper with promoted_body_locals in this inventory
do not promote CarrierInfo::merge_from to trim lifecycle owner
do not add resolver Allow for trim_helper
do not add emitter positive trim ownership claim
do not modify Rust behavior in this inventory row
```

