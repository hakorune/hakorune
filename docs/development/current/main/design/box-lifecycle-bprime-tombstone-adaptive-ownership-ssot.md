---
Status: SSOT
Decision: accepted-for-tasking — B′ eager-fini tombstone with adaptive ownership
Date: 2026-07-14
Scope: Shared/resource Box lifecycle, Ownership SSA materialization,
  ObjectCell, weak identity, Arc retirement, and adaptive runtime ownership.
Related:
  - ../../../../reference/language/ownership.md
  - ../../../../reference/language/lifecycle.md
  - ../../../../reference/language/scope-exit-semantics.md
  - binding-ssa-first-control-lowering-ssot.md
  - arc-retirement-and-ownership-substrate-ssot.md
  - object-handle-box-identity-contract-ssot.md
  - box-object-model-replacement-map-ssot.md
  - ../investigations/box-lifecycle-bprime-tombstone-adaptive-ownership-task-2026-07-14.md
---

# B′ Box Lifecycle / Ownership Constitution

## Decision

Hakorune adopts B′ as the long-term Shared/resource Box lifecycle model.
Source-level owner/alias/View/Shared selection is owned by
`docs/reference/language/ownership.md` and supersedes the older
“every Box is shareable by default” wording in this document.

```text
source boundary:
  only an explicit verified share boundary enters this Shared lane

MIR ownership:
  CopyOwned creates one independently consumable strong token
  DestroyOwned consumes exactly one strong token

explicit fini():
  Alive -> Finalizing -> Dead
  runs the logical finalization transaction once
  tears down payload eagerly
  consumes no ownership token

last strong token:
  runs structural drop glue if payload remains
  never invokes user-defined fini()

last weak token:
  permits control-cell reclamation and slot-generation advance

Shared physical strategy:
  correctness-first SharedRc (atomic)
  later derived StaticUnique / LocalRc / SharedRc plans
```

The core B′ semantics are accepted. Adaptive modes, concurrent promotion,
direct-pointer projections, and global Box carrier replacement remain gated
implementation branches. They are not implied by this decision.

A normal source-level Box API does not expose physical `free`, `reclaim`, or
the selected RC strategy. Deterministic resource shutdown is `fini()`, an
ownership lifetime ends through verified forwarding/`DestroyOwned`, and
physical reclamation remains a runtime materialization decision. Unique local
ownership may bypass this Shared representation entirely. A future raw-memory
API requires a separate language Decision; it is not part of B′.

## Why this is one constitution, not one implementation box

Four truths must remain separate:

| Owner | Owns | Must not own |
| --- | --- | --- |
| language lifecycle | logical `fini`, Alive/Dead behavior, explicit cleanup order | RC counters, MIR values, pointer layout |
| Ownership SSA | Owned/Borrowed/None token discipline and forwarding | Alive/Dead, physical RC mode, object identity |
| runtime ObjectCell substrate | physical counts, payload slot, generation, leases, reclamation | BindingRef reaching values, source cleanup policy |
| optimizer / representation planner | proven StaticUnique/LocalRc/SharedRc strategy | source-visible ownership semantics |

Binding SSA remains the only `BindingRef -> ValueId` reaching-value authority.
`MirOwnershipKindV1` remains the only MIR token classification. Neither may
contain `StaticUnique`, `LocalRc`, `SharedRc`, lifecycle state, or runtime counts.

## Normative laws

### 1. `fini()` and ownership destruction are different operations

```text
fini():
  logical object transition
  user hook and deterministic stored-token cleanup
  eager payload teardown
  receiver/caller token is not consumed
  other independent owner tokens are not implicitly retained or destroyed

DestroyOwned(value):
  one MIR token consume
  physical strong-count delta = -1 when materialized by RC
  user-fini calls = 0

last-strong structural drop:
  memory-safety drop glue
  remaining strong field tokens and native payload storage are released
  user-fini calls = 0
```

Scope exit runs registered `cleanup` bodies first. A cleanup body may call
`box.fini()` explicitly. Scope-owned tokens are destroyed afterward. Merely
ending ownership never implies `box.fini()`.

Fields whose verified storage contract is Shared carry Shared owner tokens.
Owning Unique fields instead receive a forwarded owner and need no RC solely
because they are fields. Parent finalization releases and clears whichever
stored owner tokens exist in deterministic reverse declaration order; it does
not implicitly call child user `fini()`. A parent that owns a child resource
semantically must call `child.fini()` explicitly in its user hook.

Stored-token teardown may reduce the same object's physical count through a
self-field. The invariant is specifically that the `fini()` invocation does
not consume its receiver/caller token, not that no count can change anywhere
inside finalization.

### 2. State is not one overloaded enum

The runtime may encode these axes compactly, but their meanings stay distinct:

```text
logical lifecycle:
  Alive | Finalizing | Dead

payload:
  Present | Dropping | Absent

root residency:
  StrongPresent | WeakOnly | Reclaimable
```

`Finalizing` is a runtime transaction state, not an ordinary source-visible
state. `Dead` is created only by explicit `fini()`. If the last strong token
disappears while Alive, structural drop removes the payload without inventing
a successful user finalization.

A valid strong token to Dead may be forwarded or duplicated with `CopyOwned`
for identity/tombstone observation; this does not restore payload or Alive.
Creating a new weak token from Dead and upgrading an existing weak token to
Dead are both rejected.

The historical word `Freed` means `strong_count == 0` and structural payload
reclamation is complete. A small generation-bearing weak tombstone may still
exist. `Dead` with a remaining strong token is not Freed even though its
payload is already Absent. The control cell itself is reclaimed only when both
strong and weak counts are zero.

### 3. Finalization is a transaction

The correctness contract is:

```text
1. one caller wins Alive -> Finalizing
2. new ordinary payload-access leases are rejected
3. outstanding ordinary leases drain
4. the winner receives one privileged FinalizerLease
5. user hook runs at most once
6. stored strong and weak tokens plus payload teardown complete
7. payload becomes Absent
8. state publishes Dead
```

The privileged lease is required because a `fini()` hook may need to inspect
or clear its own fields or invoke a helper on itself after ordinary access has
been closed. It is owner-branded, unforgeable, and non-escapable. Reentrant
`fini()` by the winner is a typed failure; a call after Dead is an idempotent
no-op. Concurrent losing callers do not run the hook; they wait for Dead and
return the same successful/idempotent result in the accepted baseline. Exact
memory ordering, hook-error propagation, and method-lock ordering must be
sealed before shared-family activation.

If a hook or cleanup step fails, the existing scope-exit rule applies: attempt
the remaining release steps, preserve the primary error, and then fail fast.
Failure must not reopen an already-torn-down payload or publish a partial
Alive state.

### 4. Weak upgrade is one linearizable operation

A weak reference is `(slot, generation)` identity, not a payload pointer.
Upgrade succeeds only when one operation proves all of:

```text
slot generation matches
lifecycle == Alive
strong count > 0
one new strong token is acquired before reclamation can win
```

Checking generation/state and incrementing the strong count in separate
unguarded steps is forbidden. `Finalizing`, `Dead`, weak-only, stale, and
reclaimed cells all reject upgrade.

Generation wrap retires the slot permanently. Raw handle bit layout remains
opaque and is a separate ABI decision.

Weak values themselves require a lifecycle law before production adoption.
Ordinary bit-copy cannot duplicate a custom weak count and later drop it twice.
The first Ownership SSA profile therefore rejects WeakRef. A later design row
must select exactly one verified representation, for example a weak-owned token
under the existing ownership opcodes, dedicated weak copy/destroy operations,
or a separate ObjectCell-backed weak value. `last weak` reclamation cannot be
activated before that choice is co-sealed with transport and backend behavior.
When payload teardown removes a weak field, it destroys that weak token and
decrements the weak count; it never upgrades, traverses, or finalizes the weak
target.

### 5. Identity and owner/root tokens are distinct

```text
BoxIdentity:
  language object identity = slot + generation

Owned MIR value / runtime strong root:
  one independently consumable owner of that identity

host handle:
  an external root/token referring to an identity
  not automatically the identity itself
```

Creating two host handles for one object creates two roots; it does not create
two object identities. Within ownership-managed MIR, `CopyOwned` is the only
strong-duplication operation and preserves object identity while creating a
new owner token. Construction, successful weak upgrade, and explicit host/FFI
ingress may create new roots through their own verified ABI operations.
Value-like `clone_box` may create a new object identity and is therefore not an
alias for `CopyOwned`.

### 6. One physical counter authority per object

The final runtime must not keep independent local and shared counts alive at
the same time. Use one tagged physical strategy or a correctness-first single
atomic representation:

```rust
enum RcStrategyV1 {
    StaticUnique,
    LocalRc,
    SharedRc,
}
```

The concrete counter storage belongs to the selected variant. Parallel
`local_strong` and `shared_strong` fields are forbidden because they create
two lifetime truths.

`Immortal` is not an owner-count strategy. It is a separate explicit
process/runtime-root residency class with its own shutdown, fini, weak,
identity, and observability questions. It remains parked until a concrete
singleton/type-descriptor consumer gets a separate Decision and proof ladder.

The first identity-bearing ObjectCell implementation is `SharedRc` only, and
`SharedRc` uses atomic counts. `LocalRc` is the non-atomic strategy.
`StaticUnique` means that a closed proof has shown that no independent owner
token can be created over the object's full lifetime. ScopedAlias/View loans
do not violate this property because they add no owner. `StaticUnique` is not a
source type and does not contain a dormant first-owner-copy promotion
mechanism.

A function-local proof does not survive a generic Return, outbox, or unknown
call merely because the Owned token is forwarded. Interprocedural
StaticUnique requires a separately sealed parameter/result/call-edge
capability whose caller continuation is included through the terminal consume;
otherwise the representation selector chooses LocalRc/SharedRc.

The baseline representation selector is conservative:

```text
no possible independent owner:
  StaticUnique

possible same-thread independent Shared owner / CopyOwned:
  LocalRc

possible cross-thread share or unknown publication:
  SharedRc
```

These strategies form a monotone selection lattice, not an implicit runtime
state machine. There is no downgrade after publication or owner duplication. A
later `PromotableUnique` optimization may add a first-independent-owner
`PromotableUnique -> LocalRc`
transaction, but it must be a separate evidence-gated row and is not required
for B′. `CopyOwned` alone does not perform thread promotion. Cross-thread
publication requires an explicit capability/representation plan and one
linearization point.

### 7. Direct pointers are private leased projections

Public and cross-boundary identity remains opaque `ObjectHandle` /
`BoxIdentity`. A direct pointer may exist only as an internal projection such
as `StrongObjectRef` or `PinnedObjectRef` under a typed lifetime/lease.

```text
raw pointer as language identity = forbidden
stale raw pointer then generation check = forbidden
movable Vec element address as stable pointer = forbidden
pointer cache that secretly owns an Arc root = legacy, not proof
```

Before any direct-pointer fast path, the substrate must prove non-moving
storage plus reclamation safety through a strong root, pin, lease, hazard,
epoch, or equally explicit mechanism.

StaticUnique proves absence of independent strong-owner duplication only. It
does not by itself prove a stable address, absence of ScopedAlias/View loans,
or permission to elide a pointer lease.

An `ObjectLease` pins the payload/cell for its lexical extent. Last-strong
structural reclamation cannot free or reuse storage while any valid lease
exists; it must defer or drain according to the sealed lease law. A reference
derived from a lease cannot escape that lease.

### 8. ObjectCell cannot be a second RC beneath Arc

During migration, an ObjectCell-shaped adapter may observe lifecycle state
while `Arc` remains the physical owner. It may not maintain a second production
strong count and claim ownership authority. A named family switches physical
ownership atomically after its family gate is green.

### 9. Plugin lifecycle is excluded until its ABI is split

Current plugin carriers call the plugin `fini` route from Rust `Drop`. That is
incompatible with B′ because last-strong drop would invoke user finalization.
Plugin families remain legacy until they have separate owners for:

```text
logical explicit fini route
structural instance-destroy/drop glue
```

If the current plugin ABI exposes only one operation for both meanings, a
separate ABI decision is required. The first ObjectCell family must exclude
plugins, unknown FFI ownership, weak handles, and cross-thread sharing.

### 10. Cycles and optimization

Base strong cycles may leak. Back-pointers should use weak references. A later
cycle detector/collector is optional and never runs user `fini()`.

Static-unique direct reclaim, stack allocation, header elision, LocalRc, ARC
pair removal, and recurrence/escape analyses are derived materialization
strategies. They do not decide the source-level owner/alias/share contract.

The derived reclaim form is always:

```text
terminal DestroyOwned
+ verified StaticUnique object plan
+ backend materialization
  -> immediate structural drop/reclamation when legal
```

There is no second `ReclaimUnique` MIR consume authority. An unchecked raw
pointer free is outside the normal Box/ObjectCell contract; double-free or
use-after-free cannot be claimed to remain local to one Box.

Header/control-cell elision requires proof that all of these are unobservable:

```text
object identity
weak reference
host / FFI handle
explicit fini tombstone
reflection / registry publication
cross-thread sharing
```

## Correctness-first architecture

```text
Canonical source + resolved identity
        │
        ▼
Binding SSA
  reaching ValueId authority
        │
        ▼
Verified Ownership SSA
  Owned/Borrowed/None and path-sensitive forwarding
        │
        ▼
ObjectCell ABI adapter
  CopyOwned / DestroyOwned / explicit FinalizeObject
        │
        ▼
generation-tagged stable storage
  lifecycle transaction + strong/weak counts + payload slot
        │
        ▼
optional derived representation plan
  StaticUnique / LocalRc / SharedRc
```

The long-term runtime API should expose semantic operations rather than Arc
accidents:

```text
copy_owned
destroy_owned
begin_finalize / finish_finalize
borrow_payload under ObjectLease
weak_create / weak_upgrade / weak_destroy
identity
```

Public/source `obj.fini()` must enter `FinalizeObject` (final spelling may be
an ABI operation rather than a MIR opcode). Only that transaction may dispatch
the user hook. Direct source-to-hook calls are forbidden.

The Dead tombstone retains immutable identity, type, and diagnostic/hash
metadata required by the language. A user payload-backed `toString` or helper
method is not re-entered after Dead merely to implement observation.

Exact public opcode/API spelling beyond already accepted `CopyOwned` and
`DestroyOwned` belongs to its implementation row.

## Current implementation gaps

The current repository is not B′-complete:

```text
InstanceBox:
  separate Arc<Mutex<...>> state and finalized booleans

logical dead truth:
  InstanceBox flag + global FINALIZED_BOXES + plugin AtomicBool

host handles:
  generation-0 reusable slots and Arc-backed payload roots

weak handles:
  separate registries and no single co-sealed generation/state authority

plugin carriers:
  Drop and finalize_now share the user fini route

FiniOwner:
  currently mixes route/provenance with transition ownership; ObjectCell must
  become the single transition/once/order owner and legacy FiniOwner becomes
  route metadata or retires

pointer caches:
  Arc-pinned raw projections rather than ObjectCell leases
```

These are migration inputs, not alternative authorities. The execution order
is owned by the related B′ taskboard.

## May claim

After this decision only:

```text
B′ is the accepted long-term Shared/resource lifecycle constitution
fini and ownership-token destruction are semantically separate
adaptive RC is a Shared-lane optimizer/runtime strategy, not source authority
ObjectIdentity and owner/root tokens must be separate
normal Box source/API exposes no manual physical-free operation
production runtime behavior has not changed
```

## Must not claim

```text
ObjectCell is implemented or production-active
all Box families use generation-tagged identity
current plugin Drop obeys B′
current weak/host registries are ABA-safe
all-atomic baseline proves adaptive ownership
StaticUnique or LocalRc is production-safe
source-level unique/reclaim or a general unsafe raw-Box lane exists
direct pointers are safe without a typed lease and stable storage
global Arc retirement
cross-thread share/promotion safety
cycle collection
Wasm or every backend owns the lifecycle vocabulary
```

## Stop conditions

Stop implementation or publication if a slice:

```text
calls user fini from DestroyOwned, last-strong drop, or generic Rust Drop
puts physical RC mode or lifecycle state in VerifiedOwnershipSsaV1
creates a second BindingRef -> ValueId or alias authority
keeps local and shared counters as independent truths
places Immortal root residency in the owner-count strategy enum
conflates StaticUnique selection with a lazy PromotableUnique transaction
reconstructs StaticUnique from strong_count == 1 after owner duplication or publication
uses CopyOwned as an implicit thread-publication operation
checks weak generation/state before a separate unguarded strong increment
uses raw pointer identity or dereferences a stale pointer before validation
exposes manual physical free/reclaim as an ordinary Box operation
adds a second ownership consume opcode for derived unique reclamation
uses a source annotation as the proof of a physical RC strategy
confuses two distinct owner tokens of one identity with one token consumed twice
places a production ObjectCell refcount beneath Arc refcounting
cuts over generic BoxRef or plugin as the first identity-bearing family
reuses a slot on generation wrap
lets finalizer self-access use an ordinary lease
holds a cell/slab/method lock while invoking a user hook or drop callback
lets public fini bypass the lifecycle transaction and invoke the hook directly
allows partial payload publication after failed finalization
keeps a second logical-Dead authority beside ObjectCell
skips destruction of stored weak tokens during payload teardown
reuses a slot while any weak token remains
silently accepts unsupported backend/object families
claims adaptive performance before the SharedRc baseline is correct
```

## Durable constitution

1. `record` remains an identity-free structural value.
2. A normal `box` enters the Shared/ObjectCell lane only through a verified
   explicit source/ABI boundary; local scoped aliases do not imply Shared.
3. Binding SSA owns current values; Ownership SSA owns token discipline.
4. In ownership-managed MIR, `CopyOwned` alone duplicates a strong owner;
   `DestroyOwned` consumes one.
5. Explicit `fini()` creates Dead and does not consume its receiver token.
6. Runtime reclamation and Rust `Drop` never imply user `fini()`.
7. Weak upgrade requires one atomic generation/state/strong acquisition law.
8. Physical RC mode is derived, one-way, and non-observable.
9. Strong cycles may leak in the base profile.
10. Family activation is atomic and unsupported families fail fast.
11. Normal Box source/API has no manual physical-free operation; verified
    terminal ownership consumption is the sole input to derived reclamation.
