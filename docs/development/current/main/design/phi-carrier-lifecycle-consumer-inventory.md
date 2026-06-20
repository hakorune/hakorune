# PHI Carrier Lifecycle Consumer Inventory

Status: SSOT
Scope: lifecycle-sensitive consumers of `CarrierInfo` PHI carrier state.

## Purpose

`CarrierInfo::from_variable_map` and `CarrierInfo::with_explicit_carriers`
are fixture-guarded as snapshots from an owner-carrying read `BorrowView`.

This document names the remaining carrier/PHI lifecycle consumers before any
general lifecycle resolver consumes these facts.

## Source Entrypoints

```text
src/mir/join_ir/lowering/carrier_info/types.rs
src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
src/mir/join_ir/lowering/scope_manager.rs
src/mir/loop_route_detection/support/body_local/carrier.rs
src/mir/loop_route_detection/support/body_local/digitpos.rs
src/mir/loop_route_detection/support/body_local/condition.rs
```

## Already Proven

```text
CarrierInfo::from_variable_map:
  plan=CarrierSnapshotFromBorrowView
  reads VariableContext map through owner-carrying BorrowView
  mutates_VariableContext=0
  publishes_variable_map=0

CarrierInfo::with_explicit_carriers:
  plan=ExplicitCarrierSnapshotFromBorrowView
  requested carrier names are owned inputs
  missing carrier remains fail-fast
  mutates_VariableContext=0
  publishes_variable_map=0
```

These plans do not model PHI state after snapshot creation.

## CarrierVar.join_id

Definition:

```rust
pub join_id: Option<ValueId>
```

Observed lifecycle:

```text
initial value:
  None in carrier snapshot constructors

meaning of Some(ValueId):
  JoinIR-side carrier value produced by header PHI / carrier parameter setup

current producer:
  not owned by carrier snapshot constructors
  no production assignment to Some(ValueId) is currently identified

current consumers:
  CarrierInfo::resolve_promoted_join_id
  ScopeManager promoted local lookup path
  tests / plan bridge fixtures that construct CarrierVar directly
```

Lifecycle boundary:

```text
join_id assignment is a separate lifecycle owner
snapshot plans must not claim it
resolver must not require join_id until the assignment producer is named
```

Future row:

```text
PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER-INVENTORY-001
```

Producer search result:

```text
production CarrierVar constructors:
  initialize join_id=None

production mutation:
  no `carrier.join_id = Some(...)` assignment found

production Some(ValueId):
  none found

test/fixture Some(ValueId):
  scope_manager tests only
```

Implication:

```text
join_id lifecycle is not currently a proven production plan.
Before resolver work, decide whether join_id is stale vocabulary,
test-only vocabulary, or an unimplemented producer boundary.
```

## promoted_body_locals

Definition:

```rust
pub promoted_body_locals: Vec<String>
```

Observed lifecycle:

```text
initial value:
  empty in carrier snapshot constructors

producers:
  body-local carrier promotion helpers
  merge_from copies and deduplicates entries from another CarrierInfo

consumers:
  CarrierInfo::resolve_promoted_join_id
  ScopeManager promoted local lookup path
```

Lifecycle boundary:

```text
promoted names are owned strings in CarrierInfo
promotion/merge ownership is not part of VariableContext map snapshot
name-convention based lookup remains a consumer detail, not resolver truth
```

Future row:

```text
PHI-PROMOTED-BODY-LOCALS-LIFECYCLE-PROBE-001
```

## trim_helper

Definition:

```rust
pub trim_helper: Option<TrimLoopHelper>
```

Observed lifecycle:

```text
initial value:
  None in generic carrier snapshots

producer:
  trim/body-local carrier support sets Some(TrimLoopHelper)

merge behavior:
  CarrierInfo::merge_from clones helper from the other CarrierInfo when present

consumers:
  CarrierInfo::trim_helper()
  body-local condition tests / trim route support
```

Lifecycle boundary:

```text
trim_helper is route-specific owned metadata
generic VariableContext snapshots must not claim trim route lifecycle
```

Future row:

```text
TRIM-HELPER-CARRIER-LIFECYCLE-INVENTORY-001
```

## CarrierInfo::merge_from

Observed behavior:

```text
mutates CarrierInfo
deduplicates carriers by name
sorts carriers for determinism
clones trim_helper when present
deduplicates promoted_body_locals
```

Lifecycle boundary:

```text
merge_from is not a BorrowView read snapshot
merge_from is an owned CarrierInfo mutation / merge owner
```

Future row:

```text
CARRIER-INFO-MERGE-FROM-LIFECYCLE-PROBE-001
```

## Read-Only Consumers

```text
carrier_count:
  read-only count

is_multi_carrier:
  read-only predicate

find_carrier:
  read-only borrowed lookup result

trim_helper:
  read-only borrowed lookup result

resolve_promoted_join_id:
  read-only lookup, but depends on promoted_body_locals + join_id having
  already been produced
```

These are not plan producers. Resolver work may consume them only after their
input owners are named.

## Initial Policy

```text
inventory_only=1
implementation_started=0
resolver_started=0
new_HakoLifecyclePlan_kind=0
converter_emission_added=0
```

The next implementation-capable row should pick exactly one owner:

```text
join_id assignment
join_id producer absence / retirement decision
promoted_body_locals ownership
trim_helper ownership
merge_from ownership
read-only resolver skeleton
```

## Stop Lines

```text
do not treat CarrierInfo snapshots as PHI lifecycle complete
do not start a general resolver before PHI carrier consumers are named
do not make promoted_body_locals naming convention resolver truth
do not merge trim route metadata into generic VariableContext snapshot plans
do not claim full VariableContext or MirBuilder lifecycle parity
```
