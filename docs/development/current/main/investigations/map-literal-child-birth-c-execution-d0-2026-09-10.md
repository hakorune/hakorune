---
Status: design_stop
Task: MAP-LITERAL-ORDINARY-NEW-HOME-CHILD-BIRTH-C-EXECUTION-D0
Date: 2026-09-10
Priority: design the selected C child physical consumer before activation
PreviousCard: MAP-LITERAL-ORDINARY-NEW-HOME-CHILD-BIRTH-HANDOFF-I0
NextCard: MAP-LITERAL-ORDINARY-NEW-HOME-CHILD-BIRTH-C-EXECUTION-I0
---

# C child New/Birth/Home physical consumer D0

## Six-line brief

```text
Decision: keep the existing Rust final handoff and caller-aware compiled-entry/JSON contract as the sole meaning owner; design only the C physical consumer for a selected ordinary child New/Birth/Home flow.
Source authority + canonical issuer: FinalizedRootHandoffV1, PublishedLifecyclePhysicalProgramV1, CompiledEntryContractV1, and the existing runtime ABI descriptor/session; C consumes issued rows and never infers a child target from names, ValueId, or MIR.
Non-authority: C role/name text, function ordinal alone, Birth definition presence, generic JSON, row counts, static registry, or a missing child row repaired from root state.
Fail-fast boundary: before LLVM emission, require caller function index, exact New/Birth/HomeRelease/Reclaim rows, destination/receiver/target/argument correspondence, runtime layout/session compatibility, and cleanup order; reject omitted, duplicate, foreign, or unsupported child shapes.
Smallest next slice: census the existing C child guards and one exact selected child New→Birth→HomeRelease path, then implement one physical consumer row only after the owner/session and old-edge delete set are accepted.
Non-claims: no field/unknown values, native arrays, general Map widening, runtime redesign, new receipt/schema, fallback/retry, direct/linked exit30, or whole Map/R7 closure.
```

## Current observed boundary

The Rust handoff slice landed at `c3fa1f8a75`. It retains caller-scoped
`FinalizedBirthActualsV1`, maps ordinary physical functions back to their selected
owner, and makes compiled-entry and physical JSON Birth matching caller-aware.
The child-only Birth+Map/New focused test, the existing root Birth regression, the
JSON transport, and shared cleanup-coordinate projection are green. This is Rust
contract evidence; it does not open the C consumer.

The remaining C guards are the selected lifecycle indexed-flow gates for child
`new_box`, Birth call, and `HomeRelease`/`ReclaimUnpublished`. Their existing
contract is the boundary to audit. The C layer must receive a complete issued
program/session and may only project already-checked rows into LLVM calls.

## Design tasks

1. Enumerate the exact C caller/consumer functions and current stopped tokens for
   child New, Birth, HomeRelease, and ReclaimUnpublished. Record the exclusive
   old-edge delete set; do not widen the source admission here.
2. Reconcile one runtime-owned layout/session with the Rust compiled-entry rows.
   The session may cache indexes, but it must not become a second source authority.
3. Define the smallest C frame/row projection that carries caller function index,
   New destination, Birth target/receiver/arguments, and cleanup coordinates.
4. Prove positive and negative cases at the C boundary: one child New/Birth,
   duplicate/omitted/foreign rows, caller-index collision, and cleanup-before-
   install rejection. Only then select the I0 production edge.

## Stop conditions

Return to design_stop if the C consumer needs a source key/name re-resolution,
a new semantic receipt, an implicit root fallback, or cannot identify a finite
caller/session owner. Keep all child field/unknown/native and direct/linked
claims parked until this D0 has a named physical owner and acceptance.
