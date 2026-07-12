# 3457 - MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001

## Status

Selected by the completed 3455 consultation on 2026-07-12. This card is an
inventory task, not a hard-authority activation. Do not choose an owner or
change runtime, route, backend, publication, or Source Selfhost authority
until the inventory is complete and reviewed.

## Decision

```text
3455_direction = focused_inventory_first
preferred_next_authority_kind = Fact
registry_rule_candidate_kind = RegistryDescriptor
plan_authority_selection = 0
boundary_authority_selection = 0
caller_contract_widening = 0
```

The first preferred candidate after inventory is a narrow
`MapStoreKeyDomainFactOwner` for `I64Const` only. `I64Value` remains pending
because its current evidence depends on `MirType::Integer` and has no isolated
source/provenance owner yet.

## Scope

Target only:

```text
surface = SetSurfacePolicy
route_kind = MapStoreI64
policy_row_id = map_store_i64_set_surface
key_domain = I64
stored_value_domain = Any
```

Classify authority statements, not whole files. Candidate kinds are closed:

```text
Fact
Policy
Plan
Boundary
RegistryDescriptor
ValidationProjection
```

## Candidate inventory

### A — Constant-key Fact

```text
candidate_id = fact.mapstore.key_domain.i64_const
statement = resolved key origin is ConstValue::Integer
source = canonical MIR Const instruction / copy-chain origin
consumer = write_routes::match_generic_set_route
projection = GenericMethodKeyRoute::I64Const
eligibility = candidate
```

### B — Dynamic integer-key Fact

```text
candidate_id = fact.mapstore.key_domain.i64_value
statement = key is a dynamic integer value
current evidence = MirType::Integer
eligibility = pending
blocked_reason = source/fresh provenance owner not isolated
```

### C — Route Plan

```text
candidate_id = plan.mapstore.set.route_decision
producer = write_routes + Hako policy decision
eligibility = blocked
blocked_reason = would move route-selection authority
```

### D — Registry descriptor

```text
candidate_id = registry.mapstore_i64.backend_descriptor
source = spec/mir/generic_method_routes.toml
scope = route_id / emit_kind / helper_symbol / C descriptor mapping
eligibility = conditional
blocked_reason = backend/lowering authority remains closed
```

Registry-only fields may be candidates. Fields duplicated by the Hako policy
row remain projections and must not become a second policy owner:

```text
route_id, emit_kind, helper_symbol, c_need_kind, c_set_routes = RegistryDescriptor
key_domain, stored_value_domain, result_shape, effect_class,
mutation_class, publication_policy, core_op, lowering_tier, value_demand = Hako Policy
```

### E — Mutation boundary

```text
candidate_id = boundary.mapstore.set.mutation
eligibility = blocked
blocked_reason = runtime mutation and publication remain closed
```

### F — Caller contract

```text
candidate_id = projection.mapstore_i64.caller_orientation
status = already_closed
eligibility = not_a_hard_authority_candidate
```

## Required row schema

The inventory must be machine-readable and include:

```text
candidate_id
candidate_kind
semantic_statement
authority_source
stable_identity
current_producer
refresh_or_rebuild_owner
consumers
derived_projections
independent_oracle
fail_fast_boundary
fixture
authority_conflicts
behavior_delta
eligibility
blocked_reason
```

## Fail-fast inventory rules

Reject the inventory if any row has:

```text
missing source authority
duplicate authority for one field
derived artifact used as authority
caller contract used as Fact/Plan owner
MirType-only fact marked hard
registry projection marked semantic policy
Plan selected without complete Facts
Boundary selected without complete Plan
consumer missing
freshness/rebuild owner missing
fail-fast boundary missing
fixture missing
scope widening to MapStoreAny/ArrayAppendAny/Delete
```

Stable diagnostic tags, if diagnostics are needed:

```text
[mirbuilder/authority_inventory/source_missing]
[mirbuilder/authority_inventory/dual_owner]
[mirbuilder/authority_inventory/projection_as_authority]
[mirbuilder/authority_inventory/mirtype_only_fact]
[mirbuilder/authority_inventory/consumer_missing]
[mirbuilder/authority_inventory/freshness_missing]
[mirbuilder/authority_inventory/failfast_missing]
[mirbuilder/authority_inventory/fixture_missing]
[mirbuilder/authority_inventory/scope_widening]
```

Diagnostics must be default-off, one-line, and guarded by the existing debug
logging contract.

## Acceptance

```text
mapstore_i64_const_key_fact_candidate = 1
mapstore_dynamic_i64_fact_candidate = pending
mapstore_i64_plan_candidate = blocked
mapstore_i64_boundary_candidate = blocked
mapstore_i64_registry_descriptor_candidate = conditional
mapstore_i64_caller_projection_closed = 1
hard_authority_activation = 0
route_behavior_change = 0
runtime_mutation_authority = 0
backend_lowering_authority = 0
publication_execution = 0
mapstore_any_opened = 0
array_append_any_opened = 0
delete_opened = 0
scalar_known_wide_opened = 0
source_selfhost_claim = 0
```

After the inventory is green, select at most one minimal Fact-owner slice. If
source authority or consumer completeness is ambiguous, return to design
consultation instead of selecting a convenient-looking owner.
