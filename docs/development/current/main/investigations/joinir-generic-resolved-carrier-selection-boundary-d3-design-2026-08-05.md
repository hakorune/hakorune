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
capability identity must never be reconstructed from route IDs or registry
order. `LegacyFacts` is constructible only for a typed `NotApplicable` or
`ProvenOutsideTarget` disposition. A target row with missing/invalid handoff is
`UnresolvedStop`, not a silent legacy fallback.

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
production selector callers of BindingRef capability: 0
current Both trace: [V0, V1] -> V0 success; no V1 debt attempt
```

This card is a design consultation boundary. A green test-only matrix does
not authorize production selection or parent D2 closeout.
