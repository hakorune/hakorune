# Trim Route Lowering Inventory

Status: inventory
Scope: trim route lowering boundary after lifecycle producer and emitter
surface probes.

## Purpose

The lifecycle lane has already bounded these pieces:

```text
TrimRouteInfo::to_carrier_info:
  produces trim_helper metadata
  records promoted_body_locals
  does not lower trim routes

CarrierInfo::merge_from:
  can clone existing trim_helper metadata
  does not claim trim lifecycle ownership

Lifecycle emitter surface:
  can render one merge_from verified-plan surface to MIR
  does not emit trim route lowering
```

This document names the remaining seam:

```text
trim route lowering:
  converts proven trim route metadata into executable route logic
```

That seam is not implemented in this row.

## Source Entrypoints

```text
route-shape recognizer:
  src/mir/builder/control_flow/facts/route_shape_recognizers/skip_whitespace.rs

body-local trim detector / promoter:
  src/mir/loop_route_detection/support/body_local/trim_detector.rs
  src/mir/loop_route_detection/support/body_local/carrier.rs
  src/mir/loop_route_detection/support/body_local/condition.rs

trim metadata helper:
  src/mir/loop_route_detection/support/trim.rs

carrier metadata transport:
  src/mir/join_ir/lowering/carrier_info/types.rs
  src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
```

## Existing Producers

`detect_skip_whitespace_shape` observes this AST route shape:

```text
loop(cond) {
  body...
  if check_cond {
    carrier = carrier (+|-) const
  } else {
    break
  }
}
```

`LoopBodyCarrierPromoter::try_promote` delegates trim detection to
`TrimDetector` and returns `TrimRouteInfo`.

`TrimRouteInfo::to_carrier_info` converts route info into carrier metadata:

```text
carrier_info.trim_helper = Some(TrimLoopHelper::from_route_info(self))
carrier_info.promoted_body_locals.push(var_name)
```

## Existing Metadata

`TrimLoopHelper` owns route-local metadata:

```text
original_var
carrier_name
whitespace_chars
```

It also exposes small read-only helpers:

```text
carrier_type() -> "Bool"
initial_value() -> true
whitespace_count()
is_whitespace(ch)
is_safe_trim()
is_trim_like()
has_valid_structure()
```

These helpers are metadata queries. They are not an executable lowering owner.

## Existing Consumers

`CarrierInfo::trim_helper()` is read-only:

```text
CarrierInfo::trim_helper() -> Option<&TrimLoopHelper>
```

`CarrierInfo::merge_from` clones existing trim metadata when present:

```text
if other.trim_helper.is_some():
  self.trim_helper = other.trim_helper.clone()
```

The existing emitter surface may render comments for copying existing
metadata. It does not render executable trim loop logic.

## Denied Boundaries

Promoted-name resolution remains denied until a production join-id producer
exists:

```text
resolve_promoted_join_id:
  depends on promoted_body_locals and join_id-bearing carriers
  no production Some(ValueId) join_id producer exists yet
```

The trim route lowering owner must not be inferred from:

```text
trim_helper presence alone
promoted_body_locals presence alone
helper method names
source variable names
emitter comments
```

## Required Future Owner

A future trim route lowering probe must explicitly decide:

```text
input proof:
  route shape evidence
  trim_helper metadata
  promoted body-local mapping
  join_id / carrier identity proof when needed

output proof:
  executable route decision or Deny(reason)

non-goals:
  backend behavior change in the first proof row
  generated program execution claim
  rustc adapter facts
```

## Decision

```text
trim_route_lowering_inventory=1
trim_route_lowering_owner_selected=0
trim_route_lowering_implemented=0
trim_helper_is_metadata=1
promoted_body_locals_are_name_records=1
promoted_name_resolution_still_denied=1
emitter_surface_does_not_lower_trim=1
backend_behavior_changed=0
```

## Next Candidate Row

```text
TRIM-ROUTE-LOWERING-DECISION-PROBE-001:
  produce a read-only Allow/Deny-style decision over existing trim route
  metadata, without backend lowering.
```

## Stop Lines

```text
do not implement trim route lowering in this inventory
do not treat trim_helper presence as an executable route proof
do not resolve promoted names without a production join_id producer
do not add backend lowering
do not claim generated program execution
do not start rustc adapter work in this row
```
