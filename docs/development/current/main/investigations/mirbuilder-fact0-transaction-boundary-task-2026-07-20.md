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

### `FACT0-TX0-FASTMEM-RECEIPT0-M0` — closed (2026-07-20)

The canonical physical path is fixed without expanding the selected owner:

```text
physical MemOp receipt owner = 1
  MirBuilder::emit_fastmem_memop

direct helper call sites = 8
  value facade = 1
  FastMem FieldLoad/FieldStore = 2
  FastMem index store = 1
  FastMem intrinsic direct-effect branches = 4
```

Both value facade methods reduce to that same helper. `note_fastmem_memop`
performs the existing current-function/region preflight and increments the
region counter before the following `emit_instruction(MemOp)`. Therefore a
missing current block has a stable pre-I0 shape: the physical MemOp is absent,
but the counter is one larger. No caller-specific source policy, result fact,
or FastMem FieldLoad reservation joins this inventory.

`FACT0-TX0-FASTMEM-RECEIPT0-P0` is now the sole next row.

### `FACT0-TX0-FASTMEM-RECEIPT0-P0` — closed (2026-07-20)

Receipt-local temporal tests now freeze the pre-I0 baseline:

```text
direct successful MemOp:
  physical MemOp = 1
  emitted_memop_count = 1

direct helper with no current block:
  physical MemOp = 0
  emitted_memop_count = 1     # pre-I0 residual

value facade with no current block:
  physical MemOp = 0
  emitted_memop_count = 1     # same shared-owner residual
```

The same test module retains S0 preflight proof: preparation has counter delta
zero, while its isolated test-only commit increments exactly once; missing
function and unknown-region preparation leave metadata unchanged. P0 does not
connect the prepared receipt to production emission. The next row is therefore
the one-consumer I0 cutover.

`cargo test -q --lib fastmem` has two independent baseline-red expectations and
is not an acceptance gate for this receipt row. Both tests predate the existing
FastMem `MemOp::FieldLoad`/`FieldStore` route: one expects the full MemOp list
to contain only `TableIndex`, and the other expects ordinary `FieldGet`/
`FieldSet` inside a FastMem region. The focused `fastmem::receipt` module is
green (6/6), and `cargo check --all-targets` is green. Reconcile those stale
FastMem behavior fixtures only in a separate maintenance row after receipt G0;
do not change their source/route expectations in this timing-only row.

### `FACT0-TX0-FASTMEM-RECEIPT0-I0` — closed (2026-07-20)

`MirBuilder::emit_fastmem_memop` is now the sole production consumer of the
prepared receipt. Its fixed order is:

```text
prepare current-function/registered-region receipt
-> emit existing MemOp
-> commit emitted_memop_count
```

The legacy `note_fastmem_memop` pre-emission counter writer is removed. The
same helper still owns both direct MemOp and value-facade paths, so a failed
emission now leaves both paths with zero physical MemOps and zero counter
receipts. Successful direct emission remains one physical MemOp and one
receipt. No FieldLoad type/site/origin reservation, region schema, caller
policy, fallback, or retry changed. `FACT0-TX0-FASTMEM-RECEIPT0-G0` is now the
sole next row.

### `FACT0-TX0-FASTMEM-RECEIPT0-G0` — closed (2026-07-20)

The existing `mirbuilder-type-fact-partition` guard now seals the selected
timing owner without creating a guard family:

```text
shared preparation consumer = 1
shared post-emission commit consumer = 1
legacy pre-emission counter owner = 0
counter increment owner = 1
commit follows physical MemOp = 1
```

`cargo test -q --lib fastmem::receipt` passes 6/6, the partition guard and its
unit suite pass, and `cargo check --all-targets` passes. The two known full
FastMem stale expectations remain explicitly parked and are not masked. The
next frontier is `FASTMEM-FIELDLOAD0-D0`: select its authority boundary before
any new implementation row.

## Next producer selection: `FASTMEM-FIELDLOAD0`

### Decision

`FASTMEM-FIELDLOAD0-D0` selects Candidate C-prime: one
**behavior-preserving FieldLoad reservation/completion lifecycle**. It does
not reclassify every FastMem FieldLoad fact as a post-success receipt.

```text
FastMem FieldLoad
  pre-emission reservation:
    exact FastMem access-site
    declared destination type, when present

  physical owner:
    existing emit_fastmem_memop(FieldLoad)

  post-success completion:
    missing-declared Integer compatibility
    existing field-result origin publication
```

This is the narrowest truthful boundary. The current FastMem arm is not an
ordinary FieldGet: it records its layout-verified field site and, when a
declared type exists, reserves that destination type before physical MemOp
emission. A failed MemOp therefore already retains those two reservations,
while the missing-declared Integer compatibility entry and field-result origin
remain absent. Moving every effect after emission would be a behavior-changing
receipt redesign, not a FACT0 timing cleanup.

### Authority and non-authority

```text
selected owner:
  MirBuilder::build_field_access_from_value, region != None arm only

declared type:
  existing declared_field_type_for_value

pre-emission site:
  existing record_field_access_site

physical instruction and region counter:
  existing emit_fastmem_memop / FASTMEM-RECEIPT0

missing-declared Integer completion:
  existing FastMem FieldLoad compatibility law

field-result origin:
  existing publish_field_result_origin
```

Not authority in this row:

```text
ordinary PreparedOrdinaryFieldGetPostSuccessV1
TypeFactDecisionV1 for missing -> Integer compatibility
FieldStore or indexing FieldStore
generic FastMem value-MemOp result typing
region/layout metadata schema or verification
CorePlan FieldGet, metadata::propagate, finalization repair
general origin monotonicity or ValueId rollback
```

### Exact task order

```text
FACT0-TX0-FASTMEM-FIELDLOAD0-D0   closed
  -> FACT0-TX0-FASTMEM-FIELDLOAD0-S0
  -> FACT0-TX0-FASTMEM-FIELDLOAD0-M0
  -> FACT0-TX0-FASTMEM-FIELDLOAD0-P0
  -> FACT0-TX0-FASTMEM-FIELDLOAD0-I0
  -> FACT0-TX0-FASTMEM-FIELDLOAD0-G0
```

`S0` adds one private Builder-free prepared lifecycle vocabulary with zero
production consumers or writes. `M0` freezes the direct owner, two semantic
call entrances, and the five observable timing states. `P0` proves declared
and missing declaration success/failure states without using the two known
stale full-FastMem expectations. `I0` connects only the selected FieldLoad arm
and preserves its reservation/completion ordering. `G0` extends the existing
FACT0 partition guard; it creates no guard family.

### Required timing matrix

```text
record-site failure:
  no destination/type/MemOp/origin

declared T + MemOp failure:
  site + destination T reservation
  no physical MemOp, completion, or origin

missing declaration + MemOp failure:
  site + destination allocation
  no type entry, physical MemOp, completion, or origin

declared T + success:
  site + T reservation + physical MemOp + origin-if-known

missing declaration + success:
  site + physical MemOp + Integer compatibility + origin-if-known
```

The ValueId cursor is outside this matrix. Both public entrances remain
delegates into the same semantic owner:

```text
build_field_access
  -> build_field_access_from_value

compound-place read
  -> build_field_access_from_value
```

### Stop conditions

Stop and reopen the design boundary if this row requires any of the following:

```text
moving the site or declared reservation after MemOp emission
missing -> Integer through exact-type authority
changing FASTMEM-RECEIPT0 or physical MemOp ownership
FieldStore, index store, generic value-MemOp, or ordinary FieldGet inclusion
region/layout schema or source/runtime/final-metadata inference
new persistent ValueId/type/origin maps
fallback, retry, whole-Builder rollback, or a source/check file >= 800 lines
```

### `FACT0-TX0-FASTMEM-FIELDLOAD0-S0` — closed (2026-07-20)

`src/mir/builder/fastmem/field_load.rs` now owns one private, Builder-free,
non-Clone `PreparedFastMemFieldLoadLifecycleV1`. It snapshots only:

```text
FastMem load-site reservation descriptor
declared type reservation: absent | stored MirType
missing-declared completion: inactive | Integer compatibility
field-result origin completion: absent | publish class
```

The disconnected payload writes no type, site, origin, metadata, instruction,
or cache. Its tests retain declared exact, declared `Unknown`, and
missing-declared cases so `Unknown` is not accidentally promoted into an exact
fact law. The existing production branch remains untouched. M0 is the sole
next row.

### `FACT0-TX0-FASTMEM-FIELDLOAD0-M0` — closed (2026-07-20)

The production inventory is exactly:

```text
semantic FieldLoad owner = 1
  build_field_access_from_value, region != None

semantic entrances = 2
  ordinary field-access descent
  compound-place read descent

physical MemOp/counter owner = 1
  emit_fastmem_memop / FASTMEM-RECEIPT0
```

No second FieldLoad publisher exists. The selected owner keeps the current
five-state split: site reservation always precedes allocation; declared type
reservation precedes physical emission; missing-declared Integer and origin
completion follow only successful emission. `FieldStore`, including the index
store shape, has a different transaction and remains excluded. P0 is the sole
next row.

### `FACT0-TX0-FASTMEM-FIELDLOAD0-P0` — closed (2026-07-20)

Focused temporal tests freeze the four direct states at the selected owner:

```text
declared FieldLoad failure:
  site + declared Box reservation
  no FieldLoad, region receipt, or result origin

missing-declared FieldLoad failure:
  site only
  no type completion, FieldLoad, region receipt, or result origin

declared FieldLoad success:
  site + declared reservation + FieldLoad + region receipt + origin

missing-declared FieldLoad success:
  site + Integer compatibility + FieldLoad + region receipt
```

The tests intentionally observe no ValueId cursor rollback. They are separate
from the two stale full-FastMem behavior expectations. I0 is the sole next
row.

### `FACT0-TX0-FASTMEM-FIELDLOAD0-I0` — closed (2026-07-20)

The selected FastMem FieldLoad arm now consumes the prepared lifecycle in its
existing observable order:

```text
prepare resolved site/type/origin dispositions
-> reserve layout site
-> allocate fresh destination
-> reserve declared type, when present
-> existing emit_fastmem_memop(FieldLoad)
-> complete missing Integer compatibility and result origin
```

No ordinary FieldGet, FieldStore, index store, generic value-MemOp, region
schema, or physical receipt policy was touched. The old direct
`publish_field_result_origin` branch helper is gone: origin disposition is
snapshotted before emission and the selected lifecycle publishes it only after
the existing physical receipt succeeds. Declared `Unknown` remains a stored
pre-emission reservation rather than entering an exact-fact decision.

### `FACT0-TX0-FASTMEM-FIELDLOAD0-G0` — closed (2026-07-20)

The existing FACT0 partition guard now seals this lifecycle without adding a
guard family:

```text
selected FieldLoad lifecycle consumer = 1
direct fields.rs lifecycle effects in selected arm = 0
site reservation owner = 1
declared/integer type lanes = 2
origin completion owner = 1
reservation -> MemOp -> completion order = fixed
```

The active type-writer inventory deliberately removes the former `fields.rs`
direct writer and records the two scoped lifecycle lanes in
`fastmem/field_load.rs`; it is not a new persistent map or a general type
authority. Focused lifecycle tests pass 3/3, timing tests pass 4/4, FastMem
receipt tests pass 6/6, the partition guard and its unit suite pass, and
`cargo check --all-targets` passes. The formerly stale full-FastMem
expectations are closed by the maintenance row below. Return to the explicit
`ARRAY-WRITE-OBSERVE0-D0` selection frontier before opening another code row.

## `FASTMEM-EXPECT0`: physical-surface expectation maintenance

### `FASTMEM-EXPECT0-D0/I0/G0` — closed (2026-07-20)

This maintenance row changes no compiler behavior. Two old test expectations
still described FastMem region field access as ordinary `FieldGet`/`FieldSet`,
despite the existing FastMem route having retired those instructions in favor
of `MemOp::FieldLoad`/`MemOp::FieldStore`.

The authority is only the emitted physical FastMem surface plus existing
FastMem access metadata:

```text
region layout fixture:
  TableIndex(page_table)
  -> FieldLoad(owner_id)
  -> FieldStore(local_free_head)

owner-equality branch fixture:
  one FieldLoad(owner_worker_id)
  one FieldStore(used)

legacy FieldGet/FieldSet for those fields:
  zero
```

Only `fastmem/tests/region.rs` and `fastmem/tests/branch.rs` change. No
MirBuilder, route, receipt, FieldLoad lifecycle, type, origin, metadata,
runtime, backend, or guard policy changes. The focused witnesses and
`cargo test -q --lib fastmem` pass 91/91.

## Next producer selection: `ARRAY-WRITE-OBSERVE0-D0`

Three independent read-only inventories select one design stop before another
producer cutover. `emit_array_element_write` already publishes its Void result
after successful physical emission; the residual is earlier:

```text
observe_array_write_call
  -> receiver Array<T>/Unknown and copy-chain observation
  -> later LocalSSA/materialization
  -> physical ArrayElementWrite or generic Call
```

The observation appears before physical work in the unified known-Array path,
the BoxCall specialization, and the generic unified path. A later failure can
therefore retain the receiver fact. The next D0 must decide whether the first
admission may isolate one canonical known-`ArrayElementWrite` route. It must
not fold generic Call, Map observation, FieldStore/index reservations, or
FastMem receipts into the same row.

```text
ARRAY-WRITE-OBSERVE0-D0
  -> select one pre-emission observation authority and failure boundary
  -> then, and only then, open its code-facing S0
```

### `ARRAY-WRITE-OBSERVE0-D0` — closed (2026-07-20)

Candidate A-prime is selected: **post-success observation at the two existing
known-ArrayElementWrite specializations**. `observe_array_write_call` remains
the sole owner of receiver-local `Array<T>`/`Unknown` and copy-chain facts; the
row changes only when its existing call is made.

```text
selected callers:
  UnifiedCallEmitter ArrayBox direct specialization
  BoxCall ArrayBox direct specialization

selected physical receipt:
  existing try_emit_known_array_method_write
  -> existing emit_array_element_write

success:
  physical ArrayElementWrite
  -> existing observer on the semantic source receiver/arguments

failure or non-write:
  observer delta = 0
  existing non-write route continues unchanged
```

The observer may not move inside `try_emit_known_array_method_write`: the
BoxCall route deliberately lowers a physical LocalSSA receiver while the
observer must retain the original semantic receiver and argument values. The
generic unified Call observer remains pre-finalization and outside this first
row; changing it would mix generic Call receipt, Map observation, and an
independent failure boundary.

```text
ARRAY-WRITE-OBSERVE0-I0
  -> move the two selected observer invocations after a true physical-write result
  -> add success/failure temporal witnesses and direct-route parity
  -> extend the existing FACT0 partition guard only if the owner count needs it

ARRAY-WRITE-OBSERVE0-G0
  -> prove exactly two selected post-success consumers
  -> prove generic Call/Map, FieldStore/index, and FastMem remain excluded
```

Stop if preserving the semantic source receiver requires a second fact map,
moving generic Call observation, deriving facts from physical receiver copies,
or a fallback/retry after physical emission failure.

### `ARRAY-WRITE-OBSERVE0-I0` — closed (2026-07-20)

The two selected specializations now invoke the existing observer only after
`try_emit_known_array_method_write` returns `true`. The unified path retains
its exact `Callee` and original arguments; BoxCall retains its separately
captured semantic receiver/arguments while its physical route may use LocalSSA
materialized values. No Array fact policy moved or changed.

One dedicated 146-line temporal witness module proves both selected callers:

```text
physical write failure:
  ArrayElementWrite = 0
  receiver remains Box(ArrayBox)

physical write success:
  ArrayElementWrite = 1
  semantic receiver becomes Array(Integer)
```

### `ARRAY-WRITE-OBSERVE0-G0` — closed (2026-07-20)

The existing FACT0 partition guard now seals the narrow structural boundary:

```text
unified direct post-success observer consumers = 1
BoxCall direct post-success observer consumers = 1
generic unified observer consumers = 1, unchanged
generic Call/Map, FieldStore/index, FastMem = excluded
```

Focused temporal tests pass 6/6, the reusable partition guard and its unit
suite pass, `cargo check --all-targets` passes, and every touched source/check
file remains below 800 lines. Return to `NEXT-PRODUCER-D0` before selecting the
next fact producer.

## Next producer selection: `MAP-WRITE-OBSERVE0-D0`

Three independent, read-only inventories select Map write observation as the
next residual receipt seam. `observe_map_write_call` owns the existing
receiver-local `map_value_types` and `map_literal_value_types` policy for
`Set`, `Delete`, and `Clear`, but its two callers currently run before the
physical `Call` can fail:

```text
direct Unified:
  observe semantic source S
  -> LocalSSA/finalize exposes the fact on final receiver R
  -> physical Call

terminal BoxCall:
  capture semantic S
  -> LocalSSA L
  -> observe S
  -> physical Call

BoxCall -> Unified:
  capture semantic S
  -> LocalSSA L
  -> observe S
  -> delegate
  -> Unified observes L
  -> physical Call
```

A naive one-site move would change current receiver-keyed facts. The first
admission must preserve the existing route-specific observation sequence, but
make every observation conditional on the one successful physical Call receipt.

### `MAP-WRITE-OBSERVE0-D0` — selected (2026-07-20)

Candidate C-prime is selected: **a private, non-Clone, Builder-free prepared
Map write replay chain**. It carries only existing semantic call descriptors
and reuses the existing `observe_map_write_call` owner after success. It does
not create an alias map, infer Map identity, or replace the existing map fact
policy.

```text
terminal BoxCall receipt:
  physical Call success -> replay S

direct Unified receipt:
  physical Call success -> replay S -> replay R only when R != S

BoxCall -> Unified receipt:
  physical Call success -> replay outer S -> replay delegated L
  -> replay final R only when R != L
```

The order is the existing fact-observation order, not a new route or runtime
order. A failed Call leaves both map-fact maps unchanged.

| Concern | Authority |
| --- | --- |
| Map Set/Delete/Clear fact policy | existing `types::map_value::observe_map_write_call` |
| semantic source descriptor S | existing pre-LocalSSA inputs |
| delegated descriptor L | existing BoxCall LocalSSA output/delegated Unified input |
| final receiver R | existing Unified finalized callee/arguments |
| replay schedule and duplicate suppression | one prepared receipt chain |
| physical success | existing `MirBuilder::emit_instruction(Call)` result |

Non-authorities:

```text
Map Get result annotation
method/receiver-name or runtime-tag inference
new persistent ValueId alias/type/origin maps
LocalSSA propagation policy
generic Call receipt API, router/environment policy
FieldStore/index, Array, FastMem, final metadata, fallback, retry
```

### `MAP-WRITE-OBSERVE0-S0` — sole next code-facing row

```text
production behavior delta = 0
production consumers = 0
Builder/type/origin/map-fact writes = 0
```

Add one small private prepared descriptor/replay vocabulary next to
`map_value.rs`. It may accept only existing `MapBox` Set/Delete/Clear call
descriptors; retain S/L/R as owned existing `Callee` plus argument descriptors;
encode the three schedules above; and reject malformed construction before any
commit exists. It performs no Builder mutation, LocalSSA, routing, emission,
or Map Get work.

M0 freezes the direct/terminal/delegated route inventory. P0 supplies Map
Set/Delete/Clear success/failure witnesses before I0 connects the two existing
physical emission endpoints.

### `MAP-WRITE-OBSERVE0-S0` — closed (2026-07-20)

`src/mir/builder/types/map_value/post_success.rs` now owns the disconnected
non-Clone replay product. It has no Builder, type/origin/map-fact map,
LocalSSA, router, physical Call, or Map Get dependency. Its only retained
payload is the existing `Callee` plus argument descriptors, classified through
the existing `MapMethodId` surface.

Focused structural proof fixes:

```text
accepted: Set, Delete/remove, Clear
excluded: Get and non-Method call shapes
direct Unified schedule: S -> R
delegated schedule: S -> L -> R
equal adjacent receiver: one observation
operation drift: typed rejection before a replay exists
production consumers/map-fact writes: 0
```

`cargo test -q --lib types::map_value::post_success::tests` and
`cargo check --all-targets` are green. All added source remains below 800
lines. `MAP-WRITE-OBSERVE0-M0` is now the sole next row: it must freeze the
actual route and timing inventory without connecting this product to
production.

### `MAP-WRITE-OBSERVE0-M0` — closed (2026-07-20)

The production route inventory has exactly two physical Call receipt owners
and three pre-I0 observation shapes:

```text
direct Unified:
  observe S
  -> finalize_call_operands materializes R and propagates existing map facts
  -> emit Call

terminal BoxCall:
  materialize L
  -> observe S
  -> emit Call

BoxCall -> Unified:
  materialize L
  -> observe S
  -> delegate
  -> observe L
  -> finalize to R when distinct
  -> emit Call
```

The sole existing physical receipt is `emit_instruction(MirInstruction::Call)`.
`metadata::propagate` is an existing LocalSSA fact transfer, not a new Map
authority. Map Get, router policy, environment selection, Array/FastMem,
FieldStore/index, final metadata, and all non-Call routes remain outside the
row.

### `MAP-WRITE-OBSERVE0-P0` — closed (2026-07-20)

Seven focused baseline witnesses in
`calls/unified_emitter/map_write_timing_tests.rs` freeze both the current
success coverage and the exact failure residual to remove at I0:

```text
direct Unified Set failure:
  Call = 0, but S receives Integer map facts

direct Unified Delete/Clear failure:
  Call = 0, but seeded facts are removed

direct Unified success:
  Set preserves S plus finalized LocalSSA receiver coverage
  Delete/Clear remove facts after one Call

terminal BoxCall (unified mode explicitly disabled):
  Set failure leaves S facts; success observes S only

BoxCall -> Unified success:
  preserves S plus delegated LocalSSA receiver coverage
```

The environment toggle is a test-only route selector protected by a local
mutex and restored on drop; it is not an I0 runtime authority. I0 changes the
failure assertions to require no Map fact delta while retaining the enumerated
success-path receiver coverage and the existing Map policy owner.

`MAP-WRITE-OBSERVE0-I0` is the sole next row. It may connect the prepared
schedule only at the two existing Call receipt endpoints; it must retain the
outer semantic descriptor privately across BoxCall-to-Unified delegation and
must not add a generic receipt API.

### `MAP-WRITE-OBSERVE0-I0` — closed (2026-07-20)

The pre-Call Map observer is removed from direct Unified and BoxCall. One
prepared schedule is now consumed only after successful physical Call:

```text
direct Unified:
  prepare S -> append finalized R when distinct -> Call success -> replay

terminal BoxCall:
  prepare S -> Call success -> replay

BoxCall -> Unified:
  prepare outer S -> private handoff -> append delegated L
  -> append final R when distinct -> Call success -> replay
```

The handoff is a private `UnifiedCallEmitterBox` entry and carries only the
non-Clone prepared replay. It does not change public call APIs, routing,
LocalSSA propagation, map fact policy, or generic Call receipt semantics.
`Set` failures now publish no fact, while failed `Delete`/`Clear` retain their
seeded fact state. Successful writes preserve the existing source/final
receiver coverage.

### `MAP-WRITE-OBSERVE0-G0` — closed (2026-07-20)

The existing FACT0 partition guard now enforces:

```text
direct Unified Map replay preparation = 1
Unified S/L/R append sites = 2
Unified pre-receipt Map observation = 0
Unified receipt replay consumer = 1

BoxCall semantic-source preparation = 1
BoxCall -> Unified private handoff = 1
terminal BoxCall receipt replay = 1

PreparedMapWriteReplayV1 = 1
replay module MirBuilder/map-policy dependency = 0
```

Focused Map (7), Array (4), and existing call-temporal (16) tests pass, as do
the reusable FACT0 partition guard and its Python unit suite. All touched
source/check files remain below 800 lines. Return to `NEXT-PRODUCER-D0` before
selecting any further fact producer.

### Stop conditions

Stop `MAP-WRITE-OBSERVE0` and reopen consultation if preserving S/L/R requires:

```text
a persistent alias or receiver-fact map
changing LocalSSA propagation or map fact policy
AST/name/runtime-tag inference
a generic Call receipt abstraction or router rewrite
Map Get policy, FieldStore/index, Array, or FastMem work
fallback/retry after selected Call failure
a source/check file at or above 800 lines
```

### `FIELDSTORE-OBSERVE0-D0` — selected (2026-07-20)

Three read-only audits selected the smallest remaining receipt seam: the
ordinary, non-FastMem `MirInstruction::FieldSet` access-site metadata.  The
current common field-assignment entry appends its `store` access site before
route selection; an ordinary no-current-block failure can therefore leave a
site without a physical FieldSet.  The field-origin maps already publish only
after the physical route succeeds.

The first profile is deliberately narrow:

```text
region = None
is_known_weak = false
declared_field_contract_identity = None
physical instruction = ordinary FieldSet
```

`FIELDSTORE-OBSERVE0-S0` is the sole next code-facing row.  It will add one
private Builder-free prepared ordinary-store access-site descriptor, including
the existing source span and immutable site inputs, with zero production
consumers.  M0/P0 must freeze the ordinary success/failure timing, then I0 may
append exactly one site only after the successful FieldSet receipt.

Excluded and parked:

```text
WeakFieldWrite
typed-array contract claims
FastMem FieldStore
ordinary FieldGet
index-store route fan-out
generic Array Call observation
field-origin/type policy
CorePlan and finalization
```

Stop for a design consultation if the ordinary profile cannot retain its exact
site/span without changing a listed excluded owner, adding a persistent
ValueId map, reusing a generic receipt API, or adding fallback/retry.

### `FIELDSTORE-OBSERVE0-S0` — closed (2026-07-20)

`fields/store_post_success.rs` now owns one disconnected non-Clone-free
Builder-free descriptor for the selected ordinary FieldSet site.  It retains
only the already-resolved source span, base ValueId, optional receiver box
name, and field spelling.  It has no metadata append, instruction, route,
type/origin fact, contract, weak-write, FastMem, index, or commit capability.

Focused descriptor tests cover exact resolved inputs and an absent receiver
identity without synthesizing one; `cargo check --all-targets` is green.  M0
is now the sole next row and must prove the actual ordinary FieldSet failure
residual before any production consumer is connected.

### `FIELDSTORE-OBSERVE0-M0` — design stop (2026-07-20)

The direct ordinary fixture is now executable: with no current block,
`FieldSet = 0` while the pre-emission store access-site count is `1`.  This
confirms the residual.  It also exposes a boundary that this row must not
silently cross: the common access-site append currently precedes
`emit_known_weak_field_write`, whose `false` result is the only existing
ordinary-route discriminator and whose `true` branch may already emit a
physical WeakFieldWrite.

An ordinary-only receipt connection therefore requires one of:

```text
A. one shared, pure weak-field classification product reused by the emitter
   and FieldSet route selection;
B. a provisional access-site append plus later cancellation;
C. a duplicate declaration-registry weak-field query in fields.rs;
D. moving the shared append after WeakFieldWrite.
```

Only A is plausibly compatible with one authority.  B introduces a temporary
metadata mutation/cancellation lifecycle; C creates a second weak-route owner;
and D changes the excluded WeakFieldWrite timing.  `FIELDSTORE-OBSERVE0-I0`
is forbidden pending a dedicated weak-classification design decision.  No
FieldSet, weak, contract, FastMem, index, type/origin, or access-site behavior
has changed in this row.

### `WEAKFIELD-CLASSIFY0-D0` — consultation brief (2026-07-20)

Three independent follow-up audits agree that the only safe continuation is a
shared, read-only weak-route preflight.  This is a design consultation, not an
implementation authorization: `FIELDSTORE-OBSERVE0-I0` remains forbidden until
this boundary is explicitly accepted.

#### Recommended candidate: A-prime

The existing weak-field owner must classify the route once and return one
private, non-Clone preparation product:

```text
prepare_known_weak_field_write(...)
  -> Ordinary
  |  KnownWeak(prepared existing weak emission inputs)
```

The product is created by the existing weak-field module from only the current
base-origin and `user_box_field_decls` truth.  `KnownWeak` retains the exact
declaration-order field index, complete declaration fingerprint, and existing
weak contract identity, so issuance never re-queries the registry.  The
preflight itself writes no MIR instruction, access-site metadata, type fact,
origin fact, contract claim, or persistent map.

The shared field-assignment entry then has two deliberately different timing
lanes:

```text
KnownWeak:
  preserve the existing pre-emission access-site append
  -> existing FastMem weak rejection or WeakFieldWrite issuance

Ordinary, region = None, no typed-array contract identity:
  prepare the closed ordinary-store descriptor
  -> physical FieldSet receipt
  -> append one access site
```

Typed-array contract lanes retain their current pre-emission ordering in this
row.  They are not silently folded into the ordinary receipt profile.

#### Evidence and excluded alternatives

The audits fixed these current facts:

```text
weak success:
  WeakFieldWrite = 1, FieldSet = 0, site is recorded before weak issuance

weak FastMem:
  exact existing weak-FastMem error remains issuance-time behavior

ordinary no-current-block failure:
  FieldSet = 0, pre-I0 site = 1
```

The alternatives are rejected for concrete authority reasons:

```text
provisional append/remove:
  fastmem_field_access_sites is append-ordered; cancellation adds a second
  lifecycle, ordering, and intermediate-observer contract

duplicate registry query in fields.rs:
  produces two weak-route authorities and can drift on schema/error order

move the common append after weak issuance:
  changes the excluded weak-field timing contract
```

#### Proposed code-facing order after acceptance

```text
WEAKFIELD-CLASSIFY0-S0
  pure private classifier + prepared known-weak product
  production consumers = 0

-> WEAKFIELD-CLASSIFY0-P0
  classifier matrix, weak success/failure/FastMem timing parity

-> WEAKFIELD-CLASSIFY0-I0
  existing weak issuer consumes its prepared product exactly once
  fields route classifies once; only selected ordinary/no-contract FieldSet
  moves its access-site append after receipt

-> WEAKFIELD-CLASSIFY0-G0
  classification owner = 1; registry re-query = 0; provisional cancellation = 0

-> resume FIELDSTORE-OBSERVE0-I0
```

#### Non-authorities and stop conditions

```text
not authority:
  weak_fields_by_box cache, typed-array contract lookup, FastMem region,
  source/method names, runtime tags, final metadata, fallback or retry

stop:
  a second registry lookup, a persistent route map, access-site cancellation,
  weak-site movement after receipt, typed-array timing changes, or a generic
  field transaction API
```

The sole decision requested is whether to accept A-prime as the next owner.
Until then, no production field-assignment wiring is authorized.

### `FASTMEM-RECEIPT0` historical stop conditions

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
