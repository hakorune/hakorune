---
Status: Landed
Date: 2026-07-05
Scope: MirBuilder hard-authority migration selection rules.
---

# MIRBUILDER-HARD-AUTHORITY-SELECTION-RULES-001

## Decision

MirBuilder migration now selects the next slice by hard authority contract, not
by leaf pilot availability.

The next selection pass must inventory three tracks:

```text
Fact track:
  read-only input snapshot -> fact DTO owners under src/mir/builder/**

Plan track:
  facts -> recipe / plan DTO owners, especially REGISTRY rule units under
  src/mir/builder/control_flow/plan/**

Boundary track:
  exclusions and missing capability boundaries:
  lowering, MIR mutation, route execution, typed-object/static-helper support,
  ID allocation, executor/verifier movement
```

## Selection Checklist

A candidate is eligible for the first hard-authority implementation only when
all of these are true:

```text
read_only_or_plan_dto = 1
rust_oracle_json_fixture_possible = 1
symbolic_ids_only = 1
no_mir_mutation = 1
no_backend_lowering = 1
no_route_execution = 1
no_id_allocation = 1
no_new_hako_backend_capability = 1
```

## Exclusions

Do not select these as the next hard-authority slice:

```text
formatter / label / tag / classifier leaf
backend lowering
MIR mutation
route execution
ID allocation
typed-object/static-helper backend support
Rust executor / verifier movement
Source Selfhost claim
```

## Worker Use

Workers may be used for inventory, but each worker must keep to one track:

```text
Worker A: Fact track
Worker B: Plan track
Worker C: Boundary track
```

The integration step chooses one candidate only. Do not merge inventory work
with implementation in the same slice.

## Stop Condition

If the best candidate requires new `.hako` backend capability, typed-object
support, MIR mutation, backend lowering, or allocation authority, stop and write
a failed-selection/design card. Do not widen the implementation to force the
candidate through.

## Non-Claims

- No Source Selfhost claim.
- No new HakoAdopted owner.
- No Rust deletion.
- No backend lowering or MIR mutation migration.
