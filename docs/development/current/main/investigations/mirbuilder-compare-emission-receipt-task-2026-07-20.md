---
Status: Active design-stop task
Date: 2026-07-20
Scope: one checked physical `MirInstruction::Compare` receipt before Bool fact publication
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessor: docs/development/current/main/investigations/mirbuilder-resolved-direct-call-task-2026-07-20.md
---

# COMPAREEMIT0: establish Compare instruction receipt

## Decision

`FACT0-I1-COMPAREEMIT0-D0` is the current design stop. It does not authorize
an implementation yet.

The candidate is the shared builder-owned helper
`src/mir/builder/emission/compare.rs::emit_to`. Its two paths are:

```text
current function + current block present
  -> cf_common::emit_compare_func(...)
  -> unit return; a missing block can add no instruction

otherwise
  -> MirBuilder::emit_instruction(Compare)?

both paths
  -> direct Bool value_types write
```

The direct Bool fact therefore has no single checked physical instruction
receipt. This is a BoxShape issue, not a Bool inference issue.

```text
COMPAREEMIT0-D0
  -> COMPAREEMIT0-S0
  -> COMPAREEMIT0-M0
  -> COMPAREEMIT0-P0
  -> COMPAREEMIT0-I0
  -> COMPAREEMIT0-G0
```

Only D0 is selected. S0 remains forbidden until D0 fixes the checked receipt
authority and the `cf_common` compatibility boundary.

### `COMPAREEMIT0-D0` — closed (2026-07-20)

The selected receipt owner is a two-part builder boundary:

```text
strict current function/block membership preflight
  -> MirBuilder::emit_instruction(MirInstruction::Compare { ... })
```

`emit_instruction` itself rejects an absent current-block id, but its ordinary
`ensure_block_exists` behavior may create a stale named block. The preflight
must therefore prove that the current block already belongs to the current
function before calling it. The D0 cut is intentionally one-way:

```text
builder-owned Compare completion:
  must use the checked Builder receipt

cf_common::emit_compare_func:
  remains a non-Builder JSON-v0 compatibility helper
  and is not a COMPAREEMIT0 consumer
```

Replacing the builder's silent function/block path is an I0 fail-fast behavior
tightening, not an S0 change. It neither changes JSON-v0 callers nor opens the
legacy CompareOperator call route. Missing/stale builder state must become an
error before a Bool fact can commit.

`COMPAREEMIT0-S0` is now the sole next code-facing row.

### `COMPAREEMIT0-S0` — closed (2026-07-20)

One private, map-free `compare_type.rs` product now prepares only the fixed
`MirInstruction::Compare -> Bool` decision:

```text
Missing / StoredUnknown -> Publish(Bool)
Bool                    -> Idempotent(Bool)
other exact type        -> typed conflict
```

It has no Builder, ValueId, instruction, operand, commit, or production
consumer. `COMPAREEMIT0-M0` is now the sole next row; it must prove every
builder caller and current failure timing before I0 may replace the direct
Bool write.

### `COMPAREEMIT0-M0` — closed (2026-07-20)

The selected builder consumer count is exactly four:

```text
ops/comparison.rs     ordinary direct comparison
ops/unary.rs          direct logical-not expansion
exprs_peek.rs         CheckExpr dispatch comparison
exprs_enum_match.rs   enum-match tag comparison
```

Every caller propagates `emit_to` failure with `?`; none catches an error and
retries a raw route. Four `cf_common::emit_compare_func` callers exist only in
the JSON-v0 bridge and stay outside the Builder receipt boundary.

Current timing is now fixed precisely:

```text
no current block:
  existing Builder fallback errors before the direct Bool write

current function + valid current block:
  cf_common appends Compare
  -> direct Bool write

current function + stale current block:
  cf_common returns unit without an instruction
  -> direct Bool write
```

Calling `emit_instruction` alone would auto-create the stale block; that is
why I0 must first make the existing-block membership check explicit. After
that preflight, the checked Builder emitter is the only physical receipt and
the prepared Bool decision can remain uncommitted on every preflight or emit
error. `COMPAREEMIT0-P0` is now the sole next row.

### `COMPAREEMIT0-P0` — closed (2026-07-20)

Two direct builder fixtures now establish the behavior that I0 must preserve
or tighten at the receipt boundary:

```text
valid current function + existing current block:
  one physical Compare at the requested destination
  -> current transient Bool receipt

absent builder context:
  error
  -> destination type absent
  -> value-kind publication absent
```

The stale-block case is deliberately not asserted as compatibility: M0 proved
that its current silent `Bool` publication lacks a physical Compare. I0 must
replace it with strict preflight rejection, then add the exact stale-block
failure fixture in the same change. Origin, metadata, cache, retry, and JSON-v0
`cf_common` behavior are not part of P0.

`COMPAREEMIT0-I0` is now the sole next row.

### `COMPAREEMIT0-I0` — closed (2026-07-20)

The shared builder helper now follows one receipt transaction:

```text
strict current-function/current-block membership preflight
  -> prepare fixed Bool decision
  -> MirBuilder::emit_instruction(Compare)
  -> non-fallible Bool commit
```

The old direct `cf_common` emission and direct type-map write are gone from the
builder helper. Its JSON-v0 callers remain untouched. Missing context and a
stale block now fail before instruction creation, type publication, or implicit
block creation. The valid builder receipt remains one physical Compare plus
Bool. `COMPAREEMIT0-G0` is now the sole next row.

## Evidence and selection

Three independent post-`RESOLVED-DIRECT-CALL0` audits found no remaining direct
writer ready for immediate exact-fact cutover.

| Candidate | Finding | Disposition |
| --- | --- | --- |
| shared Compare emitter | one shared Bool writer; function/block path has no receipt | selected D0 |
| ArrayElementWrite | post-success Void exists, but eager metadata and receiver facts mix | `ARRAYWRITE0-D0` parked |
| Unified Call annotations | annotations can run after failed call emission | `CALL-RECEIPT0-D0` parked |
| FieldGet / FastMem | typed allocation precedes emission and FastMem has its own fallback | parked |
| metadata propagation | mixes type, origin, String, Map, Record, and environment compatibility | parked |

The helper's known builder callers are ordinary comparison, unary direct-not,
CheckExpr/enum-match support. They are consumers of one Compare completion
operation. The legacy CompareOperator call route is not a COMPAREEMIT0
consumer.

## Selected receipt law

Decide exactly one authority law:

```text
successful builder-owned physical Compare emission
  -> exact Bool transient fact
```

One strict builder preflight validates current-function presence, current-block
presence, and membership in that function's block table. Only then may the
canonical Builder emitter provide a physical Compare receipt.

## Conditional S0 boundary

If D0 establishes one receipt owner, S0 may add only a private map-free
prepared Bool decision:

```text
Missing / StoredUnknown -> Publish(Bool)
Bool                    -> Idempotent(Bool)
other exact type        -> typed conflict
```

The fixed `MirInstruction::Compare` result representation is the sole type
authority. S0 owns no operand inference, generic comparison solver, Builder,
ValueId, MIR mutation, TypeContext write, or production consumer.

M0/P0 must then prove receipt-before-publication and zero Bool/origin/metadata/
cache/retry delta on checked emission failure. I0 may connect exactly one
post-success commit. G0 must extend the existing
`mirbuilder-type-fact-partition` guard rather than add a new guard family.

## Exclusions and stop conditions

```text
not in scope:
  operand types or values
  source operator spelling
  CompareOperator name or environment toggles
  runtime tags / finalized metadata / origin
  generic Select / PHI / FieldGet / FastMem
  Unified Call / Array / Map / metadata propagation

stop if:
  all cf_common callers must change compatibility contract together
  direct Compare and legacy Call-backed CompareOperator must cut over together
  receipt needs Call, FieldGet, PHI, or Array transaction authority
  Bool must publish before physical instruction success
  source name, operand type, runtime tag, or final metadata becomes authority
  failure requires fallback or retry
  origin, metadata, backend, runtime, grammar, or ownership changes
  a touched source/check file reaches 800 lines
```

## Decision lock

> **COMPAREEMIT0 selects a design stop, not a premature Bool-fact cutover.
> The shared builder Compare helper has two emission paths, and its
> function/block compatibility path is unit-return and can silently omit an
> instruction for a missing block. D0 must establish one physical success
> receipt for builder-owned Compare completion before a private fixed-result
> Bool decision may commit. Call/operator compatibility, operand inference,
> origin, metadata, FieldGet, Array, PHI, finalization, fallback, and retry
> remain out of scope.**
