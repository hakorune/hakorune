---
Status: Current T2 design stop; explicitly selected by CURRENT_STATE
Date: 2026-08-14
Decision: resume at S6C authority closure, not at the already-landed schema rows
Scope: M8 LoopV0 forward ScanWithInit source/Facts/Recipe; no physical activation
---

# LOOP-RECIPE-TYPED-CALL-VALUE-D0

## Current Capsule

- **Current decision:** the V2 typed schema and neutral operation split are
  landed; S6C still needs one exact resolver/CoreMethod-backed
  `length/substring/TextEq` source relation and one complete Facts-to-Recipe
  issuer before implementation is safe.
- **Current implementation status:** Loop rows 1--10 are closed, M8 S6A/S6B
  are closed, and no forward `ScanWithInit` Facts/producer or production
  physical selector is active.
- **Next ordered task:** keep the S6C audit result as a T2 dependency stop:
  the existing declaration/Home/Query issuers cover only the bounded `I64`/
  `Unit` user-instance cohort, and the CoreMethod manifest does not issue a
  neutral `Text` receiver/result Home target contract. Open the new
  `LOOP-CORE-METHOD-INSTANCE-TARGET-D0` design dependency first; keep S6C
  `NoSafeSlice` if that contract boundary is absent.
- **Production stop line:** no scan selector, physical route, fallback, or
  production caller is opened by this design row.
- **Retirement finish line:** after a real S6C implementation and parity,
  update the reference contract in that same implementation commit; legacy
  scan facts/builders remain until an explicit cutover row deletes them.

## Resumption brief

```text
Decision: resume Loop at JOINIR-LOOP-M8-LOOPV0-SCANS-S6C as one T2 authority design stop; do not replay closed rows 1--10 or select production row 11 early.
Source authority + canonical issuer: resolved forward ScanWithInit source, a
CoreMethod-manifest callable/Home target contract, a separate source-bound
call relation, existing typed input/effect owners, and one future S6C
Facts-to-Recipe producer.
Non-authority: selected-Dynamic substring/indexOf receipts, raw AST/name lookup, legacy scan builders, MIR/CheckedCallOut IDs, physical operation order, and task-map history.
Fail-fast boundary: missing/foreign/duplicate target, result/Home/effect/site relation, incomplete role coverage, or Loop Return versus callable Tail drift remains NoSafeSlice before Facts/Recipe publication.
Smallest next slice: read-only issuer/caller census for exact length/substring/TextEq and input/condition/body/step/Return coverage; accept one bounded BoxCount only if every required authority already exists or one named neutral issuer can be added.
Non-claims: no SplitScan/CharMap/ArrayJoin/BoolPredicateScan, physical canary, production selector, fallback/retry, legacy deletion, Dynamic receipt reuse, or new backend.
```

## Authority census and bounded task DAG — 2026-08-14

The read-only S6C audit is complete. It is a design result, not an issuer or
Facts receipt:

```text
closed evidence:
  LoopRecipeV2 { Text, CallSlot, TextEq } is a structural wire only;
  CoreMethodContractBox/generated rows own StringLen/StringSubstring
  op/arity/result/effect and runtime-owner metadata;
  resolver declaration, Home, Query, body-carrier, and contract products
  exist for the bounded user-instance I64/Unit cohort.

missing authority:
  no neutral CoreMethod callable/Home target contract co-seals StringBox
  receiver, StringLen/StringSubstring result/Home relations, exact arity,
  non-suspending/non-control obligation, and the generated manifest row;
  no separate source-bound relation co-seals the exact call site/owner,
  receiver expression, arguments, or result site;
  no source-bound S6C CallSlot relation or complete Facts-to-Recipe producer.

therefore:
  S6C remains NoSafeSlice/design_stop. Selected-Dynamic
  substring/indexOf receipts are a different owner and are not evidence.
```

Audit anchors (these are evidence pointers, not new authorities):

```text
src/mir/resolved_semantics/instance_method_declaration.rs
  ResolverSemanticValueTypeV1 = I64 | Unit; no Text declaration/result class.
src/mir/resolved_semantics/home_abi.rs
  HomeCapabilitySchemaV1 = I64UnitTrivial; no StringBox/Text Home projection.
src/mir/generated/core_method_contract_rows.rs
  StringLen/StringSubstring rows provide receiver_box, arity, op,
  result_kind, and PureRead only; no source-site or Home relation.
src/mir/source_call_target/model.rs
  source target vocabulary is Static or DynamicMember; no resolver-issued
  instance CoreMethod target capability exists.
src/mir/loop_recipe_contract/schema_v2.rs
  CallSlot is deliberately receiver/args/result keys only and cannot repair
  any of the missing source/target/Home axes.
```

The ordered task DAG is now explicit and bounded:

```text
S6C-AUTHORITY-CENSUS-R0                         CLOSED (read-only)
  -> LOOP-CORE-METHOD-INSTANCE-TARGET-D0       T2 dependency (new)
       -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0  T2
            -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0/I0 T2
                 -> JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-I0 T2
                      -> S6C parity / canary / later production rows
```

The generic `LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0/I0` remains a separate
parked row for user-declared instance methods. It must not be relabeled as the
StringBox/CoreMethod issuer. The new `LOOP-CORE-METHOD-INSTANCE-TARGET-D0`
must define one neutral generated-contract owner; it must not infer `Text`
Home from `MirType` or `CoreMethodResultKind`, and must not pair a target by
Box/method name after the generated row is sealed. Its first exact co-seal
must include:

```text
same CoreMethod manifest/schema brand
StringBox receiver semantic contract
StringLen: arity 0 -> I64, PureRead
StringSubstring: arity 2 -> Text/StringValue, PureRead
explicit receiver/parameter/result Home relation (no default)
non-suspending + non-control policy
runtime owner/export profile as a downstream projection only
```

The next source-bound relation consumes/borrows that target and adds the exact
source expression site, owner/frame, receiver expression, ordered arguments,
and result site. It owns no target lookup, CoreMethod re-lookup, Recipe key,
`ValueId`, `BasicBlockId`, ABI, or physical layout. Source-site identity is
therefore not duplicated inside the reusable target capability.
Only after that relation is closed may an S6C producer issue Facts for the
exact roles below and let the producer mint Recipe-local keys:

```text
inputs: subject Text, needle Text, initialized index I64
condition: read index + length(subject) + I64 comparison
body: read index + index+1 + substring(subject,index,index+1)
      + TextEq + conditional Return(index)
step: read/add/write index
tail: callable Return(-1), outside Loop Facts
```

Acceptance for the dependency row is fail-closed and source-first:

```text
positive: one same-brand CoreMethod target pair with exact arity/result/effect,
          explicit Home relation, ABI/profile, manifest brand, and no
          suspension/control;
negative: foreign/duplicate/swapped target, String vs StringBox mismatch,
          wrong arity/result/Home/effect, missing source site, Text inferred
          from MIR/CoreMethod output, name lookup, or partial Facts coverage;
          foreign/duplicate owner-frame or swapped receiver/argument/result
          site cardinality is rejected;
guard: CoreMethod target issuer has one named source-bound consumer,
       source-bound relation has one S6C producer consumer,
       S6C producer count is zero until that relation closes, and no selected
       Dynamic receipt is imported by the Loop lane.
```

Non-claims remain strict: no Builder/MIR/CheckedCallOut/Boundary, no physical
IDs or ABI, no production selector, no fallback/retry, no legacy retirement,
and no new `Verified*`/`Prepared*` product is issued by this census.

## Change

Keep S6C as a design stop and split the work into two bounded rows:

1. this prerequisite: a profile-neutral typed call/value vocabulary and its
   source/effect ownership;
2. a later `ScanWithInit` source observer/producer using that vocabulary.

Do not combine `ScanWithInit`, `SplitScan`, `CharMap`, `ArrayJoin`, and
`BoolPredicateScan` into one Facts union or one implementation row.

## Evidence and boundary

The existing `LoopOperationV1` admits only `ReadBinding`, `ConstI64`,
`BinaryI64`, `CompareI64`, and `WriteBinding`; `LoopValueClassV1` admits only
`I64`, `Bool`, and `Unit`. The forward scan fixture
`apps/tests/scan_with_init_ok_min.hako` requires typed string values and calls
for `length` and `substring`, a typed string equality, a loop return, and a
callable tail return. The old scan builders reconstruct AST and therefore are
not portable source authority.

This is a BoxShape/schema gap, not a source `Unresolved` outcome. Exact Recipe
counts are intentionally not published until the vocabulary is accepted.

## Contract

The prerequisite must define one profile-neutral operation/value family with
these properties. The schema-version decision is explicit: do not silently
widen the pre-production numeric V1 wire. The typed cohort will use an
explicit `LoopRecipeV2` artifact/schema version.

1. **Typed values:** preserve existing `I64`, `Bool`, and `Unit`, and add only
   an explicitly named logical `Text` value domain for forward `ScanWithInit`.
   `Text` does not imply pointer representation, ownership, GC, or ABI. A
   source-bound value/Home contract supplies those facts. `Handle`, `Any`,
   `Opaque`, Array, collection, and nominal Box values are not in this cohort.
2. **Typed call leaf:** add one Recipe-local call slot carrying an optional
   receiver `ValueKey`, ordered argument `ValueKey`s, and an optional result
   `ValueKey`. The Recipe never contains a method/Box name, MIR callee,
   resolver capability, `ValueId`, `BasicBlockId`, or runtime lookup string.
   The source-bound call relation owns the resolver-issued target, exact
   receiver/parameter/result contract, Home relation, effects, suspension and
   control disposition, ABI profile, and exact source expression site.
3. **Allowed first profile:** calls must be exact, non-suspending, non-control,
   and admitted by the existing callable/type contract. Missing signature,
   unsupported effect, ownership mismatch, or unknown result class freezes
   before Recipe/Core publication.
4. **Typed comparison:** add a fixed `TextEq(Text, Text) -> Bool` operation.
   Do not import or reuse the If-only direct-call schema. Numeric comparison
   remains the existing operation family; cross-domain coercion is rejected.
5. **Input ownership:** string parameters, the character parameter, and the
   initialized index are explicit input-source relations. Do not create a
   scan-specific second input owner; extend the existing typed input relation
   family only after its parameter/initializer distinction is sealed.
6. **Control/tail boundary:** `return i` is a logical loop `Return` exit when
   the Loop algebra owns it. The outer `return -1` remains callable Tail and
   completion evidence. Neither tail nor ABI is absorbed into Loop Facts.
7. **Source/effect evidence:** each call, typed comparison, input, read/write,
   and exit has an exact resolver source site, owner/frame brand, and effect
   relation. Facts retain semantic roles and `BindingRef`; producers alone
   mint Recipe keys. No AST reread or synthetic AST is allowed.

The exact enum/wire names and normalized item/value counts are part of the
typed-call D0 acceptance. They must not be guessed in the S6C implementation
card. If the contract cannot be made neutral and reusable, keep S6C parked.

## Required CoreMethod target + source-bound relation

The current `ResolvedCallableRefV1` is free-static only. It is not sufficient
for `subject.length()` or `subject.substring(...)`. Before a scan observer can
be implemented, one neutral generated CoreMethod target capability must
co-seal target identity, receiver/parameter/result types, Home relations,
effects, suspension/control, ABI profile, and the CoreMethod manifest brand.
The exact source expression is a separate source-bound relation that borrows
that target and adds owner/frame, receiver expression, ordered arguments, and
result site. No layer may recover either product from a method or Box name.

Facts may retain the semantic role, `BindingRef`, and exact site; the producer
may mint the local call-slot and value keys. The reusable target capability
does not own a source site, Recipe key, `ValueId`, `BasicBlockId`, or physical
layout.

Home is not a `LoopValueClass`. Unknown Home relation, result class, effect, or
target identity is a pre-Recipe failure, not an opaque value fallback.

`return i` is `LoopRecipeItemV1::Exit(Return { value })` when the Loop algebra
owns that exit. The outer `return -1` remains callable Tail/Completion. ABI,
Tail, and Completion never enter Loop Facts or the neutral Recipe.

Disposition precedence is fixed:

```text
Rejected (identity / foreign / duplicate / forged)
  > Unresolved (missing target / signature / Home / effect / site / coverage)
  > Declined (fully observed but wrong family shape/effect)
  > Candidate (exact complete contract)
```

`NoSafeSlice` is a development state while the contract or issuer is absent;
it is not a fifth source disposition.

## First implementation slice after D0

Only the forward `ScanWithInit` fixture is eligible. Its semantic roles are:

```text
inputs: string subject, text needle, initialized i64 index
condition: read i; subject.length(); i < length
body: read i; compute i+1; subject.substring(...); text equality;
      conditional Return(i)
step: read i; add one; write i
tail: callable Return(-1), outside the Loop Recipe
```

This role list is not a Recipe golden and supplies no physical IDs. The later
families are separate rows: `SplitScan` needs explicit-else joins and mutable
start/collection effects; `BoolPredicateScan` needs predicate-call contracts;
`CharMap` and `ArrayJoin` need their own text/collection operation contracts.

## Disposition matrix

| Outcome | Boundary |
| --- | --- |
| `Candidate` | Exact resolver call targets, typed values/effects, source coverage, control/tail split, and ownership all seal together. |
| `Declined` | Fully observed forward source, but wrong step/operator/direction, extra effect, nested shape, or unsupported family policy. |
| `Unresolved` | Source site, callable signature, result class, Home/effect relation, or complete coverage is unavailable. |
| `Rejected` | Foreign owner/frame, duplicate role, mismatched target, forged capability, or incoherent source relation. |

`NoSafeSlice` remains the development state while this vocabulary is absent;
it is not a fifth source disposition.

## Done for this design row

- one tracked design Decision and task order are accepted;
- current numeric-only schema limitation is recorded with the exact fixture;
- explicit V2 schema boundary, logical `Text`, local Call slot, `TextEq`,
  source-bound instance target, Home/effect ownership, fail-fast precedence,
  and Loop/Tail split are fixed;
- the 781-line demand and 725-line verifier modules are scheduled for the
  behavior-neutral `LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0` before schema growth;
- one-family implementation order is fixed (`ScanWithInit` first);
- non-claims forbid AST reuse, opaque fallback, route-specific operation
  kinds, Builder/MIR/physicalization, selector, retry, and production;
- the later implementation row explicitly updates
  `docs/reference/mir/loop-recipe-contract.md` and affected module READMEs in
  the same commit as the landed schema/observer/producer and tests.

## Ordered successor rows

```text
S6C-AUTHORITY-CENSUS-R0                         CLOSED (read-only)
  -> LOOP-CORE-METHOD-INSTANCE-TARGET-D0       T2 dependency
       -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0  T2
            -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0/I0 T2
                 -> JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-I0 T2
                      -> S6C parity / canary / later production rows
```

The generic `LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0/I0` remains a separate
parked row for user-declared instance methods; it must not be relabeled as the
StringBox/CoreMethod issuer. `LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0` and the
already-closed `LOOP-RECIPE-V2-TYPED-SCHEMA-CALLSLOT-I0` stay historical
predecessors, not the current restart pointer. Every landed typed
schema/observer/producer row updates the reference contract and affected
module READMEs in the same commit; legacy scan facts/builders are deleted only
after production parity and callers-zero evidence.

## D0 owner map (design only)

The next D0 may name types and owners without issuing them. The proposed
boundary is:

```text
src/mir/resolved_semantics/core_method_instance_target.rs
  VerifiedCoreMethodInstanceTargetV1
  CoreMethodInstanceTargetIssuerV1
  = generated CoreMethod row + explicit Home/ABI/profile contract

src/mir/source_call_target/model.rs
  VerifiedSourceBoundCoreMethodCallV1
  SourceBoundCoreMethodCallIssuerV1
  = borrowed target + owner/frame + source receiver/args/result sites
```

This CoreMethod product gets a dedicated issuer/catalog; it is not another
variant of `VerifiedSourceCallTargetV1` and is not inserted into its existing
`Static`/`DynamicMember` rows. Those rows already feed static publication and
Dynamic selector/physical consumers. The dedicated catalog co-seals the
manifest brand with the source owner/frame brand, while the relation issuer
borrows the target and performs only site/cardinality checks. It performs no
method/Box lookup, generated-row relookup, Recipe-key/ValueId/BasicBlockId
issuance, ABI reclassification, or physical-ID issuance.

`CoreMethodManifestBrandV1` is an opaque projection of the generated manifest
schema/row brand, not a new semantic authority. `Home` may be issued only by
an explicit resolver Home capability for `StringBox`/`Text`; it is never
inferred from `MirType`, `CoreMethodResultKind`, or the Recipe wire. The
existing `CallableHomeAbiIssuerV1`/`I64UnitTrivial` schema is user-instance
authority and must not be widened in place. A future `StringBoxText` schema
is a separate BoxCount with distinct schema, resolver, manifest, and relation
brands, while reusing only the Home relation mechanism. The source-bound
issuer checks one owner/frame brand, one receiver site, exactly `arity`
ordered argument sites, and one result site when the target has a result.
Foreign, duplicate, or swapped sites reject before Facts/Recipe.

### LOOP-CORE-METHOD-MANIFEST-HOME-ISSUER-D0

```text
Decision: design a separate manifest-backed StringBox/Text Home issuer; do not widen I64UnitTrivial in place.
Source authority + canonical issuer: CoreMethodContractBox/generated row brand plus an explicit CoreMethod Home capability issuer.
Non-authority: user-instance declaration/Home catalog, MIR types, ResultKind, names, DynamicMember receipts, or Recipe wire.
Fail-fast boundary: foreign/mixed schema or brand, wrong receiver/arity/result/effect/ABI, missing Home, and duplicate target reject before any receipt.
Smallest next slice: specify issuer input/output, exact StringLen/0 and StringSubstring/2 positives, negative matrix, and one reusable guard; no consumer yet.
Non-claims: no source-bound call product, Facts/Recipe producer, Builder/MIR/Boundary route, fallback, retry, or production switch.
```

Mode gate: this owner map is design-only. `work_mode` remains `design_stop`
until the manifest-backed CoreMethod Home issuer exists and its positive /
negative / guard evidence satisfies the source-backed receipt gate. No
`Verified*`/`Prepared*` semantic receipt, source-bound consumer, Facts/Recipe
producer, Dynamic import, or production switch is authorized by this D0.

The generated `StringSubstring` row currently advertises the union arity
`[1, 2]`. The target issuer must specialize it by operation and exact arity;
the S6C target identity is
`(manifest brand, StringBox, StringSubstring, arity = 2)`. Its sealed contract
is `StringBox -> (I64, I64) -> Text/StringValue`, `PureRead`,
non-suspending/non-control, with the manifest-derived ABI/profile. The union
row, arity 1, `StringIndexOf`, aliases, or a runtime-owner mismatch never
crosses the target boundary. The source relation then co-seals the subject
receiver and ordered `[index, index + 1]` argument sites as one-to-two-to-one
receiver/args/result cardinality.

## Stop

Do not implement S6C if the proposed call/value contract still requires a
method-name lookup, an opaque result, If-specific schema reuse, guessed item
counts, AST reconstruction, or a route-local adapter. Return to this design
boundary and close the missing authority first.

## Decision revision — 2026-08-08: typed V2 schema row is closed

The worker audits are integrated into one boundary decision:

```text
LoopRecipeV2 wire
  -> CoreMethod-manifest instance-call target capability
  -> source-bound call relation / verifier
```

These are three different products and three different rows. The wire never
contains a method name, Box name, resolver capability, ABI profile, Home
relation, effect set, MIR identity, physical ID, or runtime lookup string.

The first bounded implementation row is therefore:

```text
LOOP-RECIPE-V2-TYPED-SCHEMA-CALLSLOT-I0
```

It owns only the profile-neutral schema/artifact types and a structural
verifier for the new typed vocabulary. It may add:

```text
LoopRecipeArtifactV2
LoopRecipeV2
LoopValueClassV2::{I64, Bool, Unit, Text}
LoopOperationV2::CallSlot { receiver, args, result }
LoopOperationV2::TextEq { left, right, result }
```

`CallSlot` is a Recipe-local logical operation. The first cohort's admission
will later require a receiver and a result, but the wire keeps the receiver
and result optional so that the schema does not encode resolver policy. The
schema row does not admit static/resultless calls, because no target issuer or
source relation exists yet; those are later policy rows.

The structural verifier checks only canonical keys, referenced values, typed
operation domains, duplicate definitions, and schema version. It does not
claim source existence, callable target resolution, Home/effect/ABI validity,
Builder/MIR/CFG/PHI lowering, Loop/Tail/Completion integration, or physical
activation. `Text` is a logical value class only; representation and ownership
remain source-bound contracts.

The normalized `ScanWithInit` operation/item counts remain provisional until
parameter-input relations, instance target issuance, and source-bound call
relations are sealed. No fixture or scan observer is part of this row.
