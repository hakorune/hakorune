# Promoted Body Locals Lifecycle Inventory

Status: inventory
Scope: `CarrierInfo.promoted_body_locals` ownership in the Rust-to-Hako
lifecycle lane.

## Purpose

`promoted_body_locals` records original loop-body-local variable names that
were promoted into carrier-space metadata.

It is an owned `Vec<String>` inside `CarrierInfo`, but current lifecycle
fixtures still deny it as an independent lifecycle owner. This inventory names
the producers, merge behavior, and consumers before a probe or resolver claims
the owner.

## Source Entrypoints

```text
src/mir/join_ir/lowering/carrier_info/types.rs
src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
src/mir/join_ir/lowering/scope_manager.rs
src/mir/loop_route_detection/support/body_local/carrier.rs
src/mir/loop_route_detection/support/body_local/digitpos.rs
```

## Data Owner

Definition:

```rust
pub promoted_body_locals: Vec<String>
```

Meaning:

```text
original LoopBodyLocal variable names promoted to carrier metadata
```

It is not a join_id producer and not a general route-lowering proof.

## Default Constructors

Generic carrier constructors start empty:

```text
CarrierInfo::from_variable_map:
  promoted_body_locals=Vec::new()

CarrierInfo::with_explicit_carriers:
  promoted_body_locals=Vec::new()

CarrierInfo::with_carriers:
  promoted_body_locals=Vec::new()
```

## Producers

Trim producer:

```text
TrimRouteInfo::to_carrier_info:
  promoted_body_locals.push(self.var_name.clone())
```

DigitPos producer:

```text
DigitPos promotion:
  promoted_body_locals.push(detection.var_name.clone())
```

These producers record names only. They do not produce `join_id`.

## Merge Behavior

`CarrierInfo::merge_from` appends missing promoted names:

```text
for promoted_var in other.promoted_body_locals:
  if !self.promoted_body_locals.contains(promoted_var):
    self.promoted_body_locals.push(promoted_var.clone())
```

This is existing owned-string merge behavior, but it is not a full
promoted-name lifecycle owner claim.

## Consumers

Name-to-carrier lookup:

```text
CarrierInfo::resolve_promoted_join_id(original_name)
```

Scope lookup path:

```text
LoopBreakScopeManager::lookup
  -> CarrierInfo::resolve_promoted_join_id
```

The lookup still depends on a later `join_id` producer and naming convention
for `is_<name>` / `is_<name>_match`.

## Current Fixture Boundary

Current resolver / verifier / emitter fixtures deny promoted-name ownership:

```text
CarrierInfo.promoted_body_locals.lifecycle_owner:
  DenyUnresolvedBoundary(promotion_owner_not_selected)

CarrierInfo::merge_from verifier:
  denied_boundaries includes "promoted_body_locals lifecycle owner"

TrimHelperCarrierProducer:
  records promoted_body_locals but denied promoted_body_locals lifecycle owner
```

## Decision

```text
promoted_body_locals_lifecycle_owner_selected=0
promoted_body_locals_inventory_only=1
default_carrier_snapshots_start_empty=1
trim_producer_records_name=1
digitpos_producer_records_name=1
merge_from_deduplicates_names=1
join_id_producer=0
resolver_allows_promoted_body_locals_owner=0
emitter_claims_promoted_body_locals_owner=0
```

## Next Candidate Rows

```text
PROMOTED-BODY-LOCALS-PRODUCER-PROBE-001:
  fixture-guard trim/digitpos producers as name recorders only

PROMOTED-BODY-LOCALS-RESOLUTION-PROBE-001:
  later, only after join_id producer status changes

EMITTER-PARSER-MIR-CHECKABLE-SURFACE-001:
  separate because emitter acceptance must not imply promoted-name ownership
```

## Stop Lines

```text
do not use promoted_body_locals as join_id evidence
do not merge trim helper payload with promoted-name ownership
do not allow resolver/emitter positive ownership claim in this inventory
do not modify Rust behavior in this inventory row
```

