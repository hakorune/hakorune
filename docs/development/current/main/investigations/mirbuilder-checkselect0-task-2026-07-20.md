---
Status: Active D-prime task
Date: 2026-07-20
Scope: one successful `CheckExpr` accumulator `Select` exact-Integer producer
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessors:
  - docs/development/current/main/investigations/mirbuilder-exact0-const-task-2026-07-20.md
  - docs/development/current/main/investigations/mirbuilder-copy-unknown-origin-task-2026-07-20.md
  - docs/development/current/main/investigations/mirbuilder-staticload0-task-2026-07-20.md
---

# CHECKSELECT0: exact `CheckExpr` accumulator publication

## Decision

`FACT0-I1-CHECKSELECT0-D0` is closed. The sole next code-facing row is
disconnected `CHECKSELECT0-S0`.

```text
CHECKSELECT0-S0
  -> CHECKSELECT0-M0
  -> CHECKSELECT0-P0
  -> CHECKSELECT0-I0
  -> CHECKSELECT0-G0
```

The admitted semantic producer is one successful `MirInstruction::Select`
inside `MirBuilder::build_check_expression`:

```text
CONST0 exact Integer one/zero
  -> CheckExpr accumulator `ok`
  -> successful Select(cond, ok, zero)
  -> exact Integer destination fact
```

The accumulator pair is already exact Integer by the closed CONST0 contract;
the condition is a control input, not representation authority. This row does
not claim generic `Select` inference.

## Authority boundary

The sole type authority is the fixed `CheckExpr` accumulator construction:

```text
then_val = previously exact Integer `ok`
else_val = exact Integer `zero`
```

`CheckExpr` conditions, AST spelling, source name, runtime value, origin,
finalized metadata, and general `Select` shape are non-authorities. The row
does not inspect source syntax beyond the existing `build_check_expression`
loop and must not infer a result type from the condition.

The exact lifecycle is:

```text
condition lowering succeeds
  -> fresh destination
  -> Select emission succeeds
  -> transient Integer fact commits
```

An emitted `Select` failure commits no type fact. The fresh `ValueId` cursor
is not rolled back. Final metadata remains the ordinary finalization snapshot,
never a lowering-time fallback.

## `CHECKSELECT0-S0` — closed (2026-07-20)

```text
production behavior delta = 0
production consumers = 0
Builder / ValueId / MIR / TypeContext writes = 0
```

Add one private, map-free prepare-only product adjacent to
`src/mir/builder/exprs_check.rs`. It prepares `Integer` through the existing
`TypeFactDecisionV1` from an optional destination entry:

```text
Missing or StoredUnknown -> Publish(Integer)
Integer                  -> Idempotent(Integer)
foreign concrete type    -> typed conflict
```

S0 owns no condition lowering, accumulator construction, destination
allocation, `Select` emission, commit, metadata publication, or production
consumer. It is a fixed-result producer decision, not a reusable `Select`
type solver.

Closed evidence:

```text
private owner:
  src/mir/builder/exprs_check/select_type.rs

Missing / StoredUnknown -> Publish(Integer)
Integer                 -> Idempotent(Integer)
foreign concrete type   -> typed conflict

production consumers / Builder / ValueId / MIR / TypeContext writes = 0
```

Focused verification:

```text
cargo fmt --check
cargo test -q --lib select_type
cargo check --all-targets
```

`CHECKSELECT0-M0` is now the sole next row.

## M0 and P0 requirements

### `CHECKSELECT0-M0` — closed (2026-07-20)

The current one-site timing inventory is:

```text
`emit_integer(1)` and `emit_integer(0)` publish CONST0 Integer
empty CheckExpr returns `one`
each non-empty item lowers its condition, emits one Select, then writes Integer
```

`emit_instruction` returns before the following write when `current_block` is
absent or block creation fails, so a failed Select cannot reach the current
destination type publication. No early function-metadata write exists; normal
finalization later snapshots `type_ctx.value_types`.

The accumulator invariant is inductive: CONST0 publishes Integer for `one` and
`zero`; each successful Select destination is directly published Integer before
it becomes the next `ok`. The condition is never read as an Integer authority.
Compare, operator-call, PHI, field, and generic branch producers do not join
this one-site inventory.

`CHECKSELECT0-P0` is now the sole next row.

### `CHECKSELECT0-P0` — closed (2026-07-20)

P0 fixes:

```text
empty CheckExpr returns the existing CONST0 value
one and multiple items preserve Integer induction
failed Select publishes no destination type
ordinary finalization snapshots the transient Integer fact
Missing/Unknown/Idempotent/conflict decision behavior is exact
```

Closed evidence:

```text
empty CheckExpr:
  returns the existing CONST0 Integer and emits no Select

multiple CheckExpr items:
  every successful Select destination is Integer
  final result is the last Select destination

failed Select emission:
  destination type entry = absent

ordinary finalization:
  snapshots the transient result Integer into function metadata
```

Focused verification:

```text
cargo fmt --check
cargo test -q --lib select_type
cargo test -q --lib exprs_check::tests
cargo check --all-targets
```

`CHECKSELECT0-I0` is now the sole next row. It may connect the prepared
decision only after the existing accumulator Select succeeds.

## `CHECKSELECT0-I0` — closed (2026-07-20)

I0 has exactly one production connection: after the successful accumulator
`Select` in `build_check_expression`. It prepares after condition lowering and
before emission, then commits only after success. The direct
`value_types.insert(dst, Integer)` is removed.

```text
Select failure -> prepared commit = 0
successful Select -> transient Integer only
origin / metadata publication = 0
```

## `CHECKSELECT0-G0` — closed (2026-07-20)

G0 extends the existing `mirbuilder-type-fact-partition` manifest guard; it
does not create a new guard family. It freezes:

```text
CheckExpr Select decision owners = 1
CheckExpr Select post-emission commit consumers = 1
direct exprs_check type writers for this producer = 0
origin / metadata publication = 0
generic Select consumers = 0
```

The active writer inventory removes the direct `exprs_check.rs` writer and
adds exactly one `exprs_check/select_type.rs` owner. The guard strips test
modules before testing the production direct-write exclusion.

Focused verification:

```text
tools/checks/run_row_guard.sh --only mirbuilder-type-fact-partition
bash tools/checks/current_state_pointer_guard.sh
cargo fmt --check
cargo test -q --lib select_type
cargo test -q --lib exprs_check::tests
cargo check --all-targets
```

`CHECKSELECT0` is complete. It does not select the next EXACT0 producer.

## Exclusions and stop conditions

Parked: Compare (which first needs an emission-success receipt), operator-call
result typing, generic `Select`, PHI, FieldGet, Call, origin, Unknown
retirement, and finalization repair.

Stop before I0 if the fixed accumulator ceases to be two already-exact Integer
values; a `TypeRegistry`, origin, metadata, generic condition type, or source
name becomes necessary; or the type decision cannot be prepared before
`Select` mutation. Do not introduce retry, fallback, a second type map, source
grammar/runtime/backend/ownership changes, or a source/check file at or above
800 lines.

## Decision lock

> `CHECKSELECT0` selects one narrow exact producer: a successful CheckExpr
> accumulator Select whose `then_val` and `else_val` are already exact Integer
> through CONST0. Its condition never supplies representation authority. S0 is
> a disconnected `TypeFactDecisionV1` preparation owner; I0 later connects one
> post-emission transient commit in `build_check_expression`; G0 extends the
> existing FACT0 guard. This does not establish generic Select inference.
