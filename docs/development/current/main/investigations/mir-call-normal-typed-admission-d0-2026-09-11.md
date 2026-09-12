---
Status: closed__Fast__MirCallNormalTypedAdmission__2026-09-11
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

Fail-fast boundary: `PublishedMirBackendView::try_new_selected_normal` (the
existing view owner’s selected policy) must reject a mixed selected normal
module before `emit_published_view_body`, C transport, or object creation,
with a stable named terminal. The generic compatibility view remains
unchanged; a legacy-only selected module is outside this bounded mixed-shape
row until its producer is cut over.

Smallest next slice: add one existing-owner admission predicate and focused
fixtures for canonical typed-only, Legacy-only compatibility, and mixed typed
plus Legacy input. Keep the current typed row projection and compatibility
caller; do not add a fallback, resolver, route, or semantic receipt.

Non-claims: no `LegacyCallV0` deletion, JSON-v0 migration, Dynamic V4 change,
VM/WASM parity, backend rewrite, source semantic change, or performance claim.

## Acceptance

The positive canonical fixture contains only `MirInstruction::Call(MirCall)`
with `Callee::Global` for an already published Static/Free/Print target and
reaches `CanonicalTyped`. The compatibility fixture continues to use the
generic view with explicit `LegacyCallV0` and remains
`ExplicitCompatibility` for its existing caller. The negative fixture starts
from a valid typed module with at least two selected calls and mutates one
selected call to `LegacyCallV0` while retaining its typed callee; it must
return the named pre-artifact admission error. The fixture must assert that
no body JSON, C invocation, or object is produced after rejection.

No retry or name repair is allowed. The guard row remains parked until these
three outcomes are observable. If the existing callback cannot distinguish a
selected normal module from compatibility-only input without changing an
unrelated owner, return to `NoSafeSlice` rather than widening this row.

## Worker audit (2026-09-11)

The finite owner/caller audit found one selected normal admission owner:
`NormalDefaultPublishedPipelineV1::compile` through its existing published
callback and `PublishedMirBackendView::try_new`. The canonical issuer is
`MirInstruction::Call(MirCall)`; `LegacyCallV0` remains the explicit
compatibility issuer. The existing view can therefore enforce one named
pre-artifact mixed-shape rejection without a second view, resolver, fallback,
or semantic receipt. The current source-backed fixture still emits a
legacy-only typed Global, so this row deliberately does not reclassify that
existing route; a producer cutover is a separate row.

The implementation row is limited to that admission predicate and its
mutation-discriminating fixtures. It does not claim LegacyCallV0 retirement,
JSON-v0 migration, Dynamic/VM/WASM parity, or object/EXE production until the
selected corridor guard is revalidated.

## Required order

```text
normal typed admission
  -> existing canonical corridor guard revalidation
  -> any LegacyCallV0 retirement row
```

## Implementation evidence (2026-09-11)

The existing normal published pipeline now selects
`PublishedMirBackendView::try_new_selected_normal` for its selected
Static/Free/Print admission. The predicate keeps canonical
`MirInstruction::Call(MirCall)` rows on the typed route and rejects a module
that mixes a selected typed call with `LegacyCallV0` before body transport or
object creation, using `SelectedNormalUsesLegacyCallV0`. The generic
compatibility view is unchanged, and the current legacy-only source-backed
route remains on its existing path until a separate producer cutover.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib 'selected_normal_admission_'
3 passed
bash tools/checks/mir_call_canonical_corridor_guard.sh
[mir-call-canonical-corridor-guard] ok
```

The broader `published_backend_view` batch reached 81 passed, 2 unrelated
pre-existing array failures, and 1 ignored test; those failures are outside
this admission row. This row makes no LegacyCallV0 retirement, producer
cutover, Loop PHI, OBJ/EXE, or whole-suite-green claim.
