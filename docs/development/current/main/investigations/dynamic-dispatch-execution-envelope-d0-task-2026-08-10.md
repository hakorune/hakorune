---
Status: Dynamic operation schema/verifier I0 closed; full producer D0 next
Date: 2026-08-10
Decision row: `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0`
Closed row: `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-I0`
Next row: `LOOP-V2-DYNAMIC-FULL-PRODUCER-D0`
Parent: `source-bound-dynamic-method-dispatch-d0-task-2026-08-10.md`
Mode: design stop / complete unchanged-source V2 producer boundary
---

# Dynamic dispatch execution envelope

The unchanged production fixture already issues exact source-backed
`substring/2` and `indexOf/1` Dynamic targets through one route-neutral
catalog. This card closes the selector-independent language contract and owns
the ordered implementation ladder from that target catalog to production
execution.

Normative language authority:

```text
docs/reference/language/dynamic-invocation.md
```

## Accepted decision

Dynamic invocation has one language-wide indivisible contract. It is not a
provider-specific contract and is not assembled by callers from freely
composable axis receipts.

```text
source authority:
  ordinary MethodCall syntax
  + resolver-proven Dynamic receiver origin
  + exact VerifiedSourceBoundDynamicMemberCallV1

effect:
  OpaqueObservable

ordering:
  SynchronousNonDetached

suspension:
  MaySuspend

outcome/control:
  Normal(SelfContainedDynamicCarrier) | Fault
  CallableBounded

receiver/arguments:
  BorrowedNoEscapeForInvocation

normal result:
  SelfContainedDynamicCarrierToCaller
```

`SynchronousNonDetached` and `MaySuspend` are compatible: the next source
operation waits for completion while the current continuation may suspend.
No implicit detach or await is introduced.

Runtime tags may choose physical storage and drop mechanics only. They do not
select semantic effect, Fault, suspension, or Home relations. A borrowed
receiver/argument result is forbidden. A normal result publishes exactly one
opaque self-contained carrier; a Fault publishes none and does not roll back
earlier effects.

## Owner table

| Meaning | Owner | Non-authority |
| --- | --- | --- |
| selector-independent language contract | `docs/reference/language/dynamic-invocation.md` | provider manifest, method name, runtime tag |
| exact source/message identity | route-neutral source target catalog | Recipe, MIR, runtime lookup |
| atomic semantic envelope | one canonical Dynamic-envelope issuer | public partial-axis constructors |
| later local install/move/release | Home Flow | runtime tag or result decoder |
| implementation compatibility | provider admission | source resolver |
| available implementation set | one immutable admitted registry | per-call registry rebuild |
| image/binding/lifecycle lease | frozen executable plan | semantic envelope |
| exactly one call and result/Fault publication | invocation transaction | retry/fallback chain |
| semantic-to-physical effect/Home projection | later named verifier | `EffectMask::ALL`, `MirType::Unknown` reverse inference |

The semantic-envelope issuer may use private axis verifiers, but it returns
only one complete product. It introduces no provider, ABI, image, `type_id`,
method ID, function address, or physical route.

## Typed failure matrix

| Condition | Result |
| --- | --- |
| language authority or canonical issuer absent | development state `NoSafeSlice`; no empty product |
| valid static/non-Dynamic source row | retained and unselected |
| foreign brand/source, duplicate row, arity mismatch, target mismatch | rejected before envelope publication |
| a backend cannot preserve the envelope | reject before effects |
| provider needs to retain an input or return an input borrow without an admitted ownership relation | provider admission reject |
| missing/ambiguous plan, unavailable image, malformed input/result, invocation failure | terminal `Fault`; no result and no retry |
| normal completion | publish one `SelfContainedDynamicCarrier` exactly once |

Fault never becomes `Void`, `Unit`, `Option`, or `Result`. Callee Return is
consumed at the callable boundary; Break, Continue, non-local Return, and
postfix-`?` do not escape the invocation.

## I0 — complete semantic-envelope catalog

`DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-I0` is the smallest executable row.

### Implementation

- add one resolver/semantic module whose public issuer consumes or retains the
  complete route-neutral source target catalog;
- issue one exact semantic-envelope row for every selected Dynamic member
  source site in the unchanged full production fixture;
- retain valid static/non-Dynamic rows as unselected input evidence;
- keep every axis selector-independent and identical for `substring/2` and
  `indexOf/1`;
- expose no public constructor for effect-only, Home-only, Fault-only, or
  suspension-only products;
- split before 760 lines and never allow a source file to reach 800 lines;
- update the owner README and this reference receipt in the same implementation
  slice.

### Required tests

```text
positive:
  unchanged full fixture
  substring/2 exact row
  indexOf/1 exact row
  complete selected coverage
  static/qualified helper rows retained and unselected
  same selector-independent contract on every selected row

negative:
  foreign target catalog/brand
  duplicate selected source row
  missing selected row
  source/target/arity mismatch
  partial-axis construction API absent
  Recipe/Builder/MIR/provider/runtime imports absent
```

### Compiler-widening rule

If the unchanged accepted source exposes a valid row that the I0 issuer cannot
carry, fix the compiler boundary or stop at a new explicit design question.
Do not shrink/rewrite the fixture, rename a method, fabricate a nominal target,
or add a selector-specific exception merely to make the row pass.

### I0 nonclaims

```text
no Recipe value or CallSlot
no Builder / MIR / CFG / PHI
no physical EffectMask or Home projection
no provider admission or executable plan
no runtime invocation or result decoding
no selector-specific String/Text/I64 refinement
no retry/fallback deletion yet
no production activation
```

## Ordered task ladder after I0

1. `LOOP-RECIPE-DYNAMIC-VALUE-D0`
   - define the logical Dynamic value carrier missing from the current typed
     Recipe schema;
   - preserve the semantic envelope without provider or physical facts.
2. `LOOP-RECIPE-DYNAMIC-CALLSLOT-I0`
   - co-seal exact source target, complete envelope, operands, and result slot;
   - no execution plan.
3. `BOXCALL-PROVIDER-ADMISSION-SEAL-I0`
   - prove provider contract compatibility;
   - publish one immutable admitted registry; duplicate overwrite is rejected.
4. `DYNAMIC-RUNTIME-EXECUTABLE-PLAN-I0`
   - freeze one target/ABI/function address plus image and lifecycle lease;
   - remove per-call registry rebuild and semantic name repair from this lane.
5. `DYNAMIC-RUNTIME-FAULT-RESULT-I0`
   - one invocation transaction;
   - exact normal carrier publication or terminal Fault;
   - no malformed decode to zero/Void and no reinvocation for short buffers.
6. `DYNAMIC-PHYSICAL-CANARY-I0`
   - fresh unpublished function session;
   - named effect/Home projection;
   - success and Fault whole-session behavior.
7. `DYNAMIC-PRODUCTION-CUTOVER-I0`
   - switch one named production caller;
   - delete that caller's retry, arity fallback, handler cascade, receiver
     repair, secondary plan, and legacy writer in the same commit.
8. `DYNAMIC-ALL-INGRESS-LIFECYCLE-CLEANUP0`
   - complete ingress parity;
   - image pin/lease and `fini != destroy` closure;
   - retire `SlowDynamic`, mutable overwrite, silent reentrancy, and remaining
     compatibility routes after their last caller is gone.

Each row is bounded by its own card or a clearly named section in this rolling
card. Do not open a parallel Dynamic dispatcher or preserve a fallback merely
to maintain historical behavior.

## Runtime invariant

Once admitted, execution is exactly:

```text
actual runtime receiver class
+ checked selector / arity
+ one immutable admitted registry
  -> one frozen executable plan with image/lifecycle lease
  -> one invocation
  -> Normal(one carrier) | Fault
```

Missing, ambiguous, rejected, or failed selection is one Fault. There is no
second plan, arity-0 retry, by-name semantic repair, provider fallback,
receiver repair, or same-effect reinvocation.

## Reference closeout rule

Every implementation row updates its landed status in the owning reference
and module README in the same commit. Future claims must distinguish accepted
language target, semantic issuer activation, Recipe activation, physical
canary, and production cutover. This D0 does not make any of them live.

## I0 closeout

`DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-I0` landed on 2026-08-10.

```text
src/mir/dynamic_invocation_contract/
  README.md
  mod.rs
  model.rs
  catalog.rs
  tests.rs
```

The implementation owns the complete route-neutral target catalog. It does
not copy keys into a second envelope map; every Dynamic arm projects one
borrow-scoped `VerifiedDynamicInvocationEnvelopeRefV1`, and every Static arm
remains retained and unselected. This makes missing/duplicate envelope rows
structurally impossible after successful issue.

Acceptance evidence:

```text
unchanged parser_scan_loop_box.hako:
  complete Dynamic target/envelope rows = 7
  skip_while/4 substring/2 envelope = 1
  skip_while/4 indexOf/1 envelope = 1
  selector-specific contract variants = 0

focused tests:
  cargo test -q dynamic_invocation_contract::tests --lib
  cargo test -q source_call_target::dynamic_member_tests --lib

crate check:
  cargo check -q --lib

source file maximum in new owner:
  227 lines
```

The issuer has no Recipe, Builder mutation, MIR effect, provider, runtime,
retry, or fallback consumer. Production envelope consumer count remains zero.

## Recipe Dynamic value Decision — accepted

`LOOP-RECIPE-DYNAMIC-VALUE-D0` is closed. The unchanged source exposed a
valid language value that the current Recipe v2 cannot name. The compiler is
widened; the fixture is not reduced, rewritten, selector-refined, or copied
into a smaller source.

### One logical class plus one source relation

The accepted boundary uses both products; neither replaces the other:

```text
LoopValueClassV2::Dynamic
  = the sole Recipe-local logical class truth

source-bound Dynamic relation
  = why an exact source value/call has that class
  + exact target and indivisible invocation envelope
```

`Dynamic` is not `Unknown`, `Any`, `Opaque`, a runtime tag, a provider type,
a Home state, or `MirType::Unknown`. Selector spellings such as `substring`
and `indexOf` never refine it to `Text` or `I64`.

The v2 wire may express type-consistent Dynamic values, inputs, bindings,
reads/writes, carriers, CallSlot normal results, and Return values. This is
logical expressivity only. Semantic-program publication still requires the
exact source relations for every admitted Dynamic row. Dynamic predicates are
forbidden, and the existing I64/Text/Bool operations keep their exact domains.
Dynamic joins and After payloads require a future `LoopJoinSigV2`; the current
V1 JoinSig is not widened or reused. An iteration-local result such as `ch`
does not become a carrier merely because its class is Dynamic.

V1 schema, verifier, normalization, JoinSig, and adapters remain unchanged.

### CallSlot/source co-seal

The V2 wire remains name- and target-free:

```text
CallSlot { receiver, args, result }
```

An item-keyed sibling relation later co-seals one exact CallSlot item with one
borrowed `VerifiedDynamicInvocationEnvelopeRefV1`. It stores only the new
relation between the Recipe item and the exact source call. It does not copy
the selector, target, effect, Home, suspension, Fault, receiver/argument keys,
or result keys already owned by the Recipe and envelope catalog.

Issue-time validation checks the exact caller and source site, receiver,
ordered arguments, result site, result presence, and Dynamic result class.
Unknown, missing, duplicate, foreign, reordered, reused, or extra Loop-call
relations reject before semantic-program publication. Valid envelopes for
other functions remain retained and unselected.

### Full-fixture membership census

The unchanged module-wide envelope catalog contains seven Dynamic calls:

```text
skip_while/4:
  substring/2 + indexOf/1 = 2 Loop-owned calls

scan_until_newline/3:
  substring/2 = 1 Loop-owned call

scan_escape/4:
  substring/2 x 2 = 2 non-Loop calls

scan_escape_piece_and_skip/4:
  substring/2 x 2 = 2 non-Loop calls
```

The first bounded Recipe relation uses the complete catalog and selects the
exact two `skip_while/4` source sites while retaining all seven rows. It does
not claim that all seven are Loop calls. The later module batch golden may
select three Loop calls across two Recipes while retaining four valid
non-Loop rows.

```text
module envelope catalog = 7
skip_while Recipe relations = 2
catalog after co-seal = 7
```

Selection is by exact callable/Loop membership and source site, never by
selector. If that projection is unavailable, the compiler gains the missing
membership authority; the source is not narrowed.

### Fault boundary

`CallSlot { result: Some(dynamic_value) }` defines that value on the Normal
path only. Fault is not:

```text
a Recipe value
a result=None encoding
a LoopExitKindV2 row
a JoinSig edge or payload
a lexical Return/Break/Continue
Completion or DraftSeal Return
```

Fault leaves Recipe lexical control and later binds to one callable-bounded
failure/cleanup transaction. No result or `ch` Home exists on Fault. The
physical preflight remains `NoSafeSlice` until that canonical failure issuer
is designed; no fallback, retry, synthetic Return, or zero/Void result is
allowed.

### `ch` boundary

The normal `substring` result may flow directly as a Recipe SSA value into
the `indexOf` argument. A separate exact relation retains:

```text
CallSlot normal result
-> local ch declaration
-> exact lexical read
-> zero rebind / no escape / same iteration scope
```

Home installation and iteration-scope cleanup are later Home rows. The Recipe
producer must not synthesize a carrier, PHI, `WriteBinding`, After payload, or
`release` for `ch`.

## Revised ordered task ladder

### 1. `LOOP-V2-OPERAND-DEFINITION-GUARD-R0`

BoxShape-only verifier repair before widening the value vocabulary.

```text
implementation:
  one common checked-use helper for all V2 operation operands
  reject use before a verified input/operation definition
  validate Return value-key existence

acceptance:
  existing seven typed-schema tests remain green
  forward receiver/argument/numeric/TextEq/Return use rejects
  no new value class or accepted source shape
  reference and module README updated in the same commit
```

### 2. `LOOP-RECIPE-V2-DYNAMIC-VALUE-I0`

```text
implementation:
  add LoopValueClassV2::Dynamic only
  accept type-consistent V2 Dynamic input/value/read/write/carrier/
    CallSlot normal result/Return structure
  retain exact I64/Text/Bool operation domains
  keep V1 byte- and behavior-unchanged

acceptance:
  Builder-free minimal V2 golden
  Dynamic predicate/domain mismatch/duplicate/undefined rejects
  V1 Dynamic decode/adapter absent
  no source/envelope relation or semantic-program publication
  full envelope regression still reports seven retained rows
```

### 3. `LOOP-V2-DYNAMIC-OPERATION-D0/I0`

Add explicit selector-independent Dynamic numeric operations required by the
source, including Add and Less. Do not overload `BinaryI64`/`CompareI64` and
do not infer a result class from a method name.

### 4. unchanged `skip_while/4` full V2 Recipe producer D0/I0

Produce one complete V2 Recipe for the unchanged source. A producer may issue
private source-to-key claims beside the artifact, but it may not publish a
partial call-only Recipe or a durable intermediate `Verified*` key map.

### 5. `LOOP-V2-DYNAMIC-CALL-RELATION-COSEAL-I0`

Validate the producer's private source-to-key claims and issue the item-keyed
sibling product over the complete verified Recipe and borrowed complete
envelope catalog. The unchanged fixture must prove seven retained, two exact
`skip_while/4` call relations, and five valid unselected rows. Missing,
duplicate, foreign, reordered, wrong-class, resultless, static, or reused
relations reject.

Correction (`LOOP-V2-DYNAMIC-SOURCE-RECIPE-ENVELOPE-COSEAL-D0`, 2026-08-10):
the implementation row is not a two-call partial product. Its sole issuer
consumes the complete full-Recipe candidate and validates all six binding
claims and all twenty-eight source claims atomically. The exact partition is
25 source/Recipe/prelude roles, one deferred `ch` local role, and two deferred
Callable Tail roles; binding roles partition as five source/Recipe/prelude and
one deferred `ch` local. The complete immutable envelope catalog is borrowed,
not consumed. Exact owner plus exact source site selects I6 and I7; selector
text is never authority. The current seven/two/five cardinality is a fixture
golden, not a language constant. The output is a bounded source-bound Recipe
product, not a final semantic program; JoinSigV2, Fault compatibility, `ch`
Home, Tail, and Completion remain later owners.

### 6. Dynamic iteration-local relation and Home

The former combined `LOOP-V2-DYNAMIC-LOCAL-VALUE-D0/I0` is revised. The
existing atomic co-seal already owns V10, the `ch` declaration/BindingRef,
its exact I7 read, and the I6/I7 envelopes. The first executable row is the
behavior-neutral `LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0`: close exact Loop-body scope
and full-use coverage in the existing source issuer, then lend only a neutral
borrow-scoped view from the existing product.

Home remains `NoSafeSlice` until a general destination classifier, JoinSigV2,
Dynamic Fault/callable exit authority, and CFG-complete Home Flow exist. A
self-contained Dynamic carrier is not unconditionally one Home. I6 Fault
installs nothing; any Fault after I6 Normal, inner Return, and normal backedge
must later cross the same lexical cleanup owner exactly once. Synthetic
`release`, runtime-tag semantic classification, and Dynamic-specific Home or
physical ports are forbidden.

### 7. `LOOP-RECIPE-V2-JOINSIG-DYNAMIC-D0/I0`

Open only when an actual Dynamic binding/carrier/After payload is required.
Use a V2 class-bearing JoinSig; do not widen V1.

### 8. control and execution rows

```text
JoinSig-authorized If / Return
-> full semantic-program co-seal
-> callable Fault propagation binding
-> full-program preflight
-> physical canary
-> DraftSeal / collector
-> provider/runtime plan
-> named production cutover and same-edge legacy deletion
```

Every valid source row that exceeds the current compiler boundary opens the
smallest named compiler-widening row or stops at `NoSafeSlice`. No task may
rewrite the source, reduce the fixture, add a selector special case, or use a
legacy fallback to bypass that boundary.

## R0 closeout — V2 operand definition guard

`LOOP-V2-OPERAND-DEFINITION-GUARD-R0` is closed. One common checked-use path
now rejects known-but-not-yet-defined V2 operands for CallSlot receiver/args,
numeric operations, TextEq, WriteBinding, If conditions, and Return values.
Unknown Return keys reject at the Exit item instead of passing structural
verification. Duplicate definitions remain a separate fail-fast error.

```text
focused test:
  cargo test -q loop_recipe_contract::typed_schema_v2_tests --lib
  12 passed

new value classes/source shapes:
  0
```

The row changes no V1 wire, source relation, Recipe operation, JoinSig,
Builder/MIR path, provider/runtime route, retry, fallback, or production
caller. `LOOP-RECIPE-V2-DYNAMIC-VALUE-I0` is the next bounded compiler
acceptance row.

## Dynamic value I0 closeout

`LOOP-RECIPE-V2-DYNAMIC-VALUE-I0` is closed. V2 now serializes and verifies
one exact `LoopValueClassV2::Dynamic` member. The structural golden covers a
Dynamic input, binding, ReadBinding result, carrier entry, CallSlot normal
result, WriteBinding, and Return value with one consistent logical class.

Focused negatives reject Dynamic predicates, Dynamic operands in I64/Text
operation domains, mixed carrier classes, and V1 decoding. The prerequisite
definition guard still rejects forward/undefined/duplicate values.

```text
focused schema tests:
  cargo test -q loop_recipe_contract::typed_schema_v2_tests --lib
  18 passed

unchanged envelope regression:
  cargo test -q dynamic_invocation_contract::tests --lib
  5 passed
  complete module catalog = 7
```

This is still a structural V2 wire receipt. It issues no source-value row,
target/envelope co-seal, Dynamic Add/Less operation, local Home, V2 JoinSig,
Fault edge, Builder/MIR value, provider/runtime plan, retry, fallback, or
production caller.

The next row is the design-only
`LOOP-V2-DYNAMIC-CALL-SOURCE-VALUE-RELATION-D0`. It must decide the minimal
source-issued relation from exact receiver/argument/result sites and roles to
producer-issued Recipe keys without copying target/envelope truth. No relation
code is allowed before that decision is accepted.

## Source-value relation premise audit — `NoSafeSlice`

`LOOP-V2-DYNAMIC-CALL-SOURCE-VALUE-RELATION-D0` found that its proposed I0
would be premature. The source authority is complete, but no full V2 Recipe
producer currently exists: `LoopRecipeArtifactV2` has only schema/verifier
fixtures. The unchanged `skip_while/4` also needs logical Dynamic Add/Less
operations that the V2 algebra cannot yet express.

Issuing Recipe keys for only the two calls would create a partial logical
truth and a second pairing step. Therefore no standalone `Verified*`
source-value product is opened now.

### Existing complete source authority

```text
callable identity:
  declaration catalog + exact catalog/owner link

Loop identity:
  VerifiedCallableLoopMembershipV1
  + frame / scope / region

full skip_while source:
  6 exact binding roles
  + 28 exact statement/expression roles
  + existing two-site Completion

neutral MethodCall source:
  exact call/result site
  + receiver site/binding
  + ordered argument sites
  + checked arity

Dynamic target/envelope:
  exact source-bound target
  + complete module envelope catalog
```

The exact call placement is already source-sealed:

```text
substring:
  Body(1)/LoopBody(0)/Initializer(0)

indexOf:
  Body(1)/LoopBody(1)/IfCondition/Lhs
```

No future issuer may rediscover this membership using selector names or raw
path-prefix matching.

### Final relation shape

After a complete producer exists, source-to-Recipe validation becomes a
private phase of one atomic CallSlot/source/envelope co-seal, not a durable
partial product. The producer supplies unsealed item/receiver/ordered-argument/
result key claims beside the complete Recipe. The canonical issuer validates
them against the source inventory, verified Recipe, and borrowed module-wide
envelope catalog, then retains only the minimal item-to-exact-source-call
relation.

The per-Recipe product must not consume the module-wide catalog or store an
`EnvelopeRef` pointing into itself. It borrows the catalog, so the same seven
rows can serve `skip_while/4` and later `scan_until_newline/3`. Borrow-scoped
views resolve the original target/envelope on demand; selector, target,
effect, Home, Fault, and operand keys are not copied.

### Corrected prerequisite order

```text
LOOP-V2-DYNAMIC-OPERATION-D0/I0
  -> unchanged skip_while full V2 Recipe producer D0/I0
  -> private source-value claim validation
  -> LOOP-V2-DYNAMIC-CALL-RELATION-COSEAL-I0
```

The future co-seal acceptance remains:

```text
complete module envelopes = 7
skip_while selected        = 2
valid retained unselected  = 5
```

All five unselected rows also use `substring`, proving that selection is by
exact callable/Loop membership and source site rather than selector. Missing
or duplicate roles/items, foreign owner/Loop/caller, reused sites/items,
reordered arguments, wrong receiver/result site, resultless/non-Dynamic
CallSlot, and extra relations reject atomically.

`NoSafeSlice` here is a development-order result, not a source disposition.
The source is unchanged; the compiler vocabulary and producer are widened
first.

## Dynamic operation D0 — accepted compiler widening

The unchanged `skip_while/4` source owns four logical arithmetic/comparison
expressions. The earlier P1/S0 migration proof covers only the root condition
and terminal step; it is not complete source or Recipe authority.

```text
LoopCondition       i < end             Dynamic x Dynamic -> Bool
SubstringEndAdd     i + 1               Dynamic x I64     -> Dynamic
InnerIfCondition    indexOf(ch) < 0     Dynamic x I64     -> Bool
StepAdd             i + 1               Dynamic x I64     -> Dynamic
```

The V2 wire adds two exact operation variants. A one-case operator enum or an
`AnyDynamicOperator` family would add abstraction without an admitted second
operation and is not introduced in I0.

```rust
DynamicAdd {
    left,
    right,
    result,
}

DynamicLess {
    left,
    right,
    result,
}
```

Exact I0 domains:

```text
DynamicAdd:
  (Dynamic, I64) -> Dynamic

DynamicLess:
  (Dynamic, Dynamic) -> Bool
  (Dynamic, I64)     -> Bool
```

`ConstI64` remains the sole logical owner of the literal values. The source
contains three distinct literal sites (`1`, `0`, `1`); equal literal values do
not merge their source anchors. No embedded delta field and no
`ConstDynamic` operation is added.

The operation result is the normal-path value only. Canonical language
semantics remain those in `docs/reference/language/types.md`:

```text
DynamicAdd  -> Normal(Dynamic) | Fault(TypeError)
DynamicLess -> Normal(Bool)    | Fault(TypeError)
```

Fault is not `result = None`, `false`, `Void`, an Exit, JoinSig edge, Return,
Completion, or DraftSeal value. The wire does not copy an effect/Fault
envelope into each row; a later private semantics view derives the fixed
operation contract from the verified variant. Optional OperatorBox/provider
routes are not logical authority. A physical route that cannot preserve the
language operator result/Fault contract must reject before Builder effect; it
may not retry or fall back.

### I0 acceptance

```text
positive:
  schema round-trip for DynamicAdd
  schema round-trip for DynamicLess
  exact three admitted domains above
  complete unchanged-source golden:
    2 Add + 2 Less
    2 Dynamic results + 2 Bool results
    3 distinct ConstI64/source anchors
  inner Less consumes the prior Dynamic CallSlot result
  substring Add remains a temporary value
  StepAdd alone feeds the exact WriteBinding/rebind

negative:
  I64 + Dynamic
  Dynamic + Dynamic in the Add I0 cohort
  I64-left Dynamic operation
  Bool/Text/Unit operands
  wrong result class
  Sub, Mul, or non-Less comparison
  BinaryI64/CompareI64 with a Dynamic operand
  undefined or forward operand, including the CallSlot result
  missing/duplicate/reused/swapped source item or literal site
  selector-based indexOf-to-I64 refinement
  treating the old two-row P1 proof as four-row coverage
  V1 decode/adaptation
```

### I0 nonclaims

```text
no full V2 producer
no source/Recipe/envelope co-seal
no local ch Home
no V2 JoinSig, If, Return, or Completion activation
no Builder/MIR/physical operator writer
no provider/runtime plan
no retry/fallback or production caller
no source or fixture narrowing
```

The implementation slice updates this card, the Loop Recipe reference, and
the module README with focused schema tests in the same commit. If the exact
unchanged source exceeds the accepted domains, the compiler vocabulary is
widened through another named Decision; the source is never rewritten or
reduced to fit the verifier.

## Dynamic operation I0 closeout

`LOOP-V2-DYNAMIC-OPERATION-I0` is closed. V2 now serializes and structurally
verifies exact `DynamicAdd` and `DynamicLess` variants with the accepted domain
table. The implementation reuses the common operand-definition guard and
defines only normal-path results.

The dedicated structural golden contains the four unchanged-source roles:

```text
DynamicLess Dynamic x Dynamic -> Bool
DynamicAdd  Dynamic x I64     -> Dynamic
DynamicLess Dynamic x I64     -> Bool
DynamicAdd  Dynamic x I64     -> Dynamic
```

Three separate `ConstI64` rows carry `1`, `0`, and `1`. The first Add remains
a temporary result, the second Add alone feeds `WriteBinding`, and the inner
Less consumes a prior Dynamic CallSlot result. Focused negatives reject
reversed/mixed unsupported domains, wrong result class, and forward use.

```text
focused Dynamic operation tests:
  cargo test -q typed_schema_v2_dynamic_operation_tests --lib
  8 passed

complete V2 schema regression:
  cargo test -q loop_recipe_contract::typed_schema_v2 --lib
  26 passed

unchanged envelope regression:
  cargo test -q dynamic_invocation_contract::tests --lib
  5 passed

line counts:
  schema_v2.rs                                  221
  typed_schema_v2.rs                            672
  typed_schema_v2_tests.rs                      485
  typed_schema_v2_dynamic_operation_tests.rs    300
```

This row changes no V1 wire, source observer/relation, full producer, Home,
JoinSig, Builder/MIR writer, provider/runtime plan, fallback, or production
caller. The next row is design-only: define the one complete unchanged-source
V2 producer and its private source-to-key candidate without publishing a
partial `Verified*` relation.
