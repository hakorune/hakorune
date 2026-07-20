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

## D0 question

Decide exactly one authority law:

```text
successful builder-owned physical Compare emission
  -> exact Bool transient fact
```

The preferred shape is a checked builder-owned Compare emission boundary that
returns success only after the exact current block contains the Compare
instruction. `cf_common::emit_compare_func` may remain a function-level
compatibility helper, but cannot be a builder receipt unless D0 gives it an
explicit checked contract.

D0 must inventory, without code changes:

1. every `emission::compare::emit_to` caller and route;
2. missing-function, missing-block, and stale-block outcomes;
3. the smallest receipt owner preserving all selected direct Compare callers;
4. the one non-fallible post-success Bool commit owner;
5. no-fact failure deltas.

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
