---
Status: Resolver callable-contract I0 landed; current pointer is S6C binary/input D0 design stop
Date: 2026-08-14
Decision: co-seal existing Resolver MethodCall source rows, explicit LoopBody frame membership, and the existing manifest target in one non-Clone I0 product; design the missing AST-free binary/operator, typed-input, and Condition|Body placement source boundary before opening the source-bound relation/S6C producer
Scope: M8 LoopV0 forward ScanWithInit source/Facts/Recipe; no physical activation
---

# LOOP-RECIPE-TYPED-CALL-VALUE-D0

## Current Capsule

- **Current decision:** the V2 typed schema, neutral operation split, and
  Resolver callable-contract I0 are landed. The next frontier is
  `LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0`; source-bound relation and
  complete Facts/Recipe remain unopened.
- **Current implementation status:** Loop rows 1--10, M8 S6A/S6B, the
  CoreMethod/Home target issuer, and Resolver callable-contract I0 are closed
  with focused positive/negative evidence. No forward `ScanWithInit`
  Facts/producer or production physical selector is active.
- **Next ordered task:** `LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0` — design
  the operator-bearing binary source row and typed subject/needle/index input
  relation. Do not reuse selected-Dynamic relations or issue Facts/Recipe.
- **Production stop line:** no scan selector, physical route, fallback, or
  production caller is opened by this design row.
- **Retirement finish line:** after a real S6C implementation and parity,
  update the reference contract in that same implementation commit; legacy
  scan facts/builders remain until an explicit cutover row deletes them.

## Resumption brief

```text
Decision: continue Loop at `LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0`; do
not replay closed rows 1--10 or select production row 11 early.
Source authority + canonical issuer: resolver declaration/catalog and
`VerifiedResolvedMethodCallSourceV1`/callable source ledger for exact owner,
site, receiver, ordered args, result, and LoopBody frame; the new I0 issuer
co-seals those rows with `VerifiedCoreMethodInstanceTargetV1`.
Non-authority: generated rows alone, `home_abi.rs` I64/Unit defaults, MIR or
ResultKind inference, raw AST/name lookup, selected-Dynamic receipts, the
Resolver I0 as a source-bound relation, legacy scan builders, CheckedCallOut
IDs, physical order, and task-map history.
Fail-fast boundary: missing Text/Home contract, foreign/mixed owner or frame,
missing/duplicate/swapped source relation, or unknown effect/suspension/control
rejects before the I0 product is issued.
Smallest next slice: T2-design `ResolvedBinaryExpressionSourceV1`, the
resolver-owned subject/needle/index typed-input relation, and a canonical
`ResolvedLoopPlacementV1::{Condition,Body}` view, then co-seal their coverage
with the landed length/substring callable contract. Leave
implementation and Facts/Recipe issuance closed until every issuer is named.
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
  the Resolver callable-contract I0 now closes the source/frame/target bridge;
  the source-bound relation, typed inputs, and complete S6C Facts/Recipe
  producer remain NoSafeSlice/design_stop. Selected-Dynamic substring/indexOf
  receipts are a different owner and are not evidence.
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
  Loop membership/frame products; the landed Resolver callable-contract I0
  now co-seals the call-site-to-loop-frame bridge with the target capability.
```

The ordered task DAG is now explicit and bounded:

```text
S6C-AUTHORITY-CENSUS-R0                         CLOSED (read-only)
  -> LOOP-CORE-METHOD-INSTANCE-TARGET-D0       CLOSED (accepted design)
       -> LOOP-CORE-METHOD-INSTANCE-TARGET-I0  CLOSED (bounded manifest evidence)
            -> LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0  CLOSED (accepted design)
                 -> LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-I0  CLOSED bounded bridge
                      -> LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0  T2
                           -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0  T2
                                -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0  T2
                                     -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0/I0 T2
                                          -> JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-I0 T2
                                               -> S6C parity / canary / later production rows
```

### LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0 (accepted)

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

The D0 is accepted and its bounded I0 is landed: the resolver issuer,
source-row/frame containment API, exact Text/StringBox/Home/effect axes, and
negative matrix are named below. If the next S6C design still needs a
method-name lookup, MIR-derived Text/Home inference, or an unowned frame tuple,
it must return to `NoSafeSlice` rather than widening the row.

### LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-I0 (LANDED)

```text
Decision: issue one non-Clone resolver callable-contract product; do not add
a Static/Dynamic source-call enum arm or reuse Dynamic receipts.
Source authority + canonical issuer: existing resolver MethodCall source row,
sealed loop-region index/LoopExecutionFrameKeyV1, and existing
VerifiedCoreMethodInstanceTargetV1, co-sealed by one resolver-owned issuer.
Non-authority: AST/name lookup, selector-based target selection, MIR/Recipe/
physical IDs, only_loop_site(), generic route plans, and source_call_target
Static/Dynamic products.
Fail-fast boundary: foreign owner, absent source inventory, missing exact loop
bundle, non-LoopBody path, ambiguous/mixed frame, receiver mismatch, wrong
arity/ordinal/result site, wrong target brand/op/result/Home/effect/policy,
duplicate/swapped source rows reject before product issue.
Smallest next slice: none; implementation landed in the resolver module below
the source split budget and production consumer remains zero.
Non-claims: no Recipe keys, Facts producer, Builder/MIR/Boundary/LLVM,
physical ABI, production selector, fallback, retry, or legacy retirement.
```

I0 acceptance:

```text
API: issue_method_call_loop_body_membership(function, call, selected_loop)
     -> exact resolver frame witness;
     issue_resolver_core_method_callable_contract(function, call,
     selected_loop, target) -> non-Clone product.
Source checks: call/receiver/all args/result in one owner inventory;
  args.len == target arity; ordinals are exactly 0..arity; result_site=call;
  selector is only a canonical/alias cross-check; target is never selected by
  selector.
Frame checks: selected loop has one sealed bundle; call is a descendant of
  that loop's LoopBody, not LoopCondition/outside/another loop; no implicit
  only_loop_site selection.
Target checks: manifest/schema/target brand, StringBox receiver,
  StringLen/0 -> I64 or StringSubstring/2 -> Text, PureRead,
  NonSuspendingNonControl, and explicit Home relations.
Ownership: target is consumed by value; source/frame rows are borrowed only
  during validation and the resulting product owns exact source sites plus
  the resolver frame witness; Clone/into_parts/raw constructors are absent.
Negative: foreign/mixed owner/frame/brand, missing/duplicate/swapped site,
  call outside LoopBody, nested/ambiguous frame, QualifiedUnbound/CurrentOwner/
  Other receiver, wrong op/arity/result/Home/effect/policy, and name/MIR
inference. Production caller = 0; S6C producer = 0. Evidence: two focused
resolver tests pass; the callable-source-ledger family (8 tests) passes; the
CoreMethod manifest, Loop precutover, pointer, and diff guards pass.
```

The code-facing I0 must stay below the 760-line split trigger and 800-line
hard stop. It must reuse the existing resolver source/frame APIs rather than
adding a second path inventory.

D0 evidence is retained as the I0 review gate: any implementation that still
needs a method-name lookup, MIR-derived Text/Home inference, or an unowned
frame tuple must fail closed and return the pointer to the D0 boundary.

### LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0 (CURRENT DESIGN STOP)

```text
Decision: keep S6C at NoSafeSlice until Resolver owns an operator-bearing
binary source row, a typed subject/needle/index input relation, and canonical
Condition|Body placement; co-seal those with the landed length/substring
callable contract before any Facts or Recipe producer is opened.
Source authority + canonical issuer: the existing resolver source inventory,
binding/declaration ledger, and the landed callable-contract issuer are
reusable primitives; the missing issuer must preserve binary operator/result
site and Text/I64 input class without rereading AST or MIR.
Non-authority: BodyExpressionShapeV1's `Other` binary kind, syntax-facts AST
walks, LoopRecipeV2's structural TextEq/CallSlot wire, MIR/ResultKind bits,
selected-Dynamic substring/indexOf receipts, selector/name lookup, or guessed
value classes/effects.
Fail-fast boundary: missing/foreign/duplicate/swapped owner, operator, result
site, operands, binding/initializer, Text/I64 class, Condition|Body placement,
Loop frame, effect, control, or return/tail coverage rejects before
source-bound relation, Facts, Recipe key, Builder/MIR, or production effect.
Smallest next slice: design `ResolvedBinaryExpressionSourceV1` for TextEq,
Less, and Add, one resolver-owned typed input/initializer relation for
subject:Text, needle:Text, and initialized index:I64, and a placement view that
admits `length` in Condition while keeping substring/TextEq/step in Body;
define exact cardinality and the later source-bound consumer without
implementing it here.
Non-claims: no S6C Facts/Recipe, Recipe key, source-bound receipt, physical
lowering, production selector, fallback/retry, or legacy retirement.
```

The current audit proves only the following partial bridge:

```text
length/0 + substring/2 (only when the selected call is already in Body)
  -> Resolver MethodCall source row
  -> LoopBody frame containment
  -> StringBox/Text CoreMethod target
```

It does not prove `TextEq(Text, Text) -> Bool`, `i < length`, `i + 1`,
`i = i + 1`, or `subject/needle/index` typed ownership. Existing binding rows
carry owner/origin but no value class, and existing binary source rows erase
the operator/result relation. Therefore the next implementation cannot be a
Facts observer or Recipe producer. It must first close this D0 or remain
`NoSafeSlice`.

The actual fixture also places `s.length()` in `LoopCondition`, not Body. The
landed I0's Body-only containment must not be widened ad hoc by a consumer; D0
must issue a canonical placement view with `Condition` and `Body` variants and
co-seal the exact placement for every call/operator row.

The fixture itself is also not a typed source authority yet:
`apps/tests/scan_with_init_ok_min.hako` declares `find_ok(s, ch)` without
parameter annotations, while `FunctionSyntaxViewV1` retains parameter names
and body but drops `ParamDecl` type declarations. The D0 must explicitly choose
one source-backed solution—typed declaration preservation/annotation or a
separately issued call-flow input product—and reject the other as unavailable.
String literals, method shape, parameter names, or MIR result classes cannot
silently prove `Text` or `I64`.

Required negative matrix for the D0: foreign or mixed owner/frame, missing or
duplicate input binding, swapped subject/needle/index, non-Text receiver or
needle, non-I64 index, `==` with a non-Text operand, non-Bool comparison result,
wrong Less/Add operand or write target, call/operator outside the selected
LoopBody, extra unsupported effect/control, and `return -1` incorrectly
absorbed into Loop Facts. No AST/name/MIR inference or selected-Dynamic receipt
reuse may satisfy any of these rows.

### Resolver contract D0 witness and I0 boundary

The D0 must close one exact resolver-side witness before any code-facing
relation product is issued:

```text
source rows:
  VerifiedResolvedMethodCallSourceV1
    = owner + call/receiver/argument/result sites + resolver receiver class
      + selector/arity (cross-check only, never target selection)

frame rows:
  VerifiedCallableLoopMembershipV1 / LoopExecutionFrameKeyV1
    = same resolver owner + one exact LoopBody containment witness
      + scope/region pair; never only_loop_site() on a multi-loop owner

contract rows:
  explicit StringBox/Text receiver and Home relation
  StringLen/0 -> I64 and StringSubstring/2 -> Text/StringValue
  PureRead + non-suspending + non-control policy
  generated manifest/schema brand cross-check
```

The bridge must prove `method_call.site` is an expression descendant of the
selected frame's sealed `LoopBody` source path and that receiver, every
ordered argument, and the result site belong to the same owner inventory. A
call outside the frame, a nested/foreign frame, or a multi-loop owner without
an explicit selected frame is unresolved/rejected; it is never repaired by a
route-local path scan or `only_loop_site()`.

The later I0 may use a short-lived borrowed callback while validating, then
issue a non-`Clone` source-bound product that owns only call/receiver/argument/
result sites and the frame witness. It may move/borrow the existing
`VerifiedCoreMethodInstanceTargetV1`; it may not reissue generated rows or
store AST, selector authority, Recipe keys, `ValueId`, `BasicBlockId`, ABI, or
physical layout. The exact owned-vs-HRTB choice is an I0 decision, not a D0
guess.

I0 acceptance is therefore: one named resolver issuer, one owner/frame bridge,
exact call/receiver/argument/result cardinality (`args.len == arity`, ordinal
coverage `0..arity`, `result_site == call_site`), same manifest/schema brand,
and negative tests for foreign/mixed/duplicate/swapped sites, call-outside-
LoopBody, nested/ambiguous frame, `QualifiedUnbound`/`CurrentOwner`/`Other`
receiver, wrong StringBox/result/Home/effect/policy, and name/MIR inference.
Until these are design-accepted, source-bound relation I0 and S6C remain
closed.

The generic `LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0/I0` remains a separate
parked row for user-declared instance methods. It must not be relabeled as the
StringBox/CoreMethod issuer. Before the landed I0, the manifest-target
evidence alone was not a Resolver callable contract; the current I0 still
must not infer `Text` Home from `MirType` or `CoreMethodResultKind`, or pair a
target by Box/method name after the generated row is sealed. Its exact co-seal must
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
  -> LOOP-CORE-METHOD-INSTANCE-TARGET-D0       CLOSED (accepted design)
       -> LOOP-CORE-METHOD-INSTANCE-TARGET-I0  CLOSED (bounded evidence)
            -> LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-I0 CLOSED
                 -> LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0 T2
                      -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0/I0 T2
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
