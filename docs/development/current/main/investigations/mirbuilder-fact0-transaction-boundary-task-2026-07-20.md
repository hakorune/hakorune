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

## `FACT0-TX0-FIELDGET0-D0` — closed (2026-07-20)

### Decision: Candidate R′ — ordinary FieldGet receipt boundary

The selected owner is one ordinary, non-FastMem source FieldGet path only:

```text
src/mir/builder/fields.rs
  MirBuilder::build_field_access_from_value
  region == None
```

It is neither a generic field transaction nor a generic allocator/rollback
API. It accepts the already-resolved `declared_type`, existing field-origin
facts, and the ordinary field-access observation; it emits exactly the existing
`MirInstruction::FieldGet` and commits the resulting facts only after that
instruction succeeds.

The evidence for the split is exact:

```text
current ordinary order:
  record_field_access_site
  -> declared field lookup
  -> alloc_typed (when declared)
  -> FieldGet emission
  -> publish_field_result_origin

no-current-block failure:
  FieldGet = 0
  -> declared destination type remains
  -> ordinary field-access metadata row remains
  -> field-result origin remains absent
```

The existing success witness proves that a typed `ArrayBox` field reaches the
physical `FieldGet` with `declared_type = Box(ArrayBox)` and remains typed
through finalization. The existing failure witness proves the type residual;
D0 source inspection additionally establishes the retained access-site row.

### Selected receipt law

The future I0 lifecycle is fixed now:

```text
1. resolve existing declared field type and origin/access-site inputs
2. prepare one non-mutating ordinary FieldGet post-success product
3. allocate a fresh ValueId with next_value_id only
4. emit the existing ordinary FieldGet instruction
5. after receipt only:
   a. commit the exact declared type through TypeFactDecisionV1
   b. append the ordinary field-access metadata row
   c. publish the existing field-result origin
6. return the destination
```

For a declared field, the product may make an exact type proposal only for a
non-`Unknown` `MirType`; an absent declared type makes no type proposal. The
current declared-field parser produces concrete representation types, including
`Void`, scalar types, and `Box(name)`, so this is not a new inference rule.

On FieldGet emission failure:

```text
physical FieldGet = 0
destination transient type delta = 0
field-result origin delta = 0
ordinary field-access metadata delta = 0
fallback/retry = 0
```

The fresh ValueId cursor and already-lowered base remain outside rollback. This
is not a whole-Builder transaction.

### Durable S0 product

`FACT0-TX0-FIELDGET0-S0` is the sole next code-facing row. It introduces one
private, non-Clone, Builder-free prepared product, tentatively named
`PreparedOrdinaryFieldGetPostSuccessV1`.

It may own only immutable resolved inputs needed after receipt:

```text
declared type proposal
ordinary field-access-site descriptor
existing field-result origin disposition
```

It may not own a `ValueId`, `MirBuilder`, `TypeContext`, mutable metadata,
MIR instruction, source AST, final metadata, or a new registry. Construction
may snapshot existing source-derived inputs through a thin Builder wrapper, but
the product itself neither reads nor writes Builder state.

### `FACT0-TX0-FIELDGET0-S0` — closed (2026-07-20)

`src/mir/builder/fields/post_success.rs` now owns one private non-Clone
`PreparedOrdinaryFieldGetPostSuccessV1`. It prepares only:

```text
declared exact type -> existing TypeFactDecisionV1 proposal
ordinary access-site receiver/field descriptor
field-result origin disposition
```

It owns no `ValueId`, `MirBuilder`, `TypeContext`, metadata, instruction, or
commit operation. Absent declared type becomes the existing no-publication
decision; declared `Unknown` is rejected; `Void` remains exact. Four focused
unit tests freeze these outcomes. No production caller is connected.

`FACT0-TX0-FIELDGET0-M0` is now the sole next row. It must inventory the
single ordinary `region == None` caller and prove all FastMem/CorePlan/direct
FieldGet routes remain excluded before I0 is considered.

### `FACT0-TX0-FIELDGET0-M0` — closed (2026-07-20)

The future receipt connection has exactly one semantic production owner:

```text
src/mir/builder/fields.rs
  MirBuilder::build_field_access_from_value
  ordinary region == None branch
```

The raw direct entry and compound-assignment read path both reach this same
owner; neither is a second FieldGet fact publisher.  The disconnected S0
product has zero production consumers through M0.

The producer inventory also fixes the following exclusions:

```text
FastMem region != None:
  MemOp::FieldLoad and its missing-declared Integer compatibility write

CorePlan FieldGet:
  normalizer-owned preallocated ValueId/type/origin;
  lowerer emits only the preplanned FieldGet instruction

record/property/helper reads and FieldSet:
  separate instruction and fact owners
```

The direct failure baseline remains intentionally visible until I0:

```text
ordinary typed FieldGet with no current block:
  FieldGet instruction = 0
  destination type residual = 1
  ordinary access-site metadata residual = 1
  field-result origin residual = 0

ordinary untyped FieldGet with no current block:
  FieldGet instruction = 0
  destination type residual = 0
  ordinary access-site metadata residual = 1
  field-result origin residual = 0
```

`FACT0-TX0-FIELDGET0-P0` is now the sole next row. It must freeze the typed
and untyped success/failure temporal matrices, including the retained
pre-I0 residuals, without connecting `PreparedOrdinaryFieldGetPostSuccessV1`
to production.

### `FACT0-TX0-FIELDGET0-P0` — closed (2026-07-20)

The in-process ordinary FieldGet matrix now fixes the current behavior before
the receipt connection changes it:

```text
typed success:
  FieldGet = 1
  destination type = Box(ArrayBox)
  destination origin = ArrayBox
  ordinary access-site metadata = 1

typed no-current-block failure:
  FieldGet = 0
  destination type residual = Box(ArrayBox)
  destination origin residual = 0
  ordinary access-site metadata residual = 1

untyped success:
  FieldGet = 1 with declared_type = None
  destination type/origin = absent
  ordinary access-site metadata = 1

untyped no-current-block failure:
  FieldGet = 0
  destination type/origin residual = 0
  ordinary access-site metadata residual = 1
```

Every ordinary metadata row is checked as a non-FastMem `load` for the exact
base, owner, field, route, and fallback policy. The proof does not connect the
S0 product and leaves production behavior unchanged.

Focused evidence:

```text
cargo test -q --lib temporal_witness  # 16/16
```

`FACT0-TX0-FIELDGET0-I0` is now the sole next row. It may connect exactly the
ordinary `region == None` branch so type, access-site metadata, and origin all
commit after a successful physical `FieldGet`; FastMem and CorePlan remain
excluded.

### `FACT0-TX0-FIELDGET0-I0` — closed (2026-07-20)

The one ordinary `region == None` branch now follows the selected receipt
order:

```text
resolve declared type, receiver owner, and existing field origin
-> prepare PreparedOrdinaryFieldGetPostSuccessV1
-> reserve fresh ValueId
-> emit ordinary FieldGet
-> commit exact type, access-site metadata, and field origin
```

The post-success product remains non-Clone and owns the only receipt commit.
Its type lane consumes the existing `TypeFactDecisionV1`; access-site append
and origin publication use the existing metadata/origin stores only after the
physical instruction succeeds. The product does not retain Builder state or a
ValueId.

The temporal matrix now observes the intended I0 failure law:

```text
typed or untyped ordinary FieldGet with no current block:
  FieldGet instruction = 0
  destination type/origin delta = 0
  ordinary access-site metadata delta = 0
```

FastMem keeps its former pre-emission `MemOp::FieldLoad` and compatibility
write. CorePlan FieldGet remains preplanned and untouched.

Focused evidence:

```text
cargo test -q --lib temporal_witness  # 16/16
cargo test -q --lib post_success      # 4/4
cargo check --all-targets
```

`FACT0-TX0-FIELDGET0-G0` is now the sole next row. It may extend the existing
FACT0 partition guard with one ordinary receipt owner and must reject
pre-emission ordinary type/site/origin writes without adding a guard family.

### `FACT0-TX0-FIELDGET0-G0` — closed (2026-07-20)

The existing `mirbuilder_type_fact_partition_guard.py` now freezes the active
replacement without rewriting the historical `field_collection_unsafe`
inventory:

```text
ordinary receipt prepare consumer = 1
ordinary receipt commit consumer = 1
ordinary post-success exact type decision/commit owner = 1
ordinary post-success access-site/origin commit owner = 1 each
ordinary pre-emission type/site/origin effects = 0
metadata::propagate consumer = 0
FastMem/CorePlan receipt consumers = 0
```

It also enforces the source/check file limit for the ordinary branch, payload,
temporal witness, and shared guard. The active direct-writer inventory records
the one new post-success type owner while retaining FastMem's independent
writer.

Final evidence:

```text
tools/checks/run_row_guard.sh --only mirbuilder-type-fact-partition
python3 tools/checks/lib/mirbuilder_type_fact_partition_guard_tests.py
bash tools/checks/current_state_pointer_guard.sh
cargo test -q --lib temporal_witness
cargo test -q --lib post_success
cargo check --all-targets
```

`FACT0-TX0-FIELDGET0` is complete. The next FACT0 producer family is an
explicit selection frontier: it must not be inferred from the historical
inventory, and this card does not activate FastMem, CorePlan, FieldSet,
metadata propagation, finalization repair, or origin-wide policy.

### Explicit exclusions

```text
FastMem region != None:
  excluded
  MemOp::FieldLoad, note_fastmem_memop, and missing-declared Integer fallback
  remain their own owner

CorePlan FieldGet:
  excluded
  its normalizer preallocates plan ValueIds/types/origins and its lowerer only
  emits a preplanned instruction

FieldSet / weak writes / property reads / record-local reads:
  excluded

field declaration lookup, receiver provenance, metadata::propagate,
finalization repair, TypeFactDecisionV1 semantics, and origin-wide policy:
  unchanged
```

The frozen FACT0 partition profile `field_collection_unsafe` retains its
historical `FACT0-I1-FIELDGET0` prerequisite. G0 may add an active replacement
mapping/guard for this one receipt owner, but must not rewrite that immutable
inventory to hide the old residual.

### Task order

```text
FACT0-TX0-FIELDGET0-D0       closed
  -> FACT0-TX0-FIELDGET0-S0  sole next code-facing row
  -> FACT0-TX0-FIELDGET0-M0
  -> FACT0-TX0-FIELDGET0-P0
  -> FACT0-TX0-FIELDGET0-I0
  -> FACT0-TX0-FIELDGET0-G0
```

M0 must freeze the one ordinary producer and the excluded FastMem/CorePlan
routes. P0 must prove typed and untyped success/failure matrices: typed failure
currently leaves type plus site metadata and no origin; untyped failure leaves
site metadata only; post-I0 failures leave none of those facts. I0 connects
only the `region == None` branch. G0 extends the existing FACT0 partition guard
without a new guard family.

### Stop conditions

Stop and return to consultation if this row needs any of the following:

```text
FastMem, CorePlan, FieldSet, record, or property-read activation
alloc_typed followed by removal / implicit rollback
a generic allocation or FieldGet transaction API
TypeFactDecisionV1 handling Unknown as exact
new persistent ValueId maps or final-metadata authority
field-name, method-name, runtime-tag, or source-path special cases
retry/fallback after failed FieldGet
whole-Builder rollback
source/check file at or above 800 lines
```

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

## Next producer selection: `FASTMEM-RECEIPT0`

### Decision

The explicit next-producer frontier is closed by a three-worker, read-only
inventory. It selects the shared FastMem physical-MemOp receipt as the next
durable slice:

```text
FACT0-TX0-FASTMEM-RECEIPT0-D0    closed
  -> FACT0-TX0-FASTMEM-RECEIPT0-S0    sole next code-facing row
  -> FACT0-TX0-FASTMEM-RECEIPT0-M0
  -> FACT0-TX0-FASTMEM-RECEIPT0-P0
  -> FACT0-TX0-FASTMEM-RECEIPT0-I0
  -> FACT0-TX0-FASTMEM-RECEIPT0-G0

then, separately:
  FASTMEM-FIELDLOAD0-D0
```

The selected physical owner is exactly:

```text
MirBuilder::emit_fastmem_memop
  -> validate the current function and exact region
  -> emit one physical MemOp
  -> publish one emitted_memop_count receipt
```

Today that helper calls `note_fastmem_memop` before `emit_instruction`. A
missing current block therefore leaves `emitted_memop_count += 1` with no
physical `MemOp`. The same helper is the canonical endpoint for the FastMem
value facade, FastMem intrinsic calls, field stores, index stores, and the
FastMem FieldLoad branch. This is one receipt fact with one physical owner;
its correction does not choose a result-type, origin, or source-shape policy.

### Receipt law

`FASTMEM-RECEIPT0` must preserve current preflight errors while splitting
validation from publication:

```text
1. validate current function and exact registered region, mutation = 0
2. prepare one private non-Clone MemOp receipt for that region
3. emit the existing physical MemOp
4. only on success, increment emitted_memop_count exactly once
```

Failure law:

```text
physical MemOp = 0
emitted_memop_count delta = 0
retry/fallback = 0
```

This row does not claim ValueId cursor rollback or whole-Builder rollback.
It does not move the existing field-access site, typed destination reservation,
missing-declared `Integer` compatibility write, or field-result origin in the
FastMem FieldLoad branch. Those are the separate `FASTMEM-FIELDLOAD0-D0`
frontier after the generic MemOp receipt is green.

### Candidate disposition

```text
selected:
  shared FastMem emitted_memop_count receipt

parked:
  FastMem FieldLoad type/site/origin reservation
  Array write observation before known ArrayElementWrite
  Array element-result / receiver-chain facts
  CorePlan preplanned FieldGet
  metadata propagation, finalization repair, direct Copy, unary/operator
```

Array write is not selected even though its physical `ArrayElementWrite`
already has a receipt-ordered `Void` publication: its Array observation is
performed before final operand/call emission in a distinct call-owner seam.
Joining it to this generic MemOp row would mix two durable semantic slices.

### `FACT0-TX0-FASTMEM-RECEIPT0-S0`

```text
production behavior delta = 0
production consumers = 0
```

Add only a private preflight/prepared-receipt vocabulary. It must retain the
validated `FastMemRegionId` and expose neither `MirBuilder`, metadata, a
mutable region reference, nor a type/origin/result policy. The constructor
must perform no metadata write; the commit API must be non-fallible and remain
unconnected in S0.

M0 inventories every existing `emit_fastmem_memop` facade and records the
pre-I0 missing-current-block counter residual. P0 adds success/failure temporal
proof for direct helper and representative facade paths. I0 connects only the
shared helper, preserving validation error order and moving exactly the counter
increment after successful `emit_instruction`. G0 extends the existing FACT0
partition guard; it must not add a new guard family.

### `FACT0-TX0-FASTMEM-RECEIPT0-S0` — closed (2026-07-20)

`src/mir/builder/fastmem/receipt.rs` now owns one private,
non-Clone `PreparedFastMemMemOpReceiptV1`. Its pure `prepare` path validates
only the existing current-function and registered-region law, retaining the
same stable error spelling without mutating region metadata. Its non-fallible
`commit` holds the sole future increment and is intentionally unconnected from
production emission in S0.

Focused tests prove:

```text
valid preparation changes emitted_memop_count by 0
commit changes it by exactly 1
missing function and unknown region reject with metadata delta 0
```

No `MirBuilder` reference, metadata reference, value type/origin policy, or
instruction is retained in the receipt. The legacy `note_fastmem_memop` remains
the only production timing owner until I0. `FACT0-TX0-FASTMEM-RECEIPT0-M0` is
now the sole next row.

### Stop conditions

Stop this row if it requires any of the following:

```text
FastMem FieldLoad type/site/origin movement
FastMem region registration or region metadata schema change
source AST, field/method name, runtime tag, or final metadata as authority
new persistent ValueId maps
generic FieldGet/Call transaction API
Array or Map observation activation
retry, fallback, or whole-Builder rollback
source/check file at or above 800 lines
```
