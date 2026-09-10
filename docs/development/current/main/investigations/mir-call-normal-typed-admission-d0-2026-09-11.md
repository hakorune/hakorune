---
Status: Selected design stop
Date: 2026-09-11
Decision: MIR-CALL-NORMAL-TYPED-ADMISSION-D0
Parent: docs/development/current/main/investigations/mir-call-canonical-corridor-guard-i0-2026-08-20.md
ProductionCaller: `try_compile_published_view_object` via the normal published callback
ReplacementCell: existing `PublishedMirBackendView` admission; no second view or receipt
---

# MIR-CALL-NORMAL-TYPED-ADMISSION-D0

## Six-line brief

Decision: Close the selected normal Static/Free/Print admission so a
`LegacyCallV0` cannot be mistaken for a canonical typed row merely because its
callee is typed. This is a bounded admission fix before GUARD-I0, not a global
Call-schema retirement.

Source authority + canonical issuer: the final published `MirModule` and the
existing `MirInstruction::call` / `MirCall::new` issuers. The existing
`PublishedMirBackendView::try_new` is the sole selected normal admission owner
and may only classify the published shape it receives.

Non-authority: `LegacyCallV0.func`, names, `ValueId::INVALID`, C whitelist
strings, JSON-v0, VM tail lookup, Dynamic V4, and the R7 observation manifest.
No source target is reissued here.

Fail-fast boundary: `PublishedMirBackendView::try_new` (or its existing
private validation helper) must reject a mixed selected normal module before
`emit_published_view_body`, C transport, or object creation, with a stable
named terminal. A LegacyCallV0-only module remains explicit compatibility.

Smallest next slice: add one existing-owner admission predicate and focused
fixtures for canonical typed-only, Legacy-only compatibility, and mixed typed
plus Legacy input. Keep the current typed row projection and compatibility
caller; do not add a fallback, resolver, route, or semantic receipt.

Non-claims: no `LegacyCallV0` deletion, JSON-v0 migration, Dynamic V4 change,
VM/WASM parity, backend rewrite, source semantic change, or performance claim.

## Acceptance

The positive canonical fixture contains only `MirInstruction::Call(MirCall)`
with `Callee::Global` for an already published Static/Free/Print target and
reaches `CanonicalTyped`. The compatibility fixture contains only explicit
`LegacyCallV0` and remains `ExplicitCompatibility` for its existing caller.
The negative fixture starts from a valid typed module and mutates one selected
call to `LegacyCallV0` while retaining its typed callee; it must return the
named pre-artifact admission error. The fixture must assert that no body JSON,
C invocation, or object is produced after rejection.

No retry or name repair is allowed. The guard row remains parked until these
three outcomes are observable. If the existing callback cannot distinguish a
selected normal module from compatibility-only input without changing an
unrelated owner, return to `NoSafeSlice` rather than widening this row.

## Required order

```text
normal typed admission
  -> existing canonical corridor guard revalidation
  -> any LegacyCallV0 retirement row
```
