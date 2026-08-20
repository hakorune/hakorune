---
Status: Active design stop
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-DISPOSITION-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-transport-d0-2026-08-20.md
ProductionCaller: none; design only
ReplacementCell: one source-owned typed Script disposition and shared source identity
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-DISPOSITION-D0

## Six-line brief

Decision: Define one source-owned canonical Script disposition that combines
the existing direct-static observation and exact physical-input facts. Do not
implement or connect the canonical caller in this D0.

Source authority + canonical issuer: one future issuer at the retained
canonical Script source boundary must co-seal source identity, observation,
Facts/Recipe/Join inputs, and the physical-input result. Existing target
inventory and result-bundle issuers are inputs to that owner, not competing
disposition authorities.

Non-authority: `noncandidate_count`, AST pointers, path/display strings,
`RawScriptBodyRecipeV1`, selected claim ledgers, Builder ordinals, target names,
or a canonical-side rescan cannot issue a disposition or source identity.

Fail-fast boundary: before any detached physical effect, validate source/window,
owner/site, complete body coverage, target/key/cardinality, ordered operands,
terminal, and representation. Candidate-mixed, missing, duplicate, foreign,
stale, or physical-input failures are `IntegrityInvalid`, never `NonCandidate`.

Smallest next slice: design the single `DirectStatic | NonCandidate |
IntegrityInvalid` owner and its A-to-C-to-B handoff. A later I0 may add one
move-only carrier to `CanonicalCoreSourcePlanCompileRequestV1`.

Non-claims: no source admission change, canonical production switch, detached
Call/publication/Return change, raw fallback or retirement, JSON-v0/VM change,
ABI/backend/performance claim, or selected-normal cutover.

## Existing evidence and exact gap

The current selected lifecycle provides two useful but non-equivalent inputs:

```text
VerifiedScriptDirectStaticCallTargetInventoryV1::issue
  source_call_target/script_direct_static.rs:119-246
  observes all retained MethodCall sites and the candidate subset.

VerifiedScriptDirectStaticResultBundleV1::issue
  builder/normal_script_direct_static_result_bundle.rs:137-276
  validates candidate result/representation rows.
```

Neither issuer emits the required three-state disposition. A nonzero or zero
`noncandidate_count` is not enough to prove completeness, and a candidate
that fails after target observation is an integrity error, not a raw-recipe
fallback. The existing `VerifiedScriptDirectStaticPhysicalInputV1::issue`
can validate the physical rows, but it must be consumed by the same owner that
issued the source disposition; calling it independently at the canonical
entry would create a second physical-input authority.

The source identity gap is equally concrete:

```text
selected semantic Facts: AST-owned identity
canonical front door: display/path receipt identity
```

A filename, display path, or pointer address cannot join these products. The
future owner must co-seal one canonical source identity (for example an
explicit source-bytes digest plus profile/seal) at the parse/source boundary,
then carry it through the source plan, disposition, request, and detached
entry. No identity may be reconstructed from AST names or statement ordinals.

## Required disposition shape (design only)

```text
DirectStatic(carrier)
  exactly one complete direct-static candidate
  exact source identity/window/owner/site
  exact target/key/cardinality and ordered operand rows
  ExactI64 and FinalSequence | RootReturn

NonCandidate(observation)
  complete retained-row observation
  every row explicitly non-candidate
  zero candidate rows and zero integrity errors

IntegrityInvalid(reason)
  missing/duplicate/foreign/stale row
  candidate mixed with incomplete coverage
  source identity/window/owner drift
  target/arity/operand/terminal/representation mismatch
  physical-input issuance failure
```

`NonCandidate` must never be synthesized because a carrier is absent. A
Script source with an observed candidate but no valid carrier is terminal
`IntegrityInvalid`; the existing `RawScriptBodyRecipeV1` route may only be
selected from an explicitly issued `NonCandidate` disposition.

## A-to-C-to-B ownership

```text
A  retained source/Facts/Recipe/Join producer
   observes one canonical Script source and lends all exact inputs

C  CanonicalScriptDirectStaticDispositionV1 (design owner)
   co-seals identity + observation + physical input exactly once

B  CanonicalCoreSourcePlanCompileRequestV1
   transports the move-only disposition and performs no re-resolution

detached entry kernel
   consumes DirectStatic only; existing Call/publication/exit owner
```

The selected-normal Builder bridge remains a separate route until an explicit
production cutover. It may not issue a second canonical disposition for the
same production call. The canonical request must consume a typed Script
decision, not an optional carrier that silently falls back.

## Acceptance matrix for the later I0

Positive:

- one source disposition issuer emits exactly one decision and one identity;
- complete direct-static candidate moves once with unchanged owner, sites,
  keys, operands, terminal, and representation;
- complete zero-candidate observation emits `NonCandidate` and keeps the raw
  recipe route unchanged;
- detached kernel remains the sole Call/publication/exit owner.

Negative:

- candidate mixed with missing/duplicate/foreign/stale rows;
- target inventory and result bundle from different source identities;
- AST pointer, filename, ordinal, or name-only re-pairing;
- physical input failure converted to `NonCandidate`;
- missing carrier for a known candidate falling back to Raw;
- clone/replay/second issue of the disposition;
- selected-normal, Deferred, Compatibility, or RawLegacy product presented as
  canonical Script disposition.

## NoSafeSlice conditions

Keep this row at design stop if the issuer cannot see all retained Script rows,
if source identity requires pointer/name/path inference, if existing input
issuers cannot be co-sealed without a second authority, or if the canonical
request must reparse/re-resolve the AST. Do not open the carrier I0 until these
conditions are closed and the source identity contract is accepted.

