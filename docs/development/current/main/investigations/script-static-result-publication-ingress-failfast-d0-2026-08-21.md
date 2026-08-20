---
Status: bounded P0 implementation complete — selected-normal only
Date: 2026-08-21
Decision: SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: existing Cataloged callable/static and me terminal owners; no new switch
ReplacementCell: distinguish publication capability absence, exact row absence, and source loss
Classification: BoxShape; source-lineage witness and pre-descent seam proof closed
Execution row: SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-P0
---

# SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-D0

## Six-line brief

Decision: Split the static-result publication ingress from the already-closed
Script claim-ingress P0. `UnlocatedCompatibility -> Ok(None)` is a real
compatibility seam hole; it must not reach the ordinary terminal when a
source-bound publication owner is installed.

Source authority + canonical issuer: the existing Cataloged caller/site
transport and `VerifiedStaticCallResultPublicationOwnerV1` remain the source
authority. The existing unified receipt emitter is the sole Call issuer and
`PreparedStaticCallResultPublicationV1` is the sole type-publication owner.
This row issues no new source, Recipe, Join, or target product.

Non-authority: `Option::None`, `UnlocatedCompatibility`, AST names/spans/
ordinals, `ValueId`, `MirType`, owner absence, the legacy terminal, warning
logs, and fallback/retry cannot authorize an ordinary result or a new target.

Fail-fast boundary: immediately after the effect-free `StaticReceiver` or
`me` route is selected and before receiver/argument descent. Terminal-level
`None` handling is too late because the current terminal receives lowered
`ValueId`s.

Smallest next slice: `SCRIPT-STATIC-RESULT-PUBLICATION-INGRESS-FAILFAST-P0`,
after the prerequisite `SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-P0`.
The prerequisite must preserve Cataloged lineage when transport produces an
unlocated context; only then may this row add the typed owner/source ingress
and exhaustive StaticReceiver/me connection. If the witness cannot be carried
without source-admission widening, remain at `NoSafeSlice`.

Non-claims: ScriptRoot admission, canonical Script cutover, claim-ledger
changes, callable Compatibility retirement, raw retirement, ABI/backend,
performance, and any broad source-classifier expansion.

## Audit evidence

The current path is:

```text
raw_static_result_publication.rs
  Unlocated/foreign/missing source or catalog -> Ok(None)
method_call_terminal.rs
  None -> ordinary global terminal
```

The terminal is reached after `lower_all`, so changing only its `None` arm
cannot satisfy an effect-free contract. `MethodCall` statement transport can
also produce `CallObject/UnlocatedCompatibility` while the source inventory
still owns a Cataloged MethodCall row. The same gap exists on the lowered `me`
terminal. The current Script claim-ingress P0 is a separate owner and remains
closed; do not merge the two ledgers or add a second AST matcher.

## Outcome vocabulary

```text
Unavailable
  no publication capability/owner is installed on this compatibility or test port;
  the port is outside the source-bound publication contract.

Absent
  exact Cataloged source context and owner are present, but no row exists for
  the exact site; only this state may preserve the old ordinary terminal.

Selected
  exact owner/site/target/arity validation succeeded and one handoff is claimed;
  it must use the existing receipt emitter and cannot return to the old route.

Error
  owner-backed source context is missing, unlocated, foreign, stale, or drifted;
  stop before child effects and discard the candidate without retry.
```

`Unselected` from the publication owner may be translated to `Absent` only
after the exact Cataloged source context has been proven. It must never be a
generic `None` that hides source loss or owner drift.

## Classification-completeness receipt

The ingress review is complete only when every outcome has one named owner and
one explicit pre-effect/terminal/fallback policy. This is the finite routing
table for this row:

| outcome | issuer/authority | before child effects | allowed terminal | fallback |
|---|---|---|---|---|
| `Unavailable` | port capability with no publication owner | leave the source-bound contract; no claim | compatibility/test owner only | existing compatibility behavior, never source-backed success |
| `Absent` | verified Cataloged site plus publication owner, with no exact handoff row | no owner mutation | existing ordinary terminal | allowed only for this exact no-row case |
| `Selected` | exact publication handoff owner/site/target/arity | atomically claim once | existing Call receipt + publication | none; failure discards candidate |
| `Error` | owner-backed source context or handoff validator | freeze before descent | typed error only | no ordinary/raw terminal, retry, or `None` |

Every negative witness in this card must map to exactly one row above. In
particular, source loss is `Error`, not `Unavailable` or `Absent`; a terminal
`Option::None`, wildcard arm, or default compatibility label may not collapse
these states. This table follows the repository-wide
`Candidate | Declined | Unresolved | Rejected` completeness rule while using
the publication owner’s vocabulary.

## Acceptance

Positive:

- located Cataloged + selected owner emits one existing Call receipt and one
  publication;
- located Cataloged + no row yields explicit `Absent` and preserves the old
  route without mutating the owner;
- no-owner compatibility/test port reports `Unavailable` and remains outside
  this source-bound contract;
- StaticReceiver and lowered-`me` routes use the same typed outcome vocabulary;
- duplicate/consumed rows fail without reissue or retry;
- the pre-descent selected path reuses the existing ordered argument driver and
  receipt emitter exactly once.

Negative:

- owner-backed `UnlocatedCompatibility` has zero argument effects, Call
  receipts, publication, and legacy terminal executions;
- owner-backed missing source context and non-Cataloged lineage are distinct
  typed errors (`SourceContextMissing` / `ForeignLineage`), never the
  compatibility `Unavailable` state;
- owner-backed missing context and foreign lineage freeze before descent;
- declaration/catalog/target/owner drift has no ordinary fallback;
- emitter or publication failure discards the isolated candidate and cannot
  retry through the ordinary/raw terminal;
- `Option::None` is not used as an ingress decision;
- no second source classifier, target resolver, AST MethodCall matcher, Call
  emitter, or Script publication owner is introduced.

## P0 implementation boundary and guard

This card is not executable until the source-lineage-witness prerequisite is
green. `UnlocatedCompatibility` with a preserved Cataloged witness must reach
this ingress as `Error`, not as `Unavailable` or `Absent`.

The P0 may touch only a new publication-ingress child, thin forwarding in the
existing source/structured ports, the StaticReceiver and `me` route heads,
focused effect-order tests, and a reusable guard. It must not widen source
admission or change `raw_invocation_source_transport.rs` responsibilities
without a split first. The guard must assert:

```text
owner-backed Unlocated/foreign -> explicit freeze error       = 1
exact Cataloged no-row -> ordinary route only                  = 1
StaticReceiver/me outcome match exhaustive                      = 1
legacy fallback after Selected/Error                            = 0
terminal-only None decision                                     = 0
second Call emitter/publication owner                          = 0
source/AST target re-resolution                                 = 0
source/check files >= 800 lines                                 = 0
```

The prerequisite is now closed by commit `629ece8290`: transport preserves
`expected_lineage` through source-loss, child, and reborrow paths, and its
guard pins the finite state vocabulary. The P0 ownership split is therefore:

```text
new publication-ingress child
  -> classify capability + RawInvocationSourceContextV1
  -> exact Cataloged only: existing collector take()
  -> Unselected = Absent; Selected = move to the existing bridge

RawInvocationChildPortV1 / RawStructuredChildScopePortV1
  -> thin forwarding only

StaticReceiver route head
  -> after Script claim ingress, before receiver/argument descent

me route head
  -> after effect-free prepare, before execute/lower_all
```

`raw_invocation_source_transport.rs`, the old terminal owner, and the generic
recursive port remain semantic no-growth owners in this row. Both route heads
now own `Absent | Selected | Error`; compatibility ports remain `Unavailable`
and never source-backed success, and the old late terminal hook is retired.

## P0 evidence

The implementation is now closed at the accepted boundary. A new
`static_result_publication_ingress.rs` classifies source context before any
child effect: genuine compatibility remains `Unavailable`, exact Cataloged
no-row is `Absent`, one handoff is `Selected`, and a preserved Cataloged
lineage that lost its node is `SourceLocationLost`. The StaticReceiver and
lowered static `me` route heads use exhaustive matches and the selected path
reuses the existing ordered argument driver, generic Call receipt emitter,
and `PreparedStaticCallResultPublicationV1` exactly once. The old
late-terminal `Option<ValueId>` publication hook and its duplicate module are
retired; no second AST matcher, Call emitter, target resolver, or publication
owner was added.

Focused evidence:

```text
cargo test --profile quick -p nyash-rust static_result_publication_ingress --lib  = 3 passed
cargo test --profile quick -p nyash-rust member_route --lib                         = 13 passed
cargo check --profile quick -p nyash-rust                                               = PASS
tools/checks/script_static_result_publication_ingress_guard.sh                         = PASS
```

The guard also pins the finite outcome vocabulary, source-loss error, no
late-terminal hook, transport/source-file line limits, and the absence of a
second AST matcher. This is a selected-normal BoxShape closure only; no
canonical Script consumer, production switch, Compatibility/Deferred/RawLegacy
retirement, ABI, or performance claim is opened.

## Stop line and ordered follow-ups

Stop as `NoSafeSlice` if a source-backed owner cannot be classified before
descent, if the exact source site is lost in the selected route, or if the
only repair requires widening source admission. After this P0, the independent
rows are:

1. `ME-CALL-ARITY-FAILFAST-D0` — decide strict-default compatibility and move
   header arity validation before argument effects where the existing header
   owner permits it.
2. `MIR-ROOT-APP-MODE-UNDECIDED-FAILFAST-D0` — make `root_is_app_mode=None` a
   freeze before registration/lowering instead of `unwrap_or(false)`.
3. `NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0` — document Complete vs
   Deferred ownership, destination, and the no-fallback/no-retry contract.
4. `CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0` — use the existing Brand
   cutover tracker; census Compatibility callers before any package admission.
5. `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1` — census typed rejection to String
   boundaries and retain variants until the top diagnostic boundary.
6. `SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-CONSUMER-D0` — name the consumer of
   `required_callee_i64_arguments`; do not infer it from ValueId/MirType.
7. `SCRIPT-DIRECT-STATIC-PHYSICAL-DELEGATION-DOC0` — align Recipe/Join input,
   bridge, emitter, publication, and exit-owner wording without adding an
   authority.

No row above opens canonical production, raw retirement, or performance work.
