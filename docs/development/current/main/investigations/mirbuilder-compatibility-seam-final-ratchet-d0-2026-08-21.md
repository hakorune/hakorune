---
Status: accepted design map — first row is publication ingress P0
Date: 2026-08-21
Decision: MIRBUILDER-COMPATIBILITY-SEAM-FINAL-RATCHET-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-claim-ingress-failfast-d0-2026-08-21.md
ProductionCaller: existing compatibility seams only; no new production switch
ReplacementCell: close new/old boundary holes one owner and one fail-fast row at a time
Classification: design map; each implementation row is classified independently
---

# MIRBUILDER-COMPATIBILITY-SEAM-FINAL-RATCHET-D0

## Six-line brief

Decision: The new Source → Facts → Recipe → Join pipeline is healthy; the
remaining risk is concentrated at compatibility seams where a missing source
context, relaxed arity, undecided lifecycle state, or typed error is silently
converted into an old route. Close these as independent bounded rows, not as a
global cleanup or a new migration registry.

Source authority + canonical issuer: each row keeps its existing authority:
publication owner/caller site, header observation, root lifecycle state,
parser/callable disposition, or typed rejection enum. Existing unified Call and
publication emitters remain the sole physical issuers. This map issues no
semantic product and no production route.

Non-authority: `Option::None`, `unwrap_or(false)`, warning logs, AST names or
ordinals, `ValueId`/`MirType`, compatibility success, Deferred labels, string
diagnostics, historical modules, and performance results cannot select a route
or prove a source relation.

Fail-fast boundary: every row must stop at its first owner boundary, before
child effects or physical publication. A terminal-level error is insufficient
when the terminal receives already-lowered values. No fallback, retry, empty
product, or guessed NonCandidate is allowed.

Smallest next slice: `SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-P0`,
under its own card. It is the next implementation row because the publication
owner has the same source-loss hole as the recently closed claim ingress but a
different owner and pre-descent seam.

Non-claims: no builder.rs cleanup sweep, Call representation rewrite, canonical
Script cutover, Compatibility semantic admission, raw retirement, ABI/backend
change, SIMD/optimizer work, or performance measurement.

## Audited boundary inventory

| Priority | Task | Classification | Evidence / owner | Current disposition |
|---|---|---|---|---|
| P0 | `SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-P0` | BoxShape prerequisite | `raw_invocation_source_transport.rs` + `RawInvocationRootLineageV1` | selected before publication ingress; preserve Cataloged witness before source collapse |
| P0 | `SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-P0` | BoxShape candidate | `raw_static_result_publication.rs` + StaticReceiver/me route heads | depends on the lineage-witness row; no physical implementation yet |
| P1 | `ME-CALL-ARITY-FAILFAST-D0` | classification design stop; likely BoxCount if default acceptance changes | `method_call_handlers.rs` + `builder_me_call_arity_strict` | separate row; strict default and pre-effect timing must be decided |
| P1 | `MIR-ROOT-APP-MODE-UNDECIDED-FAILFAST-D0` | BoxShape candidate | `nonmain_static_box_lifecycle.rs` `root_is_app_mode.unwrap_or(false)` | separate row; freeze `None` before registration |
| D0 | `CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0` | BoxCount only if new source shape is admitted | existing `brand-constructor-consumer-cutover-d0.md` tracker | do not duplicate; census callers and source authority first |
| D0 | `NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0` | docs/design stop | `normal_callable_semantic_source.rs` Deferred | define destination and no-fallback contract before code |
| P1 | `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1` | behavior-neutral refactor series | bridge/publication/loan/manifest String boundaries | census first; preserve typed variants |
| D0 | `SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-CONSUMER-D0` | design stop | `required_callee_i64_arguments` in publication demand | name one consumer; never infer from physical values |
| D0 | `SCRIPT-DIRECT-STATIC-PHYSICAL-DELEGATION-DOC0` | docs-only | Recipe/Join → bridge → emitter → publication → exit | align wording, no new authority |

The publication card is intentionally separate from the closed
`SCRIPT-DIRECT-STATIC-CALL-CLAIM-INGRESS-FAILFAST-P0`: the symptom is similar,
but the production owner, handoff API, and pre-descent route are different.
Compatibility callable remains an explicit old route, not an untyped silent
fallback; its retirement is already tracked by the Brand cutover cards.

## Shared acceptance law

Each implementation row must provide a positive, negative, focused test, stable
guard, and owner README/reference receipt. A row is not complete when a new
owner exists; it closes only when the old boundary has zero unauthorized
callers or its remaining compatibility scope is explicitly typed and tested.

Before implementation, each routing row must also carry a finite
classification-completeness table. It must enumerate the selected state, the
explicit no-candidate/neither state, and every unresolved or rejected state,
then bind each to one authority, pre-effect behavior, terminal, and fallback
policy. `Option::None`, wildcard matches, `unwrap_or(default)`, and generic
compatibility labels are not valid substitutes. A negative fixture must map to
one named state; if the table cannot be made finite and authority-backed, the
row remains `NoSafeSlice`.

```text
source-backed mismatch -> typed freeze before effects
exact no-row           -> only explicitly permitted Absent/compat route
selected               -> one linear claim and one physical owner
failure after claim    -> candidate discard, never fallback/retry
```

No row may change the current S6C evidence thresholds, WSL/native authority,
backend selection, or production promotion policy.
