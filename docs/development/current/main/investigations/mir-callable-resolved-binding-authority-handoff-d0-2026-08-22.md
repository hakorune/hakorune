---
Status: accepted; Candidate A selected by read-only worker audit; implementation moved to CALLABLE-CANONICAL-TRIVIAL-ROW-I0
Task: CALLABLE-RESOLVED-BINDING-AUTHORITY-HANDOFF-D0
Date: 2026-08-22
Priority: choose one authority-safe handoff for source-backed cataloged callable BindingRef rows
Parent: MIR-CALLABLE-SEMANTIC-NESTED-IF-SOURCE-ROW-I0
PreviousCard: mir-callable-semantic-nested-if-source-row-d0-2026-08-22
NextCard: CALLABLE-CANONICAL-TRIVIAL-ROW-I0
---

# Callable resolved-binding authority handoff D0

## Six-line brief

```text
Decision: keep the parser/resolver `VerifiedResolvedFunctionV1` as the only BindingRef authority and do not make the existing callable lowerer appear canonical by adding install/finish around legacy BindingId allocation.
Source authority + canonical issuer: `SelectedCallableLoweringInputRefV1::source()` and its parser/resolver-issued `VerifiedResolvedFunctionV1`; the existing canonical lowerer/session owns resolved BindingId publication only when its source facts and physical lifecycle are co-sealed.
Non-authority: `ResolvedBindingLoweringStateV1` lifecycle gate, `CallableSemanticLoweringState` alone, `variable_map`, raw `BindingId`, parameter position, variable name, source transport, AST reread, and a second semantic/physical receipt.
Fail-fast boundary: before `build_static_method_draft_with_port_v1` starts body effects; if the existing lowerer cannot consume the resolver-owned BindingRef authority without legacy allocation, reject the unpublished callable session and do not install/finish a partial authority.
Smallest next slice: census the existing canonical lowerer’s parameter/local BindingRef lifecycle against the ordinary cataloged-static source package, then choose one bounded bridge or keep the route closed; no general callable-body expansion.
Non-claims: no nested-If source-row changes, W6 expansion, AST/name/ordinal pairing, fallback, second physical writer, publication, old Recipe retirement, production switch, backend, or performance.
```

## Why this is a separate design stop

The source-row I0 correctly changes the ordinary cataloged-static signature
branch from an ambient Script-root source context to the already-issued
function-root `RawInvocationSourceTransportV1<()>`. The parser-backed row is
then visible to `CallableSemanticLoweringState::read_variable`.

That handoff exposes a second boundary in the positive `Scan.run(value)`
fixture. The route uses
`capture_resolved_function_pending_session_v1`, whose success contract
requires resolved BindingRef authority to be installed and completed. The
body lowerer, however, calls the existing port-aware callable path, which
uses legacy BindingId allocation. Installing the resolver product by itself
therefore fails at the deliberate legacy-allocation veto; omitting it fails at
session cleanup. Neither result is a safe implementation.

The worker audit classified this as `NOSAFESLICE / separate blocker` and
identified `SelectedCallableLoweringInputRefV1::source().function()` as the
authority candidate. This card records the design boundary only; it does not
authorize a new `Verified*` product or a bridge implementation.

## Accepted Decision

The follow-up worker audit selected **Candidate A** for one bounded source
shape. The existing resolver input and canonical preflight already provide the
needed handoff; no callable-specific BindingRef receipt and no second BindingId
allocator are needed.

```text
SelectedCallableLoweringInputRefV1::source()
  -> CanonicalLoweringPreflightV1::verify_function(input)
  -> existing CanonicalFirstFamilyPlanV1
  -> existing canonical trivial lowerer
  -> existing resolved session / draft collector
```

The first admitted row is the shape represented by the small positive fixture
`Scan.run(value) { return value }`: one static cataloged callable, one source
parameter, and a final variable return. The fixture name is test evidence only;
selection is by the existing canonical preflight and exact resolver products,
never by name, ordinal, or AST reconstruction.

The current I0 does not admit loops, nested calls, local declarations, typed
signatures, direct-call families, instance receiver lanes, or the parser-scan
program as a whole. A row outside this bounded canonical preflight is an
explicit outside/unsupported disposition for this slice. It must not be
silently retried through the canonical path, and a canonical preflight failure
must not fall through after effects begin. The existing compatibility route is
only retained where its caller is explicitly classified outside this I0; it is
not a failure fallback from the canonical row.

The worker also confirmed that Candidate B would promote
`CallableSemanticLoweringState` into a competing Binding authority and that
Candidate C would only preserve a safe stop rather than advance the canonical
replacement. Therefore `install/finish` is allowed only inside the existing
canonical lowerer lifecycle, never around `build_static_method_draft_with_port_v1`
or any legacy `allocate_binding_id()` call.

## Authority census

| Concern | Existing owner | Must not be promoted |
| --- | --- | --- |
| parser/resolver BindingRef rows | `VerifiedResolvedFunctionV1` / owner forest | source transport, names, AST reread |
| callable semantic site reads | `CallableSemanticLoweringState` | semantic state as resolver authority |
| resolved-session lifecycle veto | `ResolvedBindingLoweringStateV1` | gate as BindingRef issuer |
| canonical BindingId/value lifecycle | existing canonical resolved lowerer | ordinary legacy lowerer by assertion |
| ordinary cataloged-static physical draft | `build_static_method_draft_with_port_v1` | automatic canonical claim |
| function-session cleanup | `CanonicalFunctionLoweringSessionV1` | weakening the success contract |

## Candidate boundary

```text
resolver-owned BindingRef/function source
  -> one existing canonical parameter/local lifecycle
  -> existing resolved lowerer/session
  -> existing physical draft finalizer
```

The next design census must determine whether this can reuse an existing
canonical lowerer without reconstructing source rows or introducing a second
BindingId authority. A design that only adds:

```text
resolved_binding_state.install(...)
body lowerer using legacy BindingId allocation
resolved_binding_state.finish(...)
```

is explicitly rejected by this card.

## Acceptance for the future design

```text
one resolver source authority and one BindingId lifecycle
parameter/local rows remain source-backed and owner-bound
no legacy allocation after canonical authority installation
all fallible source/semantic checks happen before physical effects
session success proves the same authority is installed and complete
failure discards the unpublished session with no publication
ordinary cataloged-static caller count and old edge are measured
```

For the selected I0 row, the stronger acceptance is:

```text
preflight plan issuer = existing CanonicalLoweringPreflightV1 exactly once
plan consumer = existing canonical trivial lowerer exactly once
legacy CallableSemanticLoweringState body consumer = 0
legacy BindingId allocation after canonical install = 0
canonical draft collector publication = 1
canonical reject -> compatibility fallback = 0
```

## NoSafeSlice conditions

Keep `design_stop` if any of these remain:

```text
canonical lowerer cannot consume the existing cataloged callable source
parameter/local facts require name/ordinal/AST reconstruction
legacy and canonical BindingId allocators both remain active
install/finish is used as a wrapper around legacy allocation
source-row repair requires W6 or a second receipt
failure can publish a function or collector row before authority completion
the only route is fallback or a second physical writer
```

The I0 also remains `NoSafeSlice` if the existing preflight cannot classify the
small positive row as one of its already-owned canonical plan variants, if the
plan has to be reconstructed after the source callback, or if the physical
method symbol/ABI requires a second name-based mapping. In that case the
bounded row is not widened; the next action is another design stop.

## Bounded implementation task

`CALLABLE-CANONICAL-TRIVIAL-ROW-I0` is the only implementation task opened by
this Decision.

1. Add a pre-effect source-to-plan seam at the ordinary cataloged-static
   source-backed caller. It must consume the existing
   `ResolvedFunctionLoweringInputV1` once and retain the existing
   `CanonicalTrivialBindingSsaPlanV1` only inside the lowering callback.
2. Reuse the existing canonical trivial lowerer and resolved function session.
   Parameter declarations and the final return must be published through the
   existing `ResolvedIdentityStateV1`; do not call the port-aware legacy body
   driver for the admitted row.
3. Preserve the existing physical-signature loan and collector owner. If the
   current physical symbol cannot be carried without a second mapping, stop at
   `NoSafeSlice` rather than deriving it from a name or AST.
4. Keep outside shapes explicit. Do not broaden the preflight, add a fallback,
   or change parser-scan loop/call/local behavior in this task.
5. Add one positive and one negative focused test, one structural guard for the
   canonical caller/legacy allocator boundary, and update the owner README and
   reference receipt in the same slice.

The task is complete only when the small positive source-backed callable reaches
one collected canonical draft and all pre-effect rejects leave Builder,
collector, and publication state unchanged.

## Ordered successor

```text
1. CALLABLE-RESOLVED-BINDING-AUTHORITY-HANDOFF-D0  (accepted)
2. CALLABLE-CANONICAL-TRIVIAL-ROW-I0  (next implementation slice)
3. MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0  (remains parked until 2 closes)
```
