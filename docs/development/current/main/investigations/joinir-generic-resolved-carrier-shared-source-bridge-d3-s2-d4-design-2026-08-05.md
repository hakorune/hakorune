Status: Option 3 accepted; D4-WITNESS0 closed; next route migration is design-only
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-family-overlap-census-d3-s2-p3-task-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-DESIGN0-D3-S2-D4`
Decision: accepted Option 3; first executable slice is cfg(test)-only source-window witness

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

The worker premise audit is complete and the user accepted Option 3 for
taskization. This accepts only the source-window ownership design and selects
the bounded `D4-WITNESS0` execution contract below. Exact family disjointness,
winner precedence, and every production caller remain closed.

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
  closed: Option 3 and its sole owner/receipt contract are accepted

D4-WITNESS0 (cfg(test)-only)
  issue one non-Clone resolver-owned window; lend paired raw/resolved views;
  exercise the bounded observed matrix below; production caller=0

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

The first executable slice after the accepted D4-DESIGN-ACCEPT0 is only the
`cfg(test)` witness. It must not add a production import, a shared family
classifier, a selector, a Recipe/key issuer, or a Builder/MIR caller.

# Selected execution task: D4-WITNESS0

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-WITNESS0-D3-S2-D4-S0`

## Structural placement

Create exactly one new source/check file:

```text
src/mir/shared_loop_source_window.rs
```

Register it privately from `src/mir/mod.rs` with only:

```rust
#[cfg(test)]
mod shared_loop_source_window;
```

Do not place it under `resolved_semantics`: that layer currently has no
dependency on `compiler`, while the canonical source unit is owned there, so
the placement would create a reverse dependency. Do not place it under
`compiler`: that would make the compiler product look like the neutral bridge
owner and `src/mir/compiler/mod.rs` is already near the 800-line boundary.
The private MIR-root test seam may borrow both layers without adding a
production dependency or facade export.

The new file targets 180-300 lines and must remain below 800. Add no new shell
guard and no new investigation file. This existing card is the task receipt.

## Sole receipt API

The implementation shape is:

```text
VerifiedResolvedSourceUnitV1
  + exact LocatedStmtV1 for one Loop
  -> issue_shared_loop_source_window_v1(...)
  -> VerifiedSharedLoopSourceWindowV1<'a>       // non-Clone, non-Copy
  -> with_views(self, |raw_view, resolved_view| ...)
```

The receipt borrows the canonical source unit, owns the non-Clone resolved
loop forest, and retains resolver-issued owner/origin/source-kind/site/frame.
The raw view borrows the original `Loop` condition and body from the exact
located statement; it does not clone, flatten, rewrite, or reconstruct AST.
The resolved view carries the same branded owner/site/frame and forest.

`with_views` consumes the sole receipt and lends both views in one closure.
There are no public/test constructors for either view, no two independently
constructible receipts, no `Clone`/`Copy`, and no root facade re-export.

Reuse the existing canonical owners and navigation only:

```text
VerifiedResolvedSourceUnitV1::resolve_function
ResolvedFunctionLoweringInputV1 / root function input
FunctionSourceViewV1::{root_body,body_stmt}
LocatedStmtV1
VerifiedResolvedFunctionV1::{resolved_loop_source,
  resolved_loop_source_forest}
VerifiedResolvedLoopSourceV1::frame_key
```

Do not call `try_build_loop_facts`, `CanonicalLoweringPreflightV1`, registry
selection, or any Generic/NestedPredicate/DirectAccum/A+ classifier in this
slice. Those consumers belong to later route migration/family-boundary work.

## Executed witness matrix

The first slice must execute and assert only these rows:

```text
positive canonical nested source:
  reuse crate::mir::compiler::nested_function_for_p3_test()
  root body item 1 is the exact outer Loop; item 0 is the non-Loop negative
  DeclaredFunction owner/origin/source-kind
  exact root body + exact outer Loop site
  resolver forest root and frame match
  raw/resolved views report the same branded owner/site/frame
  raw view borrows the original condition/body

foreign source unit / located statement:
  reject before view publication

non-Loop located statement:
  reject before view publication

same-shape source from a second resolver invocation:
  distinct owner/source identity; no name/AST/coordinate pairing

forest empty/root/site or frame mismatch when representable through existing
typed test ingress:
  typed reject before view publication
```

Use one reject enum owned by the witness and preserve the nearest existing
resolver/navigation reason. Suggested stable categories are `ForeignOwner`,
`NotLoop`, `SourceNavigation`, `SourceLookup`, `SourceForest`, `ForestEmpty`,
`ForestRootMismatch`, `FrameMismatch`, and `UnsupportedSourceKind`. Do not add
generic catch-all or map a reject into another family.

## Explicitly deferred matrix

The following are recorded as `Deferred` or `NoStandaloneRow`, not claimed as
executed coverage in D4-WITNESS0:

```text
external duplicate/overlapping windows (constructor is intentionally absent)
ScopeBox flatten lineage
raw name -> BindingRef projection and shadowing policy
Lambda/upvar/capture/escape windows
synthetic, transferred, or opaque subtrees
raw Generic facts/schedule and resolved family classification
exact disjointness, precedence, selector, Recipe/key, Binding SSA/ValueId/PHI
Builder, MIR, Return, ABI, Home, debt, retry, fallback, runtime
```

Existing resolver/source guards may be cited as supporting evidence, but they
do not become D4 witness coverage until the witness itself consumes the
corresponding canonical input.

## Acceptance and gates

```text
RUSTFLAGS='-Awarnings' cargo test --lib shared_loop_source_window -- --nocapture
cargo build --lib
RUSTFLAGS='-Awarnings' cargo test --lib generic_d3_s2_p -- --nocapture
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Static caller census must show bridge names only in the new file and the
private `#[cfg(test)]` module declaration. Production import/caller/export is
zero. `src/mir/compiler/mod.rs` stays below 800 lines, every changed/new
source/check file stays below 800, and the active workstream stays below 1000.

The implementation/test commit must update, in the same commit:

```text
this D4 receipt and CURRENT_STATE.toml
docs/reference/mir/generic-loop-stage-matrix.md
src/mir/resolved_semantics/README.md
src/mir/loop_structural_facts/README.md
src/mir/builder/control_flow/plan/generic_loop/README.md
the active workstream by compact replacement, not append
docs/reference/mir/metadata-facts-ssot.md only if its source/facts contract changes
```

The closeout must distinguish every `ObservedReject`, `Deferred`, and
`NoStandaloneRow`; it must not say that D4 proves classifier disjointness or
authorizes production migration.

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

# Forbidden beyond D4-WITNESS0

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

# D4-WITNESS0 closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-WITNESS0-D3-S2-D4-S0`
is closed as a private `#[cfg(test)]` witness in
`src/mir/shared_loop_source_window.rs`. The MIR root declares the module, and
the module owns one non-`Clone` receipt whose consuming `with_views` call lends
paired raw/resolved views. Four focused tests cover the nested-loop positive,
foreign owner, non-loop, and equal-shape distinct-session rejects. The focused
bridge suite, P3 regression suite, and `cargo build --lib` are green; no
production import/caller/export was added and no classifier disjointness is
claimed. The new source stays below 800 lines and the compiler/workstream
limits remain green.

The same implementation commit updates the reference matrix and the resolved,
facts, and Generic-loop README boundaries. The bridge remains transport-only;
facts, preflight, selector, Recipe, Builder/MIR, retry/fallback, runtime, and
production authority remain closed.

# Next design-only task: DirectAccum canonical route

The next safe row is
`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-ROUTE-MIGRATION-DIRECT-ACCUM-DESIGN0-D4-S1`.
It is design/test-only and must first decide the route matrix, not edit
production code. The recommended first consumer is the existing resolved
DirectAccum source probe at the `CanonicalLoweringPreflightV1::verify` seam.
The resolver-owned `VerifiedResolvedSourceUnitV1` / `FunctionSourceViewV1`,
loop forest, source kind, owner, and frame remain the sole identity authority;
the D4 receipt is only a borrowed transport view. The route must preserve one
DirectAccum ingress, keep NestedPredicate-before-DirectAccum precedence, and
leave Generic/A+ and the raw `LoopRouteContext` edge untouched.

The design card must specify exact DirectAccum envelope membership, the adapter
seam, old-edge retirement, typed pre-effect rejects, and the counterexample
where `NotCandidate` is ordinary fallback rather than Generic retry. No
selector, Generic classifier, Registry, Recipe, Binding SSA/PHI, Builder/MIR,
runtime, or production caller is authorized by this row. Any later migration
commit must update `docs/reference/**` and all affected current support pages
in that same commit.

# Current next action

Stop at the design boundary above. Do not begin production route migration
until the DirectAccum route matrix is accepted. Exact family disjointness and
cross-family selection remain `NoSafeSlice`.
