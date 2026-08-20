---
Status: parked — depends on source-only A design stop
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

Source authority + canonical issuer: the landed
`CanonicalSourceBytesDigestV1` is issued once by `read_once` and moves
unchanged through `NormalSourcePlanReceiptV1` and
`CanonicalCoreSourcePlanCompileRequestV1`. The future
`CanonicalScriptDirectStaticDispositionV1` is the one C issuer: it must be
called from a source-backed A boundary that can lend the retained target,
Facts/Recipe/Join, and physical-input products, then co-seal them once.
Existing target inventory and result-bundle issuers are inputs to C, not
competing disposition authorities.

Non-authority: `noncandidate_count`, AST pointers, path/display strings,
`RawScriptBodyRecipeV1`, selected claim ledgers, Builder ordinals, target names,
or a canonical-side rescan cannot issue a disposition or source identity.

Fail-fast boundary: before any detached physical effect, validate source/window,
owner/site, complete body coverage, target/key/cardinality, ordered operands,
terminal, and representation. Candidate-mixed, missing, duplicate, foreign,
stale, or physical-input failures are `IntegrityInvalid`, never `NonCandidate`.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0`
must first define a Builder-free source producer and its shared A handoff.
Only after that design closes may C define the single `DirectStatic |
NonCandidate | IntegrityInvalid` owner and B add a move-only carrier.

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

The front-door identity issuer and transport are now resolved and implemented;
the cross-domain identity join is still part of this design stop:

```text
selected semantic Facts: AST-owned identity
canonical front door: display/path receipt identity
```

A filename, display path, or pointer address cannot join these products. The
front door now issues the explicit source-bytes digest at `read_once` and
carries it through the source plan and request, while selected Facts still
carry their own source-backed AST identity. The future A-to-C caller must
co-seal the digest/profile with that retained source context; C may not infer
equality from names, pointers, or statement ordinals. A missing or changed
digest/profile is `IntegrityInvalid`, never a new source or a raw fallback.

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
   must be invoked once for the canonical retained Script source and lend all
   exact inputs; current selected-normal A is not yet a canonical caller

C  CanonicalScriptDirectStaticDispositionV1 (design owner)
   co-seals identity + observation + physical input exactly once

B  CanonicalCoreSourcePlanCompileRequestV1
   must transport a required move-only Script disposition and perform no
   re-resolution; its current request carries only plan/admission/receipt

detached entry kernel
   consumes DirectStatic only; existing Call/publication/exit owner
```

The selected-normal Builder bridge remains a separate route until an explicit
production cutover. It may not issue a second canonical disposition for the
same production call. The canonical request must consume a typed Script
decision, not an optional carrier that silently falls back. If A cannot be
called from the canonical front door without reconstructing Facts or
re-resolving the AST, the row remains `NoSafeSlice` and no carrier is opened.

## Current caller gap and design boundary

The exact current paths are:

```text
normal_default_root_catalog_lifecycle.rs:453-623
  issues selected-normal target/result/Recipe/Join products (A)

canonical_core_dispatch.rs:94-130
  consumes only plan/admission/receipt (B input)

canonical_core_dispatch.rs:526-560
  still prepares RawScriptBodyRecipeV1 directly

normal_file_canonical_core_vm.rs:29-64
  is only an explicit reference caller, not the canonical production handoff
```

Therefore the next design decision is not “add a field and pass it through.”
It is to name one source-backed caller that invokes A before `compile_script`,
then lets C consume A's already-issued products. A canonical-side AST scan,
pointer/path join, or selected-normal product copy is forbidden. The typed
request shape should make Script input required (`DirectStatic`, explicit
`NonCandidate`, or terminal `IntegrityInvalid`) rather than an `Option` whose
absence silently selects Raw.

Worker consensus (2026-08-20): four read-only audits independently found the
same boundary. The digest I0 is closed; C is designable but its A-to-C-to-B
caller is absent; existing physical kernels remain non-authority; builder.rs
cleanup and physical bridge work stay parked.

## Acceptance matrix for the later I0

Positive:

- one source disposition issuer emits exactly one decision and one identity;
- the canonical front door invokes that issuer exactly once for the retained
  Script source and moves the same digest/profile into it;
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

## Remaining issuer and caller stop

The digest/profile identity contract is implemented and closed by commits
`376ee016b2` and `b99275e802`. The remaining work is split: first define the
Builder-free source-only A producer in
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0`, then define C and the
real A-to-C-to-B caller. None may invoke a second resolver, AST scan,
physical-input issuer, or Raw fallback.

## NoSafeSlice conditions

Keep this row at design stop if the issuer cannot see all retained Script rows,
if the canonical front door has no real A caller, if source identity requires
pointer/name/path inference, if existing input issuers cannot be co-sealed
without a second authority, or if the canonical request must reparse/re-resolve
the AST. Do not open the carrier I0 until the issuer and caller contract is
closed; the digest prerequisite alone is insufficient.
