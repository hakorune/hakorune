---
Status: accepted design stop — bounded pre-descent implementation next
Date: 2026-08-21
Decision: ME-CALL-ARITY-FAILFAST-D0
Parent: docs/development/current/main/investigations/script-static-result-publication-ingress-failfast-d0-2026-08-21.md
ProductionCaller: existing `me.method(...)` lowering only; no new caller
ReplacementCell: header arity mismatch must not become a post-effect warning
Classification: BoxShape; strictness timing/default only, no new source shape
Execution row: ME-CALL-ARITY-FAILFAST-P0
---

# ME-CALL-ARITY-FAILFAST-D0

## Six-line brief

Decision: Make `me` lowered-global arity strict by default and validate a
header-backed mismatch before argument descent. Preserve the explicit
`NYASH_ME_CALL_ARITY_STRICT=0` compatibility override as a named legacy state;
an unset flag is not permission to continue silently.

Source authority + canonical issuer: `MeCallHeaderObservationPortV1` issues
`MeCallParameterObservationV1::Present`, and
`prepare_me_lowered_call_v1` owns the receiver/parameter snapshot. The header's
`parameter_count` is the expected arity; the route's source argument count plus
the explicit `me` receiver is the provided arity for instance calls.

Non-authority: AST spelling alone, `ValueId`/`MirType`, lowered argument count
after effects, warning logs, `StaticFallback`, missing headers, the legacy
terminal, and `NYASH_ME_CALL_ARITY_STRICT` when unset cannot authorize a
mismatch or invent a header.

Fail-fast boundary: after effect-free `me` route preparation and header
observation, before publication ingress, receiver/argument descent, Call
emission, or result publication. Only a `LoweredGlobal` row with a real header
and mismatched provided arity is selected; inline/standard/fallback routes keep
their existing owners.

Smallest next slice: `ME-CALL-ARITY-FAILFAST-P0` — add the default-on policy
and one source-backed pre-descent validator, then keep the existing ordered
driver and terminal for matching calls. No source admission or compatibility
callable cutover is part of this row.

Non-claims: no global method-arity redesign, callable Compatibility retirement,
Script Deferred repair, Brand cutover, ABI/type change, fallback/retry policy,
production switch, or performance measurement.

## Classification-completeness receipt

Every `me` route must map to one named state before child effects:

| state | authority/condition | before effects | allowed terminal | fallback |
|---|---|---|---|---|
| `NotApplicable` | no enclosing Box owner | no route selected | existing `None` result | existing outer route only |
| `Inline` | verified record/setter helper fact | helper-specific preflight | existing inline owner | no lowered-global retry |
| `LoweredGlobalMatch` | header `Present`, expected == provided | proceed to ordered descent | existing lowered-global terminal | none |
| `LoweredGlobalMismatchStrict` | header `Present`, expected != provided, strict flag unset or `1` | typed reject, zero child effects | freeze error only | no warning/legacy terminal |
| `LoweredGlobalMismatchCompat` | header `Present`, expected != provided, explicit flag `0` | retain documented legacy timing | existing compatibility terminal | explicit opt-in only |
| `HeaderMissing` | observation is `Missing` | do not guess an expected count | existing Standard/StaticFallback policy | no synthetic header |
| `Standard` | bound `me` plus verified standard-method route | existing standard preflight | existing standard terminal | no lowered-global retry |
| `StaticFallback` | no bound `me`, no lowered-global header | existing static route | existing static owner | no guessed arity |

`LoweredGlobalMismatchStrict` is the only newly hardened failure state. A
`None`, `unwrap_or(default)`, warning, or `Missing` observation may not be
used to merge it with `HeaderMissing` or `LoweredGlobalMismatchCompat`.

## Acceptance

Positive:

- unset `NYASH_ME_CALL_ARITY_STRICT` behaves as strict `1`;
- explicit `=1` rejects static and instance lowered-global mismatches before
  argument effects, Call emission, or publication;
- matching lowered-global calls still use the existing ordered argument driver
  and terminal exactly once;
- inline, standard, static-fallback, and header-missing routes preserve their
  existing owners and do not synthesize a header.

Negative:

- mismatch with unset/`1` produces one stable freeze error and zero argument
  effects;
- explicit `=0` is the only compatibility escape and is visible in tests and
  docs; no other default or warning path enables it;
- instance comparison counts the explicit `me` receiver exactly once;
- missing/foreign header observations never become a numeric default;
- mismatch cannot retry through ordinary/static/inline routes after rejection.

## P0 boundary and non-growth

The implementation may touch only the builder flag helper, the effect-free
`MeCallPolicyBox` preparation seam, focused tests, a reusable guard, and this
owner README/card. Do not add a second method-call matcher, change the header
authority, alter `MethodCall` syntax, or grow a source file to 760 lines.

The pre-descent validator may call the existing strictness helper but must not
lower arguments, inspect emitted MIR, or infer arity from `ValueId`/`MirType`.
The explicit `=0` compatibility state remains a non-claim for C-parity and
does not authorize a production cutover.

