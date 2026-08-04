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
  -> canonical GenericLoopFacts
  -> LivePreflightFrameV1 mode/raw schedule
  -> VerifiedResolvedCarrierEligibilityV1
  -> pure registry selection input
```

`VerifiedResolvedCarrierEligibilityV1` is a capability proposal, not yet a
code type. It must co-seal:

```text
function origin and source kind
exact outer/inner loop sites and frame identity
same strict-ancestor BindingRefV1 relation
complete recursive carrier observation
mode and raw schedule snapshot
facts/source ownership identity
```

The selector may consume the capability but must not re-read AST, names, route
IDs, legacy tags, plan digests, or runtime receipts. The capability must be
unforgeable outside its resolver/facts owner and must not be reconstructible
from a `Vec<String>` carrier label.

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
Rows without the sealed capability remain unresolved and retain the old
execution edge.

## Staged implementation proposal

No implementation is authorized by this design stop. If the matrix and owner
contract are accepted, the smallest later slice is:

```text
1. resolver/facts owner issues the co-sealed capability for one parsed positive
2. selector consumes only that capability in Release/Strict natural Both
3. shadowing and missing-capability rows fail before Builder mutation
4. V0 attempt is zero only for that exact proven class
5. all other rows preserve existing V0/V1 schedule and receipts
6. focused tests, caller census, and reference closeout land atomically
```

The first production slice must not touch `handlers/generic.rs`, composer,
physicalizer, PHI, MIR, backend, or global M10 scheduler deletion. Parent
Generic D2 remains `UnresolvedStop` until winner/disjointness and downstream
authority gates are complete.

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
production selector callers of BindingRef capability: 0
current Both trace: [V0, V1] -> V0 success; no V1 debt attempt
```

This card is a design consultation boundary. A green test-only matrix does
not authorize production selection or parent D2 closeout.
