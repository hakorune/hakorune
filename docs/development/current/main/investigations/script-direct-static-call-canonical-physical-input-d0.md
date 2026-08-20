# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PHYSICAL-INPUT-D0

Status: design stop; selected-normal physical bridge is closed, but the
canonical detached Script physical input has no named source-backed issuer.
No code, fixture, route switch, or semantic receipt is authorized by this
card.

Parent: `script-direct-static-call-target-d0.md`

## Current six-line brief

Decision: open one design-only BoxCount for the canonical Script physical
input. It must carry the already-issued direct-static source/Facts/Recipe/Join
meaning into a detached canonical session; it must not reuse the selected
claim ledger or widen the scalar Script recipe.

Source authority + canonical issuer: the existing
`VerifiedScriptDirectStaticResultBundleV1` and
`VerifiedScriptDirectStaticJoinHandoffV1` remain semantic authority. This D0
must name one source-backed producer that co-seals source identity, exact
call/receiver/ordered-argument sites, canonical target, `ExactI64`, and
`FinalSequence | RootReturn` before any detached physical effect.

Non-authority: `RawScriptBodyRecipeV1`, AST/name/ordinal rescans, callable-key
conversion, selected `ScriptDirectStaticClaimLedgerV1`, `ValueId`/`MirType`,
generic Call receipts, `ScriptPhysicalExitCommitV1`, backend markers, and raw or
compatibility publication cannot issue the canonical input.

Fail-fast boundary: missing, foreign, duplicate, reordered, or drifted source
payload; absent terminal/exit relation; unsupported representation; or a
producer-to-handoff path that cannot preserve the exact sites must stop before
physical allocation/effects as `NoSafeSlice`. No fallback, retry, or inferred
empty row is permitted.

Smallest next slice: design the single AST-free canonical input contract, its
one producer, its one detached-session handoff, and its one intended consumer.
Keep source admission, existing Facts/Recipe/Join, selected-normal bridge, and
all physical emission unchanged. If no existing issuer can satisfy the whole
contract, close this D0 as `NoSafeSlice` without creating a `Verified*` receipt.

Non-claims: no canonical consumer implementation, Script exit/Return or ABI
integration, production switch, raw/compatibility/Deferred retirement, MIR
Call representation cleanup, backend change, performance measurement, or
C-parity result.

## Evidence and owner census

The selected-normal bridge is already a complete bounded BoxShape:

```text
Bundle -> Recipe -> JoinHandoff -> claim
  -> ordered arguments -> existing generic Call receipt
  -> Script ExactI64 publication -> success-only scope finish
```

That path is not the canonical detached owner. The existing scalar
`RawScriptBodyRecipeV1` accepts scalar expressions and does not carry a
MethodCall with ordered argument sites. The callable-keyed static-result owner
accepts cataloged callers, not `ScriptRoot`. `ScriptPhysicalExitCommitV1` owns
final Return/signature commit only; it cannot infer a Call target or argument
payload. The selected claim ledger is session-local and cannot become a second
semantic source.

The production old edge therefore remains intentionally live:

```text
raw MethodCall AST entry
  -> StaticReceiver
  -> Absent/non-Script -> existing static handler
```

The canonical row is not eligible for retirement until every admitted
`StaticReceiver` has exact Bundle/Join coverage and the deferred/compatibility
families have an explicit canonical owner or an explicit non-production stop.

## Design boundary

The D0 must answer, without implementation:

1. Which existing source/Facts/Recipe/Join owner issues the detached input?
2. How are the exact argument expression sites carried without AST cloning or
   re-parsing?
3. How are `FinalSequence` and `RootReturn` represented without reconstructing
   completion from a `ValueId` or `MirType`?
4. Which single detached session consumes the input, and which existing
   receipt/exit owner does it call exactly once?
5. What complete caller census proves that the selected old edge can later be
   retired without fallback or route reselection?

No implementation row may open until all five answers share one source-bound
product and one fail-fast boundary. A missing answer is development
`NoSafeSlice`, not an empty/default product.

## Acceptance for this D0

- The proposed input is AST-free and identity-bound to the existing
  source/Facts/Recipe/Join rows.
- Producer, handoff, and intended consumer are each unique and named.
- Call, receiver, ordered argument, target, representation, terminal, and
  source-owner cardinality are exhaustive and drift-checked.
- Missing/foreign/duplicate/order/site/representation/exit cases are rejected
  before physical effects.
- `RawScriptBodyRecipeV1`, selected claim state, generic callable publication,
  and `ScriptPhysicalExitCommitV1` are not promoted to new authority.
- Compatibility, Deferred, RawLegacy, StaticThis, typeop, and reserved routes
  remain explicit non-claims.
- No Rust source/check file crosses the 760-line split trigger or 800-line
  hard boundary; this D0 adds no production code.

## Future order (not authorized here)

```text
CANONICAL-PHYSICAL-INPUT-D0
  -> canonical physical consumer I0
  -> one production cutover
  -> raw/compat caller-zero and old-edge retirement
```

MIR Call dual-representation retirement, metadata consumer census, builder
root-tail cleanup, main integration, and branch protection remain separate
ordered lanes.
