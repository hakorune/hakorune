---
Status: local-value D0 accepted with Home split; local-scope R0 next
Date: 2026-08-10
Closed row: `LOOP-V2-DYNAMIC-LOCAL-VALUE-D0`
Current row: `LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0` closed
Parent: `dynamic-dispatch-execution-envelope-d0-task-2026-08-10.md`
Mode: BoxShape first; unchanged-source producer second
---

# Complete Dynamic Loop Recipe

## Decision

The accepted `skip_while/4` source stays unchanged. If a valid source row is
not accepted, widen the compiler boundary or stop at a named design question;
never delete, rewrite, rename, or selector-refine the source to fit the
current verifier.

The current V2 operation vocabulary is sufficient for the complete lexical
Loop. The blocker is structural: `LoopRecipeVerifierV2` does not yet seal the
control/source invariants already owned by V1. Therefore the next executable
row is a behavior-neutral verifier/source-claim parity guard, followed by one
complete producer. No call-only or operation-only producer is permitted.

```text
unchanged resolver-backed full source inventory
        ↓ consume once
deterministic complete V2 Recipe + private source-role claims
        ↓ common V2 structural verifier
DynamicFullLoopRecipeCandidateV2
        ↓ later atomic co-seal
source + Recipe + two borrowed Dynamic envelopes
```

The candidate is not named `Verified*`: its artifact is structurally
verified, but its private source-to-key claims have not yet been co-sealed.

## Exact normalized Recipe

```text
loops      = 1
blocks     = 3
bindings   = 1   // mutable induction i only
inputs     = 4   // src, pos-derived i entry, end, pred_chars
carriers   = 1
values     = 18
items      = 17  // 15 operations + If + Exit
exits      = 1   // inner Return only
CallSlots  = 2
ConstI64   = 3   // distinct source anchors: 1, 0, 1
```

Keys and classes:

```text
L0 = root predicate Loop
K0 = condition
K1 = body
K2 = inner If then block
B0 = induction i : Dynamic
V0 = src : Dynamic
V1 = i entry derived from pos : Dynamic
V2 = end : Dynamic
V3 = pred_chars : Dynamic
C0 = Carrier(L0, B0, Dynamic, entry V1)
```

Canonical recursive item order:

| Item | Block | Logical row | Source role |
| --- | --- | --- | --- |
| I0 | K0 | `ReadBinding(B0) -> V4:Dynamic` | `LoopConditionI` |
| I1 | K0 | `DynamicLess(V4,V2) -> V5:Bool` | `LoopCondition` |
| I2 | K1 | `ReadBinding(B0) -> V6:Dynamic` | `SubstringStartI` |
| I3 | K1 | `ReadBinding(B0) -> V7:Dynamic` | `SubstringEndI` |
| I4 | K1 | `ConstI64(1) -> V8:I64` | `SubstringEndDelta` |
| I5 | K1 | `DynamicAdd(V7,V8) -> V9:Dynamic` | `SubstringEndAdd` |
| I6 | K1 | `CallSlot(V0,[V6,V9]) -> V10:Dynamic` | `SubstringCall` |
| I7 | K1 | `CallSlot(V3,[V10]) -> V11:Dynamic` | `IndexOfCall` |
| I8 | K1 | `ConstI64(0) -> V12:I64` | `InnerIfZero` |
| I9 | K1 | `DynamicLess(V11,V12) -> V13:Bool` | `InnerIfCondition` |
| I10 | K1 | `If(V13, then=K2, else=None)` | `InnerIf` |
| I11 | K2 | `ReadBinding(B0) -> V14:Dynamic` | `InnerReturnI` |
| I12 | K2 | `Exit(E0)` | `InnerReturn` |
| I13 | K1 | `ReadBinding(B0) -> V15:Dynamic` | `StepReadI` |
| I14 | K1 | `ConstI64(1) -> V16:I64` | `StepDelta` |
| I15 | K1 | `DynamicAdd(V15,V16) -> V17:Dynamic` | `StepAdd` |
| I16 | K1 | `WriteBinding(B0,V17)` | `StepTargetI` + `StepAssignment` |

```text
K0.items = [I0, I1]
K1.items = [I2..I10, I13..I16]
K2.items = [I11, I12]
L0.condition = Predicate(K0, V5)
L0.body      = K1
E0           = Return(Some(V14)), owner L0
```

`ch` is an iteration-local source relation, not a Recipe binding or carrier.
V10 flows directly into the `indexOf` CallSlot. A later local-value/Home
co-seal proves `V10 CallSlot result -> ch declaration -> exact ch read -> Home`.

The source `return i` after the Loop is Callable Tail. It is not a second
Recipe Exit. The retained two-site Completion is later partitioned exactly:

```text
inner return -> Recipe E0
outer return -> Callable Tail after Loop After
```

Dynamic TypeError/Fault is not lexical `If`, Exit, Return, Completion, false,
Void, or a Recipe value. The producer describes only the normal-result wire.

## Private candidate boundary

```rust
struct DynamicFullLoopRecipeCandidateV2 {
    source: VerifiedDynamicLoopFullBodySourceInventoryV1,
    artifact: VerifiedLoopRecipeArtifactV2,
    claims: DynamicFullLoopRecipeClaimsV2,
}
```

Rules:

- consume the complete source inventory exactly once;
- retain its Loop membership, six binding rows, twenty-eight source rows, and
  two-site Completion without copying or dropping them;
- claims store only semantic source role to Recipe key/role relations;
- claims store no source site, AST, selector, target, envelope, Home/effect,
  Fault, physical ID, or copied operation payload;
- no public claims accessor, arbitrary/test constructor, `first_call`,
  `take_calls`, filter, or partial product;
- expose the structurally verified artifact read-only for golden tests;
- only the later atomic co-seal may consume the candidate parts;
- borrow the complete seven-row envelope catalog later; never consume it or
  store a self-referential `EnvelopeRef`.

Non-item claims cover the exact siblings omitted from the Recipe item table:

```text
Loop -> L0
LoopConditionEnd -> V2
SubstringReceiverSrc -> V0
IndexOfReceiverPredChars -> V3
IndexOfArgumentCh -> V10
PreludeLocalI + PreludeInitializerPos -> B0/C0/V1
ChLocal + IterationLocalCh -> V10 local-value relation
OuterReturn + OuterReturnI -> Callable Tail candidate
```

## R0 — V2 control/source structural parity

This is BoxShape-only. It adds no operation, value class, source shape,
selector, route, or production caller.

The common V2 verifier must seal:

1. exact root existence and `parent=None`;
2. every non-root Loop has one earlier valid parent and one exact Loop item;
3. condition/body blocks belong to their Loop;
4. If child blocks follow the parent, belong to the same Loop, and are used
   exactly once;
5. every block, item, Loop, and Exit has exact structural use;
6. an Exit belongs to the containing block's Loop and is terminal in that
   block;
7. Break/Continue targets are an ancestor or self; Return has no fake Loop
   target;
8. recursive Loop/Block/Item preorder is canonical;
9. the V2 artifact source binding has exact Loop coverage, canonical unique
   paths, and parent/descendant path structure;
10. a resolver-issued Loop source capability has a consuming V2 source-root
    adapter; callers may not manufacture coordinates.

Path-sensitive branch value availability and control edges remain the later
JoinSigV2 owner. R0 must not create a second control-flow graph or claim that
`Return | Fallthrough` is already JoinSig-authorized.

Required negatives:

```text
invalid root parent
foreign condition/body/If block owner
reused or unused block/item/Loop/Exit
If child before parent
Exit owner mismatch
Exit followed by another item
invalid Break/Continue ancestry
non-canonical recursive preorder
missing/duplicate/foreign source Loop row
duplicate or non-descendant source path
manual V2 source coordinates
```

Implementation shape:

```text
typed_schema_v2.rs
  public verifier orchestration + typed operation domains
typed_schema_v2_structure.rs
  neutral topology/use/preorder/terminal checks
source_binding.rs
  one shared source-path verifier over V1/V2 Loop shape
```

Do not grow `typed_schema_v2.rs` beyond the 760-line split trigger. Every
source file remains below the 800-line hard limit.

### R0 closeout

R0 is landed as a behavior-neutral compiler correction.

```text
typed_schema_v2_structure.rs
  root/parent ownership
  exact block/Loop/Exit use
  If child ownership and order
  terminal Exit
  Break/Continue ancestry
  recursive preorder

source_binding.rs
  one shared V1/V2 Loop-source path verifier

resolved_source_adapter.rs
  consuming VerifiedLoopRootSourceV1 -> V2 root claim
```

The verified V2 artifact now owns a non-Clone structurally verified source
claim rather than carrying the raw DTO. The prior Dynamic-operation test
fixture no longer aliases condition and body to one physical logical block;
it uses the exact two-block structure required by the common contract.

Focused evidence:

```text
cargo test -q loop_recipe_contract::typed_schema_v2_structure_tests --lib
  6 passed
cargo test -q loop_recipe_contract::typed_schema_v2 --lib
  37 passed
cargo test -q resolved_root_adapter_issues_v2_root_without_manual_coordinates --lib
  1 passed

typed_schema_v2.rs                 717 lines
typed_schema_v2_structure.rs       296 lines
typed_schema_v2_structure_tests.rs 221 lines
```

R0 added no operation, value class, source shape, selector, Recipe producer,
JoinSig, physical route, or production caller. Path-sensitive control remains
the later JoinSigV2 owner.

## I0 — complete producer

After R0 is green, implement the deterministic producer under:

```text
src/mir/compiler/dynamic_full_body_recipe/
  README.md
  mod.rs
  mapping.rs
  claims.rs
  tests.rs (split before the line trigger)
```

The neutral `loop_recipe_contract` must not import this profile. Reuse the
existing provenance family; do not add a `skip_while` or method-name producer
identity.

Acceptance:

```text
unchanged skip_while source
exact normalized counts/table above
all 28 source roles and 6 binding roles consumed exactly once
one verified V2 artifact inside one unverified private candidate
inner Return retained in Recipe
outer Return and two-site Completion retained outside Recipe
ch remains a local-value relation, not B1/C1
two exact CallSlots; no selector refinement
three distinct literal keys and source anchors
zero envelope/Builder/MIR/provider/runtime imports
```

If R0 or I0 exposes another valid unsupported source row, stop and widen the
compiler. Fixture narrowing is not an acceptance strategy.

## Ordered tasks after I0

```text
1. atomic source/Recipe/CallSlot/envelope co-seal
2. iteration-local ch value/Home co-seal
3. JoinSigV2 for Dynamic carrier and Return|Fallthrough
4. semantic-program and Dynamic Fault compatibility co-seal
5. multi-exit Completion consumption + Dynamic Callable Tail/return ABI
6. full preflight and fresh-session physical canary
7. one production cutover + same-commit legacy retry/fallback deletion
```

The multi-exit row is mandatory before physical activation. The existing
completion consumer handles only one explicit return; it must later claim the
inner and outer exact site set once each, rejecting missing, duplicate, and
foreign claims.

### I0 closeout

The unchanged production `skip_while/4` source now produces one complete,
structurally verified V2 artifact inside one private unsealed candidate.

```text
1 Loop / 3 blocks / 1 Dynamic binding / 4 Dynamic inputs
1 carrier / 18 values / 17 items / 1 inner Return Exit
2 CallSlots / 2 DynamicAdd / 2 DynamicLess / 3 ConstI64
```

The resolver Loop membership is non-Clone. I0 therefore does not pretend it
can retain that product and also copy its source authority into the artifact.
The producer consumes the membership once:

```text
VerifiedCallableLoopMembershipV1
  -> resolver Loop token -> verified artifact structural path claim
  -> frame + scope/region -> retained source product
```

The retained product also owns all six binding rows, all twenty-eight source
rows, and the original two-site Completion. The private claim table contains
only source semantic roles and Recipe/sibling keys. It contains no source
sites, operation copies, selector targets, envelopes, Home/Fault facts, or
physical IDs. `ch` remains the V10 iteration-local relation; the outer Return
remains Callable Tail.

The source path is bound to the exact already-verified Recipe instance through
one consuming artifact terminal. The producer neither clones the raw Recipe
nor re-verifies and re-pairs a second instance. Source-to-key correspondence
is not claimed by the artifact alone; it remains private candidate truth until
the next atomic co-seal.

Focused evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q dynamic_full_body_recipe --lib
  3 passed
```

No source was rewritten or narrowed. No envelope co-seal, JoinSigV2,
Completion consumption, Builder/MIR, provider route, fallback, or production
activation is present.

## Next design row — atomic co-seal

`LOOP-V2-DYNAMIC-SOURCE-RECIPE-ENVELOPE-COSEAL-D0` must fix one exact issuer
before implementation:

```text
private complete candidate
+ complete seven-row Dynamic envelope catalog (borrowed)
  -> exact two CallSlot target/envelope relations
  -> complete source/Recipe relation coverage
  -> one atomic verified source-bound Recipe product
```

The D0 must validate the entire six-binding/twenty-eight-source claim table,
not only the two CallSlots. It must define ownership, full-coverage
cardinality, foreign/duplicate/missing rejects, `ch` local-value deferral, and
the boundary between normal Dynamic result and Fault. It must not add a
call-only product, consume the envelope catalog, infer selector types, claim a
final semantic program, or activate lowering. JoinSigV2, Fault compatibility,
`ch` Home, Tail, and Completion remain later owners.

### D0 decision — accepted

The sole issuer consumes the complete private candidate and borrows the
complete immutable envelope catalog:

```text
DynamicFullLoopRecipeCandidateV2
+ &VerifiedDynamicInvocationEnvelopeCatalogV1
  -> VerifiedDynamicFullLoopSourceRecipeEnvelopeV2<'env, 'decl>
```

This is a bounded source/Recipe/envelope product, not the final semantic
program. It owns no JoinSig/Continuation, `ch` Home, Tail capability,
Completion consumption, or Fault path.

#### Exact role partition

All claims are consumed. No row is discarded or silently classified:

```text
source roles:
  source/Recipe/prelude relation = 25
  deferred ch local relation     = 1   // ChLocal
  deferred Callable Tail         = 2   // OuterReturn + OuterReturnI
  total                          = 28

binding roles:
  source/Recipe/prelude relation = 5
  deferred ch local relation     = 1   // IterationLocalCh
  total                          = 6
```

`IndexOfArgumentCh` is verified now as Recipe V10 flowing from I6 into I7.
Only `V10 -> ch declaration/read -> Home` remains deferred. Prelude roles are
part of the source-bound Recipe input/carrier relation even though they are
not Loop items. The outer Return rows remain present but do not become a
Recipe Exit.

#### Exact CallSlot relations

Selection uses exact owner plus exact source call site. Selector spelling is
never an admission key.

```text
I6:
  source call/result = SubstringCall
  receiver site/binding/origin = SubstringReceiverSrc / Src
  args[0] site/value = SubstringStartI / result(I2)=V6
  args[1] site/value = SubstringEndAdd / result(I5)=V9
  Recipe = CallSlot(receiver=V0,args=[V6,V9],result=V10:Dynamic)

I7:
  source call/result = IndexOfCall
  receiver site/binding/origin = IndexOfReceiverPredChars / PredChars
  args[0] site/value = IndexOfArgumentCh / V10
  Recipe = CallSlot(receiver=V3,args=[V10],result=V11:Dynamic)
```

Both targets must be distinct and belong to one canonical caller. Their
envelope is the existing indivisible language-wide Dynamic contract. Normal
result values exist only on the normal path; Fault is not converted into a
Recipe value, Exit, Return, false, Void, or Completion.

#### Catalog lifetime and cardinality

The output borrows the catalog; it does not consume, clone, or snapshot it.
It stores the canonical caller and two minimal exact relation keys, not a
self-reference or copied target/envelope. One catalog helper owns exact
`(FunctionOwnerIdV1, SourceExprSiteV1)` lookup and missing/ambiguous rejection.
Derived relation views may borrow the catalog later without reselecting by
name.

The current full-module fixture asserts:

```text
catalog rows = 7
selected     = 2
unselected   = 5 and still valid
```

Seven is evidence for this fixture, not a language-wide acceptance constant.
The durable issuer requires complete catalog integrity and exactly two rows
for this candidate, while permitting unrelated valid catalog rows.

#### I0 file/task shape

```text
src/mir/compiler/dynamic_full_body_recipe/coseal/
  mod.rs       // sole consuming issuer and bounded output
  coverage.rs  // complete 6/28 role validation and partition
  calls.rs     // exact I6/I7 source/Recipe/envelope validation
  tests.rs     // positive, negative, lifetime, API guards

src/mir/dynamic_invocation_contract/catalog.rs
  exact owner+site lookup helper only
```

The claims owner gains one private consuming full-table `into_parts()` path.
It gains no `first`, `select`, `filter`, or partial-product API.

#### Required negative matrix

```text
identity/catalog:
  foreign owner/frame/scope
  missing or ambiguous exact envelope
  Static row or different canonical caller
  one target reused by both CallSlots

coverage:
  missing/duplicate/extra binding or source role
  wrong statement/expression kind
  wrong key domain/class/operation shape
  unauthorized target reuse

calls:
  non-CallSlot item
  resultless or non-Dynamic CallSlot
  wrong receiver/argument key, site, order, or arity
  receiver BindingRef/dynamic-origin mismatch
  wrong producer result for V6/V9/V10/V11

deferred boundaries:
  ch represented as binding/carrier/write/escape
  outer Return represented as Recipe Exit
  inner Return/Exit/Completion site mismatch

API/lifetime:
  arbitrary verified/test constructor
  Clone on candidate/output
  partial claims accessor
  catalog consumption or target/envelope copy
```

Implementation and focused tests must update this card, the module README,
and `docs/reference/mir/loop-recipe-contract.md` in the same commit.

### I0 closeout

The sole consuming issuer is landed. It validates the complete 6/28 claim
tables before binding exact I6/I7 source sites to the borrowed complete
Dynamic envelope catalog. The output retains the complete source, verified
Recipe artifact, verified coverage partition, two minimal call relation keys,
and the catalog borrow. It stores no copied target or envelope.

The implementation removed an overly narrow intermediate check that required
exactly two Dynamic rows for the callable owner. Exact owner+site lookup is
sufficient: the current fixture still proves seven total, two selected, and
five valid unselected, while future additional valid rows do not invalidate
the exact two Recipe relations. Candidate and catalog test evidence now come
from the same canonical semantic-source batch through the existing branded
catalog-callable owner link. Equal-looking source from a foreign resolver
owner rejects.

Focused evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q dynamic_full_body_recipe --lib
  9 passed
RUSTFLAGS='-Awarnings' cargo test -q dynamic_invocation_contract --lib
  5 passed
RUSTFLAGS='-Awarnings' cargo test -q source_call_target::dynamic_member_tests --lib
  5 passed
RUSTFLAGS='-Awarnings' cargo check -q
  green
bash tools/checks/current_state_pointer_guard.sh
  green
```

All touched Rust files remain below 800 lines. No source was rewritten or
narrowed. `ch` Home, JoinSigV2, Fault, Tail/Completion, physical lowering,
provider execution, retry, fallback, and production activation remain absent.

## Next design stop — iteration-local `ch`

`LOOP-V2-DYNAMIC-LOCAL-VALUE-D0` must identify the sole authority that binds
normal I6 result V10 to the exact `ch` declaration/read and the canonical Home
owner. It must define Fault-side non-installation, reassignment/escape/share
rejection, and cleanup ownership before any I0 product is added.

### D0 decision — accepted with split

The source/value relation and Home Flow are not one implementation row.
Existing authority already co-seals I6 normal result V10, the exact `ch`
declaration and BindingRef, its I7 argument read, and the two Dynamic
envelopes. It must not publish a second standalone `VerifiedCh*` product or
reconstruct those relations from names.

Before that relation can be lent to later neutral owners, the resolver-backed
source issuer must additionally close:

```text
ch declaration owner scope = exact Loop-body scope
lexical reads              = exactly the I7 argument read
assignment targets         = zero
capture / return / store   = zero
share / explicit release   = zero
other escape               = zero
```

The next executable row is therefore behavior-neutral
`LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0`. It strengthens the existing source/co-seal
boundary and may expose only a borrow-scoped neutral local-value view from the
existing verified product. It adds no owned semantic product, Recipe key,
Home root, flow state, cleanup plan, or physical port. If the resolver lacks
an exact scope/use-closure query, R0 widens that neutral compiler boundary;
it never infers scope from names or raw ordinals and never narrows the source.

Home installation remains `NoSafeSlice`. A self-contained Dynamic carrier is
not unconditionally one Home: it may contain a trivial payload, an
owner-bearing payload, or a weak payload. Runtime tags choose physical drop
mechanics only and cannot issue semantic Home meaning. The future neutral
Home destination classifier and CFG-complete Home Flow issuer must own:

```text
body entry:
  ch absent

I6 Fault:
  V10 absent
  ch install = 0
  ch cleanup = 0

I6 Normal(V10):
  scope-local self-contained carrier becomes available exactly once

I7 invocation:
  borrowed-noescape use; availability unchanged

I7 or later Fault:
  lexical scope cleanup exactly once, then Fault

inner Return:
  lexical scope cleanup exactly once, then Return

normal fallthrough/backedge:
  lexical scope cleanup exactly once before the next iteration
```

The static `BindingRef(ch)` is not one runtime Home instance across all loop
iterations. Each iteration must prove `Absent -> Available -> Absent`; an
Available carrier may not cross the backedge. Cleanup timing belongs to the
exact lexical Loop-body exit, not last-use optimization. Home Flow issues the
release obligation, the common exit transaction orders cleanup, and C-prime
DropPlan owns terminal hook/field/native teardown.

Corrected order:

```text
1. LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0
2. LOOP-RECIPE-V2-JOINSIG-DYNAMIC-D0/I0
3. Dynamic Fault / callable exit-transaction authority
4. HOME-LOCAL-SELF-CONTAINED-CARRIER-D0/I0
5. final V2 semantic-program co-seal
6. common physicalization and production cutover
```

Hard stops remain: no synthetic `release`, no `ch` Recipe
binding/carrier/PHI/WriteBinding, no runtime-tag Home inference, no
profile-specific Home or physical port, no Tail/Completion, provider route,
retry, fallback, or production activation.

## Nonclaims

```text
no source rewrite or fixture narrowing
no selector-specific Text/I64 refinement
no standalone Verified source-to-key product
no second source/envelope co-seal or standalone local product
no Home, effect, Fault, JoinSigV2, continuation, Tail ABI, or Completion use
no Builder / MIR / CFG / PHI
no provider/runtime plan or invocation
no retry/fallback
no physical or production activation
```

## R0 closeout — exact iteration-local source closure

`LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0` is closed. The existing source issuer now
checks, from resolver-owned inventories only:

```text
BindingRef(ch).owner_scope = exact Loop-body scope
lexical reads of ch         = [IndexOfArgumentCh]
binding rebinds of ch       = 0
nested capture demands      = 0
```

The existing atomic source/Recipe/envelope product now retains one private
local relation and lends `DynamicIterationLocalValueRefV2<'_>`. The view
borrows the exact declaration and read rows and carries only the already
verified V10 producer I6 and consumer I7 keys. It is not independently
constructible and creates no second source, Recipe, or lifetime authority.

Evidence:

```text
cargo check --lib
  green
cargo test --lib dynamic_full_body_source_tests
  6 passed
cargo test --lib dynamic_full_body_recipe::coseal::tests
  5 passed
all touched Rust files
  below 800 lines
```

No source/fixture was rewritten or narrowed. Home installation, cleanup,
JoinSigV2, Fault, Tail/Completion consumption, Builder/MIR/CFG/PHI, provider
execution, retry, fallback, and production activation remain absent.

The next row is the independent design stop
`LOOP-RECIPE-V2-JOINSIG-DYNAMIC-D0`; it must be reviewed before implementation.
