---
Status: active design stop — co-sealed source-to-selection handoff
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Predecessor: joinir-generic-nested-carrier-d3-bindingref-design-2026-08-05.md
Decision: provisional — no production selector change until handoff seals
---

# Generic resolved-carrier source-to-selection boundary

## Boundary

The scoped D3 BindingRef matrix is closed as test-only evidence. Production
selection cannot consume it yet: the current facts product is carrier labels,
the registry selector receives `CanonicalLoopFacts` only, and the router's
`LivePreflightFrameV1` has no resolver/source capability. Adding a V0
suppression arm now would create a second semantic authority.

This card designs the missing co-sealed handoff. It does not implement V0
suppression, V1 precedence, Recipe/JoinSig/PHI production, Retry/fallback
removal, or a route/Builder/MIR/backend cutover.

## Required co-sealed capability

The future production handoff must be one sealed value created from the same
source invocation:

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
  facts: CanonicalLoopFacts
  eligibility: VerifiedResolvedCarrierEligibilityV1
  seed: PreflightSeedV1
  invocation: InvocationSealV1
}
```

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

## Production-free protocol matrix and acceptance

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

Every reject is typed, pre-effect, and `UnresolvedStop` (or an explicitly
terminal freeze target); no retry or fallback is allowed. The matrix must prove
that one `InvocationSealV1` binds source, facts, seed, and frame, and that a
mismatched seal rejects. Its acceptance receipt includes:

```text
proposed capability production caller/import census = 0
selection/router/frame/handler/composer edits = 0
parent D2/winner-equivalence/runtime claims = 0
```

Only after this protocol and the full D2 matrix are accepted may a separate
implementation card add the neutral capability and selector input.

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
  remains Builder-local and AST-bearing; `PreflightSeedV1` and
  `InvocationSealV1` belong to the later router/adapter boundary, and carrier
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
  fallback, or parent-D2 winner claim was added. Touched Rust files are 515 and
  590 lines; the focused projector (5) and protocol (8) filters pass.

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

## Selected bounded task — source-backed planner suppression

Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-PLANNER-SUPPRESSION0-D2-S1`

Decision: accepted as one cfg(test)-only mode-co-seal row. It repairs the D1
evidence boundary without selecting a winner or creating a production owner.

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

### Stop

- Return to design if raw is not exactly `[V1]`, V0 facts remain, the recursive
  carrier or identity seal is incomplete, mode is synthetic/split, a second
  source shape or production API is needed, or any Builder effect occurs.
  V0-only, Unavailable, Ambiguous, NoRecursive, and full parent D2 remain
  separate unresolved rows.

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
