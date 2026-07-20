---
Status: Active D′ architecture task
Date: 2026-07-20
Scope: First independently sealed exact-fact producer after COPY0
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessor: docs/development/current/main/investigations/mirbuilder-copy-unknown-origin-task-2026-07-20.md
---

# FACT0-I1-EXACT0: canonical Const exact type publication

## Decision

`FACT0-I1-EXACT0-D0` is closed. The first code-facing slice is not the broad
`simple_exact` fixture bucket. It is one physical instruction family only:

```text
FACT0-I1-EXACT0-CONST0-S0
  -> FACT0-I1-EXACT0-CONST0-M0
  -> FACT0-I1-EXACT0-CONST0-P0
  -> FACT0-I1-EXACT0-CONST0-I0
  -> FACT0-I1-EXACT0-CONST0-G0
```

`CONST0` owns the six canonical fallible `MirInstruction::Const` emitters in
`src/mir/builder/emission/constant.rs`:

```text
ConstValue::Integer -> MirType::Integer
ConstValue::Bool    -> MirType::Bool
ConstValue::Float   -> MirType::Float
ConstValue::String  -> MirType::String
ConstValue::Null    -> MirType::Void
ConstValue::Void    -> MirType::Void
```

Each row shares one authority chain:

```text
exact ConstValue representation
  -> existing TypeFactDecisionV1 preflight for fresh dst
  -> successful fallible Const emission
  -> one non-fallible exact-type commit
```

`Void` is exact. `Unknown` is never a `CONST0` proposal. ValueId cursor
rollback is not claimed.

## Why Const only

The partition's `simple_exact` label is a census bucket, not a shared semantic
transaction. The audit found incompatible producer classes:

| Writer class | Why it is excluded from CONST0 |
| --- | --- |
| Compare emission | The current function/block route can silently emit nothing, so it lacks a checked defining-instruction receipt. |
| Select result | Its `Integer` result is a constructed-expression rule, not a Const representation. |
| Operator call / unary | It depends on legacy operator or call-route authority; negative integer literal is only a duplicate post-Const annotation. |
| StaticDataLoad | It has a sealed `u16` representation but also writes finalized metadata early; it needs its own timing proof. |
| Resolved/literal post-emit annotations | These are profile-specific or duplicate writers, not the canonical Const producer. |

The only shared component is the existing `TypeFactDecisionV1`.

## CONST0 authority and transaction law

### Sole type authority

The `ConstValue` variant is the only representation authority. No source name,
AST re-read, operand heuristic, runtime tag, finalized metadata, or string map
determines the exact type.

`CONST0` prepares against the current transient `value_types` entry for the
fresh destination. The existing decision law remains authoritative:

```text
Missing / Unknown + exact T -> Publish(T)
Exact(T) + exact T          -> Idempotent(T)
Exact(U) + exact T, U != T  -> typed conflict before emission
```

### Commit and failure law

```text
1. allocate destination ValueId
2. derive exact Const type and prepare TypeFactDecisionV1
3. emit MirInstruction::Const
4. only after successful emission, commit Publish(T)
5. for String only, publish existing string_literals companion fact
```

`string_literals` remains a value-content companion, not type authority. It
must not publish if the Const instruction or type preflight fails. Origin, map,
record, and finalized-function metadata remain untouched.

```text
preflight conflict or Const emission error:
  Const instruction/type fact/string literal fact = 0

successful Const:
  exactly the existing exact transient type is observable
```

No fallback, retry, result backfill, or changed public emitter signature is
authorized.

## Row plan

### `FACT0-I1-EXACT0-CONST0-S0` — closed (2026-07-20)

```text
production behavior delta: 0
production consumers: 0
```

Add one disconnected Const representation-to-type decision/prepare vocabulary
and focused tests. It may borrow `TypeFactDecisionV1`; it must not add a second
type map, Builder field, direct writer replacement, or new fact facade.

The disconnected owner is `emission::constant_type`:

```text
ConstValue -> exact MirType -> PreparedTypeFactPublicationV1
```

It owns no `MirBuilder`, `TypeContext`, `ValueId`, instruction emission, fact
commit, or String companion publication. Its four focused tests seal all six
variant mappings, Missing/StoredUnknown publication, exact idempotence, and
concrete conflict. The existing direct writers remain unchanged; production
consumers are still zero.

### `CONST0-M0` — closed (2026-07-20)

Inventory all six callers and prove their instruction/error boundary. Record
String companion ordering and confirm that no Const caller requires function
metadata before finalization.

The direct implementation inventory closes one uniform six-helper boundary:

```text
emit_integer / emit_bool / emit_float / emit_string / emit_null / emit_void
  -> next_value_id
  -> emit_instruction(Const)?
  -> transient exact type write
  -> String only: string_literals write
```

`builder_emit::emit_instruction` rejects a missing current block before any
instruction mutation. Every helper uses `?`, so this failure leaves no Const
instruction, transient type, String fact, or function-metadata type entry. The
existing ValueId cursor advance is the only residual and remains outside this
row's rollback claim.

No helper reads or writes `MirFunction.metadata.value_types`. Direct literal
front ends only delegate to these helpers; normal downstream arithmetic reads
the transient type first and reaches its metadata fallback only for Unknown,
which canonical Const never publishes. Existing metadata readers are either
Box-specific or test-only. Thus no metadata-timing stop condition blocks P0.

`CONST0-P0` is now the sole next row.

### `CONST0-P0` — closed (2026-07-20)

Prove all six mappings; Missing/Unknown/idempotent/conflict decisions; Null as
Void; failed emission non-publication; and String companion non-publication on
failure.

The proof is now split at the intended boundary:

```text
constant_type unit proof:
  six exact mappings
  Missing / StoredUnknown -> Publish
  matching exact -> Idempotent
  conflicting exact -> typed rejection

constant emitter integration proof:
  six successful Const instructions -> six transient exact facts
  String -> one matching string_literals companion
  current-block failure -> no instruction, type, String, or metadata fact
```

The failure proof deliberately observes the existing non-rollback ValueId
cursor only as an excluded residual. It does not invoke finalization, origin,
or a fallback route. `CONST0-I0` is now the sole next row.

### `CONST0-I0` — closed (2026-07-20)

Replace only the six direct transient type writes in `emission/constant.rs`.
There is one production consumer family: successful canonical Const emission.
Commit comes after `emit_instruction` and before returning the ValueId.

All six public helpers now delegate to one private `emit_exact_const` path:

```text
fresh dst
  -> PreparedCanonicalConstTypeV1::prepare
  -> emit_instruction(Const)?
  -> PreparedCanonicalConstTypeV1::commit
  -> return dst
```

The commit consumes only `Publish`; an idempotent prepared fact does not
overwrite. `emit_string` writes its existing `string_literals` payload only
after that shared path returns successfully. The six direct `value_types.insert`
writes are gone. Existing successful output and missing-current-block failure
parity are covered by P0; no new type map, origin, metadata write, fallback,
retry, grammar, runtime, backend, or ownership behavior is introduced.

`CONST0-G0` is now the sole next row.

### `CONST0-G0` — closed (2026-07-20)

Extend the existing FACT0 partition/authority guard; do not create a manifest
family. It freezes:

```text
canonical Const exact decision owner = 1
canonical Const exact commit owner = 1
canonical Const production consumer family = 1
direct canonical Const type inserts = 0
Unknown proposals = 0
Const origin writes = 0
source/check files >= 800 lines = 0
```

The existing partition fixture remains the immutable historical P1 census
(47 paths / 99 direct writes). The extended guard now also derives the active
cutover surface from that census through the only approved COPY0 and CONST0
replacement map (48 paths / 96 direct writes). It therefore detects both a
historical-fixture rewrite and an unrecorded current direct writer.

CONST0-specific checks require one shared preparation call, one shared
post-emission commit call, six public delegates, one String companion write
after the shared commit, no direct Const type/origin write, and one
`TypeFactDecisionV1`/`set_type` owner. The guard's unit suite, focused Const
tests, format, pointer, and diff checks are green.

`CONST0` is complete. `STATICLOAD0-D0` is the next selected D-prime row.

## Parked successor order

After `CONST0-G0`, choose the next row in this fixed order:

```text
STATICLOAD0-D0
  sealed u16 StaticDataLoad representation; prove removal of its eager
  finalized-metadata write before changing it

COMPARE0-D0
  first obtain a checked Compare-emission receipt; preserve or explicitly
  reject current missing-block no-op behavior before a type cutover

SELECT0-D0
  constructed Select-result representation rule

LITERAL-POSTEMIT-RET0-D0
  retire duplicate post-Const annotations in builder/resolved/unary paths

OPERATOR0-D0
  legacy Call/operator-mode exact result claims
```

Static load is genuine exact publication but not a Const producer. Compare,
Select, operator calls, resolved-lowering profiles, FieldGet, Call, origin,
Unknown retirement, finalization repair, and metadata propagation remain
separate owners.

## Stops and claims

Stop and open a new consultation rather than widen CONST0 if it requires:

1. type inference from source spelling, runtime class, or operand route;
2. `Unknown` as an exact proposal;
3. retry, raw fallback, or fact backfill after failure;
4. String/map/record/origin fact as type authority;
5. finalized metadata mutation;
6. Compare, Select, StaticDataLoad, primitive-wrapper, resolved, or operator
   writers in this row;
7. a persistent ValueId map, Builder field, or manifest family; or
8. a source/check file at or above 800 lines.

After `CONST0-G0`, the compiler may claim only that successful canonical Const
emission publishes its exact transient type through one monotone decision/commit
owner, while a failed Const emits neither that type fact nor a String companion.
