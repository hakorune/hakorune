---
Status: active; fast
Task: CALLABLE-CANONICAL-TRIVIAL-ROW-I0
Date: 2026-08-22
Priority: replace one ordinary cataloged-static callable body with the existing canonical BindingRef lowerer
Parent: CALLABLE-RESOLVED-BINDING-AUTHORITY-HANDOFF-D0
PreviousCard: mir-callable-resolved-binding-authority-handoff-d0-2026-08-22
NextCard: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
---

# Callable canonical trivial row I0

## Six-line brief

```text
Decision: admit exactly one already-supported canonical trivial source shape and lower it through the existing resolver plan/lowerer; do not wrap the legacy callable body driver with resolved authority.
Source authority + canonical issuer: `SelectedCallableLoweringInputRefV1::source()` owns the exact resolver input; `CanonicalLoweringPreflightV1` issues the existing `CanonicalTrivialBindingSsaPlanV1`, and the existing canonical trivial lowerer consumes it once.
Non-authority: `CallableSemanticLoweringState`, source transport, variable_map, raw BindingId, names, ordinals, AST rereads, physical signature text, and any new callable-specific semantic receipt.
Fail-fast boundary: canonical preflight and physical-symbol/ABI checks complete before child-session body effects or collector publication; any reject discards the unpublished session and cannot retry through the canonical or legacy path.
Smallest next slice: `Scan.run(value) { return value }` as shape evidence only—one static cataloged row, one parameter, final variable return, existing canonical trivial plan, existing physical signature loan, one collected draft.
Non-claims: loops, calls, locals, instance receivers, typed signatures, general callable migration, parser-scan behavior, old Recipe retirement, production-wide cutover, backend, and performance.
```

## Authority and route

```text
installed package loan
  -> exact SelectedCallableLoweringInputRefV1
  -> CanonicalLoweringPreflightV1::verify_function(input)
  -> CanonicalTrivialBindingSsaPlanV1
  -> existing canonical trivial lowerer
  -> existing resolved draft session
  -> existing physical-signature/collector owner
```

The fixture name is not a route selector. The implementation must use the
preflight result and exact owner/source products. The one-row fixture merely
proves the first accepted shape without opening the parser-scan family.

`CallableSemanticLoweringState` remains an outside compatibility projection for
rows not admitted by this I0. It is never installed together with the
canonical resolved BindingRef lifecycle for the admitted row. A canonical
preflight reject is terminal for this row; it is not an invitation to retry
with `build_static_method_draft_with_port_v1`.

## Finite state

| State | Owner | Effect | Allowed next state |
| --- | --- | --- | --- |
| `SelectedInput` | installed semantic package | none | `Preflighted` or typed outside/reject |
| `Preflighted` | existing canonical preflight | none | `SessionReady` |
| `SessionReady` | resolved function session | unpublished context only | `Lowered` or discard |
| `Lowered` | existing canonical trivial lowerer | unpublished MIR only | `CollectorReady` or discard |
| `CollectorReady` | physical signature + draft collector | no publication yet | `Collected` |
| `Collected` | existing collector | one publication | terminal |
| `RejectedBeforeEffect` | typed task boundary | no Builder/collector effect | terminal discard |
| `OutsideI0` | explicit admission boundary | no canonical claim | existing explicitly classified outside lane |

`OutsideI0` is a classification, not a post-failure fallback. The admitted row
must not enter `CallableSemanticLoweringState` before the canonical lowerer.

## Acceptance evidence

Positive:

```text
one source-backed static cataloged row
one canonical preflight plan
one resolved BindingRef parameter publication
one final variable return through ResolvedIdentityStateV1
one physical draft with the existing qualified symbol/arity
one collector row
legacy CallableSemanticLoweringState body calls = 0 for this row
legacy BindingId allocation while resolved authority is installed = 0
```

Negative:

```text
preflight rejects loop/call/local/typed/direct-call shape before body effects
physical symbol or signature mismatch rejects before collector mutation
canonical reject never retries the legacy body driver
foreign owner/input cannot be paired with the selected plan
session failure restores Builder and leaves collector/publication unchanged
```

Structural guard:

```text
canonical admitted row -> CanonicalLoweringPreflightV1 exactly once
canonical admitted row -> canonical trivial lowerer exactly once
canonical admitted row -> build_static_method_draft_with_port_v1 callers = 0
canonical admitted row -> CallableSemanticLoweringState construction = 0
canonical reject -> fallback/retry = 0
all touched Rust files < 760 lines; 800 is hard stop
```

## NoSafeSlice

Stop and return to design if any of the following is observed:

```text
the existing plan cannot be moved from the same input into the canonical lowerer
the canonical lowerer cannot preserve the physical qualified symbol and arity
parameter/return BindingRef rows need name/ordinal/AST reconstruction
legacy BindingId allocation remains reachable after canonical install
the collector requires a second physical writer or second signature authority
an outside shape needs an implicit fallback to make the positive fixture pass
```

## Commit sequence

1. `feat: admit one canonical trivial callable row` — pre-effect plan seam and
   canonical lowerer connection only.
2. `test: prove canonical callable row has no legacy fallback` — focused
   positive/negative evidence and reusable guard.
3. `docs: close canonical callable row i0` — README/reference/current pointer,
   gate output, and caller census.

No loop, parser-scan, instance, backend, cleanup, or performance work may be
included in these commits.
