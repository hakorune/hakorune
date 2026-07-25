# ENTRY-RESULT-PROJECTION0-S1 design stop

Decision: `ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME-prime-r1`
Status: accepted design; implementation authorized for the disconnected
`ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0` row only

`ENTRY-RESULT-PROJECTION0-S0` is complete as a disconnected contract and
evidence slice. The repository now has one pure source-result projection,
one sealed route selection, one source-result thunk, one backend-neutral
physical carrier, pure reference fixtures, and one normalized `ny_main`
capability adapter. None of these are connected to normal compilation,
JSON, VM execution, LLVM/native publication, or public ingress.

## Why execution stops here

The remaining question is not another value type. It is the first consumer
of `PhysicalSourceEntryCarrierV1`. Consuming the carrier directly with
`ProcessExitProjectionV1::project` would otherwise discard route, manifest,
and source-result provenance at the same boundary where process status is
created. Connecting a backend or public entry now would also silently choose
an activation policy that S0 explicitly did not authorize.

## Questions to close

### Q1 — first projection consumer

Choose exactly one owner for the next row:

```text
A (recommended): a compiler-internal, backend-neutral
   prepare_process_projection(self)
   -> PreparedSourceEntryProjectionV1
   -> infallible project(self)

B: direct ProcessExitProjectionV1 call from a backend adapter

C: public compile/runner ingress as the first consumer
```

Recommendation: A. The carrier remains the owner of route/evidence while a
prepared projection records the canonical profile. B creates backend-specific
authority too early; C mixes semantic proof with cutover.

### Q2 — evidence retention

Does the successful projection retain the consumed physical carrier beside
`ProcessTerminationV1`, or may it return only the termination value?

Recommendation: retain a non-Clone carrier/evidence aggregate until a later
explicit authority-erasure terminal. Route and manifest must not be inferred
again from process status.

### Q3 — failure law

The canonical profile is already selected, but projection still has a typed
profile/unsupported-result failure vocabulary. Choose whether failures return
the exact physical carrier in a discard-only rejection or are flattened to a
diagnostic immediately.

Recommendation: exact carrier retention with `stage()`, typed `error()`, and
`discard(self)` only. No status-zero fallback, legacy converter, retry, or
partial publication.

### Q4 — activation boundary

Choose the first allowed consumer scope:

```text
S1 disconnected projection fixture only (recommended)
S1 VM reference execution
S1 LLVM/native ny_main wiring
S1 public Raw ingress
```

Recommendation: disconnected fixture only. VM, LLVM/native, public ingress,
normal `compile_with_source`, JSON, executor, legacy retirement, and CUT0 stay
zero until a separate cutover decision.

## Non-authority

```text
module symbol/name or `NYASH_ENTRY`
backend-local entry helper
MirModule inventory
legacy 42/0 mock
positive handle decoding
source AST/catalog re-observation
current Builder state after carrier creation
```

## Accepted decision

Q1 = A: one compiler-internal, backend-neutral prepared projection consumer.

Q2 = retain the non-Clone physical carrier/evidence beside the projected
`ProcessTerminationV1` until a later authority-erasure terminal.

Q3 = typed discard-only rejection retaining the exact physical carrier. The
projection profile is canonical only; no status-zero fallback, retry, legacy
converter, or partial publication is allowed.

Q4 = disconnected projection fixtures only. VM execution, LLVM/native
`ny_main` wiring, public ingress, normal `compile_with_source`, JSON, executor,
legacy retirement, and CUT0 remain zero.

The carrier therefore gets one consuming boundary:

```text
PhysicalSourceEntryCarrierV1
  -> prepare_process_projection(self)
  -> PreparedSourceEntryProjectionV1
  -> project(self)
  -> ProjectedSourceEntryV1 {
       carrier: PhysicalSourceEntryCarrierV1,
       termination: ProcessTerminationV1,
     }
```

Projection failures happen before the carrier is consumed into the prepared
owner. The successful `project(self)` terminal is infallible and does not
publish a module, call a backend, or invoke `process::exit`.

## Candidate next row after acceptance

```text
ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0
```

Its minimum acceptance matrix is Unit, Integer 0/255, range fault,
unsupported scalar/object, source Fault, Script/App route retention, exact
rejection owner retention, and zero backend/public callers.

The Q1–Q4 decision above is now accepted. The implementation authorization is
limited to the disconnected projection-consume row and its focused guards;
production consumers and normal-entry changes remain forbidden.
