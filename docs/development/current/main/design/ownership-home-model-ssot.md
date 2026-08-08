# Ownership Home Model — cross-layer SSOT

Status: Durable cross-workstream design boundary; production activation 0

Decision:

- accepted: ownership is attached to a **Home** (storage/place), not exposed as
  a Rust-like source type;
- accepted: ordinary source uses a non-owning handle and only an explicit
  `share` operation may add an independent owner;
- accepted: statement-only contextual `release root` is the sole explicit
  early-end spelling for one verified whole-root Home; `release(value)` is an
  ordinary call and `drop` is not an alias;
- accepted: C′ terminal Home release is the sole user-`fini` authority;
  `fini {}` is a non-callable Box hook and direct `obj.fini()` is rejected;
- provisional: the smallest HomeV1 source capsule is declaration-side
  `take` parameters, contract-boundary `T from owner`, and expression-side
  `share`;
- unresolved: composite/generic classification, Shared representation,
  owning storage destinations, and exact C-speed physical ABI;
- inactive: parser, resolver, Home Flow, Builder, runtime, and backend
  production changes.

Parent current workstream:
`docs/development/current/main/workstreams/language-v1-convergence-current.md`

Source-language authority:
`docs/reference/language/ownership.md`

Parked execution board:
`../investigations/hakorune-home-ownership-task-2026-08-04.md`

Reference closeout:
`OWN-HOME-REFERENCE-CLOSEOUT0-DOC0` is mandatory after the first production
Home slice and before Home product readiness/cutover is declared complete.
It includes C′ lifecycle grammar, terminal-release, field-order, direct-call
retirement, and implementation-witness synchronization.
The reference page may not claim parser-live grammar, ABI, or physical
performance that the selected production route has not actually proved.

Related lifecycle authority:
`constructor-birth-new-lifecycle-ssot.md`

Terminal finalization authority:
`box-lifecycle-cprime-terminal-home-finalization-ssot.md`

Box member source prerequisite:
`box-member-field-method-surface-ssot.md`

The Property retirement owned there must make `obj.x` one unambiguous stored
place and `obj.x()` one ordinary call before Home Flow production adoption.

The constructor document owns `new`/`birth`/field-initializer ordering and
the direct-`birth` ban. `OWN-HOME-BIRTH-D0` on the parked board owns the
remaining Home-specific construction transaction questions, especially
partial-construction failure cleanup. Neither document silently replaces the
other.

Current-lane rule: this document does not move the row named by
`CURRENT_STATE.toml`. Ownership resumes only after the MirBuilder final-pipeline
checkpoint named there.

## Purpose

This page fixes the layer boundaries that must survive future implementation.
It does not select exact grammar or physical representation before their
design rows close.

The short model is:

```text
Home slot/place
  storage/place that can hold one lifetime-supporting Home token

Home token
  one independent lifetime claim for one object identity

Handle
  a non-owning way to observe the object supported by a Home

HomeValue
  a transferable owner token before it is installed in a destination Home

share
  the only ordinary source operation that may add an independent owner

release
  the contextual statement that ends one verified whole-root Home
```

### `new` and `birth` Home boundary

At the source level, `new` is the primary creation boundary for a fresh
object identity and its first candidate Home token. The successful lifecycle
order is fixed by the constructor SSOT:

```text
new
  -> declaration-site field initializers
  -> matching birth(args...)
  -> optional explicit field overrides
  -> publish as usable
```

`birth` is a constructor hook, not a second ownership verb and not a direct
receiver method. Its parameters use ordinary Handle/Trivial rules unless an
explicit resolved Home-demand declaration says otherwise. A constructing
`me` is unpublished; it cannot escape through storage, return, callback, or
`share` before publication. The exact cleanup/no-double-finalization rule for
field or birth failure is intentionally delegated to
`OWN-HOME-BIRTH-D0`; an unpublished outer object does not run the C′ `fini`
hook merely because construction rollback begins.

`new` and `share` therefore have different authority: `new` creates the first
Home candidate for a fresh identity, while `share` adds an independent owner
to an already existing identity.

`Home` is a compiler concept, not a planned source keyword. One Shared object
identity may have several independent Home tokens installed in different Home
slots. Object identity, Home token, Home slot, source handle, and runtime
`ObjectHandle` are never interchangeable authorities.

## Authority map

```text
source + resolved types
  -> Home taxonomy / composite classifier
  -> destination demand + callable Home ABI
  -> Home Flow over the existing CFG and Binding SSA identities
  -> verified Home plan
  -> Ownership SSA / physical ownership plan
  -> ObjectStoragePlan / backend lowering
```

### Source and resolver

Own:

- contextual ownership syntax;
- source binding/place identity;
- resolved value and storage types;
- ClosedCallable versus ContractBoundary classification.

Do not own:

- RC insertion;
- object layout;
- runtime tag based ownership selection;
- backend storage strategy.

### Home taxonomy and composite classifier

Own:

- `Trivial`, `Unique`, `Shared`, `Weak`, and `Unknown` value capability;
- whether a record/enum/container payload carries an owner;
- whether a destination is an owning Home, a handle-only input, weak storage,
  or unresolved;
- exact failure on generic/`Any`/recursive shapes that lack a sealed rule.

Do not call every record Trivial. Identity-free structure can still contain a
Box owner. `Option<Box>`, `Result<Box, E>`, and Box-bearing enum variants are
owner-bearing until a recursive classifier proves otherwise.

### Callable Home ABI

One sealed product is the only call-site authority:

```text
VerifiedHomeAbi
  receiver demand
  parameter demands
  result relation
  profile/schema identity
  source/declaration provenance
```

The callable D0 also decides whether this ABI is part of callable identity,
overload uniqueness, interface compatibility, and cache keys. Method spelling
alone is never enough.

The callable Home ABI design stop is now closed by
[`OWN-HOME-CALLABLE-ABI-D0`](../investigations/own-home-callable-abi-d0-design-task-2026-08-09.md).
The next implementation sequence consumes the landed resolver
`VerifiedInstanceMethodDeclarationCatalogV1` together with an explicit,
same-brand resolver Home-capability classification environment. One canonical
issuer returns one non-`Clone` declared catalog containing one exact
`VerifiedHomeAbi` row per declaration. The Home issuer owns no Query behavior,
body conformance, target, Recipe, physical ABI, or runtime meaning; it must not
infer capability from method names, body shape, `MirType`, or backend layout.
The first bounded row classifies an ordinary instance receiver as `Handle` and
semantic `I64`/`Unit` values as `Trivial` only through the classifier's explicit
receipt. Unknown/generic/composite capability remains `Unresolved`.

ClosedCallable bodies may infer a candidate result relation and local Home
Flow. They may not invent parameter/receiver Home demand: a plain parameter is
Handle, and only the resolved declaration supplies a consuming demand. Body
analysis verifies that declaration. ContractBoundary callables must declare or
import the exact ABI. A body does not make an exported API implicit.

ContractBoundary includes at least:

- exported or separately compiled callable;
- interface/dynamic dispatch implementation;
- callback or function value crossing an opaque boundary;
- plugin/FFI/extern declaration;
- unresolved generic callable;
- any callable whose body is unavailable to the current verifier.

Recursive SCCs, generics, and body-known function values are not automatically
ClosedCallable. Their admission requires an exact row or they remain a
ContractBoundary/rejection.

No caller reopens a body, method name, runtime tag, or old profile to infer
ownership.

### Binding SSA versus Home Flow

Binding SSA remains the sole `BindingRef -> current ValueId` authority.

Home Flow owns a separate state:

```text
Available(HomeRoot)
Consumed(at source site)
MaybeConsumed(branch provenance)
Unknown(reason)
```

It consumes Binding SSA identities and CFG edges; it must not become a second
reaching-value map.

Loop legality is a data-flow property, not a syntax ban. A consume that reaches
a backedge without a fresh replacement Home is rejected. A loop-local fresh
Home, consume-followed-by-break, or consume-then-replenish may be admitted by
later exact rows.

### Physical representation

The semantic Home product does not choose `Arc`, RC, raw pointer, arena, or
object layout. Physical owners remain:

```text
semantic Home plan
  -> Ownership SSA / semantic refresh
  -> RoutePlan
  -> ObjectStoragePlan
  -> backend/runtime
```

`shared box` and per-instance Unique-to-Shared promotion are competing
language/representation choices. Neither is selected by this page.

C-like performance is not established by grammar. It requires a verified
Unique physical route with no RC/control-cell/registry work on the measured
hot path, plus exact-front and whole-program perf/assembly evidence.

## Accepted source-direction laws

These laws are durable even while the exact grammar remains provisional:

1. `local b = a` does not add or transfer a Home; `b` is a handle.
2. An owning destination may consume exactly one available Home.
3. The destination demand, not the caller's guess, is transfer authority.
4. A terminal `return` may forward an available Home without a second
   ownership verb.
5. Only an explicit `share` source site may add a same-identity independent
   owner.
6. No hidden retain, promotion, raw fallback, or profile retry repairs a
   failed Home proof.
7. `fini {}` is an optional non-callable hook in the one terminal Home
   DropPlan. It is not a transfer spelling, direct method, or physical-free
   API.
8. `weak` remains a generation-aware non-owner and is not a normal handle or
   Shared owner.
9. Home transfer has one sealed commit point after argument preparation; a
   failed preparation does not leave a caller Home half-consumed.
10. Terminal parent finalization runs the parent hook before releasing verified
    owning fields in reverse declaration order. A child hook runs only if that
    field release is the child's terminal Home release.
11. `release root` consumes one exact available whole-root Home at its source
    point. It never force-finalizes a Shared identity or silently re-roots
    dependent handles.

## Terminal Home finalization boundary

C′ explicitly supersedes the earlier B′ separation between eager
`FinalizeObject` and ownership destruction:

```text
ordinary handle end = no owner effect
take/call/return = atomic Home forward; fini 0 during transfer
share = one explicit independent Home acquisition
release root = one explicit whole-root Home end; hook only if terminal
terminal Home release = hook -> reverse field release -> structural drop
```

The exit transaction owns lexical cleanup and local Home release requests. The
object lifecycle descriptor owns hook dispatch and field/native teardown.
Neither may reconstruct the other's order from runtime counts or AST shape.

`close()`/`shutdown()` are ordinary optional domain methods for explicit,
possibly fallible early shutdown. They are not language keywords and never
become an alternate Home or finalization authority.

The exact classification of a plain Box field as owning, handle-only, weak, or
rejected remains `OWN-FIELD-CONTAINER-DEST-D0`. C′ applies to a field only after
that field has a verified owning Home relation; it does not silently classify
every current Box field as owning.

## Provisional HomeV1 surface

Subject to the D0 rows in the taskboard, the smallest candidate is:

```hako
adopt(take node: Node)       // declaration owns a Home demand
getRoot(): Node from me      // opaque result relation, when required
adopt(node)                  // destination consumes node's Home
adopt(share node)            // if the ABI admits Shared; node remains
release node                 // end one verified whole-root Home now
```

`release root` is a statement-only contextual keyword with one identifier
root. Its AST/source carrier is not authority: one resolved whole-root place,
Home Flow, and a sealed explicit-release plan own the meaning. The first
profile accepts only an owning local or owning parameter with exactly one
Home. Ordinary/generic `release(value)` calls, owner-bearing composites,
fields, projections, containers, trivial values, and unknown capability do not
gain Home meaning; the unsupported Home forms reject before effects.

The following remain parked and are not silently promised:

- `take place_expr`;
- field move-out and empty-slot/replacement semantics;
- consuming receiver syntax;
- multi-anchor result joins and result PHIs;
- capture, suspension, cross-thread Home flow;
- explicit source `region`;
- general `owned T`, `view T`, `shared T`, `move`, or receiver-mode syntax.

The first profile also rejects borrowed results rooted in temporary receivers
unless a distinct lifetime-extension contract is later accepted.

The old `move/share/view` target remains historical input only after the Home
source SSOT is updated. It must not coexist as a second accepted surface.

## Required rejection matrix

The first verifier must distinguish at least:

| Source | Destination | Result |
| --- | --- | --- |
| available Home | Home-demand parameter/store/return | consume/forward |
| handle alias | Home-demand destination | reject with root provenance |
| fresh Home rvalue | handle-only parameter | scoped temporary; exact lifetime proof required |
| explicit `share` | handle-only parameter | reject redundant paid owner |
| explicit `share` | eligible Shared-demand destination | acquire/materialize by sealed plan |
| explicit `share` | general Home-demand destination | representation/type D0 decides compatibility |
| whole-root Home | canonical `release root` | consume now; terminal enters the sole C′ DropPlan |
| handle alias | canonical `release alias` | reject and identify supporting root |
| trivial value | ownership operation | reject as meaningless |
| unknown/generic representation | any ownership-changing edge | fail before Builder effects |

This table is incomplete until the storage and composite D0 rows close. It is
therefore a required task input, not production permission.

## Diagnostics contract

Diagnostics consume typed provenance from Home Flow and the ABI verifier. They
must name the source Home, consuming edge, conflicting branch/backedge, and a
capability-valid repair.

Stable reason families include:

- `home-unavailable-after-transfer`;
- `home-maybe-consumed-after-branch`;
- `home-consumed-on-loop-backedge`;
- `home-demand-received-handle`;
- `redundant-share-to-handle`;
- `home-result-relation-conflict`;
- `home-abi-missing-at-contract-boundary`;
- `home-capability-unknown`;
- `home-release-received-handle`;
- `home-release-conflicts-with-cleanup-capture`.

Do not emit a generic “ownership inference failed” when the verifier has more
specific provenance.

## Hard stops

- Do not activate grammar before all Home taskboard Milestones 2–4 close:
  taxonomy and every semantic D0, surface convergence, passive relation/ABI,
  straight/CFG Home Flow, argument matrix, and typed diagnostics.
- Do not infer public/opaque ABI from a body.
- Do not infer ownership from method names, runtime tags, reference counts, or
  a backend layout.
- Do not recognize `release` or `drop` by MirBuilder/backend string matching;
  the parser carrier, resolved root, and sealed Home Flow plan are the staged
  authority.
- Do not activate Box-member `fini {}` before scope-position `fini {}` aliases
  are retired or context-separated by the accepted grammar row.
- Do not dispatch user `fini` from direct receiver calls, global finalizer
  registries, generic Rust Drop, or an unverified plugin/FFI root.
- Do not activate cross-thread terminal finalization before an exact affinity
  and atomic-winner contract exists.
- Do not merge Binding SSA and Home Flow authority.
- Do not implement rollback or hidden RC as a verifier fallback.
- Do not claim C-like speed before the physical Unique route is measured.
- Do not move the current MirBuilder execution row from this parked program.

## Readiness definition

The callable-contract lane reuses this model rather than inventing a second
receiver capability. `CallableContract(query)` requires the ordinary receiver
`Handle` boundary: no Home transfer, addition, end, or escape. The Query
behavioral receipt does not issue or restate that axis. The same-declaration
`VerifiedHomeAbi` alone owns receiver/parameter demands and the result
relation, and the declared callable contract merely co-seals it with Query and
the semantic signature. Its declared behavior, body conformance, and physical
ABI are separate owners; see
`docs/reference/language/callable-contracts.md`.

HomeV1 is product-ready only when:

- one source profile produces one verified Home product;
- every production call consumes one sealed Home ABI;
- Home Flow is CFG-complete for the admitted grammar;
- ownership-changing destinations have exact witnesses;
- unsupported generic/storage/ABI cases fail before Builder effects;
- one admitted `release` source consumes one exact root Home synchronously,
  while `drop` alias, dependent-handle re-rooting, and generic guessing stay zero;
- Unique and Shared physical plans are explicit and backend-capability checked;
- one terminal DropPlan owns hook dispatch, reverse field release, and
  structural drop without a B′ fallback;
- the old SharedV1/`move-view` authority and all profile retry edges have a
  counted retirement plan.
