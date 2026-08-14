---
Status: Recipe I0 landed; JOINIR consumer is the next design boundary
Date: 2026-08-15
Decision: consume the complete Facts once into one source-retaining V2 Recipe/role/Join product
Scope: M8 LoopV0 forward ScanWithInit source/Facts/Recipe; no physical activation
---

# LOOP-RECIPE-TYPED-CALL-VALUE-D0

## Current Capsule

- **Current decision:** the complete non-Clone S6C Facts product is consumed by
  one landed Recipe producer. Its non-Clone product retains Facts, the verified
  V2 Recipe, an exact named role-to-key map, and the existing V2 Join closure;
  `LoopRecipeArtifactV2` is not issued in this row.
- **Current implementation status:** Loop rows 1--10, M8 S6A/S6B, the
  CoreMethod/Home target, placement-aware callable contract, typed-input/call
  witness, fixed source-bound relation, Exit/Tail co-seal, Facts I0, and Recipe
  I0 are closed. The product and selector remain caller-zero.
- **Next ordered task:** design the combined-product JOINIR consumer
  `JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-I0` before any Recipe-only handoff.
- **Production stop line:** no scan selector, physical route, fallback, or
  production caller is opened by Recipe I0.
- **Retirement finish line:** after a real S6C implementation and parity,
  update the reference contract in that same implementation commit; legacy
  scan facts/builders remain until an explicit cutover row deletes them.

## Resumption brief

```text
Decision: consume VerifiedS6CScanWithInitFactsV1 once and retain one exact V2
Recipe/role-map/Join product; the next consumer must borrow that combined
product and source Artifact remains a later design boundary.
Source authority + canonical issuer: Facts owns source truth; the S6C Recipe
producer alone issues Recipe-local keys; existing V2 issuers own verify/Join.
Non-authority: AST/name/order, source rewalk, MIR, physical IDs, raw JoinSig,
an Artifact/source claim reconstructed from a Facts borrow, fallback, or retry.
Fail-fast boundary: exact role map, V2 structural verification, and sole-root
carrier Join closure plus its logical transfer view all close before the
product becomes borrowable; only private read facades are lent afterward.
Smallest next slice: design the JOINIR consumer façade and its fail-fast
handoff without accepting a Recipe-only input.
Non-claims: no Artifact/provenance, physical consumer, selector, production
caller, Builder/MIR, backend, legacy deletion, fallback, or retry.
```

## Landed Recipe I0 implementation receipt — 2026-08-15

The bounded implementation follows the accepted design. Six read-only audits
had confirmed that the fixed map is representable by the existing V2 schema and
Join issuer. They also exposed one deliberate boundary:
Facts has no consuming `LoopRecipeSourceBindingV1` terminal. Recipe I0 therefore
must not invent a borrow-based source claim merely to manufacture
`LoopRecipeArtifactV2`. Retaining the original Facts beside the verified Recipe
is the smaller source-to-Recipe co-seal; Artifact/provenance stays closed until
a named consumer proves it necessary.

```text
VerifiedS6CScanWithInitRecipeProductV2
  facts: VerifiedS6CScanWithInitFactsV1
  recipe: VerifiedLoopRecipeV2
  roles: private fixed S6C role-to-key seal
  join: VerifiedLoopJoinClosureV2
  join_role_seal: VerifiedS6CJoinRoleSealV2
```

The product is non-Clone and has no `into_parts`, raw Recipe/JoinSig getter, or
owned constituent getter. One HRTB callback lends only private read facades:
the Facts view, `S6CVerifiedRecipeReadViewV2`, the fixed role view, and a
prevalidated logical-transfer view. It never lends `&VerifiedLoopRecipeV2`,
`&VerifiedLoopJoinClosureV2`, `as_recipe()`, `into_recipe()`, or `join_sig()`;
HRTB alone cannot prevent a `Clone`-based owned Recipe escape. The fixed role
struct is the sole semantic-role-to-Recipe-key authority; a map/vector and
downstream rediscovery by item order or operation shape are forbidden.

```text
produce_s6c_scan_with_init_recipe_v2(facts by value)
  -> borrow the already verified named Facts roles
  -> issue the fixed private Recipe-local key table exactly once
  -> materialize the exact map below
  -> LoopRecipeVerifierV2::verify
  -> co-check the fixed role struct against every verified Recipe domain
  -> issue_sole_root_carrier_join_closure_v2 exactly once
  -> obtain logical_transfer_view once and co-check it against the role struct
  -> publish the one non-Clone product
```

The role seal proves exact domain coverage, not merely the named hot roles:
Loops 1, Blocks 3, Bindings 1, Inputs 3, Values 15, Items 15, Carriers 1,
Exits 1. The Join co-check proves After=`L0/B0/I64`, one branch=`I8/V10`,
then=`I10` FunctionExit Return, else=Fallthrough, Return summaries=1, and
Backedges=1 before publication. The next JOINIR consumer must borrow this
combined product facade; accepting the verified Recipe alone is forbidden.

`CallSlot` deliberately carries no target/Home/effect. Those authorities stay
inside the retained source-bound Facts relation and are paired with exact
Recipe item/value keys only through the combined product view. The generic V2
verifier owns canonical keys, references, value classes, definition order,
carrier availability, blocks, and exits; it is not a source-role classifier.

### Exact target map

```text
L0 root; K0 Condition; K1 Body; K2 TextEq-then; B0 index:I64
C0 = { owner L0, binding B0, class I64, entry V2 }
inputs = V0 subject:Text, V1 needle:Text, V2 initialized-index:I64

K0: I0 Read(B0)->V3
    I1 CallSlot(V0,[])->V4                         length
    I2 Less(V3,V4)->V5
K1: I3 Read(B0)->V6
    I4 ConstI64(1)->V7
    I5 Add(V6,V7)->V8
    I6 CallSlot(V0,[V6,V8])->V9                   substring
    I7 TextEq(V9,V1)->V10
    I8 If(V10, then K2)
    I11 Read(B0)->V12
    I12 ConstI64(1)->V13
    I13 Add(V12,V13)->V14
    I14 Write(B0,V14)
K2: I9 Read(B0)->V11
    I10 Exit(E0 = Return(V11))
```

Cardinality is exact: Loop 1, Blocks 3, Binding 1, Inputs 3, Carrier 1,
Values 15, Items 15, Exits 1. The items are 13 operations + If + Loop Return.
The callable Tail `return -1` stays in retained Facts/Completion and is absent
from Recipe items, values, exits, and JoinSig Return rows.

### One implementation-coupled commit

```text
Change:
  landed loop_recipe_contract/s6c_scan_with_init.rs plus
  s6c_scan_with_init_tests.rs; consume Facts once and issue the exact product;
  old authority: none.
Contract:
  only the producer mints S6C keys; typed_schema_v2.rs (757) stays frozen;
  source Artifact, physical identities, selectors, and legacy builders stay 0.
Done:
  exact positive map/private product view; complete domain bijection and Join
  transfer parity; generic V2 key/domain/use-before-def and Join negatives remain
  green; owner README and docs/reference/mir/loop-recipe-contract.md updated;
  focused producer test passes.
Stop:
  any second source walk/Facts issuer, raw external key input, source-binding
  reconstruction, raw Recipe/Join/JoinSig escape, incomplete role coverage,
  MIR/physical/fallback/retry requirement.
```

No new top-level guard is added. Reuse the S6C/Loop/CoreMethod/pointer guard
families and keep new/touched production sources below 760 lines (800 hard
stop). `sunset = n/a`, `net_proof_delta = 0`.

The ordered frontier is:

```text
landed S6C source/CoreMethod/typed-input/call/unary/ExitTail authorities
  -> LOOP-S6C-SCAN-WITH-INIT-FACTS-D0             CLOSED T2
  -> LOOP-S6C-SCAN-WITH-INIT-FACTS-I0             CLOSED T2 BoxShape
  -> LOOP-S6C-SCAN-WITH-INIT-RECIPE-I0            CLOSED T2 BoxCount
  -> JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-I0           T2 consumer
  -> S6C parity/canary
  -> bounded selector/caller cutover
  -> latest-HEAD integration evidence
  -> legacy retirement
```

### Accepted Exit/Tail I0 contract

```text
input:
  VerifiedSourceBoundS6CCallRelationV1 by value
  VerifiedFunctionCompletionV1 by value
  CallableSemanticSourceLedgerView borrowed for exact resolver rows

classification:
  completion = ExplicitReturns(Value), count = 2, cleanup = empty
  Loop Return value = exact Local(index) resolver row
  Loop Return region = exact If-then region whose condition is TextEq
  callable Tail value = exact Integer(-1) resolver literal row
  callable Tail region = root function-body sequence and terminal site

output:
  non-Clone VerifiedS6CExitTailSourceCoSealV1
  owns both consumed products and fixed inner/tail source evidence
  lends one HRTB view; Clone/raw constructor/into_parts = 0
```

The resolver If-region index may add one narrow condition-expression lookup;
it returns the exact existing If site/bundle and never reconstructs control
from an AST. The co-seal checks owner, frame, target function, Completion site
set, region, value, and literal parity before construction. The later Facts
issuer consumes this whole product; it may not re-pair a source relation and
Completion independently.

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
API: issue(ledger, call, borrowed_membership, placement, target)
     -> non-Clone callable contract.
Source checks: call/receiver/all args/result in one owner inventory;
  args.len == target arity; ordinals are exactly 0..arity; result_site=call;
  selector is only a canonical/alias cross-check; target is never selected by
  selector.
Frame checks: membership origin/kind equals ledger; every site has the exact
  requested placement; length/0 = Condition and substring/2 = Body.
Target checks: manifest/schema/target brand, StringBox receiver,
  StringLen/0 -> I64 or StringSubstring/2 -> Text, PureRead,
  NonSuspendingNonControl, and explicit Home relations.
Ownership: membership is borrowed, target is consumed, and the contract owns
  exact sites plus frame/site/placement projections; no membership is reissued.
Negative: foreign/mixed owner/frame/brand, missing/duplicate/swapped site,
  placement drift/outside Loop, QualifiedUnbound/CurrentOwner/Other receiver,
  wrong op/arity/result/Home/effect/policy, and name/MIR inference.
```

The code-facing I0 must stay below the 760-line split trigger and 800-line
hard stop. It must reuse the existing resolver source/frame APIs rather than
adding a second path inventory.

D0 evidence is retained as the I0 review gate: any implementation that still
needs a method-name lookup, MIR-derived Text/Home inference, or an unowned
frame tuple must fail closed and return the pointer to the D0 boundary.

### LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0 / I0 (LANDED)

```text
Decision: use explicit source annotations as the only typed-input authority;
do not create a call-flow type issuer. The accepted cohort requires
`s: StringBox`, `ch: StringBox`, and `local i: i64 = 0`. Resolver then issues
one AST-free typed-input/source-frame product and co-seals Binary
`TextEq/Less/Add`, initializer/literal, and `Condition|Body` placement with
the landed callable contract.
Source authority + canonical issuer: parser `ParamDecl`/callable source rows,
resolver binding/source ledger, and one new `VerifiedS6CTypedInputRelationV1`
issuer. `StringBox` and `i64` are exact source spellings; `Text` and `I64` are
the internal logical classes. The existing resolver callable/frame I0 and
generated StringBox/Home target are borrowed, not reissued.
Non-authority: names-only `FunctionSyntaxViewV1`, unannotated `s/ch`,
call-site literals, method shape, CoreMethod result rows, MIR/ResultKind,
selected-Dynamic receipts, `BodyExpressionShapeV1::Other`, or any call-flow
inference/coercion.
Fail-fast boundary: missing/noncanonical annotation, missing local type or
literal-0 initializer, foreign/mixed owner/frame, duplicate/swapped binding or
operand, wrong operator/result/placement/effect/control, or tail leakage
rejects before source-bound relation, Facts, Recipe key, Builder/MIR, or
production effect.
Smallest next slice: none; I0 landed the resolver-owned typed-input product,
AST-free binary/initializer rows, canonical placement, and focused evidence.
Non-claims: no call-flow issuer, S6C Facts/Recipe, Recipe key, source-bound
receipt, physical lowering, production selector, fallback/retry, or legacy
retirement.
```

I0 evidence: the annotated fixture plus six fail-fast negatives pass (7/7),
`cargo check --lib` passes, all touched Rust sources remain below 760 lines,
and the historical unannotated fixture is still `MissingTypeEvidence`.
The resolver family is 298/299 green; its sole receiver-authority failure
reproduces at parent `f9f2389a4c` and is classified baseline debt. The stable
resolver guard also names an already-missing `direct_call_tests.rs` at that
parent, so its early red is stale guard inventory rather than this I0.

The landed placement I0 proves the following bridge:

```text
length/0 in Condition + substring/2 in Body
  -> Resolver MethodCall source row
  -> borrowed exact Loop membership + placement
  -> StringBox/Text CoreMethod target
```

It also retains `TextEq(Text, Text) -> Bool`, `i < length`, both `i + 1`
relations, `i = i + 1`, and exact subject/needle/index bindings. This remains
a prerequisite only: the next D0 must define one source-bound call product
before any Facts observer or Recipe producer opens.

The current fixture `apps/tests/scan_with_init_ok_min.hako` declares
`find_ok(s, ch)` without parameter annotations, while `FunctionSyntaxViewV1`
retains parameter names and body but drops `ParamDecl` type declarations. It
is therefore a required `MissingTypeEvidence` negative. The positive I0 fixture
must carry exact `s: StringBox`, `ch: StringBox`, and `local i: i64 = 0`
annotations through parser callable rows into the resolver issuer. String
literals, method shape, parameter names, or MIR result classes cannot silently
prove `Text` or `I64`, and no call-flow issuer is permitted.

Required negative matrix for the D0: foreign or mixed owner/frame, missing or
duplicate input binding, swapped subject/needle/index, non-Text receiver or
needle, non-I64 index, `==` with a non-Text operand, non-Bool comparison result,
wrong Less/Add operand or write target, call/operator outside the selected
LoopBody, extra unsupported effect/control, and `return -1` incorrectly
absorbed into Loop Facts. No AST/name/MIR inference or selected-Dynamic receipt
reuse may satisfy any of these rows.

### Resolver placement contract receipt

The existing resolver issuer is now placement-aware. It borrows the typed
product's non-Clone membership, consumes the exact target, and retains only
source sites plus frame/site/placement projections. Condition/Body drift,
outside/foreign membership, and target-placement mismatch reject before issue.
It still issues no source-bound relation, Facts, Recipe key, MIR identity, or
physical layout.

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

The V2 schema decision and typed-input D0 are closed. Keep the source-bound
and Facts/Recipe rows unopened and split the remaining work into these bounded
rows:

1. current implementation: explicit typed-input/binary/placement resolver I0;
2. later source-bound relation consuming the typed-input and callable products;
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
5. **Input ownership:** `s: StringBox`, `ch: StringBox`, and
   `local i: i64 = 0` are explicit input-source relations. The resolver typed
   input issuer owns their exact `BindingRef`, declaration/initializer sites,
   and owner/frame brand. Do not create a call-flow or scan-specific second
   type owner; the unannotated fixture is rejected as missing evidence.
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

## First implementation slice after D0 — LOOP-S6C-EXPLICIT-TYPED-INPUT-CONTRACT-I0

Only a positively annotated forward `ScanWithInit` fixture is eligible. The
existing unannotated fixture is a negative and must never be upgraded by
inference. Its semantic roles are:

```text
inputs: `s: StringBox`, `ch: StringBox`, initialized `i: i64 = 0`
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
                 -> LOOP-S6C-RESOLVER-BINARY-AND-TYPED-INPUT-D0 CLOSED
                      -> LOOP-S6C-EXPLICIT-TYPED-INPUT-CONTRACT-I0 CLOSED
                           -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0 CLOSED
                                -> LOOP-S6C-EXACT-CALL-WITNESS-R0 CLOSED T0
                                     -> LOOP-RESOLVER-CALLABLE-PLACEMENT-I0 CLOSED T1
                                          -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0 CLOSED T1
                                               -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0 CURRENT T2
                                                    -> LOOP-RECIPE-TYPED-INPUT-RELATION-I0 T2
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

## LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0 (accepted)

The direct aggregate was `NoSafeSlice`: the typed product validated but did
not retain the exact two call sites, while the landed callable issuer admitted
only `LoopBody` and therefore rejected `length/0` in `LoopCondition`. The
accepted correction reuses the existing authorities in three ordered cells:

```text
LOOP-S6C-EXACT-CALL-WITNESS-R0
  retain length_site = LoopConditionLess.rhs
  retain substring_site = TextEqual.lhs
  private borrowed role view; no selector lookup or new membership

LOOP-RESOLVER-CALLABLE-PLACEMENT-I0
  existing callable issuer accepts an explicit ResolvedLoopPlacementV1
  length/0 = Condition; substring/2 = Body
  same owner/frame/receiver/args/result/target checks remain canonical

LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0
  consume typed input plus length and substring targets issued by one
  CoreMethodInstanceTargetIssuerV1 session
  issue one non-Clone fixed { length, substring } aggregate
```

The target issuer may issue both targets by value. The relation checks their
common manifest/schema/relation brand and distinct target brands; it does not
mint a pair brand. Exact source sites come only from the retained structural
relations. The final product owns the sole typed-input membership and the two
consuming callable contracts; its HRTB view may expose bindings, calls,
placement, frame, Home/effect/ABI, and target relations without parts APIs.

Missing, duplicate, swapped, foreign, orphan, placement-drifted, or
brand-drifted evidence rejects before the aggregate and before Facts/Recipe.
No AST/name/selector lookup, generated-row relookup, Dynamic receipt, MIR/
ResultKind inference, Recipe key, physical ID, fallback, or retry is allowed.

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

## Parked cleanup queue (not current)

- `MIRBUILDER-RESTART-DOCS-SURFACE-CLEANUP-R0`: repair stale/broken Builder
  README paths and frontier text; state task-map/manifest applicability; link
  Return/fallback terminology; repair fence/date/baseline display drift.
- `RUST-WARNING-SURFACE-CENSUS-R0`: required cleanup because the current
  `cargo check` warning surface is too large to remain informal. The current
  `cargo check -q --lib` observation emits 1,827 warning headings; the task
  must normalize that into a machine-readable owner/code/count baseline first
  and classify current-change versus inherited debt, then remove unused import, private-interface,
  unreachable/dead compatibility, and stale cfg warnings in bounded owner
  slices. No blanket `allow`, semantic change, or S6C-row mixing is allowed.
- `TEST-RED-BASELINE-RETIREMENT-D0`: classify inherited builder/compiler reds
  and prevent a permanently accepted red baseline.
- `DOC-GOVERNANCE-AND-COMPAT-RETIREMENT-D0`: shrink/archive historical docs,
  tier ceremony, and census NYASH/llvmlite/VM/facade compatibility owners.

The normalized `ScanWithInit` operation/item counts remain provisional until
parameter-input relations, instance target issuance, and source-bound call
relations are sealed. No fixture or scan observer is part of this row.
