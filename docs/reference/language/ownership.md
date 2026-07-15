# Ownership and Aliasing (SSOT)

Status: SSOT — accepted target language contract

Decision: Explicit-share, owner-anchored ownership accepted on 2026-07-15

Implementation: staged; production activation is 0 until the rows named in the
parked taskboard close. Current SharedV1 behavior remains transitional.

This page is the language authority for local aliases, ownership transfer,
callable ownership ABI, and the boundary that enters shared ownership.

Related authorities:

- `variables-and-scope.md`: lexical binding and name-resolution rules
- `scope-exit-semantics.md`: cleanup and exit ordering
- `lifecycle.md`: `fini()`, Alive/Dead/Freed, weak references, and reclamation
- `../../development/current/main/investigations/hakorune-sparse-ownership-surface-task-2026-07-15.md`:
  staged implementation order

The grammar examples below describe the accepted target contract. They do not
make a spelling parser-live. `EBNF.md` and the grammar registry remain the live
syntax authorities; an unsupported ownership spelling must fail fast until its
dedicated grammar row lands.

## 1. Thirty-second rule

Ordinary Hakorune code keeps its lightweight spelling:

```hako
local a = new Node()
local b = a

b.value = 1
print(a.value)
```

`a` owns the object. `b` is a temporary, mutable alias anchored to `a`.
Creating `b` does not create another owner and does not perform ownership
runtime bookkeeping.

Only independent lifetime crosses an explicit paid boundary:

```hako
local service = make_service()
local registered = share service

register(registered) // register declares a `share` parameter
use(service)
```

Here `service` remains usable and `registered` is an independent Shared owner.
The explicit `share` line is the point where shared-lifetime bookkeeping is
allowed.

The short law is:

```text
ordinary local name reuse:
  scoped alias; no new owner

ordinary parameter / receiver:
  noescape alias

ordinary result / owning destination:
  one owner is forwarded

independent lifetime:
  explicit share
```

Do not describe this as “every second reference needs `share`”. Multiple local
names may be free aliases. `share` is required only when multiple independent
owners or lifetimes are needed.

## 2. Four source capabilities

### Unique owner

An owned rvalue, ordinary owning result, or owning destination carries one
ownership token. Examples include `new`, a factory result, a returned owner,
and a value removed from owning storage.

The compiler may forward that one token through assignment, return, a verified
consuming call, or owning storage. Forwarding does not require reference-count
traffic. The old binding cannot be used after its owner has been forwarded.

### Scoped alias

For the first accepted profile, `local b = a` creates a `ScopedBoxAlias` when
the initializer is an eligible whole-root binding.

```text
owner-token delta:
  0

ownership runtime bookkeeping:
  0

mutation:
  allowed, sequential, and non-exclusive

lifetime:
  creation through local last-use

escape:
  forbidden
```

The owner and its aliases may all read and mutate the same whole Box. They are
not `noalias`, and the optimizer must assume that their effects overlap.

### Anchored view

A `view` is a non-owning call result anchored to a receiver or parameter. The
final vocabulary reserves static roots and verified subdomains, but those are
separate later rows. The first production profile accepts only a
receiver/parameter WholeObject anchor. A View performs no ownership runtime
bookkeeping and cannot outlive or invalidate its anchor.

Callers do not guess whether a result is a view. The callable ABI declares it:

```hako
get(): view Node {
    return me.child
}

local child = node.get()
```

An ordinary, unannotated result is Owned. A method name such as `get`, `peek`,
or `current` has no ownership meaning.

### Shared owner

Shared is the explicit independent-lifetime lane. `share expr` is the only
ordinary source boundary that authorizes Unique-to-Shared conversion.

`share` preserves object identity. It authorizes the compiler/runtime to create
the shared representation and the owner tokens required by the verified
surrounding context. It is not specified as an unconditional “increment by
one”: forwarding a last owner may avoid a redundant increment, while keeping
two independent owners requires two tokens.

For an owned root expression such as `share a`, the source law is exact:

1. `a` must be an eligible Unique owner with no live aliases/views;
2. `a` is rehomed as a Shared owner and remains usable;
3. the expression result is a second, independent Shared owner.

For a fresh rvalue such as `share make_service()`, the temporary source owner
has no later source use, so only the resulting Shared owner remains observable.
The optimizer may remove redundant retain/release work in either form, but it
may not change these availability semantics.

The first profile rejects `share` applied to an already Shared value, a
scoped-alias/view operand, a weak value, or a trivial value. Shared destinations
already have compiler-managed owner-copy semantics; repeating `share` there
would obscure rather than expose the Unique-to-Shared boundary.

Once a value is in the Shared lane, the compiler may insert verified
`CopyOwned` and `DestroyOwned` operations as owning destinations require. The
programmer does not have to spell `clone` for each Shared assignment. A future
explicit clone/cost-control surface, if useful, is a separate low-level row and
is not part of the ordinary ownership vocabulary.

## 3. Source defaults

| Position | Default contract | Extra ordinary spelling |
| --- | --- | --- |
| owned rvalue (`new`, Owned call) | one owner | none |
| `local b = a`, eligible whole root | Scoped alias | none |
| ordinary parameter / receiver | mutable noescape alias | none |
| ordinary return | Owned | none |
| ordinary owning field/store | owner forward when legal | none |
| independent lifetime / Shared entry | Shared | `share` |

Hakorune does not require ordinary source code to repeat `owned`, `borrow`,
`clone`, or `move` annotations. Ownership mode is not inferred from a runtime
tag or reference count; it comes from the resolved source site and callable
ABI.

For a known trivial value, reusing the same SSA value needs no owner token and
the alias distinction is unobservable. Dynamic `Any` must never use a runtime
copy/move branch. The first ownership-bearing `Any` profile is closed: it
rejects before Builder effects. A later separately verified uniform-
representation row may activate source-driven local aliases, but it cannot
change the runtime-inference prohibition.

An ordinary owning field/store consumes or forwards one owner. If the source
owner or one of its aliases must remain usable afterward, compilation fails
and points to the `share` boundary; the compiler does not silently retain the
Unique object. Thus ownership transfer stays mostly invisible in source, while
the only operation that can introduce independent lifetime stays visible.

## 4. Callable ownership ABI

Callable declarations have a value type and an independent ownership
contract. The semantic vocabulary is:

| Position | Default | Non-default API contracts |
| --- | --- | --- |
| parameter / receiver | noescape alias | `take`, `share` |
| result | Owned (or Trivial after type resolution) | `view`, `share` |

- `take` parameter: the callee receives the caller's one owner. The ordinary
  call site need not repeat a move keyword; the verified signature and
  last-use check make consumption explicit and diagnostic.
- `share` parameter: the callee receives an independent Shared owner.
- `view` result: the result is anchored and non-owning.
- `share` result: the caller receives an independent Shared owner.

The conversion matrix is exact:

```text
Unique actual -> take parameter:
  normal call spelling; owner is consumed after the no-live-loan check

Unique actual -> share parameter:
  implicit conversion forbidden; caller supplies `share actual`

Shared actual -> share parameter:
  normal call spelling; compiler forwards/copies a Shared token as liveness
  requires

ScopedAlias / View actual -> share parameter:
  reject; end the loan and share the owner root, or use a Shared-returning API

Unique return in a `share` result function:
  implicit conversion forbidden; the return expression must cross an explicit
  `share` boundary

already-Shared return in a `share` result function:
  compiler forwards/copies one Shared token
```

These modifiers are API-definition vocabulary, not line-by-line local
annotations. Their exact contextual-keyword grammar is activated separately.
They must not be encoded through method names or `@rune Ownership(...)`.

Example target signatures:

```hako
inspect(node: Node)                 // noescape alias parameter
adopt(take node: Node)              // consumes one owner
register(share service: Service)    // receives Shared ownership

make(): Node                        // Owned result
get(): view Node                    // receiver-anchored result
service(): share Service            // Shared result
```

For an instance method, an elided `view` anchor is the receiver. For a static
function with exactly one eligible input root, it is that input. Ambiguous or
different provenance requires an explicit anchor and otherwise fails fast.

## 5. Scoped-alias V1 boundary

The first safe profile is intentionally narrow.

Allowed:

- whole-root local alias
- alias chains flattened to one owner root
- reads and sequential mutation through owner or alias
- stable aliases used inside `if` or `loop` when the definition dominates
- branch-local and loop-local aliases that end locally
- ordinary noescape calls with a sealed callable ABI

Rejected in V1:

- alias binding reassignment
- field/index/projection aliases kept in a local
- alias selected or redefined by a PHI
- return, owning field/global/collection store, or registry escape
- closure/Future/task/channel capture
- crossing `await` or `yield`
- unknown dynamic/plugin/FFI ownership ABI
- conversion to Shared while an alias remains live

The loan ends at the alias's last reachable use. While it is live, the owner
cannot be forwarded, rebound, destroyed, finalized, rehomed, or converted to
Shared. A verifier uncertainty is a compile error; it never triggers hidden RC
or a raw-pointer fallback.

Projection views and same-anchor view PHIs are later, independently gated
profiles. They must not be approximated by whole-root aliases.

## 6. Ownership bookkeeping cost law

Use precise wording in diagnostics and performance claims:

```text
ScopedAlias / AnchoredView:
  extra ownership bookkeeping = 0

Unique token forward:
  RC operations = 0

terminal Unique owner:
  direct structural drop/free where the representation permits

share boundary:
  Shared representation/control-cell work may occur

Shared owner copy/drop:
  verified RC or equivalent shared-lifetime bookkeeping may occur
```

“Zero instructions” is not the semantic claim. A pointer read, field access,
call, destructor, allocator operation, or debug-only check may still emit
instructions. The guaranteed property is that an alias/view does not add an
owner and a Unique forward does not add RC traffic.

## 7. Diagnostics are part of usability

Ownership rejection must identify the loan, conflict, next use, and useful
repairs. For example:

```text
cannot give `a` an independent lifetime: scoped alias `b` is still live

alias created at: node.hako:12:15
conflicting escape: node.hako:15:9
next alias use:    node.hako:17:7

help:
  - move the escape after the last use of `b`
  - narrow `b`'s scope
  - enter the Shared lane explicitly with `share`
```

Reject fixtures must golden-test the stable reason and repair hints, not only
the fact that compilation failed.

A debug build may maintain a non-owning shadow-loan observer to catch verifier
bugs. It may count active loan records and report source sites, poison retired
storage, or quarantine reclaimed cells. It must not retain an object, delay
reclamation, or become release-build memory-safety authority.

## 8. Compiler authority split

```text
resolver / callable ABI:
  source intent, owner root, view anchor/domain, consuming/Shared destination

Loan Flow:
  alias/view creation, use, last-use frontier, invalidation permission

Binding SSA:
  sole BindingRef -> current ValueId authority

Ownership SSA:
  owner-token creation, forwarding, copy, and consuming-use verification

Lower / backend:
  materialize the sealed contract; never rediscover it from maps, names,
  runtime tags, or observed reference counts
```

Loan Flow is not a second reaching-value map. Scoped aliases may point to the
same `ValueId` as their root while carrying no independent ownership token.

## 9. Lifecycle, weak, concurrency, and raw boundaries

- `fini()` is object lifecycle, not ownership consumption. It is forbidden
  while a local alias/view loan remains live. Shared resource tombstones and
  Alive/Finalizing/Dead/Freed are defined in `lifecycle.md`.
- `weak` is a generation-aware non-owner, not a view. Weak creation/upgrade
  keeps its existing explicit lifecycle contract.
- `share` does not imply cross-thread safety. Cross-thread sharing requires the
  separate synchronization capability and backend contract.
- Arena allocation/lifetime is a separate language Decision. It may later
  remove per-object ownership work for region-bounded graphs, but it neither
  changes the alias/share laws here nor blocks their correctness slices.
- Normal Box ownership has no silent raw-pointer fallback. A future unsafe raw
  lane requires its own language Decision and is outside this contract.
- C/plugin/host ABI terms such as borrowed/owned handles are boundary metadata;
  they do not replace Hakorune's source callable ABI.

## 10. Migration and implementation status

Current production routes still include SharedV1 behavior in which ordinary
Box aliases may materialize strong-owner copies. That is compatibility
behavior, not this target source contract.

Migration rules:

1. SharedV1 and the sparse ownership profile normalize to one resolved
   ownership product, one Binding SSA, one Ownership SSA, one MIR vocabulary,
   and one runtime authority.
2. A source unit is verified under exactly one selected profile. Canonical
   failure never retries another profile.
3. Unique-to-Shared promotion is never inferred on the sparse profile.
4. Unsupported syntax, callable ABI, representation, or backend capability
   fails before Builder/backend effects.
5. SharedV1 retires only after its source units, implicit-share producers,
   cross-profile bridges, and fallback attempts all reach zero.

Until the relevant parser, resolver, Loan Flow, Ownership SSA, runtime, and
backend rows close, this document must not be used to claim that current
assignment behavior, Box representation, or production ownership costs have
already changed.

## Durable laws

1. Ordinary local aliases and parameters remain lightweight and non-owning.
2. A Unique owner may move/forward without RC.
3. Only explicit `share` enters the independent-lifetime Shared lane.
4. Shared-lane owner bookkeeping may be compiler-managed after that boundary.
5. Ordinary results are Owned; free call-result aliases require a verified
   anchored `view` ABI.
6. No runtime tag, method name, map diff, or observed reference count decides
   ownership.
7. Binding SSA owns current values; Ownership SSA owns tokens; Loan Flow owns
   temporary permissions.
8. Uncertainty fails fast; it never becomes hidden RC, raw memory, or another
   profile retry.
