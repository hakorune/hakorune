---
Status: design_stop__2026-09-21__MainQualifiedMethodCanonicalOwner
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-CANONICAL-OWNER-D0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-qualified-method-acceptance-d0-2026-09-21.md
Implementation permission: false; design decision only
NextCard: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-CANONICAL-OWNER-I0
---

# Main qualified MethodCall canonical owner D0

## Six-line brief

```text
Decision: define one canonical source-to-MIR owner for qualified Main
  StaticBoxMethod calls before attempting acceptance implementation.
Source authority + canonical issuer: the existing resolver MethodCall source
  ledger, the retained qualified-receiver relation, and the installed Main
  source input; no AST rescan or name-only target discovery.
Non-authority: raw `inner.lower_body`, Script inventory, compatibility/VM
  routes, DirectCall loans for bare FunctionCall, and result publication ABI.
Fail-fast boundary: exact Cataloged Main owner/site, direct-owner or alias
  relation, declaration key, ordered argument sites, MethodCall expression
  coverage, canonical binding/CFG completion, and zero residual rows.
Smallest next slice: choose whether to extend the existing canonical
  MethodCall-capable Recipe/physical owner or select another already-sealed
  owner; enumerate the finite qualified direct/alias rows and named rejects.
Non-claims: no code, target lookup, fallback repair, production switch, VM,
  backend parity, result ABI, or LegacyCallV0 deletion.
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
