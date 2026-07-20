---
Status: Active D-prime task
Date: 2026-07-20
Scope: decide the receipt-versus-reservation boundary before normal Call or FieldGet fact migration
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessors:
  - docs/development/current/main/investigations/mirbuilder-copy-unknown-origin-task-2026-07-20.md
  - docs/development/current/main/investigations/mirbuilder-compare-emission-receipt-task-2026-07-20.md
---

# FACT0-TX0: separate instruction receipts from pre-emission reservations

## Decision

`FACT0-TX0-D0` is closed by the current source inventory and temporal
witnesses. It selects no generic transaction object, no type publisher, and no
production cutover. Its purpose is to stop Call and FieldGet from being treated
as variations of one simple exact-fact producer.

The two observed residuals have opposite timing and therefore distinct future
owners:

```text
generic unified Call:
  physical Call emission may fail
  -> signature / Array / Map result annotation still runs
  -> post-failure transient fact can remain

ordinary typed FieldGet:
  typed destination is allocated before FieldGet emission
  -> FieldGet emission may fail
  -> pre-emission transient type can remain
```

Neither behavior may be silently normalized by `TypeFactDecisionV1`, final
metadata, a generic post-call hook, or a shared rollback claim. The only
selected immediate executable child is the narrow Call receipt repair:

```text
FACT0-TX0-D0                 closed
  -> FACT0-TX0-CALL-RECEIPT0-S0        sole next code-facing row
  -> FACT0-TX0-CALL-RECEIPT0-M0
  -> FACT0-TX0-CALL-RECEIPT0-P0
  -> FACT0-TX0-CALL-RECEIPT0-I0
  -> FACT0-TX0-CALL-RECEIPT0-G0

then (separate design stop):
  FACT0-TX0-FIELDGET0-D0
```

`FACT0-TX0-FIELDGET0`, FastMem, ArrayWrite, direct-call materialization,
legacy unified-off routing, finalization repair, metadata propagation, and
result-policy cleanup remain independent parked owners.

## D0 evidence and authority split

### Call: one canonical receipt-order defect

`UnifiedCallEmitterBox::emit_unified_call_impl` constructs the final
`MirInstruction::Call`, calls `builder.emit_instruction(call_inst)`, then
unconditionally performs all of the following before returning that result:

```text
signature/name result annotation
Array get/pop/remove result annotation
Map get result annotation
post-call schedule observation
```

The existing temporal witness proves the defect without a source workaround:

```text
current block absent
  -> Call instruction count = 0
  -> emit returns "No current basic block"
  -> destination transient type = Integer
```

The selected Call law is consequently order-only:

```text
successful canonical generic Call receipt
  -> existing annotation owners
  -> existing post-call observation
```

The existing annotation, Array, and Map modules retain their own type, origin,
registry, lookup, and compatibility policies.
`FACT0-TX0-CALL-RECEIPT0` may defer their existing invocation, but may not
rewrite them.

### FieldGet: a distinct reservation transaction

Ordinary typed FieldGet allocates a typed destination before emitting its
physical `FieldGet`, records field-access metadata before that allocation, and
publishes field origin only after successful emission. The existing temporal
witness proves that a failed FieldGet can retain the allocated type without an
instruction. This is a reservation/rollback question, not a Call-style
post-success annotation question.

FastMem is explicitly outside ordinary FieldGet: it emits `MemOp::FieldLoad`
and has an independent Integer fallback. CorePlan preallocated FieldGet values
are another separate pre-emission route. Neither is a
`FACT0-TX0-CALL-RECEIPT0` consumer.

## `FACT0-TX0-CALL-RECEIPT0` exact contract

### Selected producer

Exactly one producer is eligible:

```text
src/mir/builder/calls/unified_emitter.rs
  UnifiedCallEmitterBox::emit_unified_call_impl
```

It is the canonical generic unified Call endpoint after final callee and
operand normalization. The selected result is not a generic all-Call policy.

### Receipt transaction

```text
1. finalize existing callee and operands
2. build the existing Call instruction
3. prepare a private, non-mutating post-success payload
4. emit the Call through the existing Builder emitter
5. only on success, consume the payload once:
     signature/name annotation
     Array result annotation
     Map result annotation
     verify_after_call observation
6. return success
```

On an emission error:

```text
Call instruction publication = 0
annotation type/origin/registry delta = 0
post-call observation = 0
fallback/retry = 0
```

The `ValueId` cursor and already-finalized operands are not newly claimed to
roll back. This is not whole-Builder rollback.

### Authority table

| Concern | Authority |
| --- | --- |
| final callee and operands | existing unified emitter normalization |
| physical Call receipt | existing `MirBuilder::emit_instruction` result |
| signature/name annotation policy | existing `calls::annotation` owner |
| Array result policy | existing `types::array_element` owner |
| Map result policy | existing `types::map_value` owner |
| post-call schedule observation | existing `emit_guard::verify_after_call` |
| failure disposition | `emit_instruction` error; no alternate route |

Non-authorities:

```text
source or method spelling
runtime tag
finalized function metadata
TypeFactDecisionV1
new ValueId-to-type/origin maps
CallMaterializer, direct resolved Call, raw unified-off path
```

## Task rows

### `FACT0-TX0-CALL-RECEIPT0-S0`

```text
production behavior delta = 0
production consumers = 0
```

Add one private, non-Clone, non-mutating prepared post-success Call payload.
It may retain only final existing invocation descriptors needed by the three
existing annotation owners and the existing schedule observer. Its constructor
may not mutate a Builder, inspect final metadata, perform annotation, or issue
an instruction.

### `FACT0-TX0-CALL-RECEIPT0-S0` — closed (2026-07-20)

`src/mir/builder/calls/unified_emitter/post_success.rs` now owns one private
non-Clone `PreparedUnifiedCallPostSuccessV1`. It is constructed only from the
already-finalized `Callee`, destination, and argument slice, and retains:

```text
optional existing signature-annotation descriptor
optional existing Array/Map result-annotation descriptor
```

It has no `MirBuilder`, type/origin/registry mutation, instruction, final
metadata lookup, annotation invocation, or commit API. Three focused tests
freeze global arity, explicit-receiver method arity, and no-destination
absence. `FACT0-TX0-CALL-RECEIPT0-M0` is now the sole next row.

### `FACT0-TX0-CALL-RECEIPT0-M0`

Freeze the exact canonical consumer inventory:

```text
selected canonical generic unified emitter = 1
already receipt-ordered direct annotated paths = unchanged
CallMaterializer = excluded
resolved direct call = excluded
unified-off legacy route = excluded
```

Prove payload preparation occurs after final operand/callee normalization and
before Call emission, while every payload effect occurs only after success.

### `FACT0-TX0-CALL-RECEIPT0-M0` — closed (2026-07-20)

The canonical consumer inventory is fixed:

```text
post-success payload producer = 1
  unified_emitter::emit_unified_call_impl

post-success payload consumers before I0 = 0
canonical future receipt consumer at I0 = 1

emit_global_unified = existing receipt-ordered compatibility entry, excluded
CallMaterializer = existing receipt-ordered direct entry, excluded
rewrite and method terminals = propagate unified-call error before annotation
resolved direct Call = sealed independent producer, excluded
unified-off legacy route = excluded
```

The canonical generic emitter finalizes `callee` and `args_local`, prepares
signature and collection descriptors from exactly that final shape, constructs
the Call, and currently invokes its annotation package regardless of the
emission `Result`. The pre-I0 temporal witness remains the proof of that last
residual. `FACT0-TX0-CALL-RECEIPT0-P0` is now the sole next row; it may add
only failure/success observation fixtures, with zero production connection.

### `FACT0-TX0-CALL-RECEIPT0-P0`

Retain the current failure witness as the pre-I0 baseline, then prove the
post-I0 matrix:

```text
no current block:
  Call/type/origin/registry/observation delta = 0

successful signature result:
  existing type parity

successful Array and Map result:
  existing type/origin/registry parity
```

### `FACT0-TX0-CALL-RECEIPT0-P0` — closed (2026-07-20)

The temporal witness now fixes all three pre-I0 residual families under the
canonical unified emitter:

```text
signature result:
  failed Call -> Integer residual

ArrayBox.get:
  failed Call -> Array element Integer residual

MapBox.get with a tracked literal key:
  failed Call -> Map value Integer residual
```

Every baseline case has `Call count = 0` and the same `No current basic block`
receipt failure. The successful signature case remains the parity witness for
ordinary metadata finalization. These are pre-I0 observations, not an accepted
failure behavior. `FACT0-TX0-CALL-RECEIPT0-I0` is now the sole next row and
must invert all three failed-call residual assertions without changing their
successful annotation policies.

### `FACT0-TX0-CALL-RECEIPT0-I0`

Connect exactly one canonical consumer:

```rust
builder.emit_instruction(call_inst)?;
prepared.commit_after_success(builder);
Ok(())
```

`verify_after_call` belongs inside that successful commit. No annotation policy
may move, and no selected failure may retry raw emission.

### `FACT0-TX0-CALL-RECEIPT0-I0` — closed (2026-07-20)

The one canonical generic unified emitter now prepares its existing annotation
descriptors after callee/operand normalization, emits the physical `Call`, and
only then consumes the private post-success payload. The payload invokes the
same signature, Array, Map, and schedule owners as before; it changes only
their receipt order.

Focused temporal witnesses prove:

```text
failed signature / ArrayBox.get / MapBox.get Call:
  Call count = 0
  destination transient type = absent

successful signature / ArrayBox.get / MapBox.get Call:
  Call count = 1
  existing Integer annotation retained
```

No selected failure retries, writes a type/origin/registry fact, or invokes
post-call schedule observation. `FACT0-TX0-CALL-RECEIPT0-G0` is now the sole
next row and may add only the existing FACT0 partition guard coverage.

### `FACT0-TX0-CALL-RECEIPT0-G0`

Extend the existing FACT0 partition guard; do not add a guard family. Freeze:

```text
canonical receipt-order consumer = 1
annotation/Array/Map invocation before successful Call = 0
post-call verification before successful Call = 0
new type/origin map = 0
files at or above 800 lines = 0
```

### `FACT0-TX0-CALL-RECEIPT0-G0` — closed (2026-07-20)

The existing `mirbuilder_type_fact_partition_guard.py` now owns the receipt
boundary without adding a guard family. It requires exactly one canonical
payload prepare/consume pair, rejects signature/Array/Map annotation and
post-call verification inside the canonical emitter, and requires their one
existing invocation each inside the post-success owner. It also keeps the
excluded `emit_global_unified` compatibility path outside this consumer count
and enforces the four touched source/check files below 800 lines.

Final evidence:

```text
mirbuilder_type_fact_partition_guard.py: green
current_state_pointer_guard.sh: green
temporal unified-emitter tests: 14/14 green
post-success payload tests: 3/3 green
cargo check --all-targets: green
```

`FACT0-TX0-CALL-RECEIPT0` is complete. The sole next blocker is the separate
`FACT0-TX0-FIELDGET0-D0` design stop; it must decide ordinary typed FieldGet
reservation semantics before any FieldGet code-facing row opens.

## Stop conditions

Stop and open a new design consultation if any one of these is needed:

1. annotation, Array, or Map policy must change rather than merely defer;
2. CallMaterializer, resolved direct Call, or unified-off routing must join;
3. a FieldGet reservation or FastMem rule is required;
4. a Call failure must publish a fact, take a fallback, or retry;
5. a final-metadata read, name heuristic, runtime tag, or source-site rule
   becomes type authority;
6. a shared generic Call/FieldGet transaction API is required before the
   one-consumer receipt can be expressed;
7. a touched source/check file reaches 800 lines.

## Claims

After `FACT0-TX0-CALL-RECEIPT0-G0`, the compiler may claim only that the canonical
generic unified Call invokes its already-existing post-call annotation and
verification owners after a successful physical Call. It may not claim generic
Call completion, all result facts monotone, FieldGet rollback, Array/Map policy
cleanup, finalization retirement, or whole-Builder transactionality.
