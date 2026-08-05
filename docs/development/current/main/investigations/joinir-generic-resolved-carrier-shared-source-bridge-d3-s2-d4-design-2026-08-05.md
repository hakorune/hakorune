Status: design brief ready; implementation stopped at policy boundary
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-family-overlap-census-d3-s2-p3-task-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-DESIGN0-D3-S2-D4`
Decision: provisional Option 3 recommended; no shared classifier or production selection

# Design consultation stop

P3 is closed as independent evidence. It confirms that raw Generic
`LoopRouteContext` observations and resolved NestedPredicate/DirectAccum/A+
preflight observations do not currently share a FunctionOwnerId, source
forest, `BindingRefV1`, frame, or invocation brand. Exact family disjointness
therefore cannot be implemented by pairing fixture labels, AST names, route
IDs, or coordinates.

This row decides whether a common source bridge is warranted and, if so, which
existing owner should provide it. It is a design task only; no source or
production selection implementation is authorized yet.

The worker premise audit is complete. The design is now sufficiently bounded
to task the next review, but it is not an implementation approval. Exact family
disjointness, winner precedence, and any production caller remain closed until
this bridge design is explicitly accepted.

# Worker design brief

## Semantic units and authority

The two current products do not observe the same semantic unit:

```text
raw Generic:
  one loop fragment (condition, body) from LoopRouteContext
  -> try_build_loop_facts
  -> raw registry schedule

resolved families:
  one whole VerifiedResolvedSourceUnitV1
  -> canonical AST + resolver forest/projection
  -> exact root/loop site and LoopExecutionFrameKeyV1
```

`ScopeBox` flattening is shape-only. It is not a source identity, and neither
`func_name`, route IDs, raw indices, schedules, fixture names, AST names, nor
equal coordinates may be used to pair the products. The resolver remains the
sole authority for function owner, source origin/kind, exact source sites,
forest topology, frame, and `BindingRefV1` relations.

The raw Generic classifier remains observation-only and may not mint a
resolver owner, source site, frame, Recipe key, selector input, or physical
identity. The resolved classifier remains observation-only and may not turn a
shared view into a winner, Recipe, `LoopBindingKeyV1`, `ValueId`, PHI, Builder,
MIR, Return, ABI, Home, debt, retry, fallback, or runtime authority.

## Current classifier arms that the bridge must cover

```text
raw Generic:
  V0/V1 NumericProgression and BodyManagedState
  CompleteNoRecursive / CompleteRecursive / Unavailable / Ambiguous carrier
  Release / Strict overlap observations
  planner-required V0 suppression

resolved:
  NestedPredicate (root + child forest)
  DirectAccum (same envelope, distinct source/policy/effect contract)
  trivial Binding-SSA straight-line family
  A+ whole-owner fallback after trivial rejection
  typed canonical rejects for root/header/metadata/signature/body/return/
  owner/upvar/unsupported-expression/control/source-navigation failures
```

## Recommended bridge shape: Option 3

Introduce one neutral, resolver-owned source window below both products:

```text
resolver-owned canonical source unit
  -> one non-Clone VerifiedSharedLoopSourceWindow<'a>
  -> borrowed raw Generic view + borrowed resolved-family view
```

The window must carry, or lend from its owner, at least:

```text
FunctionOwnerIdV1
FunctionOriginV1 / source kind
exact root body site
exact loop site / loop forest
LoopExecutionFrameKeyV1
```

Do not construct two independently pairable receipts. Prefer a consuming
`with_views(|raw_view, resolved_view| ...)` API, or an atomic
`VerifiedSharedSourcePair` that lends both views from the same resolver-owned
unit. The views are lifetime-bounded, borrowed, branded analysis views; they
cannot clone or mint source identity. If pipeline lifetimes cannot support one
owner, the pair is unrepresentable and the row remains `NoSafeSlice`.

The bridge owns only source identity and view validity. It adds no family
policy, selector, Recipe, BindingKey, physical SSA, lowering, or runtime
meaning. Its retirement edge is the later canonical route migration: raw and
resolved classifiers consume the branded views, then the bridge disappears
from downstream products once one canonical family boundary is sealed.

## Candidate disposition

```text
Option 1 — keep families separate:
  safe now; exact disjointness remains a permanent non-claim.

Option 2 — extend the resolver session into the raw route:
  rejected; couples a legacy route to resolver lifetime and risks a second
  raw/resolver authority.

Option 3 — neutral resolver-owned source-view bridge:
  recommended; one owner, one receipt, borrowed paired views, typed rejects.
```

# Typed fail-fast boundary

The bridge rejects before any classifier effect for:

```text
missing / foreign / ambiguous source unit
owner, source-kind, frame, site, or forest mismatch
window out-of-bounds, overlap, duplicate, or wrong root
ScopeBox projection mismatch
foreign BindingRef, upvar, capture, or lambda escape
transferred, opaque, synthetic, or unsupported subtree
missing resolved root/loop site or frame
```

These rejects are not fallback, retry, precedence, or evidence for a
different family. They must be typed and source-located. A source pair with
identical `(condition, body, func_name)` from two functions or compilation
sessions is a required counterexample: raw facts can compare equal while
resolver owners and source roots differ, so fragment/name pairing is invalid.

# Ordered task ladder

```text
D4-DESIGN-ACCEPT0
  accept/reject the Option 3 bridge and its sole owner/receipt contract

D4-WITNESS0 (cfg(test)-only)
  issue one non-Clone resolver-owned window; lend paired raw/resolved views;
  exercise positive identity and every typed reject above; production caller=0

D4-CANONICAL-ROUTE-MIGRATION0 (future, design-gated)
  migrate one canonical source route to the bridge; no family winner yet

D4-FAMILY-BOUNDARY0 (future)
  only after source identity is shared, define disjointness/overlap policy
  without selector, Recipe, Builder, MIR, Return, ABI, Home, debt, retry,
  fallback, or runtime claims

D4-REFERENCE-CLOSEOUT0 (same commit as any future implementation)
  update docs/reference/**, current support pages, task receipt, and focused
  gates; record exact reject boundary and remove stale design wording
```

The first executable slice, if D4-DESIGN-ACCEPT0 is accepted, is only the
`cfg(test)` witness. It must not add a production import, a shared family
classifier, a selector, a Recipe/key issuer, or a Builder/MIR caller.

# Source authority

The consultation must inspect the complete producer/classifier arms for:

```text
raw Generic: LoopRouteContext -> try_build_loop_facts -> registry schedule
resolved families: VerifiedResolvedSourceUnitV1 -> CanonicalLoweringPreflightV1
```

The design brief must name the semantic unit (whole function or loop
fragment), exact body/window membership, function/source owner identity,
execution frame, and the treatment of transferred or opaque subtrees. It must
also account for every current NestedPredicate, DirectAccum, A+, Generic,
trivial-profile, and canonical-rejection arm rather than inferring policy from
type names or one green fixture.

# Non-authority

The following are reporting evidence only and may not become a bridge key or
classifier input:

```text
fixture labels, source strings, AST names, route IDs, raw schedules
raw frame coordinates without an owner brand
plan digests, stage/debt traces, ValueId/PHI, Recipe keys, selector outcomes
```

The P3 census remains an independent report. Its
`UnresolvedStop(FamilyOverlap)` must not be reinterpreted as a winner or as a
proof that two observations describe the same source unit.

# Candidate bridge shapes

The consultation may compare, but must not implement, these bounded choices:

1. keep the families permanently separate and make disjointness a policy
   non-claim;
2. extend the existing resolver session to issue one owner/frame/source
   receipt consumed by both raw Generic observation and resolved preflight;
3. introduce a neutral source-view bridge owned below both products, with no
   route selection or physical identity.

For every candidate, record the new authority it creates, the old authority it
retires, the exact typed reject boundary, and whether a whole-function source
caller can consume one non-`Clone` receipt without loose pairing. If no
candidate satisfies those conditions, retain NoSafeSlice and do not add a
fourth census.

# Forbidden until design acceptance

```text
shared Generic/NestedPredicate/DirectAccum/A+ classifier
exact disjointness proof or winner precedence as a semantic contract
co-sealed cross-family capability without one branded source receipt
selector, eligibility, Recipe, LoopBindingKeyV1, BindingRef/ValueId/PHI
Builder, MIR, Return, ABI, Home, debt, retry, fallback, runtime, production
caller, or source AST reconstruction
```

# Acceptance

- A compact design brief names source authority, non-authority, semantic unit,
  all classifier arms, fail-fast mismatch reasons, and one counterexample.
- The recommended bridge shape either has one clear owner and retirement edge
  or explicitly rejects a shared bridge as NoSafeSlice.
- The smallest future implementation slice is named, including its
  test-only/production status and no-claim boundary.
- No implementation starts from this row until the design decision is
  accepted. Any later implementation/test landing must update affected
  `docs/reference/**` and current support pages in the same commit.
- Source/check files and the workstream remain below 800/1000 lines.

# Current next action

Worker premise audit is complete and the design brief above is ready. Exact
disjointness is not proven. Read this card with the parent D3 design and P3
census, then stop at `D4-DESIGN-ACCEPT0`; do not begin `D4-WITNESS0` until the
bridge owner, one-receipt API, and typed reject boundary are accepted.
