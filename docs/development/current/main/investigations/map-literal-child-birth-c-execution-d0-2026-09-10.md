---
Status: closed bounded C I0; implementation landed at `60bcf68add`
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

## Read-only C census result

The D0 audit confirms that the stop is a deliberate root-only admission rule,
not a missing transport row. The production caller is
`published_mir_object.rs` -> `LifecycleInvocationInputV1` ->
`hako_llvmc_compile_published_lifecycle_physical_v4`; one invocation owns the
Rust runtime session, the parsed physical document, the C index, and the LLVM
target session. No second source authority is needed.

The selected C consumer still rejects a non-root function index (`fi`) for all
four operations below:

```text
ordinary child: new_box
ordinary child: birth_call
ordinary child: home_release
ordinary child: reclaim_unpublished
```

Removing only those guards is insufficient. `lv4_indexed_admit()` also binds
Birth receiver layouts by scanning root blocks only. A child Birth therefore
has no receiver/object binding and is rejected by the existing
`birth_unit && !receiver_object_set` check. The smallest physical admission
unit is consequently the following one bounded change:

```text
allow child New
  -> bind its exact object/layout row
  -> allow child Birth using that live receiver
  -> allow child normal HomeRelease and fault ReclaimUnpublished
```

The existing rows already carry the required coordinates: New has `site` and
`object_id`, Birth has `target`, `receiver`, and typed arguments, and cleanup
has `site`, `object_id`, and `value`. The runtime ABI/layout/session remains
owned by `LifecycleRuntimeSessionV1` and the existing C target session. C may
cache these indexes for the invocation, but it must not resolve names or issue
source meaning.

## C I0 acceptance and delete set

Do not open C I0 until the following finite acceptance is written into the
focused test/guard plan:

* one selected ordinary child caller per Birth target;
* caller function index, Birth target, receiver object, argument kind/value,
  New object/layout, and cleanup coordinates match the same physical input;
* New creates one live handle, Birth consumes that handle as receiver, and
  normal/fault cleanup consumes it exactly once;
* cleanup-before-New, foreign or mismatched receiver/layout, duplicate or
  omitted cleanup, and multiple Birth callers reject before emission;
* no source-name lookup, registry repair, implicit root fallback, or semantic
  receipt/schema is introduced.

The exclusive old-edge delete set is limited to the five root-only C
assumptions: the `fi` rejection branches for `new_box`, `birth_call`,
`home_release`, and `reclaim_unpublished`, plus the root-only Birth receiver
binding scan. Generic fallback, compatibility routes, other runtime families,
and all field/unknown/native/general Map claims remain outside this I0.

**D0 decision:** accepted. The owner, rows, session, and exclusive delete set
are finite, and the receiver-binding/live-handle/normal-fault cleanup contract
above was the bounded I0 acceptance. That selected ordinary child consumer is
now implemented and closed below. Do not open any other child shape or whole-Map
route in the same series.

## C I0 implementation evidence

The selected consumer landed at `60bcf68add`. It removes only the five
root-only assumptions listed above. `lv4_indexed_admit()` now scans every
selected caller, binds one exact Birth receiver/object/layout, and rejects a
second caller for that Birth target. The indexed flow admits an ordinary child
`new_box`, `birth_call`, normal `home_release`, and fault
`reclaim_unpublished`; root, non-ordinary child, foreign receiver, and
unsupported shapes remain fail-fast.

Focused evidence after rebuilding `libhako_llvmc_ffi.so`:

* ignored Rust source test
  `issued_ordinary_child_new_birth_map_direct_exe_and_linked_object_exit_30`
  passes direct EXE and independently linked OBJ with exit 30;
* the existing ordinary-child Map-value Pair regression still passes;
* the physical parser pre-artifact C test and the V4 execution suite pass,
  including the existing malformed-input, session, tool, and artifact guards;
* the generated child physical input accepts unchanged, while temporary
  foreign-receiver and duplicate-Birth mutations both reject before emission.

The last mutation check is boundary evidence only; it is not a new source
fixture or a whole-Map claim. Field/unknown/native children, multiple Birth
callers, and broader Map/R7 activation remain outside this closed I0.

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
