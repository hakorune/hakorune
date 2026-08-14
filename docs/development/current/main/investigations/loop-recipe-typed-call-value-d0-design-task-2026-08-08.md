---
Status: Current bounded T2 design stop; manifest-target I0 evidence is closed, Resolver callable-contract D0 selected by CURRENT_STATE
Date: 2026-08-14
Decision: keep the manifest-target evidence bounded, and design the missing Resolver callable contract/owner-frame bridge before any source-bound relation or S6C producer
Scope: M8 LoopV0 forward ScanWithInit source/Facts/Recipe; no physical activation
---

# LOOP-RECIPE-TYPED-CALL-VALUE-D0

## Current Capsule

- **Current decision:** the V2 typed schema and neutral operation split are
  landed; the manifest-backed CoreMethod/Home target evidence is bounded, but
  Resolver source rows do not yet own a Text-capable callable contract or an
  exact MethodCall-to-loop-frame bridge. S6C remains `NoSafeSlice`.
- **Current implementation status:** Loop rows 1--10 are closed, M8 S6A/S6B
  are closed, and the dedicated CoreMethod/Home target issuer I0 is landed
  with focused positive/negative evidence. No forward `ScanWithInit`
  Facts/producer or production physical selector is active.
- **Next ordered task:** `LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0` — design
  the Resolver-owned Text/StringBox callable contract and exact source-site to
  loop-frame bridge that a later source-bound relation may borrow. Do not
  issue that relation or S6C Facts/Recipe yet.
- **Production stop line:** no scan selector, physical route, fallback, or
  production caller is opened by this design row.
- **Retirement finish line:** after a real S6C implementation and parity,
  update the reference contract in that same implementation commit; legacy
  scan facts/builders remain until an explicit cutover row deletes them.

## Resumption brief

```text
Decision: continue Loop at `LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0`; do not
replay closed rows 1--10 or select production row 11 early.
Source authority + canonical issuer: resolver declaration/catalog and
`VerifiedResolvedMethodCallSourceV1`/callable source ledger for exact owner,
site, receiver, ordered args, result, and loop-frame evidence; a new
canonical callable-contract issuer must be designed before a relation issuer.
Non-authority: generated rows alone, `home_abi.rs` I64/Unit defaults, MIR or
ResultKind inference, raw AST/name lookup, selected-Dynamic receipts, legacy
scan builders, CheckedCallOut IDs, physical order, and task-map history.
Fail-fast boundary: missing Text/Home contract, foreign/mixed owner or frame,
missing/duplicate/swapped source relation, or unknown effect/suspension/control
must remain NoSafeSlice before target relation/Facts/Recipe publication.
Smallest next slice: freeze the Resolver callable contract and exact
MethodCall-site-to-loop-frame bridge, then decide whether a borrowed HRTB view
or consuming target product is the only safe source-bound API.
Non-claims: no SplitScan/CharMap/ArrayJoin/BoolPredicateScan, physical canary, production selector, fallback/retry, legacy deletion, Dynamic receipt reuse, or new backend.
```

## Authority census and bounded task DAG — 2026-08-14

The read-only S6C audit is complete. Its CoreMethod/Home dependency is now
implemented as the bounded I0 below; this section remains a census, not a
source-bound or Facts receipt:

```text
closed evidence:
  LoopRecipeV2 { Text, CallSlot, TextEq } is a structural wire only;
  CoreMethodContractBox/generated rows own StringLen/StringSubstring
  op/arity/result/effect and runtime-owner metadata;
  resolver declaration, Home, Query, body-carrier, and contract products
  exist for the bounded user-instance I64/Unit cohort;
  the separate generated-brand StringBox/Home target issuer now covers
  StringLen/0 and StringSubstring/2.

remaining authority:
  no separate source-bound relation co-seals the exact call site/owner,
  receiver expression, arguments, or result site;
  no source-bound S6C CallSlot relation or complete Facts-to-Recipe producer.

therefore:
  S6C producer work and the source-bound relation remain NoSafeSlice/design_stop.
  The next bounded row is the Resolver callable contract/owner-frame bridge;
  selected-Dynamic substring/indexOf receipts are a different owner and are
  not evidence.
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
src/mir/resolved_semantics/body_shape.rs
  VerifiedResolvedMethodCallSourceV1 provides exact AST-free call site,
  receiver expression, ordered argument sites, result site, owner, selector,
  and arity, but intentionally no target/Home/effect policy.
src/mir/resolved_semantics/callable_source_ledger.rs
  CallableSemanticSourceLedgerView provides method-call rows and resolver
  Loop membership/frame products; a call-site-to-loop-frame bridge is not yet
  a co-sealed callable contract.
```

The ordered task DAG is now explicit and bounded:

```text
S6C-AUTHORITY-CENSUS-R0                         CLOSED (read-only)
  -> LOOP-CORE-METHOD-INSTANCE-TARGET-D0       CLOSED (accepted design)
       -> LOOP-CORE-METHOD-INSTANCE-TARGET-I0  CLOSED (bounded manifest evidence)
       -> LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0  CURRENT design stop
            -> LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-I0  T2
                 -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0  T2
                      -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0  T2
                           -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0/I0 T2
                                -> JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-I0 T2
                                     -> S6C parity / canary / later production rows
```

### LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0 (CURRENT)

```text
Decision: keep S6C and the source-bound relation at NoSafeSlice until one
Resolver-owned callable contract can represent the exact Text/StringBox
receiver, result/Home, effect, suspension/control policy, and owner/frame
bridge needed by the forward ScanWithInit cohort.
Source authority + canonical issuer: resolver declaration/catalog plus
VerifiedResolvedMethodCallSourceV1 and CallableSemanticSourceLedgerView;
the missing issuer must co-seal the generated manifest target brand with the
resolver owner/frame and explicit callable contract.
Non-authority: generated op/result/effect rows by themselves, I64/Unit Home
defaults, LoopRecipe CallSlot, generic route plans, selector/name lookup,
MIR/ResultKind inference, selected-Dynamic receipts, or physical IDs.
Fail-fast boundary: unsupported Text/Home contract, foreign or mixed owner/
frame/parser/manifest brand, missing/duplicate/swapped source site or frame,
unknown result/effect/suspension/control, and incomplete source coverage reject
before target relation, Facts, or Recipe issuance.
Smallest next slice: design the canonical contract and exact MethodCall-site
to loop-frame containment bridge; choose an opaque borrowed view versus a
consuming target product only after ownership and lifetime are proven.
Non-claims: no source-bound relation product, S6C Facts/Recipe producer,
Builder/MIR/Boundary/LLVM/runtime, production selector, fallback, retry,
legacy retirement, or new semantic receipt.
```

Acceptance for this D0 is design-only: name the resolver issuer, prove the
source-row/frame containment API, enumerate the exact Text/StringBox/Home and
effect axes, and publish the negative matrix. If any axis still needs a
method-name lookup, MIR-derived Text/Home inference, or an unowned frame
tuple, the row remains `NoSafeSlice` and no I0 implementation starts.

The generic `LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0/I0` remains a separate
parked row for user-declared instance methods. It must not be relabeled as the
StringBox/CoreMethod issuer. The closed manifest-target evidence is not yet a
Resolver callable contract: the future contract row must not infer `Text`
Home from `MirType` or `CoreMethodResultKind`, and must not pair a target by
Box/method name after the generated row is sealed. Its exact co-seal must
include:

```text
same CoreMethod manifest/schema brand
StringBox receiver semantic contract
StringLen: arity 0 -> I64, PureRead
StringSubstring: arity 2 -> Text/StringValue, PureRead
explicit receiver/parameter/result Home relation (no default)
non-suspending + non-control policy
runtime owner/export profile as a downstream projection only
```

After the canonical callable-contract row closes, the source-bound relation
will consume/borrow that target and add the exact
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

Historical manifest-target acceptance is fail-closed and source-first:

```text
positive: one same-brand CoreMethod target pair with exact arity/result/effect,
          explicit Home relation, ABI/profile, manifest brand, and no
          suspension/control;
negative: foreign/duplicate/swapped target, String vs StringBox mismatch,
          wrong arity/result/Home/effect, missing source site, Text inferred
          from MIR/CoreMethod output, name lookup, or partial Facts coverage;
          foreign/duplicate owner-frame or swapped receiver/argument/result
          site cardinality is rejected;
guard: the target issuer has no source-bound/S6C production consumer until
       the Resolver callable contract and relation rows close, and no selected
       Dynamic receipt is imported by the Loop lane.
```

Non-claims remain strict: no Builder/MIR/CheckedCallOut/Boundary, no physical
IDs or ABI, no production selector, no fallback/retry, no legacy retirement,
and no new `Verified*`/`Prepared*` product is issued by this census.

## Historical schema boundary and next change

The V2 schema decision is closed. Keep S6C as a design stop and split the
remaining work into these bounded rows:

1. current prerequisite: a Resolver-owned callable contract and exact
   MethodCall-site-to-loop-frame bridge;
2. later source-bound relation and typed input ownership;
3. a later `ScanWithInit` source observer/producer using those products.

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

## D0 owner map (landed contract; I0 implementation)

The D0 names the owners below; I0 issues only the target capability. The
source-bound consumer remains the next unopened boundary:

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

The existing generic `HomeDemandV1` / `HomeResultRelationV1` variants are
mechanisms, not a semantic fit for this target. The future manifest-backed
issuer must expose dedicated typed relations (for example, a `StringBox`
receiver demand, typed `I64` parameters, and `I64`/`Text` result relations)
under the same manifest and target brand. Reusing `Trivial`, `FromReceiver`,
or `FromParameter` as an untyped alias would recreate the I64/Unit authority
collision this D0 is meant to prevent. `StringLen/0` and `StringSubstring/2`
must therefore be co-sealed with their exact receiver, parameter, result,
effect, and Home relation shape; foreign, missing, duplicate, or inferred
relations reject before any Facts/Recipe product.

### LOOP-CORE-METHOD-MANIFEST-HOME-ISSUER-D0 / I0

```text
Decision: design a separate manifest-backed StringBox/Text Home issuer; do not widen I64UnitTrivial in place.
Source authority + canonical issuer: CoreMethodContractBox/generated row brand plus an explicit CoreMethod Home capability issuer.
Non-authority: user-instance declaration/Home catalog, MIR types, ResultKind, names, DynamicMember receipts, or Recipe wire.
Fail-fast boundary: foreign/mixed schema or brand, wrong receiver/arity/result/effect/ABI, missing Home, and duplicate target reject before any receipt.
Smallest next slice: issue the exact target product and focused positive /
negative evidence; keep source-bound consumption unopened.
Non-claims: no source-bound call product, Facts/Recipe producer, Builder/MIR/Boundary route, fallback, retry, or production switch.
```

### D0 exact typed Home contract (I0 implemented)

The issuer input is one generated CoreMethod row plus the same manifest/schema
brand, an exact operation/arity, and an explicit Home capability schema. Its
design-only output is a non-forgeable target contract with a distinct target
brand and dedicated typed relations:

```text
StringLen / arity 0:
  receiver = StringBoxReceiver
  parameters = []
  result = I64ToCaller
  effect = PureRead

StringSubstring / arity 2:
  receiver = StringBoxReceiver
  parameters = [I64Parameter, I64Parameter]
  result = TextToCaller
  effect = PureRead
```

`StringBoxReceiver`, `I64Parameter`, `I64ToCaller`, and `TextToCaller` are
semantic relation shapes, not aliases for `Handle`, `Trivial`,
`FromReceiver`, or `FromParameter`. The target brand, manifest/schema brand,
and relation-batch brand are co-sealed. Missing Home, a union-arity row,
foreign/mixed brands, inferred Text, wrong result/effect, or duplicate target
rejects before any target capability. I0 implements this issuer contract and
its focused negative matrix without adding a source-bound consumer or
`Verified*`/`Prepared*` execution product.

### D0 exit / I0 acceptance boundary

The design decision is bounded and `CURRENT_STATE.toml` selected
`LOOP-CORE-METHOD-INSTANCE-TARGET-I0`. The conditions below are its
implementation completion gate:

```text
source authority = CoreMethodContractBox/generated row under the same
  manifest/schema brand;
canonical issuer = one non-forgeable target product from
  (generated row, exact operation/arity, explicit typed Home schema);
positive evidence = StringLen/0 and StringSubstring/2;
negative evidence = foreign/mixed brand, wrong receiver/arity/result/effect/
  ABI, missing Home, duplicate target, and Text inferred from MIR/ResultKind;
failure terminal = reject before target receipt issuance;
guard evidence = generated-brand/issuer census and no raw name/MIR/Dynamic lookup;
non-claims = no source-bound consumer, Facts/Recipe producer, Dynamic import,
  Builder/MIR/Boundary route, production switch, fallback, or retry.
```

I0 closeout evidence: the generated manifest brand is checked before issue;
`StringLen/0` and `StringSubstring/2` specialize to distinct typed Home
relations; foreign brand, wrong receiver/effect/result, union arity, and
duplicate target tests reject before capability issuance. The issuer module
has no source-bound consumer, Facts/Recipe producer, Dynamic import, Builder,
Boundary, production switch, fallback, or retry. The source-bound call
relation is a later bounded row and must not be folded into this issuer gate.

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
