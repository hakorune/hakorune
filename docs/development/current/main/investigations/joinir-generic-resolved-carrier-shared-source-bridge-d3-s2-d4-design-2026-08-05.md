Status: Option 3 accepted; D4-WITNESS0/D4-S1-S0/D4-S2-S0/D4-S3-D0/D4-S3-S0/D4-S3-S1/D4-S3-S2/D4-S4-D0/D4-S4-S0/D4-S4-S0-D0 closed; D4-S4-S0-D0 selected
Date: 2026-08-06
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

# D4-S1 DirectAccum route design closeout

The worker audit accepts the route boundary and selects the next test-only
task:
`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-ROUTE-MIGRATION-DIRECT-ACCUM-DESIGN0-D4-S1`
is closed as design, and
`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-ROUTE-MIGRATION-DIRECT-ACCUM-WITNESS0-D4-S1-S0`
is selected.

The exact admission envelope is the existing
`probe_direct_accum_source_unit_v1` path inside
`CanonicalLoweringPreflightV1::verify`, after the unchanged
NestedPredicate probe:

```text
root static non-main FunctionDeclaration
forest owner_count = 1, no upvars/override
empty params, param_decls, uses, contracts, attrs, and return type
root body exactly [Local, Loop]
Local has two type-less integer-zero initializers
exact loop site is body[1] and owner-matched
loop body/condition/update/step/binding/frame/disjointness/completion
  satisfy the existing DirectAccum plan contract
```

`NotCandidate` is ordinary fallback only for a non-function root, a root body
whose length is not two, or a body whose first two statements are not
`Local/Loop`. Once that envelope is present, every header/local/loop/body,
source/frame, policy, prefix, demand, Recipe, completion, or effect-plan
failure is a typed terminal pre-effect reject. It never retries Generic, A+,
the registry, or a legacy AST route.

The sole authority remains
`VerifiedResolvedSourceUnitV1 -> ResolvedFunctionLoweringInputV1 /
FunctionSourceViewV1 -> resolved_loop_source/forest/frame`, while
`issue_direct_accum_plan_v1` remains the sole DirectAccum plan owner. The D4
receipt is transport only. The migration must retain exactly one ingress:
resolved unit -> DirectAccum probe -> DirectAccum plan ->
`CanonicalFirstFamilyPlanV1::Loop(DirectAccum)`. The raw
`LoopRouteContext -> try_build_loop_facts -> registry` edge is not retired by
this task.

## Selected D4-S1-S0 task

The cfg(test)-only witness may consume the existing DirectAccum fixture and
canonical unit/located loop, lend the D4 paired views from one receipt, and
assert that the exact DirectAccum envelope accepts while foreign/non-loop and
shape-negative rows reject before effects. It must not import the bridge into
production preflight or add a Generic/family classifier, selector,
Recipe/key, Builder/MIR caller, or route retirement. Production migration is
`NoSafeSlice` until a later family-boundary/selector decision resolves the
remaining Generic/NestedPredicate/DirectAccum/A+ overlap.

Any implementation/test commit must update affected `docs/reference/**` and
current support pages in the same commit, and keep changed source/check files
below 800 lines and the workstream below 1000.

# D4-S1-S0 closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-ROUTE-MIGRATION-DIRECT-ACCUM-WITNESS0-D4-S1-S0`
is closed as a test-only witness. It adds three focused tests to the private
D4 module: exact DirectAccum acceptance through the existing source-unit
probe, foreign/non-loop receipt rejects, and a one-body-statement shape that
passes source identity but is rejected by the existing DirectAccum probe before
any Builder effect. Together with the four D4-WITNESS0 tests, the bridge suite
has seven green tests. The implementation does not import the bridge into
production preflight and adds no classifier, selector, Recipe/key, Builder/MIR
caller, retry, fallback, or route retirement.

The same implementation commit updates the MIR reference matrix, resolved and
facts README boundaries, Generic-loop README, this card, and current support
pages. Changed source remains below 800 lines and the workstream remains below
1000 lines.

# D4-S2 family-boundary design closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-BOUNDARY-DESIGN0-D4-S2` is accepted,
with the worker-audited correction
`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-AUTHORITY-CORRECTION0-D4-S2-R1`.
The earlier wording placed policy after a Recipe-bearing canonical plan and
named builder `registry/selection.rs` as its future consumer. That order is
rejected: selection must precede Recipe production, and the builder registry is
legacy schedule/compatibility authority to retire rather than promote.

## Corrected authority pipeline

```text
resolver-owned shared source identity
  -> neutral AST-free family observations
  -> mir::loop_route_policy selection
  -> selected family Recipe producer
  -> Recipe verify / JoinSig seal
  -> CanonicalLoopFamilyPlanV1 / CanonicalFirstFamilyPlanV1
  -> lower
```

The durable owner map is:

```text
FunctionSemanticResolverSessionV1 /
VerifiedResolvedSourceUnitV1 / resolver views
  = sole owner of function/source identity, exact sites, forest, frame,
    BindingRef relations, and the non-Clone shared source window

future VerifiedLoopFamilyObservationSetV1
  = one resolver-branded, non-Clone candidate/disposition set plus exact
    Release/Strict/planner-required snapshot; no Recipe/key, raw schedule,
    winner, precedence, Builder, ValueId, or PHI

future CanonicalLoopFamilySelectionV1 in mir::loop_route_policy
  = sole policy owner; consumes the observation set exactly once and yields
    Selected | NoCandidate | typed Rejected/Unresolved

selected family Recipe producer
  = sole LoopBindingKeyV1 and Recipe/JoinSig/source-effect relation issuer

CanonicalFirstFamilyPlanV1
  = whole-function post-selection lowering envelope; it never selects a winner

builder registry/selection.rs and router
  = current legacy raw schedule/execution transport only; never canonical
    source identity, family policy, Recipe, or plan authority

Binding SSA / PHI
  = sole physical BindingRef -> ValueId/merge authority after verified plan
```

`NoCandidate` is valid only when a sealed observation proves that no Loop
family envelope exists. Only that result may continue to Trivial/A+ whole-unit
selection. Missing/foreign/ambiguous identity, site/forest/frame mismatch,
planner-required suppression without sealed policy, and raw/raw or
raw/resolved overlap remain typed pre-effect `Rejected/Unresolved`; they are
never absence. Once a family is selected, Recipe/verify/lower failure is
terminal: retry, re-selection, post-effect fallback, AST/name/route-ID
reconstruction, and hidden precedence are forbidden.

The future atomic production cutover must name one production selection
consumer and delete, in the same commit, the migrated portions of sequential
Nested-before-Direct preflight selection, raw `select_recipe_first_routes`,
schedule retry, Generic post-effect retry, and duplicate probe selectors.
Until that cutover, all existing production edges remain intact.

## Selected D4-S2-S0 task

Token:
`JOINIR-GENERIC-RESOLVED-CARRIER-LEGACY-SAME-SOURCE-CENSUS0-D4-S2-S0`

Change:
  Extend only the private `cfg(test)` shared-source-window seam with one
  non-exported report. Consume one paired receipt for DirectAccum and one for
  NestedPredicate under Release, Strict, and planner-required: six rows total.
  Record same owner/site/frame plus exact existing raw V0/V1 carrier and
  schedule observations and existing resolved-preflight result. Name every
  current-route field `legacy_*`; old authority deleted = none.

Contract:
  This is retirement inventory, not canonical evidence. It creates no
  reusable observation API, eligibility, winner, precedence, policy input,
  Recipe/key, production caller/export, Builder effect, retry, or fallback.
  A+/Trivial/canonical-reject and shadowing/Index/Program/CompoundAssignment/
  duplicate-write remain outside this bounded Loop-pair census.

Done:
  The six exact measured rows are frozen by focused tests; foreign/non-Loop
  receipt rejects stay terminal; production caller count and old-edge deletion
  remain zero. The implementation commit updates affected
  `docs/reference/**`, resolver/facts/Generic-loop support docs, this card, and
  `CURRENT_STATE.toml` in the same commit. All changed source/check files stay
  below 800 lines and the rolling workstream stays below 1000 lines.

Stop:
  Return to design before editing production if the census needs a public
  product, merges the two rows into a winner, treats a legacy schedule as
  canonical policy evidence, reaches Recipe/Builder, or cannot keep both views
  tied to one resolver-owned receipt.

## D4-S2-S0 closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-LEGACY-SAME-SOURCE-CENSUS0-D4-S2-S0` is
closed as a private `#[cfg(test)]` six-row retirement census. One
resolver-owned non-`Clone` receipt is consumed for each fixture/mode row:
`nested-predicate` and `direct-accum` × `Release`, `Strict`, and
`StrictPlannerRequired`. Every row records the resolver owner/site/frame and
only `legacy_*` raw facts status, V0/V1 presence, carrier summary, raw
schedule, and the existing resolved preflight family.

The exact measured values are stable across modes: nested-predicate is
`CompleteRecursive(["j", "sum"])` with legacy schedule
`[NestedLoopMinimal, GenericLoopV1]`; direct-accum is
`CompleteNoRecursive` with `[AccumConstLoop]`; all six rows are facts
`Available`, V0 absent, V1 present, and resolve respectively to
`NestedPredicate`/`DirectAccum`. The test also freezes the mode order and
owner/site/frame correspondence. This is retirement inventory only: no
selector, policy product, Recipe/key, Builder/MIR caller, retry, fallback, or
old-edge deletion was added.

The implementation commit updates the reference matrix, resolver/facts/
Generic-loop support docs, this card, `CURRENT_STATE.toml`, current mirrors,
and the workstream. The focused census test is green; changed source/check
files remain below 800 lines and the workstream remains below 1000.

## D4-S3-D0 design target (now closed)

The former next row was
`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-SELECTION-AUTHORITY0-D4-S3-D0`.
It must decide the neutral observation-set schema and the sole
`mir::loop_route_policy` selection authority from the measured rows. It may
not treat legacy schedules or current preflight order as canonical, and it
does not authorize production migration. Any later implementation must update
affected reference documentation in the same commit.

## D4-S3-D0 design closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-SELECTION-AUTHORITY0-D4-S3-D0`
is closed as a docs-only authority decision after the worker audits. The
existing `src/mir/loop_route_policy` API remains the M3 legacy 19-route
schedule/evidence adapter: route IDs, canonical raw order, cursors, and its
left-to-right evaluator are migration provenance, not semantic family policy.
The existing resolved DirectAccum and NestedPredicate lanes remain live
family-specific production paths; only the Generic resolved-carrier path is
caller-zero here. No global raw-route retirement is implied.

### Future neutral observation-set contract

The next canonical product is named
`VerifiedLoopFamilyObservationSetV1`. It is one resolver-owned non-`Clone`
source receipt/window plus:

```text
exact Release/Strict/planner-required mode snapshot
coverage seal
Box<[LoopFamilyObservationV1]>
```

Each row carries a semantic family tag (`NestedPredicate`, `DirectAccum`, or
future Generic family; never a route ID) and one typed pre-effect disposition:
`Candidate`, `Declined`, `Blocked`, or `Unresolved`. The set retains the
receipt-issued owner/origin/source-kind/site/forest/frame relation instead of
four independently constructible coordinates. It contains no AST, raw
schedule/cursor, Recipe/key, Builder/MIR/ValueId/PHI, retry, or fallback.

The sole future selector is a new family-level entrypoint in
`mir::loop_route_policy`, named `CanonicalLoopFamilySelectionV1` for the
design. It consumes the sealed set exactly once and returns only
`Selected`, `NoCandidate`, typed `Rejected`, or typed `Unresolved`.
`NoCandidate` is legal only when a sealed whole-unit proof says that no Loop
family envelope exists. Missing/foreign/ambiguous owner or frame, incomplete
coverage, planner-required suppression without a typed row, and source/
BindingRef mismatch remain rejection/unresolved; they are never absence or a
legacy fallback. `A+`/Trivial stay in a separate whole-unit selection stage.

D4-S3 does not define NestedPredicate-versus-DirectAccum precedence,
Generic V0/V1 winner/disjointness, or a Recipe handoff. The six D4-S2 rows
prove only mode-stable legacy observations and schedule/family divergence;
they do not prove overlap policy. The selected-family Recipe producer remains
the sole `LoopBindingKeyV1` owner, `CanonicalFirstFamilyPlanV1` remains
post-selection, and Binding SSA remains the sole physical ValueId/PHI owner.

### Ordered task ladder

```text
D4-S3-S0  OBSERVATION-SET0
  private cfg(test) resolver-branded observation-set witness; no selector
D4-S3-S1  MATRIX-CLOSE0
  source-backed V0/V1/Neither + mode/reject matrix; no winner
D4-S3-S2  SELECTOR-PURE0
  pure family selector test consumer; no production caller
D4-S4-D0  GENERIC-RECIPE-HANDOFF0
  design Generic producer/key/effect relation before raw cutover
D4-S4-I0-R0  GENERIC-PRODUCTION-CUTOVER0
  one family caller and same-commit retirement of only migrated raw edges
```

## D4-S3-S0 observation-set witness closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-OBSERVATION-SET0-D4-S3-S0` is
closed as a private `cfg(test)` witness in
`src/mir/shared_loop_source_window.rs`. `TestLoopFamilyObservationSetV1`
owns exactly one non-`Clone` resolver receipt, one private mode snapshot, a
loop-window-only coverage seal, and three semantic family rows. The rows use
typed dispositions but remain `Unresolved` for NestedPredicate, DirectAccum,
and Generic; no winner, precedence, or `NoCandidate` policy is inferred.

The focused test seals six sets (two existing fixtures × Release/Strict/
StrictPlannerRequired), checks that every row retains the receipt-issued
owner/origin/source-kind/site/frame relation, and consumes paired raw/resolved
views exactly once. The witness has no route ID, schedule/cursor, AST field,
Recipe/key, Builder/MIR/ValueId/PHI, retry/fallback, selector, or production
caller. The changed source remains below 800 lines.

The implementation commit updates the MIR reference matrix, this card,
`CURRENT_STATE.toml`, current mirrors, and resolver/facts/Generic-loop support
docs. It does not activate `CanonicalLoopFamilySelectionV1` or any production
Generic route.

The selected next task is
`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-MATRIX-CLOSE0-D4-S3-S1`.

## D4-S3-S1 canonical matrix closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-MATRIX-CLOSE0-D4-S3-S1` is closed
as a private `#[cfg(test)]` registry witness in
`generic_resolved_carrier_canonical_matrix_tests.rs`. It issues one
resolver-branded non-`Clone` source-window receipt for each of three parsed
fixtures (`Both`, `V1Only`, and the existing `NoStandaloneRow`) under
`Release`, `Strict`, and `StrictPlannerRequired`: nine mode/fixture sets.
Each set consumes the receipt exactly once and records owner/origin/
source-kind/site/frame identity, facts status, V0/V1 presence, carrier
provenance, and four explicit cells (`V0Only`, `V1Only`, `Both`, `Neither`).

The matrix keeps `NoStandaloneRow` distinct from a real `Neither` Generic
presence. `V0Only` and a parsed `Neither` source remain `NotYetObserved`; no
synthetic source row is admitted. Under planner-required mode the natural
`Both` fixture observes the typed mode snapshot as V1-only after V0
suppression, but the witness deliberately records this as unresolved
mode-local evidence rather than inferring an intrinsic winner or suppression
policy. A planner-required facts freeze remains fully `NotYetObserved`.

Foreign-owner and non-Loop statements remain typed receipt rejects and are
never collapsed into `Neither`. The implementation calls the facts owner
directly; legacy schedule selection, family selector/winner/precedence,
Recipe/key, Builder/MIR, retry/fallback, runtime, and production Generic
caller remain zero. The focused matrix/reject tests are green, changed source
stays below 800 lines, and the implementation commit updates this card,
`CURRENT_STATE.toml`, current mirrors, support docs, and `docs/reference/**`
in the same commit.

The selected next task is
`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-SELECTOR-PURE0-D4-S3-S2`.

## D4-S3-S2 pure selector closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-SELECTOR-PURE0-D4-S3-S2` is
closed as a private `#[cfg(test)]` neutral consumer. The new
`loop_route_policy/family_selection.rs` is separate from the legacy
19-route evaluator and exposes only test-owned typed vocabulary:
`Selected`, `NoCandidate`, `Rejected`, and `Unresolved`. The S1 registry
adapter passes only a window-complete Generic evidence row; it does not pass
AST, LoopRouteContext, fixture labels, owner coordinates, route IDs, raw
schedules/cursors, or legacy policy evidence.

The selector keeps `WindowComplete` distinct from the future
`WholeUnitNoLoopEnvelope` proof. Therefore all nine S1 source/mode rows remain
`Unresolved`: overlap, V1-only, NoStandaloneRow, and planner-mode-unsealed
evidence each retain their own typed reason. No `Selected` or `NoCandidate`
is manufactured, and foreign/non-Loop source-window rejects remain before
the selector rather than collapsing into `Neither`. No Recipe/key,
LoopBindingKeyV1, Builder/MIR, retry/fallback, runtime, or production caller
was added. The focused S2 selector test is green, all changed source/check
files remain below 800 lines, and this implementation commit updates the
reference matrix, current state/mirrors, support READMEs, and this card in
the same commit.

The selected next task is
`JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-RECIPE-HANDOFF0-D4-S4-D0`.

# D4-S4-D0 Generic Recipe handoff design closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-RECIPE-HANDOFF0-D4-S4-D0` is closed as
a worker-reviewed design stop. The current S2 `SelectedFamilyV1` is only a
test marker and does not retain source/window/`BindingRef` provenance; it cannot
feed a Recipe producer. `V1Only`/`Both` window evidence is also insufficient for
`Selected(Generic)`. A future selector must require a resolver-branded,
source-backed candidate-envelope proof and return a one-shot selected
capability that retains the consumed source lease/window, exact mode/coverage,
and resolver-issued role `BindingRef`s.

The future handoff is:

```text
resolver source/window receipt (consume once)
  -> AST-free Generic source shape + candidate-envelope proof
  -> sealed family observation set
  -> CanonicalLoopFamilySelection (Selected(Generic))
  -> Generic-specific recipe demand (consume once)
  -> Generic Recipe producer
  -> VerifiedLoopRecipe + JoinSig + BindingRef/key effect relation
  -> Recipe/source-binding verification
  -> future Generic plan/lower
```

The Generic demand is a new family-specific capability (provisional name:
`VerifiedGenericRecipeDemandV1`). It must not reuse
`VerifiedSelectedLoopRecipeDemandV1`, which is branded by the legacy 19-route
policy winner. It carries no AST, `LoopRouteContext`, route ID, schedule,
cursor, fixture label, `ValueId`, `PHI`, Builder/MIR state, retry, or fallback.
Current `GenericLoopV0/V1Facts`, `RecipeBody`, and the P2 label snapshot are
AST/Builder-derived and are rejected as handoff inputs; do not wrap, clone, or
re-resolve them by name.

The dedicated Generic Recipe producer is the sole issuer of contiguous
recipe-local `LoopBindingKeyV1`s. It atomically emits a non-Clone verified
product plus a separate internal relation that maps each issued key to the
exact resolver `BindingRef`/role/site. The portable Recipe source-path claim
and this semantic BindingRef relation remain separate capabilities. Binding
SSA alone later maps BindingRefs to physical `ValueId`/`PHI`; the producer
never does. Missing/foreign/ambiguous owner, site, forest, frame, mode,
coverage, role, carrier, or effect relation rejects before key allocation.
Recipe/JoinSig/effect verification failure is terminal: no retry, V0/V1
re-selection, A+/Trivial alias, legacy route reconstruction, or fallback.

`NoCandidate` remains legal only for a sealed whole-unit no-Loop proof.
`NoStandaloneRow`, window `Neither`, overlap, planner-unsealed evidence, and
missing provenance remain typed unresolved/rejected states. Generic must not
be disguised as DirectAccum, NestedPredicate, or the current
`CanonicalLoopFamilyPlanV1` variants.

## Ordered D4-S4 task ladder

```text
D4-S4-D0  GENERIC-RECIPE-HANDOFF0
  closed: authority, demand boundary, reject matrix, and no-claim fence

D4-S4-S0  GENERIC-SEMANTIC-DEMAND-WITNESS0 (design-gated)
  only after a real sealed Selected(Generic) candidate exists; produce one
  cfg(test)-only AST-free demand/source-lease witness; if no candidate or no
  resolver-issued Generic source shape exists, record NoSafeSlice and stop;
  any implementation commit updates docs/reference/**, current mirrors, and
  support READMEs in the same commit

D4-S4-S1  GENERIC-RECIPE-PRODUCER-WITNESS0 (design-gated)
  consume one demand, issue keys only inside the producer, and witness the
  atomic Recipe/JoinSig/effect relation with reject-before-key and no partial
  publication; production caller remains zero; implementation updates exact
  reference docs in the same commit

D4-S4-I0-R0  GENERIC-PRODUCTION-CUTOVER0 (separate design gate)
  one selector -> demand -> producer -> verify -> Generic plan caller; retire
  only migrated raw edges in the same commit, with no retry/fallback

D4-S4-C0  GENERIC-LEGACY-CLEANUP0
  post-cutover census and retirement only after parity evidence
```

## D4-S4-S0 NoSafeSlice closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-DEMAND-WITNESS0-D4-S4-S0`
is closed as a docs/static audit with disposition `NoSafeSlice`; no source
witness or production code was added. The evidence is authoritative:

| gate | result | reason |
| --- | --- | --- |
| real `Selected(Generic)` issuer/callsite | fail | selector has only the marker type; all nine S1 rows remain `Unresolved` |
| resolver AST-free Generic candidate envelope | fail | source bridge transports identity/forest/window, not Generic V0/V1 eligibility/roles |
| one-shot source + `BindingRef` lease | fail | no Generic capability carries the required role provenance |
| Generic-specific Recipe demand | fail | only legacy `VerifiedSelectedLoopRecipeDemandV1` exists and requires a 19-route winner |
| forbidden leakage | pass | no new import/caller reaches AST facts, `LoopRouteContext`, Recipe/key, Builder/MIR, retry, or fallback |

The current `GenericLoopV0Facts`/`GenericLoopV1Facts` contain `ASTNode`,
`RecipeBody`, and Builder policies. The P2 snapshot is only a fixed
`NestedWriteWithPostLoopRead` observation. The resolver provenance witness is
test-only and does not issue a Generic carrier/eligibility envelope or expose a
role-level lease. The historical handoff protocol and synthetic test receipts
are not acceptable substitutes. Therefore S0 must not add a fake
`Selected(Generic)`, wrap/re-resolve facts by name, or reuse the legacy demand.

## D4-S4-S0-D0 semantic-shape design closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE-DESIGN0-D4-S4-S0-D0`
is closed as a docs-only worker decision. The minimum future product is a
move-only capability chain; no type or constructor is implemented in this row:

```text
resolver source projector
  -> VerifiedGenericSourceLeaseV1
  -> GenericSemanticShapeIssuerV1
  -> VerifiedGenericCandidateEnvelopeV1
  -> VerifiedGenericFamilyObservationV1 (mode + coverage)
  -> SelectedGenericFamilyV1
  -> VerifiedGenericRecipeDemandV1
  -> Generic Recipe producer
```

### Issuer map

| product / field | sole issuer | contract |
| --- | --- | --- |
| owner, origin, source-kind, root/loop sites, forest, frame | resolver source projector | one opaque non-`Clone` `SourceLease`; no loose coordinate constructor |
| role claims and `BindingRef` identity/scope/ancestry | resolver source projection | exact site + resolver-issued `BindingRef`; role set is non-`Clone`, not re-resolved by name |
| carrier, condition/step, body-effect, and semantic coverage proofs | new `GenericSemanticShapeIssuerV1` | AST may be borrowed only while issuing; output is typed/AST-free and never wraps `GenericLoopV0/V1Facts` or `RecipeBody` |
| candidate envelope seal | shape issuer consuming the resolver lease | preserves the lease brand and shape atomically; no independent source identity |
| mode snapshot and whole-unit/window coverage context | neutral observation/policy owner | co-sealed once; `MatrixModeV1`, route IDs, schedules, cursors, and planner flags are not source shape |
| family outcome | family selector | moves an opaque candidate; never reconstructs, keys, or revalidates by name |
| `LoopBindingKeyV1` and key↔`BindingRef` effect relation | Generic Recipe producer | sole key issuer; atomic verified product only |
| physical `ValueId`/`PHI` | Binding SSA | later physical owner; absent from every handoff product |

`VerifiedGenericSourceLeaseV1` may borrow `VerifiedResolvedSourceUnitV1`,
`FunctionSourceViewV1`, and `LocatedStmtV1` only during issuance. The returned
capability owns no AST, source-unit reference, or lifetime-carrying view. The
existing test-only provenance and shared-window products are evidence and
cannot become this production issuer by renaming or re-exporting.

### Shape and state contract

The candidate shape is bounded typed proof, not a string label or expression
AST. It contains exact condition/step/body source sites, typed carrier and
step-placement proof, typed body-effect/exit roles, and complete forest/window
coverage. Normalized bindings/uses/assignments may be inputs, but their current
graph lacks comparator/literal/step/body-effect detail; a resolver-owned
projector must add those facts or return `Unresolved`.

```text
Unsealed
  -> SourceLease (resolver brand)
  -> CandidateEnvelope (shape + lease, consumed once)
  -> FamilyObservation (mode/coverage co-sealed)
  -> SelectedGenericFamily (selector move only)
  -> GenericRecipeDemand (demand move only)
```

Every lease, role set, envelope, observation, selected capability, and demand
is non-`Clone`/non-`Copy`; `BindingRefV1` may remain a copied identity inside a
sealed role entry. No stage may split and later re-pair forest/frame/site/role
parts, clone the source, or re-resolve names.

### Reject and readiness matrix

Reject before candidate publication or key allocation on mixed owner/session,
foreign `BindingRef`, site/forest/frame mismatch, skipped/duplicate/unknown
role, shadowing/upvar/capture escape, incomplete condition/step/body effect,
opaque/transferred/synthetic subtree, unsupported operator/placement, stale
lease, mode/coverage mismatch, or missing whole-unit proof. These are typed
`Rejected`/`Unresolved`; they are never retry, fallback, alias, or
`NoCandidate`. `NoCandidate` is legal only from a sealed whole-unit no-Loop
envelope, never from window `Neither`, `NoStandaloneRow`, `V1Only`, overlap,
or planner-unsealed evidence.

The required counterexample is two structurally identical Generic loops (or
two resolver sessions) with distinct source sites/`BindingRef`s, including a
shadowing relation. The lease issuer derives forest and per-member frames from
one `VerifiedResolvedFunctionV1`; callers cannot supply a foreign forest/frame
to mix, and a marker-only selector must not choose either loop.

D4-S4-S0-D0 is accepted only as this design contract. The future cfg(test)
witness may start after a real resolver-issued shape and a selector callsite
produce `Selected(Generic)`; otherwise it closes as `NoSafeSlice` again. Any
implementation commit updates exact references/current mirrors/support READMEs
in the same commit; no relevant `docs/reference/**` contract applies to this
cfg(test)-only cell.

## D4-S4-S0 semantic-shape schema design closeout

`GENERIC-SEMANTIC-SHAPE-SCHEMA-D1` is closed as a worker-reviewed docs-only
contract. The resolver lease is the only source/BindingRef issuer; the shape
issuer only proves semantics from a borrowed source view.

| proof | typed contents | issuer |
| --- | --- | --- |
| `CarrierProof` | role-keyed binding/site relation and carrier transfer relation | shape issuer, using lease roles |
| `ConditionProof` | exact condition site, comparator enum, operand roles, bound/literal proof, placement | shape issuer |
| `StepProof` | exact step site, operator enum, target role, delta/operand proof, placement | shape issuer |
| `BodyEffectProof` | ordered typed writes/reads/calls and exit-role effects; no raw body AST | shape issuer |
| `Coverage/Exit` | complete forest/window membership, break/continue/return coverage, opaque/transfer checks | resolver coverage + shape issuer seal |

The lease owns owner/origin/source-kind/root+loop sites, forest/frame, and a
non-`Clone` role-claim set containing exact site, `BindingRef`, scope, and
ancestry. The shape owns only the five proofs above. Neither product may carry
AST, source-unit lifetime, strings/labels, route IDs, `RecipeBody`, Builder,
MIR/PHI, `ValueId`, or legacy demand.

The source-lease witness is cfg(test)-only, exact-two-role (`NestedWrite` +
`PostLoopRead`), owner-branded, non-`Clone`, and AST/source-lifetime-free.
Forest/frames are reissued from one function and co-sealed; five tests cover
positive, foreign, shadow, duplicate/placement, and forest-mismatch cases;
upvar/capture awaits a later fixture.

## D4-S4-S0/S1 witness closeout
S0/S1 close the cfg(test)-only lease and CarrierProof handoff; no production
caller, selector/demand, Recipe/Builder/MIR, retry, fallback, or rename.
## D4-S4-S2-D1 semantic-shape extension design closeout
V1 remains immutable; V2 begins with inner-loop Condition+Step. Resolver owns
sites/forest/frame brands; shape owns operator/effect proofs; no public reference row.
## D4-S4-S2-D0 resolver role-issuer design closeout
Direct V2 issuance is `NoSafeSlice`; D0 selects branded point inventory with
`SourcePathSegmentV1` as sole topology authority; no AST/name/role/parent map.
## D4-S4-S2-S0 source-site inventory implementation closeout
Accepted: traversal records membership; seal brands owner/origin/source-kind,
point lookup, and indexed-site checks. Tests green; no downstream/public row.
## D4-S4-S2-S1 resolver role-issuer witness closeout
Accepted cfg(test)-only V2 Condition+Step catalog retains the V1 handoff,
consumes inventory/path topology, and rejects foreign/missing/misplaced/upvar/
binding-mismatch sites. Five tests pass; no downstream/public row.
## D4-S4-S3-D0 condition-step semantic-shape design stop
Next: worker-review comparator/operator/literal/delta proof semantics. Keep V2
immutable; no AST/name/parent map, selector/demand/Recipe/Builder/MIR, retry,
fallback, or production caller until that design is fixed.
