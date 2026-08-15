---
Status: design stop; child of CALLABLE-TEXT-FORMAL-CALL-RESIDENCE-D0
Date: 2026-08-15
Work mode: design_stop
Classification: T2 BoxShape; finite origin partition only
---

# TEXT-CALL-ACTUAL-ORIGIN-ROUTE-D0

This child makes the lifetime route finite before any runtime pin or compiler
actualizer is implemented. It does not broaden the admitted Text shapes.

## Six-line brief

```text
Decision: treat only source-backed ExactText ordinary formal parameters as eligible for a future mandatory callee-entry lease arm; the current physical admission set is empty, so call results, Substring/temporary values, derived Text, and every unclassified origin remain RejectBeforeEffect.
Source authority + canonical issuer: callable_parameter_contract::issuer owns the ExactText(StringBox-as-Text) formal rows; a future package-owned actual-origin issuer must additionally co-seal the argument ordinal/site, `Local(BindingRef)` reaching the original formal without rebind, whole-source target identity, callee ExactText row, two lanes, and entry-lease plan.
Non-authority: MirType/StringBox spelling alone, TextFormalBorrowV1, raw HostHandle, TextEq/Substring source shape, Recipe ordinal, MIR ValueId, Dynamic lease, retain_h, benchmark, fallback, and retry.
Fail-fast boundary: missing formal row, foreign owner/target, call-result or temporary origin, source proof ending before synchronous completion, or an unclassified arm rejects before body effect; route coverage must be exhaustive and disjoint.
Smallest next slice: issue one private route partition with exact formal coverage and explicit RejectBeforeEffect rows, then feed only the formal arm into the mandatory callee-entry lease design.
Non-claims: no runtime pin, C ABI, physical arity, TextEq route, Substring corridor, ValueId, Canonical session, Builder, production caller, fallback, or main integration.
```

## Current origin census

The only current `ExactText` semantic issuer is the ordinary callable parameter
issuer. It accepts the explicit `StringBox` spelling as logical Text and emits
an ordinal/`BindingRef` formal contract. Dynamic admission rejects this kind.
That formal row alone is not a call-site origin proof: the same `BindingRef` may
survive a local rebind. The future arm therefore requires the source-backed
argument site, `Local(BindingRef)` reaching the original formal, exact static
target, and callee formal row to be co-sealed once.

There is no source-backed physical residence issuer for a call result,
Substring result, temporary span, copied/derived Text, or a TextEq operand.
Those origins must remain explicit `RejectBeforeEffect` rows until their own
source and lifetime owner exists; they must not be silently routed through the
formal-parameter arm.

The partition is therefore intentionally small:

```text
ExactText ordinary formal parameter  -> MandatoryEntryLease candidate (not admitted)
call result / Substring / temporary  -> RejectBeforeEffect
derived / copied / unknown origin    -> RejectBeforeEffect
```

The current physical admission domain contains zero Text actuals. Until a
source-backed formal owner is issued, every physical Text path therefore
rejects before effect; the formal row is only the one named candidate that the
parent residence task may later consume.

No nonformal origin may select a lease. A source-residence-only route is not a
second arm here; it would require a separate language/profile Decision and is
parked until a source owner can prove call-complete lifetime.

## Acceptance and stop line

Acceptance requires one non-`Clone` route partition, exact coverage of the
currently empty physical admission set, no overlapping arms, and a typed
reject for every non-formal origin. The formal candidate carries no physical
IDs; it only names the source-backed owner and target identity that the parent
residence decision must eventually issue.

If the formal source owner cannot be issued without raw slot/name/MIR
inference, this child remains:

```text
NoSafeSlice::MissingTextFormalCallResidenceIssuer
```

No runtime or compiler implementation is authorized while that token remains.
