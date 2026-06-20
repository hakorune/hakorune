# PHI Carrier join_id Vocabulary Decision

Status: SSOT
Scope: `CarrierVar.join_id` lifecycle status in the Rust-to-Hako lifecycle lane.

## Decision

`CarrierVar.join_id` is not a live lifecycle producer in the current
production pipeline.

For the lifecycle lane, treat it as:

```text
status=test_fixture_or_stale_vocabulary
production_producer=0
lifecycle_plan_owner=0
resolver_dependency=0
emitter_dependency=0
```

This is not a deletion decision. The field remains in Rust code until a
separate JoinIR carrier value-space cleanup or producer design row chooses to
remove it or implement a real producer.

## Evidence

Production search:

```text
CarrierVar constructors:
  join_id=None

production mutation:
  no `carrier.join_id = Some(...)`

production literal Some(ValueId):
  none

test/fixture Some(ValueId):
  scope_manager tests only
```

## Lifecycle Policy

```text
CarrierInfo snapshots:
  must keep join_id=None

CarrierInfo::merge_from:
  must not become a join_id producer

read-only resolver:
  must DenyUnresolvedBoundary for join_id-dependent paths

verifier result:
  must deny join_id producer boundary

emitter:
  must not emit join_id producer or join_id-dependent path
```

## Future Options

```text
Retire:
  remove field and tests after proving all consumers are dead or migrated

Implement:
  add explicit production producer with its own owner, fixture, and verifier

Keep parked:
  leave as internal Rust/test vocabulary and keep lifecycle pipeline deny rules
```

Current decision:

```text
keep_parked=1
retire_now=0
implement_now=0
```

## Stop Lines

```text
do not delete join_id in this decision row
do not add dummy join_id assignment
do not make tests define production lifecycle truth
do not allow resolver/emitter to depend on join_id
do not claim PHI carrier lifecycle complete from join_id absence alone
```
