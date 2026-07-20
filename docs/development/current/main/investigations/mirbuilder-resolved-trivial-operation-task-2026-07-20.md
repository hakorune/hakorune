---
Status: Active implementation task
Date: 2026-07-20
Scope: one sealed resolved-trivial Binary/Compare type-fact producer
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessor: docs/development/current/main/investigations/mirbuilder-literal-postemit-retirement-task-2026-07-20.md
---

# RESOLVED-TRIVIAL-OP0: publish one sealed trivial-operation type fact

## Decision

`RESOLVED-TRIVIAL-OP0-D0` selects one exact producer only:
`resolved_lowering::trivial_ssa::operation::emit_binary`.

The function receives an existing sealed `TrivialRepresentationV1` only after
the resolved-trivial lowerer claims the exact expression site from its verified
profile. It emits one `BinOp` or `Compare`, then directly writes the matching
`MirType` to the fresh destination. That existing write is the selected
post-success exact-fact producer.

```text
exact resolved expression site
  -> profile.claim_value(site)
  -> TrivialRepresentationV1
  -> operation::emit_binary
  -> successful MirInstruction::BinOp | Compare
  -> exact destination type fact
```

The selected representation law is closed and name-free:

```text
InlineI64          -> Integer
InlineBool         -> Bool
InlineF64          -> Float
ExplicitVoidValue  -> Void
NullSentinel       -> Void
```

`RESOLVED-TRIVIAL-OP0-S0` is the sole next code-facing row.

```text
RESOLVED-TRIVIAL-OP0-S0
  -> RESOLVED-TRIVIAL-OP0-M0
  -> RESOLVED-TRIVIAL-OP0-P0
  -> RESOLVED-TRIVIAL-OP0-I0
  -> RESOLVED-TRIVIAL-OP0-G0
```

This is an EXACT0 producer cutover. It neither widens resolved-trivial
grammar nor treats a raw operator or generic compare path as equivalent.

### `RESOLVED-TRIVIAL-OP0-S0` — closed (2026-07-20)

One private, disconnected `operation_type.rs` product now owns the exact
`TrivialRepresentationV1 -> MirType` projection and prepares the existing
`TypeFactDecisionV1`. It is intentionally not connected to the operation
emitter yet: no Builder, ValueId, MIR instruction, `TypeContext`, origin,
metadata, cache, or commit consumer was added.

The former `operation::mir_type` is now a compatibility re-export of that one
projection, so the existing direct writer and direct-call route retain exactly
their prior behavior while representation mapping has one physical owner.

Focused evidence:

```text
operation_type tests:
  all five representations
  Missing / Unknown publication
  Void idempotence for explicit Void and Null
  concrete conflict rejection

cargo check --all-targets = green
```

`RESOLVED-TRIVIAL-OP0-M0` is now the sole next row. It must inventory the
actual callers, receipt ordering, destination freshness, and all pre-I0
representation/instruction pairs before the new decision receives one commit
consumer.

### `RESOLVED-TRIVIAL-OP0-M0` — closed (2026-07-20)

The current source inventory closes one exact emitter relationship:

```text
emit_binary production call sites = 1

lowerer BinaryOp branch:
  lower left
  -> lower right
  -> profile.claim_value(expression.site())
  -> emit_binary(builder, operator, lhs, rhs, expected)
  -> return (value, expected)
```

No raw AST router, method name, environment flag, result catalog, or final
metadata participates. The caller returns before the generic expression tail,
so it does not invoke `ensure_value_representation` as a second binary-result
publisher.

`emit_binary` itself has one fresh `next_value_id()` destination, maps the
closed profile to exactly one `BinOp` or `Compare`, and invokes
`builder.emit_instruction(instruction)?` before its present direct write. The
shared builder validates current block/function conditions before it appends
the instruction. Therefore a returned `Ok(())` is the selected operation
receipt; an `Err` reaches the caller before any operation type write.

Existing focused corpus evidence exercises both selected families:

```text
trivial Float arithmetic:
  BinOp destination has Float in finalized metadata

Void equality / inequality:
  Compare destination has Bool and executes under vm-reference
```

The profile's first executable grammar may not pair every representation with
every binary operator. P0 therefore proves all mapping decisions synthetically
and only the actual admissible Binary/Compare pairs through the resolved route.

`RESOLVED-TRIVIAL-OP0-P0` is now the sole next row. It adds actual receipt and
failure-isolation proof around the disconnected decision without connecting a
production commit consumer.

### `RESOLVED-TRIVIAL-OP0-P0` — closed (2026-07-20)

The proof now covers both sides of the selected receipt boundary:

```text
direct operation emitter:
  successful Add emits BinOp, then exposes Integer
  successful Equal emits Compare, then exposes Bool

failed operation emitter:
  no current block -> error
  transient type map = unchanged
  origin map = unchanged
  variable binding map = unchanged
```

An actual resolved-source Bool equality fixture reaches the sealed profile,
materializes `Compare`, preserves `Bool` into normal finalized metadata, and
passes verification. The existing Float fixture remains the arithmetic route
witness; feature-enabled Void comparison execution remains a representation
regression witness. The disconnected decision tests retain the complete five
representation mapping plus Missing/Unknown/idempotent/conflict matrix.

No production decision commit is connected in this row.

`RESOLVED-TRIVIAL-OP0-I0` is now the sole next row. It may add one non-fallible
commit method and replace only the post-success direct operation write.

### `RESOLVED-TRIVIAL-OP0-I0` — closed (2026-07-20)

The operation emitter now follows the fixed transaction:

```text
fresh destination
  -> prepare from profile representation and current destination entry
  -> emit BinOp or Compare
  -> commit the prepared exact fact
```

The old direct `value_types.insert` is deleted. The new commit uses the
existing exact `TypeFactDecisionV1`; Missing and stored Unknown publish the
sealed type, a matching exact fact is idempotent, and an incompatible concrete
fact rejects before instruction emission. Failed emission reaches `?` before
commit. No source profile, raw route, origin, cache, binding, metadata,
finalization, or retry behavior changes.

`RESOLVED-TRIVIAL-OP0-G0` is now the sole next row. It freezes the direct-write
replacement, one prepare/commit relationship, receipt ordering, and size cap
in the reusable FACT0 partition guard.

### `RESOLVED-TRIVIAL-OP0-G0` — closed (2026-07-20)

The existing manifest-backed `mirbuilder-type-fact-partition` guard now owns
the structural freeze. It verifies:

```text
operation.rs direct type writer = 0
operation.rs prepared decision consumer = 1
operation.rs post-emission commit consumer = 1
commit occurs after builder.emit_instruction(instruction)?
operation_type.rs TypeFactDecision prepare owner = 1
operation_type.rs type_ctx.set_type commit owner = 1
all touched source/check files < 800 lines
```

The active writer inventory removes the historical operation writer and adds
the one private owner without altering the immutable P1 census. Focused
operation, resolved Float/Bool, feature-enabled Void comparison, all-target,
row-guard, pointer, format, and diff checks are green.

`RESOLVED-TRIVIAL-OP0` is complete. It does not select the next independent
EXACT0 producer. Raw Compare/operator, Array result, FieldGet, Call
annotation, metadata propagation, finalization repair, origin policy, and
Unknown retirement remain parked until the active D-prime selector chooses one
new owner.

## Authority and transaction law

S0 introduces one private prepared product adjacent to the existing operation
owner. Its sole job is to turn the already sealed representation into the
existing exact `TypeFactDecisionV1` before instruction emission.

```rust
struct PreparedResolvedTrivialOperationTypeV1 {
    decision: TypeFactDecisionV1,
}
```

The final shape may use the repository's exact decision vocabulary directly;
it must not add a second type map, a source-site map, or a duplicate
representation classifier.

```text
1. caller claims the exact profile representation
2. operation prepares the exact type decision for its fresh destination
3. operation emits BinOp or Compare
4. only after success, the prepared decision commits
5. operation returns the destination
```

The commit is non-fallible. Fresh-destination conflict checks are completed
before instruction emission.

```text
successful emission:
  Missing -> Publish(T)
  Unknown -> Publish(T)
  Exact(T) -> Idempotent(T)

pre-existing Exact(U), U != T:
  typed conflict before emission

emission failure:
  instruction/type/origin/cache/profile-retry delta = 0
```

The row has no origin, metadata, SSA-binding, cache, finalization, runtime,
backend, or grammar authority.

## M0 inventory contract

M0 must prove, without production rewiring, all of the following:

```text
operation producer:
  exactly one direct write in operation.rs

callers:
  every emit_binary call receives representation from the existing
  profile.claim_value(expression.site()) path

instruction receipt:
  builder.emit_instruction(instruction)? precedes the direct write

instruction families:
  arithmetic -> BinOp
  comparison -> Compare

destination:
  fresh from next_value_id
```

It must inventory `ExplicitVoidValue` and `NullSentinel` even when the first
resolved-trivial corpus does not execute every operator/representation pair.

## P0 proof matrix

The disconnected owner must test:

```text
all five TrivialRepresentationV1 mappings
Missing / Unknown / matching exact / conflicting exact destination states
arithmetic and comparison instruction selection
successful post-emission commit ordering
failed emission has no exact type publication
```

M0/P0 keep production consumers at zero. Synthetic transaction tests may use
a narrow fake receipt; they must not make final function metadata a
lowering-time authority.

## I0 and G0 contract

I0 replaces only this direct writer:

```text
src/mir/builder/resolved_lowering/trivial_ssa/operation.rs
```

Exactly one operation emitter prepares and commits exactly one decision. The
existing profile claim remains the sole representation authority, and
`emit_instruction` remains the sole success receipt.

G0 extends the existing `mirbuilder-type-fact-partition` guard rather than
creating a new guard family. It must freeze:

```text
resolved-trivial operation direct writes = 0
prepared decision owners = 1
successful commit consumers = 1
profile representation classifiers added here = 0
origin / final metadata / retry writers added here = 0
all touched source/check files < 800 lines
```

## Parked adjacent writers

These are intentionally not part of this row.

```text
raw Compare:
  cf_common::emit_compare_func may take a void/no-op branch while publishing
  Bool, so COMPARE-RECEIPT0 must establish a physical success receipt first.

raw operator lowering:
  contains call/environment/name-compatible routes and belongs to OPERATOR0.

Array element result:
  array/map metadata annotation currently happens independently of the call
  emission result and needs ARRAY-FACT0 transaction work.

FieldGet:
  typed allocation is pre-emission and FastMem owns a separate fallback.

Call annotation:
  signature/name-compatible annotation and failure timing remain CALL-ANNOTATION0.

LocalSSA and metadata propagation:
  COPY0 is closed; metadata::propagate still mixes type, origin, String, Map,
  Record, and compatibility state under METAPROP0.
```

## Stop conditions

Stop before I0 and open a new design row if any of these become necessary:

```text
an emit_binary caller lacks a sealed profile representation
the destination is not fresh or needs a concrete overwrite
the selected representation conflicts with its emitted instruction law
the conflict needs fallback, retry, or raw routing
origin/metadata/SSA/cache/finalization state must change
raw Compare, raw operator, direct call, FieldGet, or array writers must move
source name, runtime tag, final metadata, or environment policy is needed
a touched source/check file reaches 800 lines
```

## Decision lock

> **RESOLVED-TRIVIAL-OP0 selects one post-success exact-fact producer:
> `trivial_ssa::operation::emit_binary`. The existing verified profile claim is
> the sole source/representation authority; the existing successful `BinOp` or
> `Compare` emission is the sole receipt. A private prepared decision may use
> the existing `TypeFactDecisionV1` to publish only the mapped exact type after
> that receipt, reject a conflicting concrete prestate before emission, and
> leave emission failure with no type, origin, cache, retry, or metadata delta.
> Raw Compare/operator, Array, FieldGet, Call annotation, metadata propagation,
> finalization, and all representation or grammar widening remain parked.**
