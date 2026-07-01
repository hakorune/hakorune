# 2034 - MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001

## Token

```text
MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001
```

## Purpose

Define nominal ID domain boundaries for bounded ID scalar owner edges before
state mutation frame or behavior recipe work.

## Result

```text
bounded_owner_count = 2
id_domain_boundary_count = 3
directable_row_count = 11
raw_i64_interchangeability_count = 0
cross_domain_assignment_count = 0

decision:
  IdDomainBoundaryBasisDefined

selected_next_card:
  MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001
```

## Boundary

The declared domains are `BasicBlockId`, `BindingId`, and `ValueId`.
Nominal transports are retained as `*AsI64`; raw i64 interchangeability,
cross-domain assignment, and inferred sentinel/reserved-ID semantics remain
forbidden.

## Non-Claims

```text
source_plan_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
```
