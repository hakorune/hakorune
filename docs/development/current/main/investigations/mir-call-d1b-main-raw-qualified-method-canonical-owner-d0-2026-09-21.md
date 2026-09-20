---
Status: accepted__2026-09-21__MainQualifiedMethodCanonicalOwner
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-CANONICAL-OWNER-D0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-qualified-method-acceptance-d0-2026-09-21.md
Implementation permission: true for the bounded I0 below; no other Main
  MethodCall family is admitted
NextCard: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-CANONICAL-OWNER-I0
---

# Main qualified MethodCall canonical owner D0

## Six-line brief

```text
Decision: define one canonical source-to-MIR owner for qualified Main
  StaticBoxMethod calls by extending the existing trivial Binding-SSA owner;
  do not add a second body lowerer or reinterpret the call as FunctionCall.
Source authority + canonical issuer: the existing resolver MethodCall source
  ledger (`VerifiedResolvedMethodCallSourceV1`) and the retained qualified-
  receiver relation (`VerifiedQualifiedReceiverCatalogRelationV1`) are
  co-sealed through the installed Main source input; no AST rescan or name-only
  target discovery.
Non-authority: raw `inner.lower_body`, Script inventory, compatibility/VM
  routes, DirectCall loans for bare FunctionCall, and result publication ABI.
Fail-fast boundary: exact Cataloged Main owner/site, direct-owner or alias
  relation, declaration key, ordered argument sites, MethodCall expression
  coverage, existing selected callee result header `ExactI64`, canonical
  binding/CFG completion, and zero residual rows.
Smallest next slice: add one per-site qualified-method Recipe row to the
  existing `CanonicalTrivialBindingSsaPlanV1`/`VerifiedTrivialCanonicalOwnerV1`
  product, consume it from `CanonicalTrivialSsaLowererV1`, recursively lower
  only existing InlineI64 argument shapes, and emit through the existing
  target-only static terminal.
Non-claims: no code, target lookup, fallback repair, production switch, VM,
  backend parity, new result ABI, or LegacyCallV0 deletion outside this I0.
```

## Why the current owner is insufficient

`CanonicalTrivialSsaLowererV1` closes resolved binding authority, but its
expression Recipe currently admits literals, variables, binary expressions,
block expressions, and bare `FunctionCall`. The qualified `MethodCall` shape
is outside that owner. The installed Main adapter therefore uses the raw
source hook only; its qualified relation handoff is physically wired, but the
source-backed Main acceptance still reaches the existing
`canonical_function_session/cleanup_failed` authority imbalance.

The read-only owner audit fixes the boundary precisely: `main_root.rs::
lower_app_main_root_body_v1` enters raw `inner.lower_body`, while
`trivial_ssa/lowerer.rs::lower_expr` has no qualified `MethodCall` arm. The
completion chain that could close this gap is already named—canonical lowerer
`lower` → `finish_profile_close` → `CanonicalSsaFunctionSessionV2::
finish_for_draft_seal` → `resolved_binding_state.finish(owner)`—but its current
Recipe cannot consume the qualified row. This is why the physical I0 bridge
is evidence of handoff only, not a source-to-MIR acceptance receipt.

Selecting a new owner must preserve the current resolver relation and the I0
target-only bridge. Adding a direct `finish` call, treating MethodCall as a
bare FunctionCall, or routing back to Script/compatibility would create a
second semantic authority and is rejected by this D0.

## Accepted owner tuple

The canonical owner is the existing Binding-SSA family. Its resolver Facts
remain `VerifiedResolvedMethodCallSourceV1`; the package relation remains the
only target/alias co-issuer. I0 adds a small qualified-method Recipe row to
the existing trivial profile, keyed by the exact `SourceExprSiteV1`, carrying
the already co-sealed declaration key, ordered argument sites, and the
existing selected callee result representation. Only `ExactI64` is admitted
in this slice. The row is not a target lookup, ABI publication, or name-based
inference.

The physical consumer stays `CanonicalTrivialSsaLowererV1` and its existing
`CanonicalSsaFunctionSessionV2` finish chain. Its MethodCall arm claims the
Recipe row, lowers each argument through the same canonical `lower_expr`
recursion with `InlineI64` enforcement, and calls
`emit_static_global_target_value_terminal_v1`. The installed Main adapter
selects this owner before raw `inner.lower_body`; unsupported shapes remain
outside this owner and are rejected or left on their already-documented
non-selected route without a new fallback.

The package-to-lowerer handoff is a narrow recipe port, not a second ledger:
the Main relation is taken once, each exact site is consumed once, and the
adapter must prove the selected callee physical header is `I64` before the
row is issued. Lowering failure discards the canonical session; successful
lowering must finish the existing profile, CFG, and binding state, then prove
the relation has no residual rows.

## I0 implementation and evidence boundary

1. Extend the existing trivial profile Facts/Recipe product with the
   qualified-method row and a finite Main admission policy.  Missing,
   duplicate, foreign, non-`QualifiedUnbound`, non-I64, unsupported argument,
   and residual rows are named rejects before Builder effects.
2. Issue the row only from the resolver ledger plus the retained Main
   relation.  Validate source site, receiver identity, declaration key,
   ordered argument sites, selected result header, and owner brand together.
3. Add the canonical lowerer MethodCall arm and the recipe port.  Reuse the
   existing session finish path and target-only terminal; do not call raw
   child lowering for an admitted row.
4. Switch only the installed App Main caller for this admitted shape.  Keep
   compatibility, VM, Script, `me`, instance, nested-owner, and non-I64 rows
   outside the I0 boundary.
5. Add focused positive direct-owner and import-alias evidence, negative
   foreign/duplicate/non-I64/argument-site/residual evidence, and one stable
   caller guard.  Acceptance requires source Facts -> Recipe -> canonical
   lowerer -> physical terminal plus cleanup and no residual relation rows.

## Finite inventory and required decision

The inventory is qualified direct canonical-owner and verified import-alias
MethodCall rows in the Cataloged App Main root. It excludes bare functions,
`me.method`, instance methods, nested-owner inheritance, ScriptRoot,
compatibility/VM, result publication, and legacy retirement.

The next design decision must name the exact Facts/Recipe/physical owner that
can lower this expression, consume the ordered argument-site ledger, close
resolved binding/CFG completion, and report missing/foreign/duplicate/residual
rows before effects. Until that tuple is accepted, implementation remains
forbidden and the current I0 stays a physical-handoff-only receipt.
