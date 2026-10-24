# Ownership and Home Flow (SSOT)

Status: Language semantics SSOT; Home direction and bounded take/share/release
target accepted; `release root` parser/source carriage live, Home semantics 0

Decision: accepted on 2026-08-04 as the successor to the earlier
`move/share/view` target; C′ terminal Home finalization and contextual
whole-root `release root` amendments accepted on 2026-08-05; bounded
take/share contextual syntax accepted on 2026-08-09.

This page is the source-language authority for ownership, ordinary aliases,
Home transfer, and the explicit boundary that adds an independent owner.

Successor design direction (2026-10-23):
[ADR-002](../../development/adr/adr-002-explicit-ownership-and-compiler-migration.md)
records explicit `move`/`copy`/`share` and `read`/`mut`/`take` as a proposed
replacement surface. Its implementation is unselected; until an explicit
cutover, the existing source rules below remain in force. The separately
accepted Array carrier destination amendment below applies now as a target.

Related authorities:

- `variables-and-scope.md`: lexical bindings and nearest-binding assignment;
- `scope-exit-semantics.md`: lexical cleanup and exit ordering;
- `lifecycle.md`: non-callable `fini {}` hook, terminal Home DropPlan, weak
  references, and reclamation;
- `dynamic-invocation.md`: the uniform borrowed-input and self-contained
  result-carrier boundary for ordinary Dynamic invocation;
- `../../development/current/main/design/ownership-home-model-ssot.md`:
  cross-layer compiler authority;
- `../../development/current/main/design/box-member-field-method-surface-ssot.md`:
  field/method source prerequisite and mandatory post-cutover reference
  closeout;
- `../../development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md`:
  parked decision and implementation order.

The old sparse ownership, Anchored View, and grammar inventory cards remain
historical evidence only. They do not restore `move`, source `view`, source
`owned/shared` modes, or a second production ownership authority.

## Availability

| Layer | Current state |
| --- | --- |
| Home direction and durable laws | accepted by this page |
| C′ last-Home finalization direction | accepted target; production 0 |
| explicit `release root` direction | parser/source carrier live; Home semantics 0 |
| generic/composite release | provisional; exact capability D0 open |
| bounded HomeV1 source grammar | `release root` live; `take`/`share` inactive |
| Box `obj.x` place / `obj.x()` call prerequisite | accepted target; Property production retirement parked |
| Rust/Hako parser and AST Home carriers | inactive / absent |
| resolver Home-demand/root and Home Flow | inactive / absent |
| callable Home ABI | bounded declaration-only receipt exists; source `take` linkage absent |
| canonical Builder/source producers | 0 |
| passive Ownership SSA / ownership opcodes | existing narrow infrastructure only |
| current ordinary Box assignment | transitional SharedV1 behavior |

The live `EBNF.md` and grammar registry remain syntax authority. Only
`release root` currently has a Rust/Hako parser and parser-owned source row;
the other examples remain targets. Parser acceptance does not issue Home
meaning. Unsupported lookalikes fail fast until their exact shared grammar row
lands.

## 1. Thirty-second rule

Every independent Box lifetime is represented by a **Home token** installed
in a **Home slot/place**.

```hako
local node = new Node()  // node receives a new Home
local alias = node       // alias only sees node's object; no new owner

inspect(node)            // ordinary handle input; node keeps its Home
adopt(node)              // if adopt declares a Home demand, node is consumed
adopt(share node)        // when the ABI admits a Shared Home; node remains
release node             // end node's verified whole-root Home now
```

The user-facing law is:

```text
ordinary use:
  handle; owner count does not change

destination that declares a Home demand:
  one available Home moves there

share:
  one independent owner is added; this may cost runtime bookkeeping

release:
  one available whole-root Home is consumed now; fini runs only if terminal
```

`Home` is an explanation and compiler product, not a planned source keyword.
Hakorune does not expose Rust lifetimes, `&`, exclusive borrow types, or
general ownership type modes in ordinary code.

## 2. Home, Handle, and value capability

### Home

A Home slot/place can hold one Home token supporting a Box lifetime. Examples
may include an owning local, field, container slot, global, registry slot,
parameter destination, or return destination, but each family needs an exact
verified destination contract.

One object identity may have multiple Home tokens in the Shared lane. Object
identity, Home token, Home slot, source handle, and runtime `ObjectHandle` are
different concepts.

An owning destination consumes at most one available Home. A Home that was
consumed cannot be used again unless a later exact operation installs a fresh
Home in that place.

### Handle

An ordinary read of a Box binding produces a non-owning handle to the object
supported by its Home.

```hako
local a = new Node()
local b = a

b.value = 10
print(a.value) // 10
```

`b` does not add an owner or perform RC bookkeeping. Handles are mutable and
non-exclusive; the optimizer must assume that `a` and `b` can alias. A handle
cannot escape beyond the Home that supports it. The selected constructor-prefix
implementation keeps owning local state separate from ordinary expression
observations: both direct Home reads and alias-chain reads yield only handles.
This structural separation does not activate Map transfer or general Home Flow.
For the selected constructor-root terminal, Trivial capability alone does not
prove an integer Add operand. Bool and locals without retained integer-kind
proof are unsupported there; rejection retains no partial terminal field-read
product. This is a compiler coverage boundary, not a general ban on integer
local arithmetic in the language.

Dynamic invocation does not change this rule based on a runtime tag. Its
receiver and arguments are borrowed without escape for the exact invocation,
and a normal result is one self-contained Dynamic carrier published to the
caller. The selector-independent contract and failure boundary are defined in
`dynamic-invocation.md`.

Dynamic operators have a separate envelope. `DynamicAdd` borrows both
operands without escape and publishes a self-contained non-aliasing carrier
only on Normal; `DynamicLess` publishes a trivial Bool. The shared carrier
lifecycle vocabulary is not Home. See `dynamic-operators.md`.

### Value capability is separate

Home is not a value type and not a runtime handle. The compiler separately
classifies a resolved value as:

```text
Trivial
Unique
Shared
Weak
Unknown
```

This recursive classification is still a D0 for records, enums,
`Option<T>`, `Result<T, E>`, containers, generic `T`, `Any`, and cycles.
Unknown never defaults to Trivial, Unique, or Shared.

An identity-free record may still carry an owner-bearing Box payload. “All
records are Trivial” is not a valid rule.

### Intrinsic numeric Array literal acquisition

Decision: numeric literal acquisition follows the existing Home law below.
Every source element participates in order; an empty literal has exactly zero
source elements. Missing compiler child relations never prove an empty literal
or an empty set of evaluation responsibilities. Root cleanup also includes
completed acquisitions after the last acquisition's own failure prefix. A
prefix proof cannot substitute for the whole root's normal completion and
outward transfer; the source body scope and function return target are distinct.
Completed source obligations must survive lowering-scope teardown and reach
both diagnostic and artifact finishing. Retaining them does not itself prove
that optimized physical operations implement those obligations. Physical lowering
must retain the exact allocation, claim, element write, Local commit and Return
bindings alongside the source obligation. Finishing validates those identities
and their order after supported optimization; it must not reconstruct missing
source relations from the resulting instructions.
The final artifact product retains this completed source and binding information;
its backend view borrows that product. Array-unissued source is distinct from a
completed empty Array literal. Retention alone grants neither constructor
semantics nor permission to execute an unsupported Array backend path.

For the supported primitive numeric `Array<T>` contract, intrinsic literal
allocation creates a builtin acquisition responsibility. From successful
allocation until Local initialization commits, its evaluation frame owns the
incomplete Array residence. Successful Local commit transfers one Home to the
exact destination; an ordinary alias remains a borrowed handle and adds no Home.
The incomplete residence responsibility and the committed local Home are distinct,
even if a backend eventually uses the same native release primitive for both.

A successfully checked numeric element write stores a primitive scalar and
creates no element Home. This does not classify an arbitrary child expression,
container, record, string or runtime handle as Trivial. Child evaluation must have
its own exact source capability and transfer contract; a failed write has not
transferred its candidate value into an owning Array slot.

The intrinsic numeric Array has an explicit builtin native structural-destruction
contract: it releases its own native residence and stored scalar representation,
and invokes no user `birth` or `fini`. This follows intrinsic literal identity and
the numeric element contract, not absence of methods in a user box or the current
runtime Arc/handle implementation. It does not decide generic container
Unique/Shared capability, `share` syntax activation or provider-owned Array policy.

On Fault, discharge the active child evaluation frame, then the incomplete Array
residence, then the caller's already-established lexical/local obligations under
`scope-exit-semantics.md`. Preserve prior observable effects and the first Fault;
do not execute later elements or roll back completed external effects. After
Local commit, its Home instead belongs to the normal scope-exit release set.
Failed native wrapping/registration retains acquisition responsibility until
cleanup; an untracked native release contract cannot be admitted. The selected
native Array distinction between returned Fault and fatal allocator termination
is fixed by the [runtime failure policy](../runtime/runtime-data-dispatch.md#selected-native-array-failure-policy);
the cleanup law does not imply universal host-OOM recovery.

This is the source target contract. The compiler's finite source issuance and
runtime activation are tracked in the
[construction workstream design](../../development/current/main/design/collection-literal-construction-ssot.md).

### Named Array append-only object slot destination

Decision (2026-10-13): an admitted builtin named Array provider may expose an
owning object slot destination for ordered `push` and a borrowed nullable `get`.
This bounded provider contract applies the existing Home law; it does not activate
general container capability, `share`, or current compiler/backend coverage.
The exact empty builtin construction and holder binding identify the provider.
They do not prove its object-slot transfer, read or destruction capabilities.
Those capabilities and each child's canonical End contract must be verified.
User declarations shadowing the builtin supply no authority for this contract.

Each original Push retains its receiver, argument evaluation, result producer
and source order. A non-null fresh child result must provide exactly one available
obligation through its original acquisition/result contract; a nullable class
claim alone cannot provide that obligation. Exact null carries no child Home.
Successful append commit transfers the available obligation into its slot once.
An ordinary borrowed candidate cannot be inserted into this owning destination
without a separately authorized acquisition; insertion never performs share.

Before commit, child evaluation or append Fault transfers nothing: discharge the
actual active child responsibilities, then established caller responsibilities
under the existing scope rules. Already committed slots remain the holder's
responsibility. A later Fault cannot restore a committed child to evaluation.
Preserve the first Fault and completed observable effects.

Get evaluates its original receiver and admitted Integer index once and lends a
non-owning nullable handle supported by the live holder and committed slot state
at that read. Missing/null remains nullable; reading creates no Home or retain.
All original contributing stores must agree on the non-null source class, using
their exact producer contracts. A future append cannot prove that an earlier
read's slot exists or stays live. A callee's formal annotation, runtime handle
bits, or boxed clone behavior cannot issue the class or loan. Keep the supporting
holder live through the loan's last use; unsupported escape or mutation rejects.

For this append-only provider, terminal End discharges live child obligations in
reverse successful-install order, then releases the Array residence. Null slots
have no child End. This order is an explicit provider contract, not an inferred
general Array order. Detach/state transitions occur under storage synchronization;
child End runs outside its locks, and terminal End cannot revive ownership.
Preserve the first Fault and attempt remaining child and residence cleanup.
Native residence release alone does not prove child obligation discharge.
An ordinary method called with a borrowed element changes no slot ownership
unless a separate verified contract explicitly authorizes that change.

The selected family requires the sole unrebound, unaliased, nonescaping local
holder, its complete original use inventory, admitted ordered Push/Get reads,
and exact allocation/commit/read/End bindings through final publication.
Unknown child capability, mixed classes, unavailable producer or cleanup contract,
and unsupported Set/Insert/removal/clear or holder escape remain rejected.
Source class preparation is distinct from execution permission: provider
capability, full call/input/result Completion, cleanup and independent physical
verification remain mandatory before the family can execute.

### Named Array opaque carrier destination target

Decision (2026-10-23): an admitted builtin named Array append destination may
receive an already source-authorized `SelfContainedDynamicCarrier` once.
Successful append commit forwards its existing exactly-once obligation into
the slot; this creates neither a source Home nor a second carrier. The exact
source producer, receiver, argument evaluation and append order remain retained.
The same carrier cannot be committed to a second slot without a separately
authorized copy/share operation. This amendment grants no new source syntax.

Before successful commit, evaluation/append Fault transfers nothing and the
candidate remains its prior owner's cleanup responsibility. After commit, only
the holder owes that carrier's disposition; later Fault cannot restore it to
the caller. Source-verified holder transfer/Return carries the installed obligations with it.
Terminal Array End drains committed carriers in reverse successful-install
order, then releases the residence, preserving the first Fault while attempting
remaining cleanup. Detachment occurs under storage synchronization; carrier
End runs outside storage locks. Payload tags select physical mechanics only.

This target is independent of Text-copy append and homogeneous object-slot
Home transfer. Neither `checked_text_end_v1` nor an object End descriptor can
discharge arbitrary carrier obligations. General `get`, `remove`, `replace`,
mixing carrier slots with Text/object-slot profiles, and independent read-result acquisition are not
activated by this append decision. Reads need a separately verified support
relation; an ordinary Dynamic invocation must still return a self-contained
carrier, never a borrowed payload disguised as one. No implicit retain/copy,
selector-based Text refinement, or backend ownership inference is authorized.
Compiler/runtime source-to-publication and Normal/Fault cleanup remain unimplemented.

### Intrinsic Map slot destination target

Decision (2026-09-09): an intrinsic Map literal's value slot is an owning
destination. This is an accepted source target, not current producer/backend
coverage or activation of general Home/share syntax. The literal expression
and its ordered entry source sites select the destination; runtime payload
tags and current MapBox clone behavior do not select ownership meaning.

| Source-authorized candidate | Successful slot installation |
| --- | --- |
| Trivial value | Store the value; create no child Home. |
| Available Home compatible with the destination | Transfer that Home once; remove the old cleanup responsibility. |
| Existing admitted Shared Home | Transfer that Home once; do not acquire another owner. |
| Explicit admitted share result | Transfer its separately issued Home; insertion itself never performs share. |
| SelfContainedDynamicCarrier | Forward its existing exactly-once carrier obligation into the slot; do not reclassify it as a source Home. |
| Ordinary borrowed handle | Not an owning-slot input without a separately authorized acquisition/transfer. |
| Unknown capability or unavailable source relation | Unsupported; no default Trivial, Shared or carrier classification. |

The selected compiler retains Value versus TransferHome in the source Map flow.
Its bounded Value evidence covers Integer/Bool literals, observed Trivial locals
and declaration-authorized exact-i64 formals. Reusing a Trivial value is not a
second Home transfer. Literal values and literal-derived local scalar kinds are
retained separately from capability. The compiler now also preserves the exact
I64 declaration contract through parameter and alias source flow; a coarse
Trivial capability alone does not supply its representation. An unsupported intervening statement
cannot preserve a usable stale local kind. A borrowed formal may remain unused without becoming an
owning-slot candidate. A self-rooted parameter handle may instead be
stored as a non-owning borrowed entry (`BorrowedHandle` payload): the
map never owns the referenced target, the enclosing owner keeps it
alive, and a scalar read Faults non-scalar rather than exposing the
handle bits as an integer. AppMain consumes exact Integer/Bool literal and known
local representations through InstallValue, retaining Home-only InstallIndexed.
Unknown scalar representation remains rejected before installation. Ordinary
callable source Completion stays separate from root admission and does not grant
its still-unconnected cleanup/Fault ABI. Source success cannot silently select
the generic MapBox path.

This matrix preserves the existing source capability and carrier authorities.
A fresh acquisition can supply its existing obligation; an alias cannot supply
the Home of its supporting root by guessing provenance. This decision does not
implement the missing composite capability, explicit share or Map source issuer.
It also does not establish a second borrowed-slot Map family.

For one entry, preserve allocation/key/child evaluation order from
`block-expressions-and-map-literals.md`. Before successful installation the
candidate obligation remains with its existing owner and the old slot remains
unchanged. A direct owning local keeps its Home until commit; a fresh child
acquisition is owned by child evaluation only after its Normal result. Failure
before acquisition creates no child obligation. Preparation/validation/write
failure before commit transfers nothing; cleanup follows the actual prior owner,
not a common candidate cleanup list.
Successful installation changes the owner exactly once. For an equal key, install
the new slot before ending the detached old obligation, as already required by
`lifecycle.md`. Subsequent cleanup Fault cannot undo installation or restore the
candidate to the caller. Preserve first Fault and attempt remaining cleanup.

An owning-slot candidate may consume the exact referenced owning binding; an
ordinary alias cannot supply its supporting root's Home. Candidate observation
is nonconsuming. Successful installation must use the same still-available
obligation in the source successor; observing it earlier is not permanent transfer
permission. This is a static source-flow requirement, not runtime name lookup.

The source owner may retain initial cleanup order plus exact transfer and
replacement points instead of full per-entry successor lists. Projecting a
prefix must preserve the same pre-commit/post-commit distinction and reverse
live-install order; it must not re-resolve keys or infer ownership in the backend.
This representation choice grants no additional source acceptance.

Each duplicate-key entry is a separate transaction. Using the same direct Home
in two entries consumes it at the first successful installation and rejects the
second use, even when the keys are equal. Two separately authorized fresh child
acquisitions instead produce distinct obligations: the second installation owns
the new value before the first value's detached obligation is ended. None of
these rules activates an otherwise unsupported child acquisition or source shape.

The semantic sequence distinguishes pre-commit write failure from post-commit
old-value cleanup failure. A single undifferentiated failure implying
“argument unconsumed” cannot represent both. Recipe/cleanup planning must carry
the committed owner state into the latter Fault path; a status number alone
cannot issue it.

Existing Map key equality, duplicate-key replacement and iteration semantics
remain unchanged. MapBox deletion keeps its existing Bool result: detach the
entry and discharge its obligation rather than introducing an ownership-returning
remove API. OrderedMapBox has a separate API and supplies no authority here.
Ending a detached obligation is scheduled outside storage locks by the physical
cleanup owner; arbitrary host clone/Drop or last-thread destruction is not the
source finalization policy.

A source-authorized exact slot read may borrow only while its support remains
valid. Such a read is not automatically admitted, and cannot be used beyond
replacement/delete/Map end without a separate lifetime proof. It must not become
a borrowed Dynamic result: `dynamic-invocation.md` requires a self-contained
normal carrier. Independent owner acquisition on a read likewise needs an
existing explicit authority, not an implicit retain.

Still open within the full Map cutover: source capability/destination issuance,
Home/carrier commit and cleanup co-seal, implementation of the accepted
[Map construction/end law](lifecycle.md#intrinsic-map-construction-and-end),
general clear semantics, slot-borrow invalidation, Dynamic read publication,
self-reference/cycles, exact runtime profile and finalizer affinity. No runtime
escape refusal is retired merely by accepting this destination target.
The next design is the single-entry source/commit/cleanup mapping, including
duplicate keys, under existing `OWN-FIELD-CONTAINER-DEST-D0`.

Implementation boundary: the shared resolver now preserves Map keys and ordered
value-child source relations in its existing body-shape product, including empty
and duplicate-key Maps. This is source structure, not an issued slot transfer.
Complete Script MethodCalls nested under Map entries remain unsupported before
effects for both direct and bound receivers. The diagnostic is now explicit
`UnsupportedMapEntryCall`; a valid structure parent is no longer reported missing.


## 3. Accepted HomeV1 syntax target; production 0

The durable semantics and bounded contextual spellings are accepted. They are
not evidence that either parser currently accepts the forms.

### Declaration-side Home demand

Canonical target:

```hako
adopt(take node: Node) {
    // the body may consume node only through a separately verified destination
}
```

`take` is a declaration-head contextual modifier with an explicit type. It is
not globally reserved and does not cross a line terminator:

```text
take node: Node  = Home-demand parameter target
take: Node       = ordinary parameter named take
take(node)       = ordinary call
obj.take()       = ordinary method
```

Call-site `take`, `return take node`, `take place_expr`, and consuming receiver
syntax are outside HomeV1. The declaration handoff owns the typed syntax row;
a body observer never rediscovers it from parameter text.

Here `take` describes the destination contract. It does not instruct the
callee to guess from runtime state. The call site is ordinary:

```hako
adopt(node)
```

If `node` owns an available compatible Home, the sealed callable ABI consumes
it. If it is only a handle alias, compilation rejects the call and identifies
the owner root.

The destination contract is the transfer SSOT. The call site may later gain
an opt-in explicit-transfer lint, but HomeV1 does not require duplicate
`take` spelling at both declaration and call.

### Ordinary handle parameter

```hako
inspect(node: Node) {
    print(node.value)
}
```

An ordinary parameter is a noescape handle. It cannot store, return, capture,
or otherwise outlive its supporting Home unless another exact contract
permits that boundary.

Passing `share node` to a handle-only parameter is rejected as a redundant
paid owner; use `inspect(node)`.

### Result relation

A result preserves the Home relation of the returned expression:

```hako
make(): Node {
    return new Node()       // fresh Home to caller
}

getRoot(): Node {
    return me.root          // handle supported by receiver Home
}

getIndependent(): Node {
    return share me.root    // independent owner, when share is verified
}
```

ClosedCallable bodies may infer a candidate result relation and verify it
locally. A ContractBoundary must state the exact relation. The candidate
borrowed result spelling is:

```hako
getRoot(): Node from me
```

`from` is not parser-live. Exact anchors, generics, multiple returns, and
boundary syntax remain D0 work. The provisional boundary rule is bare `: T`
for `HomeToCaller` (or Trivial after recursive classification) and
`: T from anchor` for an anchored handle. A bodyless Shared result spelling is
still unresolved and must be selected by the representation/surface D0; it is
not inferred from a method name or implementation body.

Returning an available local Home is terminal forwarding; it does not require
`return take local`. Returning a handle whose Home dies at function exit is a
compile error.

### ClosedCallable composite anchored results

Decision (2026-10-07): a locally verified composite result may own fresh outer
storage while a child field has the exact relation `Null`, `OwnedFresh`, or
`BorrowedFromFormal(ordinal)`. This is an accepted target contract; passive
result-origin Facts alone do not activate field storage, transfer or cleanup.

| Verified child relation | Home effect |
| --- | --- |
| Null | no child Home or supporting anchor |
| OwnedFresh | transfer an already verified acquired child Home into the outer |
| BorrowedFromFormal | retain a non-owning child handle supported by the original caller Home |

The borrowed arm never consumes an ordinary input Home, creates another Home,
or changes an ordinary parameter's declaration-only Handle demand. A caller
may pass an existing field handle when its original support is verified; the
result relation does not turn that read into field move-out. This permits the
source to inspect the original object while the dependent result is alive.

The exact construction/field destination issuer must verify the child relation
at the source constructor and store sites. A class annotation, object identity,
origin union, or pointer equality cannot select it. Multiple source paths with
the same origin retain distinct witnesses. Fresh-construction provenance is
not proof that its acquisition, transfer and cleanup have completed.

The outer is complete before ordinary publication: every acquired child has
an owned residence, every borrowed child has a verified supporting anchor,
and each residence has its exact Normal/Fault disposition. There is no pending
ownership hole completed by an ordinary result assignment. Unknown residence
or support fails before publication. The existing constructor field contract
must explicitly admit this anchored relation; an owning-only destination
cannot silently accept a borrowed field.

The destination contract is an exact per-construction refinement of the
canonical constructor plan, keyed by the branded returned-New site and
canonical field. Its issuer borrows the same constructor declaration and
sealed store plan. The bounded refinement requires a locally verified
ClosedCallable returned construction without field overrides, a unique
nonweak typed user-object field store from the original Birth parameter,
and exact constructor, parameter ordinal, ObjectFieldStores use and child
class correspondence. This rule is generic; neither a class name nor an
ordinary input's missing annotation grants the permission.

The refinement explicitly permits the three relations above, subject to a
separate verification of every actual outcome's acquisition or live support.
It changes neither the class-wide owning-field inventory nor the ordinary
parameter Home demand. A base owning-only destination remains owning-only;
a forward-origin candidate cannot convert it into a borrowed residence.
Destination permission alone cannot install a field or publish a result.

A fresh child returned through calls retains the exact selected callee,
source exit and original acquisition through each direct-return or local-return
edge. The source lifecycle verifier must corroborate the leaf's exact
result-position construction claim or returned Home, its completion, and each
transfer/discharge obligation. An unrelated construction of the same class,
a nullable class observation, or an I64 call-return relation is insufficient.
Physical emission subsequently discharges that same sealed plan; successful
emission is not a new source ownership authority.

A dependent result, its child reads and parent finalization may not outlive its
supporting Home. An anchor release is rejected when a subsequent dependent use
or finalization requires that support. A result rooted in a dying local or
unverified temporary cannot escape. ClosedCallable relays may inspect verified
scalar fields, perform separately verified effects, and return the same relation;
those effects add no ownership. Fault during the relay cleans the outer and
acquired children, preserving the borrowed anchor. Exported, dynamic or opaque
boundaries require an exact declared/imported relation and cannot infer it.

Null, fresh and borrowed alternatives remain distinguished by verified source
outcome/completion evidence. An ordinary result alias adds no Home. Borrowed
children receive no owning release on parent teardown, ignored result cleanup
or constructor Fault. This adds no `take`/`share` syntax or implicit RC.

### Independent owner

`share` is the only ordinary source operation that may add another owner for
the same identity:

```hako
registry.adopt(share service) // only when its ABI admits the Shared result
use(service)
```

The exact physical operation depends on a verified representation plan. It
may be RC/control-cell work, another Shared mechanism, or a rejected route.
No runtime tag or observed reference count selects it.

Contextual ownership `share` is a prefix over one non-group postfix operand.
Postfix binds inside the request and infix operators bind outside it:

```text
share obj.field() = Share(MethodCall(obj, field))
share obj + y     = Add(Share(obj), y)
```

`adopt(share node)` is ordinary call composition, not a second ownership
grammar. `share(...)` and `share (expr)` are permanently ordinary function
calls; a future grouped ownership form must choose another spelling. The
contextual head does not cross a line terminator. `share`, `obj.share()`, and
bindings named `share` remain ordinary outside the exact prefix position.

### Explicit early Home release

`release root` is the sole accepted early-end spelling. It is a statement-only
contextual keyword with one identifier root and a dedicated source carrier;
resolution and a sealed Home Flow plan own its meaning. It is not a generic
wrapper, method-name convention, direct `fini` call, or backend name match.
Ordinary `release(value)` and `obj.release()` remain ordinary calls. `drop`
and `unbox` forms are not aliases. The lexer does not globally reserve
`release`; declarations/bindings named `release` and `Build.release` remain
ordinary syntax. Contextual recognition requires the root identifier on the
same line; tokens separated by a line terminator are left to ordinary grammar
and do not request a Home release.

The Rust/Hako parser and descriptive roundtrip row are live. Their sole output
is syntax/source identity: exact lexical root, parser provenance, method source
site, and direct body ordinal. `BindingRef`, owning-root capability, Home
availability, terminality, DropPlan, and execution remain later authorities.
Nested Release is outside this first source cohort and fails rather than being
silently omitted.

The first profile accepts only a verified whole-root owning local or owning
parameter containing exactly one available Home. Release consumes that root at
the source point. Every handle supported by the released root becomes
unavailable, even when a different Shared Home keeps the identity alive; Home
Flow never silently re-roots a handle.

The parsed carrier, resolved root, and `VerifiedExplicitHomeReleasePlanV1`
must together show synchronous release at the source position. Owner-bearing
composites, fields, projections, containers, `me`, trivial roots, and unknown
capability remain provisional or rejected until their separate decisions
close. Generic classification does not turn this statement into a generic
function.

## 4. Syntax deliberately outside HomeV1

The first program does not promise:

- `take place_expr`;
- `move expr`;
- `owned T`, `view T`, or `shared T` in ordinary signatures;
- parameter/receiver ownership modes from the former design;
- field move-out or a moved-field empty-slot representation;
- consuming receiver syntax;
- multi-anchor result joins or borrowed-result PHIs;
- field/index/projection handles stored in locals;
- handle capture, suspension, task/channel, or cross-thread flow;
- explicit source `region`;
- `drop root` / `drop(value)` as a compatibility alias;

`take place_expr`, field take, and consuming receiver are parked together
until a real source consumer and storage replacement contract exist. They are
not forbidden forever.

An optional explicit allocation/lifetime region may be designed after a real
arena allocation/free substrate exists. Region is not required for Home
correctness and does not replace Home/share laws.

## 5. Callable Home ABI

Every production call consumes one sealed ABI; no call site reopens a body.
The conceptual product contains:

```text
receiver demand:
  Handle | Home | SharedHome | None

parameter contract:
  Handle | Home | SharedHome | Trivial

result relation:
  Unit | Trivial | HomeToCaller | FromReceiver |
  FromParameter(index) | SharedHomeToCaller
```

The passive relation vocabulary for these demands/results is landed in the
resolver as an internal, branded receipt, and the bounded `I64|Unit` Home ABI
catalog is now issued through one same-declaration co-seal. The common
direct-method parameter contract consumes one complete parser source catalog
and canonical resolved bindings: absent ordinary spelling becomes
`OpaqueHandle`, explicit `i64` becomes `ExactTrivial(I64)`, and unsupported
explicit/non-ordinary transfer rejects. It preserves zero-parameter
declarations and exposes no split/name-lookup API. `HomeDemand` is only a
one-way projection from this contract. The relation brand is batch provenance
only, not resolver nominal identity. This does not make `take`, Home grammar,
or production ABI live: Query/body conformance, Home Flow, physical ownership,
targets, and production remain separate gates.

For the verified ordinary instance cohort, the same issuer lends an
entry-only projection: batch-slot-bound `VerifiedInstanceEntryHomeLoanV1`
rows carry the exact `Receiver` binding and each parameter's projected
`home_demand()` into source Home Flow, which installs them only for the exact
matching declaration. The loan carries no result relation and publishes no
complete call-site Home ABI. Missing capability, capture demands,
static/top-level declarations, and foreign or incomplete rows all keep the
named entry unavailability rather than receiving empty obligations.

### ClosedCallable

A callable whose body and all relevant resolved facts are locally available
may derive a candidate **result relation** and local Home Flow from its body.
Parameter and receiver Home demands come only from the resolved declaration:
an absent ordinary parameter is Handle through the contract projection, while
an explicit accepted trivial contract is Trivial. Body analysis verifies that
contract and may not invent an invisible consuming parameter. The verifier
seals the combined ABI once. Private is a common ClosedCallable case, but
visibility alone is not the classifier.

### ContractBoundary

The exact ABI must be declared or imported for:

- exported or separately compiled callable;
- interface/dynamic dispatch;
- callback or function value crossing an opaque edge;
- plugin/FFI/extern function;
- unresolved generic callable;
- any unavailable body.

An exported callable with a body still has an explicit boundary contract, so
body edits cannot silently change its public Home ABI. Compiled metadata may
carry schema/profile/source dependency hashes for integrity; a user-managed
lock file is not semantic authority.

Unknown ABI fails before Builder effects. It never becomes “probably Shared”
and never falls back to another source profile.

Recursive SCCs, generics, function values, and callbacks are not inferred just
because some body text is visible. Their exact closure rule must be sealed or
they remain a ContractBoundary/rejection.

## 6. Destination matrix

The first verifier freezes exact source/destination behavior:

| Source expression | Destination | Required behavior |
| --- | --- | --- |
| available Home | Home-demand parameter | consume once |
| available Home | ordinary handle parameter | borrow for call only |
| handle alias | Home-demand parameter | reject; identify root |
| fresh Home rvalue | ordinary handle parameter | scoped temporary if verified |
| explicit `share` | Shared-demand destination | materialize one owner by sealed plan |
| explicit `share` | general Home-demand destination | representation/type compatibility D0 decides |
| explicit `share` | handle-only parameter | reject redundant paid owner |
| whole-root Home | canonical `release root` | consume now; terminal enters C′ DropPlan |
| handle alias | canonical `release alias` | reject; identify supporting root |
| trivial value | ownership-changing destination | reject meaningless operation |
| unknown/generic capability | ownership-changing destination | fail before effects |

Fields and containers use their explicitly accepted destination contracts; an
owning field is never inferred merely from its Box-typed payload. The strong
untyped-field target below resolves that field row. Other packed storage,
globals, registries, weak slots and unresolved container profiles remain separate.

### Untyped strong stored slots

Decision (2026-10-19): bare stored fields, including legacy `init { ... }`
declarations, retain dynamic value-kind changes. This is an accepted semantic
target, not production coverage or permission to infer storage from MIR types.
Typed-field and weak-field profiles retain their separate contracts, including
any already admitted declared numeric defaults.

A slot is either `EmptyField` or a tagged language value with a separately
source-issued residence disposition. `EmptyField` is the known never-written
state, not language `null`, zero, or the local `UninitializedSlot` carrier.
Reading it produces Fault. A completed object may contain known-empty fields;
this does not permit publication with pending or unknown acquired obligations.
Only a successful explicit null store installs language null.

| Source-authorized candidate | Installed residence |
| --- | --- |
| Known scalar or explicit null | Value, with no child Home. |
| Immutable String value | Independently supported String value residence, using the existing value materialization/publication owner; never blanket Trivial classification. |
| Exact available compatible Home | Transfer once into the owning slot; remove its former cleanup responsibility. |
| Already authorized self-contained Dynamic carrier | Forward its existing obligation once, without inventing a source Home. |
| Exact admitted anchored borrowed handle | Non-owning residence, only with a verified live support relation covering every use and parent finalization. |
| Unknown capability or unsupported relation | Reject before effects; no implicit share or retain. |

An owning local can supply its own still-available Home. An ordinary alias
cannot consume the Home of its supporting root. After transfer, subsequent
non-owning uses require the exact installed-slot support; transfer never
creates a second Home. A same-slot read is not by itself a new owning-store
candidate. In particular, `x.f = x.f` through a borrowed Box read is rejected
unless an exact source-proved no-op preserves the SAME residence and support
identity; payload equality cannot issue that proof. A stored borrowed anchor
must outlive the entire stored residence, with transitive dependencies preserved
across holder mutation/end. Unsupported temporary/external/self-dependent
anchors and ownership cycles reject before publication.

For a closed constructor, an exact canonical birth-side `me.field = parameter`
relation may declare a destination demand for that parameter ordinal. The
caller must supply the SAME actual capability witness: a Box arm transfers an
available Home, scalar/null arms transfer no Home, and a String arm installs
an independently supported immutable value. A borrowed Box actual cannot
satisfy an owning arm. This generalizes the existing Provided-field relation;
it does not turn ordinary method formals, rebound aliases or opaque payloads
into hidden Home-demand parameters. Before field installation the selected
constructor argument owner retains its obligation; after installation only the
field owns it. Constructor Fault drains installed fields and never restores a
caller Home or ends the transferred argument a second time.

Evaluate the Place and RHS once in source order. Prepare the candidate and
validate the destination before commit. String preparation acquires independent
Text residence before commit; copying its handle/tag is insufficient, and
pre-commit failure cancels only the actual preparation acquisition.
Pre-commit RHS/validation/write Fault leaves the old slot unchanged and cleanup with the actual prior candidate
owner. Commit installs the new value and its disposition, then detaches and
ends the old residence. A post-commit cleanup Fault does not roll back the
slot or restore a caller Home. Preserve first Fault, attempt remaining cleanup,
and execute user cleanup outside storage locks. Kind changes obey the SAME
transaction; payload and disposition cannot be committed separately.

Reads check I64/Bool/Text/object kind without bit reinterpretation or default
coercion. Scalar reads copy values. String reads either retain a verified
bounded view or materialize an independent immutable value before escape.
Box reads create no Home: they borrow the exact holder/slot support, which
replacement or holder end invalidates. Reject source uses whose support does
not cover them. Such a borrow cannot become a self-contained Dynamic result
without an existing independent acquisition authority.

Destruction uses the current installed disposition exactly once: parent fini,
then current owned field residences in reverse declaration order, then native
structural drop. Empty/scalar/null fields have no child end; borrowed fields
never end their anchor, which must remain live through parent fini. Failed
construction follows lifecycle initialized-subset cleanup and never calls the
incomplete parent fini. An EmptyField read Fault in parent fini still drains
remaining field obligations under the first-Fault rule. Runtime tags select
checked value operations, not
ownership, sharing, support provenance or finalization policy.

### Ordinary local reassignment

Ordinary local reassignment is also unresolved for HomeV1:

```hako
local b = a
b = c
```

The surface D0 must select handle rebinding, Home replacement, or rejection;
runtime value kind cannot choose among them. Uninitialized/`null` locals are a
separate row.

## 7. CFG Home Flow

Binding SSA owns current `BindingRef -> ValueId`. Home Flow consumes those
identities and separately tracks Home availability.

```hako
if cond {
    adopt(node)
}
use(node)
```

This is rejected because `node` is only maybe available after the join. The
diagnostic names the consuming branch.

```hako
loop(cond) {
    adopt(node)
}
```

This is rejected when the consumed state reaches the backedge and a later
iteration would reuse the missing Home.

The rule is not “Home transfer is forbidden in loops”. A loop-local fresh
Home, transfer followed by terminal `break`, or exact transfer-and-replenish
may be admitted when its dedicated CFG proof closes.

Home Flow must not synthesize an owner PHI or repair a conflict with hidden
sharing.

For a loop-carried Text value, a Static/Owned descriptor selects the current
representation; the descriptor itself is not a Home. Its source-proved local
residency obligation remains distinct from a borrowed Text view, including a
view of an owned string. A rebind evaluates its RHS before committing the new
residency and discharging the displaced owned value; Static has no End. RHS
Fault does not publish the candidate. Backedge/continue preserve the current
obligation; loop exit and Return discharge or explicitly transfer it once.
The descriptor's kind may implement the physical conditional-End predicate,
but only the original source residency proof authorizes that End.

A self-contained Dynamic call result is not automatically one Home. It may be
a trivial, owner-bearing, or weak payload inside one opaque carrier. Recipe
`Dynamic`, runtime tags, and physical result decoding cannot choose a Home
meaning. The carrier itself has a separate forward-or-end-exactly-once
lifecycle; this is not a Home root and does not issue `Available`.

A source-visible Dynamic Home still requires a neutral value/destination
classifier and CFG-complete Home Flow. Without that stronger source contract,
Dynamic carrier flow tracks `Absent -> Live -> Ended | Forwarded` and stays
separate from Home Flow.

For a Loop-local Dynamic carrier, the same source binding may receive a fresh
carrier on each iteration. Dynamic carrier flow proves
`Absent -> Live -> Ended | Forwarded` per iteration. Fault before normal result
publication creates no lifecycle obligation; Fault, Return, or fallthrough
after publication must discharge the lexical carrier exactly once before
propagation/backedge. Home `Available` is issued only by the separate stronger
Home classification path.

In the bounded current target, V9 remains live through the exact I6 invocation
borrow and ends after the I6 Normal/Fault outcome. V17 is forwarded through
I16 into B0 and remains live on the JoinSig Backedge. Neither chronology may be
derived from ValueKey last use.

The current B0 before a rebind is a symbolic binding-carrier lineage, not the
I13/V15 borrowed read. Its first instance comes from plain parameter `pos`,
while later instances come from forwarded V17 results. These origins may have
different displaced-end dispositions. The rebind transaction commits the new
current before discharging the old disposition; it may not assume that the
initial borrowed ingress owns an end obligation.

Home transfer also has an exact temporal boundary. Argument expressions are
prepared in source order; a later argument failure must not leave an earlier
caller Home consumed. The transfer/failure D0 selects one commit point before
production lowering. Callee implementations do not choose independently.

A borrowed result rooted in a temporary, for example
`makeTree().getRoot()`, is rejected in the first profile unless a later exact
temporary-lifetime-extension contract is accepted.

## 8. Lifecycle, weak, cleanup, and concurrency

- `share` changes independent ownership.
- `release root` ends one verified Home at its source point; a Shared
  non-terminal release dispatches no hook.
- `fini {}` is a non-callable Box hook dispatched only by the terminal Home
  DropPlan. It is not a transfer spelling or direct physical-free API.
- ordinary handle end is owner-neutral; `take` and terminal return forward a
  Home atomically and dispatch no hook in transit.
- a verified owning field releases its Home during parent teardown; the child
  hook runs only when that release is terminal. Exact field classification
  remains `OWN-FIELD-CONTAINER-DEST-D0`.
- `cleanup` owns lexical exit actions.
- `close()`/`shutdown()` are optional ordinary domain methods and add no
  language or ownership authority.
- `weak` is a generation-aware non-owner with separate upgrade rules.
- `share` does not imply `Send`, `Sync`, thread safety, or cross-thread use.
- physical reclamation belongs to the selected storage/runtime/backend plan.

These contracts must remain distinct even if one backend happens to implement
several with the same pointer or counter.

The accepted terminal relation is:

```text
last Home release
-> optional parent fini hook
-> verified owning fields in reverse declaration order
-> native structural drop
-> weak tombstone/reclaim
```

Direct `obj.fini()`, B′ Dead-with-live-Home, global finalizer dispatch, and
last-strong structural drop that bypasses a declared hook are retirement
targets. They remain implementation evidence only until the bounded C′ series
and mandatory Home reference closeout land.

## 9. Performance contract

The semantic cost law is:

```text
ordinary handle:
  owner delta = 0

Unique Home transfer:
  owner delta = 0

share:
  owner delta = +1 and shared bookkeeping may occur

release:
  owner delta = -1; Unique needs no owner counter, Shared uses its selected lane
```

This does not mean “zero instructions”. Field access, pointer loads, bounds
checks, calls, destruction, and allocator work may remain.

C-like speed is achieved only when the physical Unique route proves that the
measured hot path adds no RC, control-cell, handle-registry, or avoidable Box
birth work. The grammar alone is not a performance claim. Representation
selection remains downstream of semantic Home verification and must be judged
by the repository perf/assembly method.

The exact Shared representation is unresolved. A nominal `shared box` and
per-instance promotion are alternatives to be decided by
`OWN-HOME-REPRESENTATION-D0`; neither is accepted by this page.

## 10. Compatibility profile

Current production remains SharedV1 while HomeV1 activation is zero.

Migration laws:

1. a complete source unit selects exactly one profile;
2. HomeV1 failure never retries SharedV1;
3. unsupported syntax, ABI, destination, representation, or backend fails
   before Builder effects;
4. ordinary source stays lightweight;
5. SharedV1 retires only after implicit owner producers, cross-profile
   bridges, fallback attempts, and source users have exact zero/parked counts.

The manifest/edition spelling for profile selection remains a separate D0.

## 11. Diagnostics

Ownership diagnostics are part of the language contract. They identify:

- the Home root;
- where it was consumed or shared;
- the conflicting branch/backedge/use;
- the destination ABI;
- only repairs supported by sealed capabilities.

Examples of stable reason families:

```text
home-unavailable-after-transfer
home-maybe-consumed-after-branch
home-consumed-on-loop-backedge
home-demand-received-handle
redundant-share-to-handle
home-result-relation-conflict
home-abi-missing-at-contract-boundary
home-capability-unknown
home-release-received-handle
home-release-conflicts-with-cleanup-capture
```

Do not reduce these to “inference failed”, and do not suggest `share` or field
move-out unless that exact operation is legal.

## Durable laws

1. Ownership belongs to Home/storage, not a Rust-like source reference type.
2. Ordinary local reuse and ordinary parameters are non-owning handles.
3. The destination's sealed Home demand is transfer authority.
4. A terminal return may forward an available Home.
5. Only explicit `share` adds an independent same-identity owner.
6. Binding SSA owns values; Home Flow owns availability; Ownership SSA owns
   physical ownership transitions.
7. `fini`, `cleanup`, weak references, and physical free keep separate owners.
8. Generic/opaque/unknown capability fails fast; it is never guessed.
9. No runtime tag, method name, reference count, hidden retain, raw fallback,
   or profile retry decides ownership.
10. Parser support, production lowering, and C-like performance require their
    own named gates; this source decision alone claims none of them.
11. `release root` is the sole explicit early Home-end spelling. It has no
    Result, ordinary/generic wrapper Call, `drop` alias, field/projection form,
    hidden handle re-rooting, or MirBuilder/backend name magic.

Whole-call behavior does not create a second ownership vocabulary.
`@rune CallableContract(query)` uses the ordinary receiver `Handle` boundary:
the call does not transfer, add, end, or escape a Home. The Query receipt does
not independently own this fact; one same-declaration `VerifiedHomeAbi` is the
sole receiver/parameter/result Home authority consumed by the declared
callable contract. See
[`callable-contracts.md`](callable-contracts.md); declaration conformance and
physical ABI remain separate owners.
