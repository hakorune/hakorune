# Promoted Name Resolution Deny Closeout

Status: closeout
Scope: `CarrierInfo::resolve_promoted_join_id` lifecycle status.

## Purpose

`promoted_body_locals` producers now record original loop-body-local names.
That does not make promoted-name resolution lifecycle-safe.

`CarrierInfo::resolve_promoted_join_id` still depends on a produced
`CarrierVar.join_id`. The current lifecycle lane has parked `join_id` as
test-fixture/stale vocabulary with no production `Some(ValueId)` producer.

Therefore promoted-name resolution remains denied.

## Source Entrypoints

```text
src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
src/mir/join_ir/lowering/scope_manager.rs
```

## Consumer Shape

```text
CarrierInfo::resolve_promoted_join_id(original_name):
  1. checks promoted_body_locals contains original_name
  2. derives candidate carrier names:
       is_<name>
       is_<name>_match
  3. returns carrier.join_id only if Some(ValueId)
```

Scope manager path:

```text
LoopBreakScopeManager::lookup(name)
  -> carrier_info.resolve_promoted_join_id(name)
```

## Deny Reason

```text
promoted_body_locals_recorded=1
join_id_producer=0
join_id_dependent_paths_allowed=0
resolution_allowed=0
```

This is not a failure of promoted-name producers. It is a missing downstream
join_id producer boundary.

## Current Fixture Boundary

Resolver diagnostics:

```text
CarrierVar.join_id.production_lifecycle:
  DenyUnresolvedBoundary(no_production_Some_ValueId_producer)

claims.join_id_dependent_paths_allowed=false
```

Producer fixtures:

```text
promoted-body-locals-producer:
  denied includes "promoted name resolution"
  denied includes "join_id producer"
```

## Decision

```text
promoted_name_resolution_closed_as_deny=1
resolution_allowed=0
join_id_producer=0
resolver_selection_owner=0
converter_emission_added=0
```

## Reopen Condition

Only reopen this boundary after a separate row names and verifies a production
`CarrierVar.join_id=Some(ValueId)` owner.

```text
required_reopen_owner=PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER
```

## Stop Lines

```text
do not infer join_id from promoted_body_locals
do not allow resolver/emitter positive resolution claim
do not add dummy join_id producer
do not modify Rust behavior in this closeout row
```

