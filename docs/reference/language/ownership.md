# Ownership and Aliasing (SSOT)

Status: SSOT — accepted target language contract

Decision: Explicit-move/share, owner-anchored ownership accepted on 2026-07-15

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

### Availability matrix

| Layer | Current state |
| --- | --- |
| target source semantics | accepted by this page |
| Rust/Hako parser and AST ownership carriers | inactive / absent |
| resolver, Loan Flow, callable ownership ABI, source lowering | inactive / absent |
| `CopyOwned` / `DestroyOwned` MIR vocabulary and verifier | passive and implemented |
| Rust MIR interpreter exact opcode execution | implemented semantic-test lane |
| witnessed `llvmlite-obj` ownership lowering | implemented narrow object lane |
| canonical source producers of ownership opcodes | 0 |
| ordinary production `local b = a` | transitional SharedV1 behavior; not yet ScopedAlias V2 |

MIR JSON transport knowing an opcode does not mean a source parser, resolver,
Builder, or backend may produce/execute it. Unsupported codegen lanes fail
their ownership capability preflight rather than silently dropping the
operation.

### Accepted target grammar capsule (not parser-live)

The grammar rows will activate this contextual surface one durable slice at a
time:

```ebnf
ownership_expr := ('move' | 'share') unary_expr

parameter := IDENT (':' TYPE_REF)?
           | 'move' IDENT ':' TYPE_REF
           | 'share' IDENT ':' TYPE_REF

result_spec := ':' TYPE_REF
             | ':' 'view' TYPE_REF view_anchor?
             | ':' 'share' TYPE_REF

view_anchor := 'from' ('me' | IDENT)
```

`move`, `share`, and `view` are contextual forms, not global hard keywords.
Disambiguation is fixed as follows:

- prefix `move/share` is an ownership expression only when it is not followed
  by `(`; `move(...)` and `share(...)` remain ordinary calls;
- `move: T` and `share: T` remain ordinary parameter names, while
  `move name: T` and `share name: T` are parameter modes;
- in a result spec, `view/share` is a mode only when followed by another type
  reference; a type literally named `view` or `share` remains distinguishable
  by the following delimiter;
- the first `view` anchor admits only a receiver or parameter WholeObject root.
  Field paths, static anchors, named domains, projections, and View PHIs are
  later rows.

The live `EBNF.md` intentionally omits these productions until the grammar
registry, Rust parser, Hako parser, AST/schema carriers, and shared witnesses
land together. The target grammar above is semantic design authority, not
evidence that current source accepts it.

Known conformance gap: the current Rust and Hako return-type scanners may
discard whitespace and coalesce an inactive spelling such as `: view Node` or
`: share Service` into the ordinary type text `viewNode` / `shareService`.
`OWN-GRAM-REJECT0` in the parked implementation taskboard owns the exact
dual-parser fail-fast witness. Until it lands, do not use these lookalike
result forms and do not treat accidental parsing as ownership syntax.

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
register(share service) // register declares a `share` parameter
use(service)
```

Here `service` remains usable and the callee receives an independent Shared
owner. The explicit `share` expression is the point where shared-lifetime
bookkeeping is allowed.

The short law is:

```text
ordinary local name reuse:
  scoped alias; no new owner

existing owner transfer:
  explicit move; owner count unchanged

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

When an existing binding supplies that token to another owning local, owning
store, or consuming call, source uses the contextual `move` form:

```hako
local moved = move owner
adopt(move moved)
```

`move` forwards one token and leaves the source binding unavailable. It does
not create an independent owner and does not require reference-count traffic.
Returning a local owner is an inherently consuming terminal context, so
`return owner` does not repeat `move`.

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

### View omission is a typed ownership error

Decision: accepted target contract; production activation remains gated by
`GRAM-RESULT0` and the Anchored View taskboard.

Forgetting `view` does not silently retain the receiver field and does not
reach runtime as a leak:

```hako
get(): Node {
    return me.child
}
```

The unannotated result requires Owned, while `me.child` is anchored storage.
The ownership verifier rejects this mismatch. It is not a grammar error and it
must not be repaired by hidden Share promotion.

Required human diagnostic shape:

```text
cannot return `me.child` as Owned

source is anchored to receiver `me`
return type `Node` requires an independent owner

help:
  - use `: view Node` while the result may depend on `me`
  - when Shared acquisition is verified, use `: share Node` and an explicit
    Shared acquisition for independent lifetime
  - when field move-out is verified, use `move me.child` only when ownership
    should leave receiver storage
```

The Share alternative is emitted only when the exact field/result ABI carries
a Shared acquisition witness; plain View never silently promotes to Shared.
The `move` alternative is emitted only when the exact object-storage contract
admits moving out of that field. It means that subsequent use of the moved
storage is forbidden; this reference does not promise a particular empty-slot
representation.

The corresponding machine-readable reason is
`owned-return-from-anchored-value`, with this ordered remediation vocabulary:

```text
change_result_to_view
change_result_to_share_and_acquire
move_from_storage
```

The emitted fix list is a capability-filtered subsequence. It must never offer
Share acquisition or field move-out without the matching sealed witness.

This mismatch is a hard error, not a warning. An explicit `move` already
records user intent and receives no default warning. A future opt-in API-review
lint may inspect verified return provenance, but method names such as `get` or
`peek` are never warning or ownership authority.

Under the target contract, omission of `view` cannot itself create an
ownership leak. The principal specified leak hazards are strong Shared cycles
and explicit `unsafe raw` code. Compiler/runtime defects and plugin/FFI
ownership-contract mismatches remain separate verification concerns.

### Shared owner

Shared is the explicit independent-lifetime lane. `share expr` is the only
ordinary source operation that creates another owner for the same object
identity.

`share` preserves object identity and leaves its source owner usable. The
expression result is one additional, independently consumable Shared owner.
The physical plan depends on the sealed representation:

```text
Unique source:
  explicitly promote/rehome to Shared, then acquire the additional owner

already-Shared source:
  acquire one additional owner
```

This distinction belongs to a verified share-materialization plan. Lower and
the runtime must not guess it from a tag, pointer, or observed reference count.

For an owned root expression such as `share a`, the source law is exact:

1. `a` must be an eligible Unique owner with no live aliases/views;
2. `a` is rehomed as a Shared owner and remains usable;
3. the expression result is a second, independent Shared owner.

For an already-Shared root, the same expression law holds without rehome:
the source remains usable and the result is one additional Shared owner.

For a fresh rvalue such as `share make_service()`, the temporary source owner
has no later source use, so only the resulting Shared owner remains observable.
The optimizer may remove redundant ownership traffic, but it may not change
these availability semantics.

The first profile rejects `share` applied to a scoped-alias/view operand, a
weak value, a trivial value, or an unsupported/unknown representation. It
accepts both eligible Unique and already-Shared owners because `share` always
marks the exact source site where owner count may increase.

Once a value is in the Shared lane, `DestroyOwned` consumes an owner at its
verified terminal site. A new Shared owner still requires an explicit `share`;
ordinary assignment does not silently insert `CopyOwned`. Every production
`CopyOwned` must be traceable to an exact `share` site or to a separately
verified boundary operation with equally explicit ownership ABI.

### Copy and clone are outside ownership syntax

`value.copy()` is an ordinary call returning an Owned result. It may be the
conventional spelling of a type-specific semantic copy, but the compiler does
not infer fresh identity, deep-copy behavior, or `noalias` merely from the
method name. A future verified `Copyable` protocol may provide a
`FreshIdentityWitness`; that is a separate feature row.

`clone` has no language-level ownership meaning. A user method named `clone`
is an ordinary method. Compilers and optimizers must not interpret it as
sharing, copying, retaining, or producing fresh identity. A style lint may
recommend `share value` or `value.copy()`, but it is not semantic authority.

## 3. Source defaults

| Position | Default contract | Extra ordinary spelling |
| --- | --- | --- |
| owned rvalue (`new`, Owned call) | one owner | none |
| `local b = a`, eligible whole root | Scoped alias | none |
| ordinary parameter / receiver | mutable noescape alias | none |
| existing owner -> owning local/store/call | owner forward | `move` |
| ordinary return | Owned terminal forward | none |
| independent owner, same identity | Shared acquire | `share` |

Hakorune does not require `owned`, `borrow`, or `clone` annotations. `move` and
`share` appear only at the two operations that change owner availability:
one-owner transfer and same-identity owner addition. Ownership mode is not
inferred from a runtime tag or reference count; it comes from the resolved
source site and callable ABI.

Known primitive and record values keep their ordinary structural value
semantics; reusing the same SSA value needs no owner token, and `local b = a`
is not reclassified as a Box loan for them.

An ordinary owning field/store consumes or forwards one owner. An existing
binding therefore uses `move source`; a fresh Owned rvalue can flow directly.
If the source binding must remain usable afterward, compilation fails and
points to the `share` boundary. The compiler does not silently retain the
object.

Dynamic or unknown representation never chooses alias/copy/move at runtime.
The first ownership-bearing `Any` profile rejects before Builder effects. A
later uniform-representation row may activate source-driven aliases, but it
cannot change the runtime-inference prohibition.

## 4. Callable ownership ABI

Callable declarations have a value type and an independent ownership
contract. The semantic vocabulary is:

| Position | Default | Non-default API contracts |
| --- | --- | --- |
| parameter / receiver | noescape alias | `move`, `share` |
| result | Owned (or Trivial after type resolution) | `view`, `share` |

- `move` parameter: the callee receives the caller's one owner. An existing
  caller binding must use `move actual`; a fresh Owned rvalue may flow directly.
- `share` parameter: the callee receives an independent Shared owner.
- `view` result: the result is anchored and non-owning.
- `share` result: the caller receives an independent Shared owner.

The conversion matrix is exact:

```text
Owned binding -> move parameter:
  call site uses `move actual`; owner is consumed after the no-live-loan check

fresh Owned rvalue -> move parameter:
  normal expression spelling; its temporary owner is forwarded

Unique actual -> share parameter:
  implicit conversion forbidden; caller supplies `share actual`

Shared actual -> share parameter:
  caller supplies `share actual` when the original owner remains available,
  or `move actual` to transfer an existing Shared owner

ScopedAlias / View actual -> share parameter:
  reject; end the loan and share the owner root, or use a Shared-returning API

Unique return in a `share` result function:
  implicit conversion forbidden; the return expression must cross an explicit
  `share` boundary

already-Shared owned local return in a `share` result function:
  compiler forwards the local token

borrowed/field-backed Shared return in a `share` result function:
  explicit `share source` is required to add the caller's owner
```

These modifiers are API-definition vocabulary, not line-by-line local
annotations. Their exact contextual-keyword grammar is activated separately.
They must not be encoded through method names or `@rune Ownership(...)`.

Example target signatures:

```hako
inspect(node: Node)                 // noescape alias parameter
adopt(move node: Node)              // consumes one owner
register(share service: Service)    // receives Shared ownership

make(): Node                        // Owned result
get(): view Node                    // receiver-anchored result
service(): share Service            // Shared result
```

Example call sites:

```hako
adopt(move owner)
register(share service)
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

Shared owner acquisition at `share`:
  verified RC or equivalent shared-lifetime bookkeeping may occur

Shared owner terminal drop:
  compiler-managed shared-lifetime bookkeeping may occur
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
2. `move` forwards one existing owner without RC; return is the implicit
   terminal form.
3. Only explicit `share` adds a same-identity independent owner.
4. Shared destruction is compiler-managed, but owner acquisition remains
   source-visible as `share`.
5. Ordinary results are Owned; free call-result aliases require a verified
   anchored `view` ABI.
6. No runtime tag, method name, map diff, or observed reference count decides
   ownership.
7. Binding SSA owns current values; Ownership SSA owns tokens; Loan Flow owns
   temporary permissions.
8. Uncertainty fails fast; it never becomes hidden RC, raw memory, or another
   profile retry.
9. `copy()` and `clone()` names are not ownership authority; verified ABI and
   witnesses, never method spelling, decide their compiler meaning.
