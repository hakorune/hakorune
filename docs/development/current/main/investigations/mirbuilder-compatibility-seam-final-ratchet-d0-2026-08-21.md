---
Status: accepted design map — audit reconciliation complete; remaining rows parked
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

Smallest next slice: follow the live pointer's
`SCRIPT-DIRECT-STATIC-SOURCE-IDENTITY-I0`. This map is a parked dependency
map, not a second execution queue. Its previously listed P0 rows are now
closed in their own cards; the remaining Compatibility admission and typed
error debt stay separately gated.

Non-claims: no builder.rs cleanup sweep, Call representation rewrite, canonical
Script cutover, Compatibility semantic admission, raw retirement, ABI/backend
change, SIMD/optimizer work, or performance measurement.

## Audited boundary inventory

| Priority | Task | Classification | Evidence / owner | Current disposition |
|---|---|---|---|---|
| P0 | `SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-P0` | BoxShape | `RawInvocationRootLineageV1` transport | **closed**; Cataloged witness survives source loss and no longer collapses to `None` |
| P0 | `SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-P0` | BoxShape | publication ingress + StaticReceiver/me heads | **closed**; `Unavailable | Absent | Selected | Error` is exhaustive before effects |
| P1 | `ME-CALL-ARITY-FAILFAST-D0` | BoxShape | header observation + `builder_me_call_arity_strict` | **closed**; strict default is on, explicit `=0` is the only compatibility override |
| P1 | `MIR-ROOT-APP-MODE-UNDECIDED-FAILFAST-D0` | BoxShape | `root_is_app_mode` lifecycle seam | **closed**; `None` freezes before registration/descent |
| D0 | `CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0` | NoSafeSlice until issuer/consumer | existing compatibility cohort tracker | **parked**; transport P0 is closed, semantic admission/raw retirement are not authorized |
| D0 | `NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0` | docs/design stop | `normal_callable_semantic_source.rs` Deferred | **closed as caller-zero park**; Deferred never becomes Compatibility/None/empty Complete |
| P1 | `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1A` | BoxShape | bridge/publication outer diagnostic boundary | **closed**; typed variants survive until one existing String boundary |
| P1 | `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1B` | design stop | claim/Brand/manifest/loan `Result<_, String>` boundaries | **parked next**; preserve 11-variant ledger and manifest/loan state tables, no common-port rewrite |
| D0 | `SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-CONSUMER-D0` | design stop | `required_callee_i64_arguments` in publication demand | name one consumer; never infer from physical values |
| D0 | `SCRIPT-DIRECT-STATIC-PHYSICAL-DELEGATION-DOC0` | docs-only | Recipe/Join → bridge → emitter → publication → exit | align wording, no new authority |

The publication card is intentionally separate from the closed
`SCRIPT-DIRECT-STATIC-CALL-CLAIM-INGRESS-FAILFAST-P0`: the symptom is similar,
but the production owner, handoff API, and pre-descent route are different.
Compatibility callable remains an explicit old route, not an untyped silent
fallback; its retirement is already tracked by the Brand cutover cards.

## 2026-08-21 review reconciliation

The latest boundary audit did not find a new untracked P0 in the current
branch. Two reported holes were already closed and must stay closed by their
guards:

| audited boundary | current finite states | result | next action |
|---|---|---|---|
| static-result publication | `Unavailable`, `Absent`, `Selected`, `Error` | closed; owner-backed `UnlocatedCompatibility` is a typed pre-effect reject | no duplicate task; preserve `script_static_result_publication_ingress_guard.sh` |
| `me` lowered-global arity | `NotApplicable`, `Inline`, `LoweredGlobalMatch`, `LoweredGlobalMismatchStrict`, `LoweredGlobalMismatchCompat`, `HeaderMissing`, `Standard`, `StaticFallback` | closed; unset strict flag rejects before effects, explicit `=0` remains typed compatibility | no default relaxation or ABI change |
| undecided root mode | `App`, `NonApp`, `Undecided` | closed; `None` cannot authorize lifecycle work | keep `mir_root_app_mode_failfast_guard.sh` |
| Compatibility callable | `SourceBacked`, `TypedCompatibility`, `Unavailable`, `Neither`, `Deferred`, `Rejected`, `Discarded` | transport closed; semantic package/physical consumer and old-edge retirement remain `NoSafeSlice` | continue existing `CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0` |
| typed error boundary | `TypedError`, `OuterDiagnostic`, `Unavailable`, `Absent`, `Completed`, `NoSafeSlice` | P1A closed; claim/Brand/manifest/loan conversions remain debt | park `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1B` |

The repository-wide design rule is now explicit in
`agent-current-entry-contract-ssot.md` and enforced by
`tools/checks/routing_classification_completeness_guard.sh`: every routing,
claim, publication, admission, or lifecycle card must enumerate the selected,
neutral/neither-selected-nor-rejected, deferred/unresolved, rejected, and
terminal states, with one authority, a pre-effect boundary, and a fallback
policy for each. `None`, wildcard, `unwrap_or(default)`, and a generic
Compatibility label cannot be used as the missing row.

This reconciliation is documentation-only. It does not reopen canonical
Script, source-only A, Compatibility admission, raw retirement, Call
representation, ABI, optimizer, or performance work.

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
