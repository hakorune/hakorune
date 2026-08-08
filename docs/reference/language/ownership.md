# Ownership and Home Flow (SSOT)

Status: Language semantics SSOT; Home direction accepted, exact HomeV1 grammar
provisional, production activation 0

Decision: accepted on 2026-08-04 as the successor to the earlier
`move/share/view` target; C′ terminal Home finalization and contextual
whole-root `release root` amendments accepted on 2026-08-05.

This page is the source-language authority for ownership, ordinary aliases,
Home transfer, and the explicit boundary that adds an independent owner.

Related authorities:

- `variables-and-scope.md`: lexical bindings and nearest-binding assignment;
- `scope-exit-semantics.md`: lexical cleanup and exit ordering;
- `lifecycle.md`: non-callable `fini {}` hook, terminal Home DropPlan, weak
  references, and reclamation;
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
| explicit `release root` direction | accepted whole-root target; production 0 |
| generic/composite release | provisional; exact capability D0 open |
| exact HomeV1 source grammar | provisional; D0 rows open |
| Box `obj.x` place / `obj.x()` call prerequisite | accepted target; Property production retirement parked |
| Rust/Hako parser and AST Home carriers | inactive / absent |
| resolver, Home Flow, callable Home ABI | inactive / absent |
| canonical Builder/source producers | 0 |
| passive Ownership SSA / ownership opcodes | existing narrow infrastructure only |
| current ordinary Box assignment | transitional SharedV1 behavior |

The live `EBNF.md` and grammar registry remain syntax authority. Examples on
this page are target examples, not evidence that a parser accepts them.
Unsupported lookalikes must fail fast until their exact shared grammar row
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
cannot escape beyond the Home that supports it.

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

## 3. Accepted direction and provisional HomeV1 capsule

The durable semantics are accepted. The smallest source spelling remains
provisional until the named D0 and grammar rows close.

### Declaration-side Home demand

Candidate target:

```hako
adopt(take node: Node) {
    // the body may consume node only through a separately verified destination
}
```

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

`share(...)` remains an ordinary function call; contextual ownership `share`
is only the prefix expression form selected by its grammar row.

### Explicit early Home release

`release root` is the sole accepted early-end spelling. It is a statement-only
contextual keyword with one identifier root and a dedicated source carrier;
resolution and a sealed Home Flow plan own its meaning. It is not a generic
wrapper, method-name convention, direct `fini` call, or backend name match.
Ordinary `release(value)` and `obj.release()` remain ordinary calls. `drop`
and `unbox` forms are not aliases. The lexer does not globally reserve
`release`; declarations/bindings named `release` and `Build.release` remain
ordinary syntax.

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

parameter demand:
  Handle | Home | SharedHome | Trivial

result relation:
  Unit | Trivial | HomeToCaller | FromReceiver |
  FromParameter(index) | SharedHomeToCaller
```

The passive relation vocabulary for these demands/results is landed in the
resolver as an internal, branded receipt, and the bounded `I64|Unit` Home ABI
catalog is now issued through one same-declaration co-seal. The relation brand
is batch provenance only, not resolver nominal identity. This does not make
Home grammar or production ABI live: Query/body conformance, Home Flow,
physical ownership, targets, and production remain separate gates.

### ClosedCallable

A callable whose body and all relevant resolved facts are locally available
may derive a candidate **result relation** and local Home Flow from its body.
Parameter and receiver Home demands come only from the resolved declaration:
a plain parameter is always Handle, and only an accepted explicit Home-demand
form such as candidate `take` may consume it. Body analysis verifies that
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

Fields, arrays, maps, packed storage, globals, registries, weak slots, and
replacement/empty-slot rules remain separate D0 rows. An owning field is not
assumed merely from its Box-typed payload.

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
