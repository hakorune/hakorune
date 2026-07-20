---
Status: Active implementation task
Date: 2026-07-20
Scope: one sealed resolved direct-call Integer result producer
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessor: docs/development/current/main/investigations/mirbuilder-resolved-trivial-operation-task-2026-07-20.md
---

# RESOLVED-DIRECT-CALL0: publish one sealed direct-call result fact

## Decision

`RESOLVED-DIRECT-CALL0-D0` selects one exact producer only:
`resolved_lowering::trivial_ssa::direct_call::emit`.

The resolved-trivial lowerer has one FunctionCall route. It verifies every
argument representation as `InlineI64`, claims the exact call row by source
site, confirms argument-site equality, and sends that row to the selected
emitter. The row is already the exact first direct-call profile, so its result
is `InlineI64`.

```text
exact FunctionCall source site
  -> every argument requires InlineI64
  -> profile.claim_direct_call(site)
  -> exact argument-site equality
  -> VerifiedCanonicalDirectCallEmissionV1 materialization
  -> successful MirInstruction::Call
  -> exact Integer result fact
```

The only selected type law is therefore:

```text
VerifiedTrivialDirectCallV1.result == InlineI64
  -> MirType::Integer
```

No general `TrivialRepresentationV1` mapping is introduced here. The prior
resolved-trivial operation projection remains a distinct owner; direct-call
admission is fixed by its existing sealed profile.

```text
RESOLVED-DIRECT-CALL0-S0
  -> RESOLVED-DIRECT-CALL0-M0
  -> RESOLVED-DIRECT-CALL0-P0
  -> RESOLVED-DIRECT-CALL0-I0
  -> RESOLVED-DIRECT-CALL0-G0
```

`RESOLVED-DIRECT-CALL0-S0` is the sole next code-facing row.

### `RESOLVED-DIRECT-CALL0-S0` — closed (2026-07-20)

One private, disconnected `direct_call_type.rs` product now prepares only an
Integer fact for the existing `InlineI64` direct-call result profile. It
rejects every other trivial representation before creating a decision, and it
has no Builder, ValueId, Call, capability, source-site, or commit consumer.

The existing direct-call emitter remains behavior-identical in S0; its current
generic representation projection has not been replaced yet. This preserves
the existing route while I0 remains the sole authorized connection point.

Focused evidence:

```text
InlineI64 + Missing/Unknown -> Publish(Integer)
InlineI64 + Integer -> Idempotent(Integer)
InlineI64 + conflicting concrete -> typed conflict
InlineBool/InlineF64/ExplicitVoidValue/NullSentinel -> typed rejection

cargo check --all-targets = green
```

`RESOLVED-DIRECT-CALL0-M0` is now the sole next row. It must inventory the
actual caller, all preflight ordering, fresh destination, materialization, and
existing Call receipt before the disconnected decision may commit.

### `RESOLVED-DIRECT-CALL0-M0` — closed (2026-07-20)

The source inventory establishes one selected production caller:

```text
direct_call::emit call sites = 1

FunctionCall lowering:
  every child argument lowers first
  -> every child requires InlineI64
  -> profile claims exact call row by expression site
  -> row argument source sites exactly equal lowered child sites
  -> direct_call::emit
```

The profile analyzer independently seals every finite direct-call row with
`result = InlineI64`; it rejects a non-i64 argument before direct-call profile
publication. The emitter then verifies current owner/symbol and the installed
VM-only direct-call capability, allocates a fresh result, performs pure
row-owned materialization, calls `emit_instruction(instruction)?`, and only
then reaches the current type write.

Existing unit evidence already fixes the row's target/ordered arguments/result
and pure materialization/cardinality failure. Existing compiler activation
fixtures execute the selected VM-only direct-call capability. No raw source
name, target reconstruction, result catalog, or final metadata is consulted by
the selected emitter.

`RESOLVED-DIRECT-CALL0-P0` is now the sole next row. It must add the
disconnected decision matrix plus selected-emitter receipt/failure isolation
proof before I0 replaces the direct write.

## Authority and transaction law

S0 adds one private prepared Integer decision, using the existing
`TypeFactDecisionV1` and no second mapping table.

```text
1. all header/owner/symbol/capability preflight succeeds
2. exact direct-call row is known to have InlineI64 result
3. fresh destination receives prepared Integer decision
4. verified row materializes Call instruction
5. Call emission succeeds
6. prepared decision commits
```

```text
Missing / StoredUnknown destination -> Publish(Integer)
matching Integer -> Idempotent(Integer)
conflicting concrete destination -> typed error before materialization/emission
materialization or emission failure -> type/origin/metadata/binding/cache/retry delta 0
```

The commit is non-fallible. It does not write origin, finalized metadata,
`TypeRegistry`, canonical-call capability, binding SSA, source-site state, or
caller ledger.

## M0 inventory contract

M0 must establish all of these without production rewiring:

```text
direct_call::emit production callers = 1

caller order:
  argument child lowering
  -> InlineI64 requirement per argument
  -> claim_direct_call(exact source site)
  -> argument-site equality
  -> direct_call::emit

emitter order:
  current-header owner/symbol preflight
  -> installed capability preflight
  -> fresh result allocation
  -> verified row materialization
  -> builder.emit_instruction(instruction)?
  -> current direct type write

row result:
  InlineI64 exactly
```

M0 must also confirm that the selected direct-call route has no raw name lookup
or raw target authority after the sealed row is consumed.

## P0 proof matrix

The disconnected decision requires:

```text
Missing / Unknown / matching Integer / conflicting concrete destination
exact Integer candidate only
no direct-call profile result other than InlineI64 admitted
materialization failure -> no prepared commit
no-function/no-block emission failure -> no type/origin/metadata/binding/cache fact
actual exact direct-call source route retains Call and Integer result parity
```

The source/runtime test remains the established VM-only direct-call capability
proof. It must not add a raw Call, MethodCall, dynamic ABI, caller ledger, or
result-representation widening.

## I0 and G0 contract

I0 changes only `trivial_ssa/direct_call.rs`:

```text
remove direct value_types.insert(result, ...)
prepare exact Integer before materialize/emission
commit after successful builder.emit_instruction(instruction)?
```

G0 extends the reusable `mirbuilder-type-fact-partition` guard. It freezes:

```text
direct_call.rs direct type writer = 0
direct_call.rs prepare = 1
direct_call.rs post-emission commit = 1
direct_call_type.rs TypeFactDecision prepare owner = 1
direct_call_type.rs type_ctx.set_type commit owner = 1
general trivial-representation classifier here = 0
origin/final metadata/TypeRegistry/retry writers here = 0
all touched source/check files < 800 lines
```

## Parked adjacent writers

```text
ArrayElementWrite:
  optional destination and eager function-metadata Void publication mix result,
  snapshot, and collection route authority.

Array/Map result annotations:
  they run after an emission result object without first proving success and
  also observe receiver facts before emission.

FieldGet:
  typed allocation is pre-emission and FastMem has a separate Integer fallback.

raw Compare/operator:
  generic compare can take a void/no-op path; raw operator has routing/env/name
  compatibility. They require separate receipt/route rows.

generic Call annotation:
  signature/name-compatible annotation and failure timing remain CALL-ANNOTATION0.

parameters:
  function skeleton identity/ABI publication is not an instruction receipt.

metadata propagation:
  type/origin/String/Map/Record compatibility remains METAPROP0.
```

## Stop conditions

Stop before I0 if any of these become necessary:

```text
a direct-call row has a result other than InlineI64
the fresh result destination has to overwrite a concrete fact
materialization or emission mutates facts before success
the direct-call emitter gets a second production caller
source site / argument-site equality needs reconstruction or raw name lookup
conflict needs fallback, retry, raw Call, or compatibility routing
origin, metadata, TypeRegistry, binding SSA, capability, runtime, backend,
grammar, ownership, or finalization policy must change
a touched source/check file reaches 800 lines
```

## Decision lock

> **RESOLVED-DIRECT-CALL0 selects exactly one post-success type producer:
> the sealed resolved-trivial `direct_call::emit` result. The exact
> `VerifiedTrivialDirectCallV1` row remains the sole source/ABI/result
> authority, and its first admitted profile fixes `InlineI64 -> Integer`.
> After all existing header, symbol, capability, and pure materialization
> preflight, one private prepared Integer decision may commit only after the
> existing Call emission returns success. The row does not widen direct calls,
> general representation mapping, raw Calls, method calls, result inference,
> origin, metadata, TypeRegistry, runtime, backend, grammar, or ownership.**
