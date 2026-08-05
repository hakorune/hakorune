---
Status: active design stop — D3-S2-D1 authority map accepted; P0 closed; P1 selected
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Predecessor: joinir-generic-nested-carrier-d3-bindingref-design-2026-08-05.md
Decision: accepted authority realignment; no production selector change
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-BOUNDARY-D3-DESIGN0-D0`
---

# Generic resolved-carrier source-to-selection boundary

## Current authority correction — D3-S2-D1

The worker-adjudicated D3-S2-D1 decision supersedes the earlier pseudotypes in
this card wherever they conflict. Those older protocol sketches remain
historical test evidence only.

```text
resolver
  -> source sites / forest / owner brand / BindingRefV1 roles
neutral structural facts (only after P0/P1)
  -> AST-free eligibility; no Recipe key or schedule
Recipe producer
  -> sole LoopBindingKeyV1 issuer
  -> Recipe + JoinSig + BindingRef-to-Recipe-key relation
one non-Clone canonical plan
  -> facts + eligibility + Recipe + route-affecting inputs
selector
  -> consumes only the canonical plan
```

`PreflightSeedV1`, `InvocationSealV1`, and the four-field
`VerifiedResolvedCarrierSelectionInputV1` are rejected as separate production
authorities. The canonical plan's linear ownership is the invocation seal.
Binding SSA remains the physical `BindingRefV1 -> ValueId/PHI` owner and never
issues Recipe keys. `LivePreflightFrameV1` and the current registry remain
legacy transport/parity owners until the later atomic cutover.

P0 source-site totality census is closed by its machine-readable matrix. The
selected next row is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-PROJECTION0-D3-S2-P1`; it adds no
issuer, selector, Builder/MIR/PHI, Return/Home/debt, fallback, retry, or
production caller.

## Boundary

The scoped D3 BindingRef matrix is closed as test-only evidence. Production
selection cannot consume it yet: the current facts product is carrier labels,
the registry selector receives `CanonicalLoopFacts` only, and the router's
`LivePreflightFrameV1` has no resolver/source capability. Adding a V0
suppression arm now would create a second semantic authority.

This card designs the missing co-sealed handoff. It does not implement V0
suppression, V1 precedence, Recipe/JoinSig/PHI production, Retry/fallback
removal, or a route/Builder/MIR/backend cutover.

## Historical pre-D1 capability sketch (superseded)

The following sketch motivated the closed protocol tests but no longer defines
the future production types. D3-S2-D1 above is authoritative:

```text
parsed source
  -> VerifiedResolvedFunctionV1
  -> VerifiedResolvedLoopSourceForestV1
  -> resolver-issued assignment/read BindingRefV1s
  -> GenericCarrierFactsSnapshotV1
  -> PreflightSeedV1 (mode/frame flags/base schedule)
  -> VerifiedResolvedCarrierEligibilityV1
  -> RecipeFirstSelectionInputV1
  -> pure registry selection
  -> LivePreflightFrameV1
```

`VerifiedResolvedCarrierEligibilityV1` is a capability proposal, not yet a
code type. Its neutral owner should be the existing
`mir::loop_structural_facts` boundary, not the Generic composer or route
handler. It must co-seal:

```text
function origin and source kind
exact outer/inner loop sites and frame identity
same strict-ancestor BindingRefV1 relation
complete recursive carrier observation
facts/source ownership identity (without AST payload)
```

Mode, route IDs, raw schedule, and execution flags belong to a separate
`PreflightSeedV1` supplied once by the router. They are not semantic carrier
identity and must not be minted by the resolver or reconstructed from labels.
The Builder-side adapter may create a neutral
`GenericCarrierFactsSnapshotV1` from canonical facts, but the neutral issuer
must not import `CanonicalLoopFacts` or `LivePreflightFrameV1`.

The selector must consume a closed `RecipeFirstSelectionInputV1` rather than an
optional capability field. `CanonicalLoopFacts` is AST-bearing and
source-blind, so it must not travel as an independently pairable reference
next to a capability. The resolved input is an opaque, private, non-`Clone`
wrapper that keeps the facts snapshot, eligibility, preflight seed, and
invocation seal together:

```text
VerifiedResolvedCarrierSelectionInputV1 {
  facts_snapshot: GenericCarrierFactsSnapshotV1
  eligibility: VerifiedResolvedCarrierEligibilityV1
  seed: PreflightSeedV1
  invocation: InvocationSealV1
}
```

`CanonicalLoopFacts` remains a Builder-local AST-bearing product and is not a
field of the neutral selection wrapper. A later Builder adapter may consume
the AST-bearing facts and emit the AST-free `GenericCarrierFactsSnapshotV1`
only after the same invocation seal is verified; the selector never receives
an independently pairable `CanonicalLoopFacts` reference.

The wrapper has no public constructor or `parts()` accessor. Selection consumes
it and returns a `VerifiedRouteSelectionReceiptV1` that retains the invocation
seal and eligibility for the downstream frame issuer. This prevents a caller
from pairing facts from one source invocation with a capability from another.

The input has exactly two semantic variants:

```text
Legacy(typed NotApplicable or ProvenOutsideTarget receipt)
Resolved(VerifiedResolvedCarrierSelectionInputV1)
```

`Legacy` is private and may only be issued for typed non-target dispositions;
it is not a fallback for a target row with a missing or invalid handoff. The
resolved variant is issued only after the source observation and neutral facts
snapshot are sealed. Final route selection consumes that wrapper and produces
the schedule used to issue
`LivePreflightFrameV1`; the frame is downstream of selection and is not part of
the capability, avoiding a selection/frame cycle. Missing capability is a
typed handoff rejection, never an implicit legacy fallback.

The selector/capability boundary must not re-read AST, names, legacy tags,
plan digests, or runtime receipts, or reconstruct capability identity from
external route IDs/registry order. The selector may inspect its own canonical
route IDs to apply policy. The capability must be
unforgeable outside its resolver/facts owner and must not be reconstructible
from a `Vec<String>` carrier label.

## Owner map

The authority split is fixed as follows:

```text
FunctionSemanticResolverSessionV1
  = sole issuer of exact loop sites/forest and BindingRef relations

mir::loop_structural_facts generic sibling
  = neutral issuer of typed eligibility from the resolver observation and
    neutral facts snapshot

registry/selection.rs
  = sole consumer and policy owner

router.rs
  = transport owner for the preflight seed and final LivePreflightFrameV1

handlers/generic.rs, composers, verifiers, lowerers, PHI, MIR
  = no capability or policy authority
```

The resolver must not import Builder/registry policy, and the facts owner must
not mint BindingRefs. The resolver first issues a typed
`ResolvedCarrierObservationV1`; the neutral issuer consumes that observation
and a typed Generic-facts snapshot. The Builder-side adapter then combines the
neutral facts, eligibility, and preflight seed into the private wrapper. No
owner reconstructs any of them from AST or names.

The current `route_loop` ingress has only AST plus `MirBuilder` and therefore
cannot issue this capability. A separate resolved Generic source projector
must be designed first; adding an `Option<Capability>` to the current ingress
is explicitly out of scope.

The seed must include every execution-affecting frame flag, not only mode and
schedule:

```text
strict_or_dev
planner_required
has_body_local
recipe_contract_present
recipe_first_allowed
base/unfiltered raw schedule (natural Both = [V0, V1])
source/frame/facts identity
```

If one route invocation must be proven, a private non-Clone
`InvocationSealV1` binds the source observation, facts snapshot, seed, and
frame key. A reusable or cloned frame key is not sufficient evidence of one
invocation.

## Policy owner and fail-fast boundary

The policy owner is `registry/selection.rs` and its
`CandidateSuppression`/predicate boundary. `handlers/generic.rs`, composers,
verifiers, lowerers, PHI materializers, and MIR builders are not policy owners.

Only the exact proven class may suppress the V0 attempt in a future slice:

```text
Release/Strict natural Both [V0, V1]
co-sealed resolved-carrier capability present
same BindingRef/source/frame identity
complete recursive carrier
```

Missing handoff, owner/frame mismatch, absent facts, foreign/ambiguous binding,
planner-required V0 suppression, V1-only/Neither, unsupported nested shape, or
unstable evidence must fail before Builder effects as `UnresolvedStop`. The
legacy V0 edge remains execution authority for unproven rows. No retry or
fallback may turn a post-effect failure into a new candidate.

## Matrix required before implementation

The handoff design must freeze a typed matrix before any production arm is
added:

```text
V0-only / V1-only / Both / Neither
  × Release / Strict / planner-required
  × exact carrier / shadowing / owner-frame mismatch
  × complete / NoRecursive / Unavailable / Ambiguous
  × supported / nested-wrapper / duplicate-write / Index-Program-CompoundAssignment
```

The current S2/D3 rows cover only natural Both, shadowing, and planner
suppression. They do not close this matrix or prove V0-debt-to-V1 equivalence.
The test-only protocol model enumerates the matrix axes but does not replace
source-resolver evidence or prove V0-debt-to-V1 equivalence. Rows without the
sealed capability remain unresolved and retain the old execution edge.

## Staged implementation proposal

No implementation is authorized by this design stop. If the matrix and owner
contract are accepted, the smallest later slice is:

```text
1. compiler-side resolved Generic projector obtains the resolver observation
2. mir::loop_structural_facts issues neutral typed eligibility
3. a Builder-side private adapter consumes facts + eligibility + seed into the
   opaque non-Clone wrapper
4. selector consumes only the wrapper in Release/Strict natural Both
5. shadowing and missing-capability rows fail before Builder mutation
6. V0 attempt is zero only for that exact proven class
7. all other rows preserve existing V0/V1 schedule and receipts
8. focused tests, caller census, and reference closeout land atomically
```

The first production slice must not touch `handlers/generic.rs`, composer,
physicalizer, PHI, MIR, backend, or global M10 scheduler deletion. Parent
Generic D2 remains `UnresolvedStop` until winner/disjointness and downstream
authority gates are complete.

## Issuance and consumption order

The handoff is one-way and must be testable without Builder mutation:

```text
1. resolved ingress supplies the existing resolver/source handle; it does not re-resolve
2. `try_build_outcome(ctx)` seals canonical facts and one mode/preflight seed
3. resolver issues `ResolvedCarrierObservationV1`; neutral issuer seals it with a facts snapshot, or rejects
4. builder-side private wrapper moves facts + eligibility + seed together
5. `RecipeFirstSelectionInputV1` is constructed with an explicit variant
6. registry selection applies canonical route policy to the base schedule
7. selection emits `VerifiedRouteSelectionReceiptV1` retaining invocation seal
8. `LivePreflightFrameV1` consumes/borrows that receipt and same seed
9. legacy execution continues only from the selected frame
```

No step may re-derive a prior product from AST or names. A missing or mismatched
capability is `UnresolvedStop` before Builder effects; it is not represented as
`Option<Capability>` with a silent fallback.

The selector may use its own canonical registry route IDs to apply policy, but
the identity rule above still applies. `LegacyFacts` is constructible only for
a typed `NotApplicable` or `ProvenOutsideTarget` disposition. A target row
with missing/invalid handoff is `UnresolvedStop`, not a silent legacy fallback.

## Historical production-free protocol matrix and acceptance

Before any production type or selector arm is added, a `cfg(test)` protocol
matrix must exercise the sealed handoff without Builder mutation:

```text
natural Both Release/Strict positive
shadowing negative
planner-required typed suppression
missing / foreign / ambiguous BindingRef
owner/frame/source mismatch
NoRecursive / Unavailable / Ambiguous carrier
target/stage/unstable repeat
nested wrapper / duplicate write / Index / Program / CompoundAssignment

typed non-target `NotApplicable` and `ProvenOutsideTarget` legacy rows
```

Every reject remains useful test evidence, typed and pre-effect, but the
separate seal/seed shape below is superseded. Each failure is `UnresolvedStop` (or an explicitly
terminal freeze target); no retry or fallback is allowed. The matrix must prove
that cross-invocation pairing rejects. A future implementation proves that
through the single canonical plan, not a standalone `InvocationSealV1`. Its
acceptance receipt includes:

```text
proposed capability production caller/import census = 0
selection/router/frame/handler/composer edits = 0
parent D2/winner-equivalence/runtime claims = 0
```

Only after this protocol and the full D2 matrix are accepted may a separate
implementation card add the neutral capability and selector input.

## Closed bounded task — Compound/Unavailable source matrix

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-COMPOUND-UNAVAILABLE0-D2-S3`

Decision: accepted and implemented after independent source/classifier/contract
audits. This is one cfg(test)-only source row; it does not reopen the D1 bridge
or promote the D3 eligibility witness into a production issuer.

### Change

- Parse one nested `CompoundAssignment` fixture with a scoped
  `NYASH_SYNTAX_SUGAR_LEVEL=basic` environment. Keep the accepted S2A
  outer/inner-loop topology and place the compound write inside the nested
  region, because the carrier collector's `CompoundAssignment` arm is
  `nested`-only.
- Co-seal the actual parsed source path, resolver forest/target/BindingRef,
  source/owner/frame identity, actual Release/Strict mode, and the facts
  observation from `try_build_outcome` in one private non-`Clone` test witness.
- Observe exact facts
  `Unavailable("CompoundAssignment")` and record the raw route schedule from
  the same invocation. Do not assume `[V0, V1]`; a different schedule is
  evidence and must remain typed rather than repaired.

### Contract

- The resolver target may be `BindingRebind`, but that does not make the
  carrier eligible: the facts collector owns the `Unavailable` disposition.
- The only disposition produced by this row is pre-effect typed
  `UnresolvedStop(CompoundUnavailableCarrier)`. It must not become
  `CompleteRecursive`, eligibility, `Legacy`, a winner, or V0 suppression.
- Sugar parsing, source navigation, forest/BindingRef identity, facts label,
  mode/raw repeat, and nested ownership are fail-fast premises. If any premise
  is absent or unstable, stop and record the typed rejection; do not widen the
  extractor or selector.

### Non-claims and closeout

This task adds no neutral issuer, `InvocationSealV1`, selector/router arm,
Builder/MIR/Recipe/JoinSig/PHI/backend caller, Retry/fallback deletion, or
parent-D2/winner-equivalence claim. Top-level compound behavior is not folded
into this row; the collector's silent non-nested path is a separate design
question. The implementation closeout must update this card, the Generic
SSOT, stage-matrix reference, Generic/resolved-semantics READMEs,
`CURRENT_STATE.toml`, current dashboard/workstream, affected reference
indexes, and the artifact manifest in the same commit, with every touched
source/check file below 800 lines.

### Done

- The parsed fixture uses scoped `NYASH_SYNTAX_SUGAR_LEVEL=basic`, keeps the
  accepted outer/inner-loop plus nested-`IfThen` topology, and proves the
  exact `ASTNode::CompoundAssignment { operator: Add }` path. Resolver output
  is `BindingRebind` for the same local as the post-loop read; the two-member
  forest, `DeclaredFunction` source kind, owner/frame identity, strict ancestry,
  and local BindingRef slots are co-sealed in one private non-`Clone` witness.
- Actual Release and Strict observations both produce facts
  `Unavailable("CompoundAssignment")` and raw schedule `[V0, V1]`. Fresh
  invocations keep source origin/frame/path, local slots, carrier reason, raw
  schedule, and typed disposition stable while issuing a fresh owner identity.
  The only disposition is pre-effect
  `UnresolvedStop(CompoundUnavailableCarrier)`.
- The focused D2-S3 tests are 2/2 green and the adjacent
  `generic_resolved_carrier_` suite is 25/25 green. The new test sibling is
  345 lines; only a `cfg(test)` registry module was added. No eligibility,
  Legacy, winner, V0 suppression, selector, neutral issuer, Builder/MIR,
  Recipe/JoinSig/PHI, Retry, fallback, or parent-D2 authority moved. The
  implementation closeout updates all referenced current/reference docs and
  the artifact manifest in this same commit; future production capability
  changes must update the language/reference documents again.

### Stop

After this row, return to this parent design stop. If the parser cannot
produce the nested source, facts do not emit the exact label, the schedule or
identity is unstable, or any production effect appears, close the row as
unresolved and do not invent a new source authority.

## Closed bounded task — resolved Generic projector coverage

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-PROJECTOR-DESIGN0-D0`

Decision: accepted and implemented as a `cfg(test)`-only continuation;
production handoff remains stopped. The original three-test projector is now
closed with the S2A coverage receipt below.

### Change

- Extend the existing projector test sibling with exactly one parsed
  nested-`IfThen` source shape from S2A. Navigate through the existing
  `ResolvedFunctionLoweringInputV1`/`FunctionSourceViewV1` path and co-seal the
  resolver-issued outer/inner forest, exact source sites, `BindingRefV1` pair,
  function/source-kind/frame identity, and a test-only facts identity receipt
  in one private non-`Clone` witness. This does not create the future neutral
  facts issuer.
- Add only the smallest typed negative witnesses for foreign frame/source-kind
  or facts mismatch that the same source-view boundary can construct. Keep the
  existing positive, shadowing, and foreign-owner tests as neighboring guards.

### Contract

- The projector owns source/resolver observation only. `CanonicalLoopFacts`
  remains Builder-local and AST-bearing. D3-S2-D1 supersedes the former
  separate `PreflightSeedV1`/`InvocationSealV1` route; carrier
  policy (`NoRecursive`/`Unavailable`/`Ambiguous`/planner suppression) belongs
  to the neutral facts/selector owners.
- Every mismatch rejects before Builder effects. No registry policy,
  `LivePreflightFrameV1`, selector, `Option<Capability>`, Recipe/JoinSig/PHI,
  Builder/MIR/backend, Retry, fallback, or runtime caller may be added.

### Done

- The focused `generic_resolved_projector` filter must prove one parsed S2A
  nested-`IfThen` positive and typed identity rejects; all projector/test
  files must remain below 800 lines and production caller/import census must
  remain zero.
- Pointer, diff, artifact, and line-budget guards remain green. The active
  card, parent Generic SSOT, Generic README, resolved-semantics README,
  stage-matrix reference, current pointers, and affected reference indexes are
  updated in the implementation closeout commit.

### Stop

- Return to design if the source-view path cannot prove one invocation's
  source/forest/BindingRef/frame relation, if facts identity requires a second
  semantic authority, or if any production seam is needed. Parent Generic D2,
  neutral issuer, Builder adapter, and production selector remain
  `UnresolvedStop` after this row.

### Implementation closeout — 2026-08-05

The implementation commit extends the existing projector sibling to 457 lines
and five focused tests. The positive path parses the nested S2A `IfThen`
fixture and navigates outer loop -> inner loop -> `IfThen` -> assignment target
and post-loop return through `FunctionSourceViewV1`; no hand-built positive
source path is used. The private non-`Clone` receipt co-seals the two-member
resolver forest, exact source sites, resolver-issued `BindingRefV1` pair,
function owner/origin/source-kind/frame identity, and a facts-only identity
observation without retaining `CanonicalLoopFacts`.

The focused filter is green:

```text
generic_resolved_projector: 5 passed
```

The existing shadowing and foreign-owner rejects remain green, and a fresh
cross-invocation facts observation is rejected as the typed
`FactsIdentityMismatch` witness. Production caller/import census remains zero;
no neutral issuer, router seed/invocation seal, selector, Recipe/JoinSig/PHI,
Builder/MIR/backend, Retry, fallback, or runtime authority changed. The source
file is 457 lines (<800).

## Non-authority and documentation contract

The following are corroborating only:

```text
GenericLoopCarrierObservationV1 string labels
route IDs and registry order
S1 PHI/final-value tags
synthetic test bodies
plan digests and diagnostic_effective
legacy receipts/terminal status
runtime or VM result
```

When a later implementation is authorized, the same commit must update the
parent Generic SSOT, stage-matrix reference, Generic README,
resolved-semantics README, `CURRENT_STATE.toml`, `10-Now.md`, the MIRBuilder
workstream, and all affected `docs/reference/**` navigation/status indexes.
The implementation receipt, focused matrix tests, caller census, and explicit
fail-fast/sunset contract are mandatory; documentation cannot be deferred.

## Current evidence

```text
S2 identity witness: 3 focused tests, test-only
D3 typed matrix: 1 focused test over 4 rows, test-only
handoff protocol: 6 cfg(test) tests, including a generated 1,440-row typed matrix
resolved projector closeout: 5 cfg(test) tests; parsed S2A source-view and
facts/source/frame co-seal are closed as test-only evidence
production selector callers of BindingRef capability: 0
current Both trace: [V0, V1] -> V0 success; no V1 debt attempt
next design child: `JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`
```

The resolved-projector harness is evidence only: the closed receipt proves
resolver forest, BindingRefs, source identity, frame identity, and a private
facts observation before any effect, with foreign-root, shadowing, and
cross-invocation mismatch rejection. It does not issue a production
capability, select a route, or close the parent D2 row. The upper
source-to-selection handoff remains the active design stop.

## Closed bounded task — source-backed handoff bridge

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-BRIDGE0-D1`

Decision: accepted and implemented as a `cfg(test)`-only bridge; neutral issuer
and production selection remain prohibited. The existing projector receipt now
feeds one private, non-`Clone` protocol witness without adding a production
capability.

### Change

- Add one private, non-`Clone`, AST-free test DTO bridge from the existing
  projector receipt to the handoff protocol. It must move, not re-derive, the
  same parsed invocation's resolver forest/source sites, BindingRefs,
  function/source/frame identity, facts identity, actual `try_build_outcome`
  facts observation, base raw schedule, and frame-affecting flags.
- Exercise exactly the existing S2A natural-Both source shape in Release and
  Strict. Keep the protocol's synthetic matrix and typed mismatch guards as
  neighboring evidence; `NoRecursive`/`Unavailable`/`Ambiguous` and other
  source shapes remain separate rows.

### Contract

- `VerifiedResolvedSourceUnitV1`/`FunctionSourceViewV1`, the resolver forest,
  BindingRefs, and the same invocation's canonical facts are the only source
  authorities. Carrier labels, route IDs, digests, legacy receipts, and
  runtime results are corroboration only.
- A cross-invocation facts/seed/frame pairing rejects before effects as typed
  `UnresolvedStop`; it must not become a `Legacy` fallback. Planner-required
  and other execution-mode axes are separate rows because this projector
  evidence fixes the current non-planner facts path. No neutral issuer,
  `Option<Capability>`, selector/router
  arm, `LivePreflightFrameV1`, Recipe/JoinSig/PHI, Builder/MIR/backend, Retry,
  fallback, or parent-D2 winner claim may be added.

### Implementation closeout — 2026-08-05

- `generic_resolved_carrier_projector_tests.rs` now retains the actual raw
  schedule and frame flags beside the private resolver/source/facts receipt;
  `ProjectorHandoffObservationV1` exposes only immutable test observations.
- The handoff protocol has two source-backed tests. Release and Strict both
  select the synthetic V1 witness only after the parsed S2A receipt proves
  natural `[V0, V1]`, a two-member forest, and the frame flags. A second parsed
  invocation is rejected as typed `FactsIdentityMismatch` before selection.
- The bridge remains test-only and AST-free: no neutral issuer, production
  selector/router arm, Builder/MIR/backend caller, Recipe/JoinSig/PHI, retry,
  fallback, or parent-D2 winner claim was added. At D1 closeout the focused
  projector/protocol files were 515/590 lines; the focused projector (5) and
  protocol (8) filters passed.

### Done

- A focused source-backed bridge test proves Release/Strict `[V0, V1]` from
  the parsed S2A fixture and same-invocation co-seal; a cross-invocation
  pairing mismatch is a typed pre-effect reject. Production
  caller/import census remains zero.
- All touched Rust/check files stay below 800 lines. Pointer, diff, artifact,
  and line-budget guards plus the existing projector/protocol filters are
  green. Closeout updates this card, the parent Generic SSOT, stage-matrix
  reference, current pointers, and the workstream in one implementation commit.

### Stop

- Return to design if the bridge needs a neutral facts type, a second facts or
  seed authority, AST/name reconstruction, a production import, or a second
  source shape. Parent D2, full source-backed matrix, neutral issuer, and
  production selector remain `UnresolvedStop`.

## Closed bounded task — source-backed planner suppression

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-PLANNER-SUPPRESSION0-D2-S1`

Decision: accepted and implemented as one cfg(test)-only mode-co-seal row. It
repairs the D1 evidence boundary without selecting a winner or creating a
production owner.

### Change

- Reuse only the parsed `NESTED_IF_SOURCE` S2A shape and run resolver forest,
  BindingRefs, facts, frame, actual mode, and raw selection under one real
  `Strict + planner-required` configuration scope.
- Co-seal the actual mode/preflight observation with the same source/facts
  identity receipt; do not pass a caller-supplied boolean or reread a second
  environment scope.

### Contract

- The authoritative result is raw `[GenericLoopV1]`, with V0 facts suppressed
  before selection and a typed `UnresolvedStop(PlannerRequiredV0Suppression)`.
- The row issues no eligibility, Legacy, winner, neutral issuer, selector,
  Recipe/JoinSig/PHI, Builder/MIR/backend, Retry, or fallback product.
- Release/Strict natural-Both evidence must not be re-paired with this
  planner-required receipt; a mode or invocation mismatch is a pre-effect
  reject.

### Done

- A focused source-backed test proves the same parsed S2A forest/BindingRefs,
  complete recursive carrier, actual Strict+planner-required mode, raw `[V1]`,
  V0 pre-effect suppression, typed unresolved disposition, and fresh-repeat
  stability. Existing D1/S2A/D3 filters, pointer/artifact guards, and the
  below-800-line source/check budget remain green; production caller count
  stays zero.
- Closeout: this implementation commit records the projector/protocol
  observation update and the new planner-suppression sibling. Focused generic
  projector/protocol (16), nested-carrier (1), S2A mode-boundary (1), and
  stage-matrix (1) tests pass. The parent Generic D2 source-to-selection
  boundary remains the next design stop.

### Stop

- Return to design if raw is not exactly `[V1]`, V0 facts remain, the recursive
  carrier or identity seal is incomplete, mode is synthetic/split, a second
  source shape or production API is needed, or any Builder effect occurs.
  V0-only, Unavailable, Ambiguous, NoRecursive, and full parent D2 remain
  separate unresolved rows.

## Closed bounded task — source-backed Index/Ambiguous matrix row

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-INDEX-AMBIGUOUS0-D2-S2`

Decision: accepted and implemented as one cfg(test)-only negative matrix row
after independent worker adjudication. Eligibility issuance and production
handoff remain stopped after this source-backed axis is closed.

### Change

- Reuse the parsed S2A outer/inner-loop plus `IfThen` topology and change only
  the decisive nested write to plain `items[j] = i`, with `items` a function
  parameter. Keep the inner `j` step, outer `i` step, and post-loop read.
- Co-seal the resolver's `ResolvedAssignmentTargetV1::IndexWrite`, Generic
  facts' `Ambiguous("assignment target")`, exact forest/source/frame identity,
  actual Release/Strict mode, raw schedule, and fresh-repeat result in one
  private non-`Clone` test witness. The positive BindingRebind projector may
  reject this row as `NonBindingTarget`; do not force it through the positive
  eligibility path.

### Contract

- The typed result is `UnresolvedStop(IndexWriteAmbiguousCarrier)`; it is a
  natural negative matrix row, not `ProvenOutsideTarget`, Legacy, a winner, or
  a V0/V1 precedence claim. Release/Strict should retain raw `[V0, V1]` if the
  existing facts route is unchanged.
- No AST reconstruction, second resolver/seed, `loop_structural_facts`
  production widening, selector/Builder import, Retry/fallback, or production
  eligibility/adapter/Recipe/JoinSig/PHI/MIR authority is allowed.

### Done

- Focused tests prove the parsed fixture's two-member forest, exact nested
  `IndexWrite`, exact `Ambiguous("assignment target")` carrier fact, same
  source/frame identity, actual Release/Strict raw schedule
  `[GenericLoopV0, GenericLoopV1]`, fresh-repeat stability, and typed
  pre-effect unresolved disposition. The focused `generic_resolved_carrier_`
  suite is green (19 tests, including the three new Index/Ambiguous tests).
- The implementation remains cfg(test)-only: no production resolver/facts
  widening, neutral issuer, eligibility/selector arm, Builder/MIR caller,
  Recipe/JoinSig/PHI/physicalizer, Retry, or fallback was added. The updated
  Generic/resolved-semantics READMEs, stage-matrix reference, workstream,
  current pointers, and this reference design card record the measured result
  in the same implementation commit; future implementation of the language
  path must update the reference documents again at its own closeout.

### Stop

- Return to design if the parser/resolver cannot construct `IndexWrite`, facts
  do not produce exact `Ambiguous("assignment target")`, the forest or mode is
  unstable, another source shape or production API is needed, or any Builder
  effect occurs. Eligibility protocol, neutral snapshot/issuer, sealed
  `RecipeFirstSelectionInputV1`, V0 suppression arm, V1 precedence, and parent
  D2 closeout remain separate later rows.

## Closed bounded task — source-backed eligibility protocol

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-ELIGIBILITY-PROTOCOL0-D3-S0`

Decision: accepted and implemented as one cfg(test)-only source-to-eligibility
protocol row. It does not add the future neutral production issuer, selector
input, or capability type; the parent source-to-selection design stop remains
open.

### Change

- Reuse the existing parsed S2A natural-Both source projector under one real
  Release/Strict invocation scope and seal a private, non-`Clone` test witness
  that carries the actual resolver forest/source/frame, BindingRefs, facts
  observation, mode/preflight flags, and raw `[V0, V1]` together.
- Issue a test-only eligibility disposition only for exact
  `CompleteRecursiveCarrier`, matching source/owner/frame identity, natural
  Both, and non-planner mode. Pair the existing D2-S2 `IndexWrite` /
  `Ambiguous("assignment target")` observation as a typed negative, without
  reinterpreting it as a source-backed eligibility candidate.
- Keep the existing synthetic matrix as policy-axis evidence. It may supply
  typed negative cases for shadowing, planner, NoRecursive, Unavailable,
  foreign/mismatch, and unstable seals, but those cases must not be reported as
  source-resolver evidence.

### Contract

- The only positive result is private
  `Eligible(CompleteRecursiveCarrier)` for the exact natural-Both source
  witness. All other rows are typed pre-effect
  `UnresolvedStop(EligibilityProtocolMismatch)`; none may become Legacy,
  ProvenOutsideTarget, a winner, or a selector decision.
- The witness consumes actual resolver-issued `BindingRefV1`/forest and actual
  `GenericLoopCarrierObservationV1` once. No AST/name reread, synthetic source
  identity, second facts authority, or caller-supplied booleans are allowed.
- No neutral production issuer, `VerifiedResolvedCarrierEligibilityV1`,
  `RecipeFirstSelectionInputV1`, selector/router/Builder/MIR/Recipe/JoinSig/
  PHI/physicalizer caller, Retry, fallback, or M7/M10 route work is allowed.

### Done

- Four focused tests prove actual Release/Strict natural-Both positive
  eligibility, the D2-S2 IndexWrite/Ambiguous negative, planner/shadowing/
  missing-capability pre-effect rejects, cross-invocation facts-identity
  rejection, and fresh-repeat stability. The synthetic matrix remains
  separate and green; adjacent projector tests retain owner/frame guards.
- Implementation closeout updates this card, current pointers, Generic and
  resolved-semantics READMEs, the stage-matrix reference, workstream, affected
  reference indexes, and artifact inventory in the same commit. Future
  production capability implementation must update the language reference
  documents again after its own implementation closeout.

### Stop

- Return to design if one invocation cannot provide the full source/facts/frame
  seal, if the positive path needs AST reconstruction or a second source shape,
  if IndexWrite/Ambiguous is accidentally accepted, if synthetic IDs leak into
  the source witness, or if any production import/effect appears. No full D2
  matrix, V0/V1 precedence, winner equivalence, neutral issuer, selector, or
  runtime claim is closed by this row.

## Closed bounded task — nested `IfThen` carrier coverage

Task: `JOINIR-GENERIC-NESTED-IF-CARRIER-COVERAGE0-D2-B4-S2A`

Decision: accepted and implemented — one `cfg(test)`-only evidence shape;
production remains stopped.

### Change

- Add one new test sibling under `route_entry/registry/` for a parsed outer
  loop containing an inner loop, one nested `IfThen` write to the outer `j`, a
  separate canonical inner `j` step, the outer `i` step, and a post-loop `j`
  read. Do not add an `else`, shadowing, planner-required, or second accepted
  source shape in this row.
- Observe the decisive target at
  `Body(0)/LoopBody(0)/LoopBody(0)/IfThen(0)/Target` and the post-loop read at
  `Body(1)/Value`. A minimal `pub(super)` projection from the existing
  accepted-plan test helper is allowed only under `cfg(test)`; no production
  API or re-export is allowed.

### Contract

- The resolver-issued `BindingRefV1` for the nested write and post-loop read is
  identical, owned by the same function/source/frame, and its declaration
  scope is a strict ancestor of the `IfThen` write scope. The resolved forest
  contains exactly the outer and inner loops.
- Release and Strict each record raw `[V0, V1]`; fresh direct V0 and V1 each
  reach `LowerSome` with first effect owner `GenericComposer`; V1 reports
  `CompleteRecursiveCarrier(["j"])`; per-route fresh repeats are stable; and
  the alpha-normalized V0/V1 semantic digests remain explicitly different.
- The existing legacy witness remains V0 terminal with V1 untried and no debt
  receipt. That mismatch is retained as evidence and is not treated as proof.
- Existing shadowing and planner-required witnesses remain neighboring reject
  guards. This row does not infer a winner, close parent D2, or authorize a
  selector, policy, Recipe, JoinSig, PHI, physicalizer, Builder, MIR, backend,
  Retry, fallback, or runtime change.

### Done

- `generic_d2_b4_s2_nested_if` is green under an explicit clean environment;
  existing `generic_d2_b4_s2`, `generic_d3_bindingref`, and generic Both matrix
  filters remain green; all touched Rust files stay below 800 lines; production
  caller/import census remains zero; pointer and diff guards are green.

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2_nested_if -- --nocapture
```

- The implementation commit must also update this card, the parent Generic
  SSOT, Generic stage-matrix reference, Generic README, resolved-semantics
  README, `CURRENT_STATE.toml`, the current dashboard/workstream, and every
  affected `docs/reference/**` navigation or status index. Reference closeout
  is part of implementation completion and must not be deferred.

### Implementation closeout — 2026-08-05

The implementation commit adds the 292-line sibling
`generic_nested_if_carrier_evidence_tests.rs`, a test-only source helper in the
796-line accepted-plan file, and one `cfg(test)` module registration. The
parsed fixture keeps the inner canonical `j` step separate from the nested
`IfThen` write. Release/Strict raw schedules are `[V0, V1]`; direct V0/V1
rows are fresh `LowerSome` observations owned first by `GenericComposer`, with
stable distinct digests. Resolver-issued BindingRefs, source/frame identity,
strict ancestry, the two-member loop forest, `CompleteRecursiveCarrier(["j"])`,
and the V0 terminal witness are all asserted. Production caller/import census
is zero; no selector, Recipe, PHI, Builder, MIR, Retry, fallback, or runtime
authority moved. The focused command above and the adjacent D2/D3 filters are
green under a clean environment; pointer, artifact, diff, and line-budget
guards are part of the same closeout.

### Stop

- Return to design if the source does not naturally produce `Both`, the inner
  canonical step cannot be observed separately, any evidence requires a
  production seam, or a test helper would hold overlapping `ScopedTestConfig`
  guards. A green digest difference remains evidence of unresolved semantics,
  never a selection rule.

## Stop condition after the projector witness

The projector witness does not authorize a neutral eligibility issuer or a
Builder packaging adapter. Parent Generic D2 remains `UnresolvedStop` until
the nested `Both` winner/disjointness, full overlap matrix, first-effect and
alpha-normalized candidate evidence, fresh-repeat stability, and
no-debt-to-different-winner checks are complete. The current Generic facts
product is still Builder-local and AST-bearing, so introducing a neutral
snapshot owner now would create a second semantic boundary. S2A is now closed
as bounded evidence; the next work remains the upper co-sealed source-to-
selection design stop. After D2 acceptance, the production order is resolved projector -> neutral
snapshot/eligibility issuer -> private Builder adapter -> selector consumer.

This card is a design consultation boundary. A green test-only matrix does
not authorize production selection or parent D2 closeout.

## Closed bounded premise — top-level CompoundAssignment (`D2-S4`)

The selected task was
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-TOPLEVEL-COMPOUND-PREMISE0-D2-S4`,
recorded in
`joinir-generic-resolved-carrier-source-matrix-toplevel-compound-premise-d2-s4-task-2026-08-05.md`.
The collector currently gives nested `CompoundAssignment` an explicit
`Unavailable("CompoundAssignment")` arm while a top-level compound can fall
through the non-nested path. This producer asymmetry must be observed before
selecting a source-backed `CompleteNoRecursiveCarrier` row.

The task was cfg(test)-only and result-open: it records the actual parsed AST,
resolver forest/BindingRef, source/frame identity, facts label, Release/Strict
raw schedule, and fresh-repeat stability. The outcome may be exact
`CompleteNoRecursiveCarrier`, `Unavailable("CompoundAssignment")`,
`Ambiguous(...)`, or typed `NoStandaloneRow`; no outcome is preselected. No
collector widening, neutral issuer, selector, Legacy/winner policy, Builder,
MIR, Recipe, PHI, Retry, fallback, or production handoff is authorized.

The implementation observed typed `NoStandaloneRow`: parser/resolver source
identity and the one-member forest are present, but the current facts product
is absent and the measured Release/Strict raw schedule is `[]`. Fresh repeats
keep the source/frame/binding shape and schedule stable while using a distinct
invocation owner. The row therefore does not claim
`CompleteNoRecursiveCarrier`, `Unavailable`, V0-only, Legacy, winner,
eligibility, or precedence. It adds no collector widening, neutral issuer,
selector, Builder, MIR, Recipe, PHI, Retry, fallback, or production handoff.

The implementation closeout updated this card, the Generic SSOT, the Generic
stage-matrix reference, both Generic READMEs, current mirrors, and the
artifact manifest in the same commit. A separate parsed `Both/NoRecursive`
row may be reconsidered at the parent design stop; top-level Compound remains
an explicit `NoStandaloneRow` boundary for the current facts owner.

## Accepted implementation child — NoRecursive disposition (`D2-S5-S1`)

The design boundary was
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`,
recorded in
`joinir-generic-resolved-carrier-source-matrix-norecursive-disposition-d2-s5-d0-design-2026-08-05.md`.
Workers found that parsed `CompleteNoRecursiveCarrier` is not one downstream
meaning: dedicated simple-while, local/effect V1-only, and unsupported/facts-
absent shapes can share or resemble the label. The existing D3 projector is
also a two-member recursive shape and cannot be silently reused for a
one-member source.

The boundary is now accepted for one cfg(test)-only implementation child,
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-BOTH-NORECURSIVE0-D2-S5-S1`,
recorded in
`joinir-generic-resolved-carrier-source-matrix-both-norecursive-d2-s5-s1-task-2026-08-05.md`.
It fixes one flat Assignment candidate, disposition, one-loop projector
boundary, and the measured raw-schedule owner. The candidate is
`loop(j + m < n) { j = j + 1 }` with a post-loop `j` read; the provisional
disposition is typed `UnresolvedStop(NonRecursiveOutOfTarget)` when exact
`CompleteNoRecursiveCarrier` is observed. Facts absence or empty raw schedule
is typed `NoStandaloneRow`. Simple-while, local/effect, CompoundAssignment,
eligibility, Legacy, winner, selector, neutral issuer, Builder, MIR, Recipe,
PHI, Retry, fallback, and production handoff remain outside the task.

S1 is now closed as cfg(test)-only evidence. The exact parsed flat shape
produces one loop member, exact `CompleteNoRecursiveCarrier`, and measured
Release/Strict raw `[V0,V1]`; fresh repeats preserve origin/source/frame/
BindingRef/facts shape with a distinct function owner. Its only disposition is
typed `UnresolvedStop(NonRecursiveOutOfTarget)`. No eligibility, Legacy,
winner, selector, Recipe, PHI, Builder, MIR, Retry, fallback, or production
handoff moved. The parent D3 design stop remains open for the remaining matrix
and winner/disjointness work.

## Selected docs-only child — disposition matrix and winner/disjointness

The next child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`,
recorded in
`joinir-generic-resolved-carrier-selection-disposition-matrix-d3-s1-design-2026-08-05.md`.
It is a docs-only design stop. It partitions every source-backed row into
`ResolvedCandidate`, `LegacyPreserveExistingSchedule`, `UnresolvedStop`,
`NoStandaloneRow`, or `NotYetObserved`, and fixes the two-stage
pre-effect-qualification/post-effect-corroboration protocol for the natural
recursive Both candidate. The current typed result remains
`UnresolvedStop(WinnerCorrectnessUnavailable)`; V1 winner, V0 suppression,
neutral issuer, selector, or production handoff is not authorized.

The child also corrects the handoff pseudotype: the future opaque wrapper
contains `GenericCarrierFactsSnapshotV1`, not AST-bearing
`CanonicalLoopFacts`. The selected V1-only Local row is now closed as
cfg(test)-only evidence; no production edit or handoff is authorized.

D3-S1 is now accepted as a design-only policy boundary. Its selected child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-V1ONLY-LOCAL0-D3-S1-S1`,
with task card
`joinir-generic-resolved-carrier-source-matrix-v1only-local-d3-s1-s1-task-2026-08-05.md`.
The child is closed as one parsed cfg(test) witness. It preserves the
two-column evidence/selection partition and co-seals `V0 facts=false`,
`V1 facts=true`, lexical body-local presence separately from the router's
dedicated `has_body_local=false` flag, actual frame flags, no recipe contract,
and raw `[V1]`. Its typed result is `Observed` plus
`UnresolvedStop(V1OnlyNonRecursive)`. Production handoff remains stopped.

The bounded child was
`JOINIR-GENERIC-RESOLVED-CARRIER-CANDIDATE-STAGE-SOURCE-BRIDGE0-D3-S1-S2-D0`,
with task card
`joinir-generic-resolved-carrier-candidate-stage-source-bridge-d3-s1-s2-task-2026-08-05.md`.
It is a cfg(test)-only source bridge: the parsed natural-Both source,
resolver forest/BindingRef, and fresh V0/V1 plan projections are co-sealed in
one witness. Existing synthetic `both_body()` evidence remains non-authority;
the parsed observer and actual plan rows meet only as a label-backed
projection. A `diagnostic_name()`/final/PHI label match is corroboration only;
no typed BindingRef-to-ValueId provenance or full-function post-loop return
parity may be invented. The result remains
`Observed + UnresolvedStop(WinnerCorrectnessUnavailable)`, and production
selection stays stopped.

The S2 source bridge is now closed as cfg(test)-only evidence. Its parsed
natural-Both witness co-seals resolver forest/BindingRef obligations with
fresh V0/V1 plan projections, Release/Strict raw `[V0,V1]`, direct
`LowerSome`/`GenericComposer`, forward/reverse snapshot stability, and
distinct resolver owners. V0's outer `j` projection is absent while the
nested projection retains it; V1 records outer `j`, `loop_carrier_j`, and
`loop_step_in_j`, with plan-local final/PHI agreement checked as corroboration
only. Planner-required remains raw `[V1]` and typed unresolved. The actual
legacy witness is V0 terminal/no-debt; the synthetic DTO debt/V1-terminal
negative remains unresolved. This does not add typed BindingRef provenance,
full post-loop return parity, winner correctness, or any production
issuer/selector/Recipe/PHI/Builder/MIR caller.

## Next design stop — typed provenance handoff (`D3-S2`)

The next frontier is the docs-only card
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-HANDOFF-DESIGN0-D3-S2-D0`,
recorded in
`joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md`.
S2 supplied enough evidence to reject another label-backed witness, but not
enough to select V1 or suppress V0. This card must first define one typed
source-to-plan relation, an AST-free neutral snapshot, and a non-Clone opaque
selection input. It must also define the pre-effect reject matrix and owner
order before any implementation child is selected.

The following remain separate deferred rows after that design:

```text
scalar full-function Return/PHI projection (cfg(test) only, if the existing
  return owner can observe a parsed source without a production seam)
natural V0 post-effect debt -> different V1 winner (requires a real producer;
  synthetic debt/failure injection is forbidden)
Home-bearing result/finalization meaning (separate Home design; current
  Generic carrier evidence is scalar-only)
```

Until the typed provenance relation, V0 disjointness, candidate isolation,
fresh-repeat stability, and the no-debt/different-winner boundary are proven,
the disposition remains `UnresolvedStop(WinnerCorrectnessUnavailable)`, the
old scheduler remains authority, and no neutral issuer, selector, Recipe,
PHI, Builder, MIR, Retry, fallback, or route cutover is authorized.

The first bounded implementation child under D3-S2 is
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`,
recorded in
`joinir-generic-resolved-carrier-provenance-observation-d3-s2-s0-task-2026-08-05.md`.
It is cfg(test)-only and observes resolver-issued forest/frame plus exact
`BindingRefV1` role/ancestry relations through the existing compiler-side
projector. It deliberately does not create a production neutral snapshot,
Generic `LoopBindingKeyV1` issuer, seed, opaque input, selector, or Builder
caller; any need for those returns to this D3-S2 design stop.

The S0 observation child is now closed. Its natural source witness records the
resolver-owned forest/frame, outer-to-inner parent, exact write/read sites, and
strict ancestry; shadowing, foreign owner, forest-shape, and frame mismatch
reject before any Builder effect. The four focused tests pass, the production
caller/import census is zero, and `artifact = none`. This evidence does not
create a Generic snapshot/key issuer or close winner, Return/PHI, Home, or
debt correctness; the current row returns to the D3-S2 design stop. A later
premise audit found that the forest/frame coordinates do not carry a resolver
owner/invocation brand, so equal-origin cross-session forest/role/frame mixing
is still possible. The cross-session brand audit must precede any provenance
product or neutral snapshot and is now the next design candidate.
