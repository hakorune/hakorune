---
Status: accepted design — transport-only P0 complete; publication ingress remains the next design stop
Date: 2026-08-21
Decision: SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-D0
Parent: docs/development/current/main/investigations/script-static-result-publication-ingress-failfast-d0-2026-08-21.md
ProductionCaller: existing source transport only; no new publication switch
ReplacementCell: preserve the source-domain witness before an invocation is demoted to compatibility context
Classification: BoxShape prerequisite; no new source shape or semantic product
Execution row: SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-P0
---

# SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-D0

## Six-line brief

Decision: The publication ingress cannot safely distinguish a compatibility
invocation that was never source-backed from a Cataloged invocation whose exact
source position was lost during transport. Preserve an optional expected root
lineage witness when a located context is demoted to `UnlocatedCompatibility`.

Source authority + canonical issuer: the existing
`RawInvocationRootLineageV1` issued by the invocation source transport remains
the authority. The witness is transport metadata only; it does not issue a
target, Recipe, Join, publication row, or physical result.

Non-authority: `RawUnlocatedPortalV1.reason`, AST names/spans/ordinals,
`Option::None`, owner presence alone, compatibility success, process lifetime,
legacy terminals, and fallback/retry cannot classify source loss.

Fail-fast boundary: before the transport collapses the source domain and before
receiver or argument effects. An owner-backed Cataloged witness that becomes
unlocated is `Error`; a genuinely source-free compatibility/test context is
`Unavailable`; exact Cataloged location remains eligible for the later
publication ingress `Absent | Selected` decision.

Smallest next slice: `SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-P0`.
Carry `expected_lineage: Option<RawInvocationRootLineageV1>` (or an equivalent
source-domain carrier) through unlocated context construction, child/source
transport, and reborrow paths; add exhaustive negative tests and a reusable
guard. Do not change publication ownership or physical emission in this row.

Non-claims: no source-admission widening, no new MethodCall location rule, no
publication consumer, no canonical Script cutover, no Compatibility/RawLegacy
retirement, no ABI/backend change, and no performance or production claim.

## Why this prerequisite exists

The publication card already requires four outcomes:

```text
Unavailable = no source-bound publication capability
Absent      = exact Cataloged site with no publication row
Selected    = exact Cataloged handoff claimed once
Error       = owner-backed source loss, foreign lineage, or drift
```

The current transport can turn a Cataloged `MethodCall` statement into
`UnlocatedCompatibility` and discard the root lineage at the same time. Once
that happens, the later ingress can only see “unlocated”, so it cannot tell
`Error` from genuine `Unavailable`. A terminal `Option::None` would therefore
reopen the old route after source loss. This card supplies only the missing
domain witness; the publication P0 remains the owner of `Unavailable | Absent
| Selected | Error` at the physical ingress.

## Classification-completeness receipt

The witness transport must classify every source context before child effects.
No wildcard, default, or empty witness is permitted:

| transport state | issuer/authority | before child effects | allowed continuation | fallback |
|---|---|---|---|---|
| `Located(Cataloged)` | source transport's exact root/site context | preserve exact Cataloged lineage | later publication ingress may choose `Absent` or `Selected` | none from transport |
| `Located(non-Cataloged)` | source transport's exact non-publication root | preserve the existing non-publication lineage | existing owner-specific route | no publication inference |
| `Unlocated(expected=Some(Cataloged))` | preserved `RawInvocationRootLineageV1` witness | classify as source-loss `Error` before descent | typed freeze only | no ordinary/raw terminal, retry, or `None` |
| `Unlocated(expected=None)` | compatibility/test transport with no source-bound owner | classify as `Unavailable` | existing compatibility/test owner | never source-backed success |
| `Foreign/contradictory witness` | transport validation of root/site lineage | classify as `Error` before descent | typed freeze only | no fallback or witness repair |

Every negative fixture must map to exactly one row. In particular,
`UnlocatedCompatibility` is not itself a final disposition: its preserved
lineage witness determines whether the later ingress sees source-loss `Error`
or genuine `Unavailable`.

## Acceptance

Positive:

- a located Cataloged source preserves its exact root lineage through child
  transport and reborrow without changing the existing site;
- a genuinely source-free compatibility/test context carries no expected
  lineage and remains `Unavailable` to the later ingress;
- a Cataloged context demoted to unlocated retains `Some(Cataloged(...))` and
  is rejected before receiver/argument effects;
- non-Cataloged roots retain their existing lineage and do not become generic
  publication candidates;
- the carrier is observational only: no target, publication, Call receipt, or
  semantic relation is minted by this row.

Negative:

- missing, foreign, or contradictory expected lineage freezes before child
  effects;
- a Cataloged source-loss witness cannot become `Unavailable`, `Absent`, or a
  legacy terminal through `Option::None`;
- a source-free compatibility context cannot be upgraded to `Error` merely by
  owner presence elsewhere;
- reborrow, structured-child forwarding, or transport reconstruction cannot
  drop or rewrite the witness;
- source admission is not widened just to make a MethodCall located;
- no publication owner, second source classifier, AST matcher, or fallback is
  introduced.

## P0 boundary and guard

The P0 may add only the source-domain carrier and thin forwarding through the
existing transport/structured-port seams, plus focused tests and a guard. It
must not edit the publication owner, Call emitter, result publication, or
physical bridge. The guard must assert:

```text
Cataloged -> Unlocated retains expected lineage witness       = 1
owner-backed source loss -> typed Error before effects        = 1
genuine compatibility unlocated -> Unavailable                = 1
foreign/contradictory witness -> Error                        = 1
publication/Call/AST semantic issuer in this row              = 0
Option::None as source-loss classifier                         = 0
witness dropped by reborrow/structured forwarding              = 0
source admission widening                                      = 0
source/check files >= 800 lines                                = 0
```

## Stop line and dependency

Remain at `NoSafeSlice` if the transport cannot preserve the Cataloged witness
before the source context is collapsed, if doing so requires admitting a new
source shape, or if the witness must be re-paired from AST names/ordinals.
After this P0 closes, the publication card may resume its own pre-descent
`Unavailable | Absent | Selected | Error` implementation. No physical
publication code is authorized before that dependency is green.

## P0 implementation receipt

The transport-only row is closed with no source-admission or publication-owner
change. `RawInvocationSourceTransportV1` now carries
`expected_lineage: Option<RawInvocationRootLineageV1>` through source-loss
construction, context reconstruction, child arguments, structured forwarding,
and reborrow. A Cataloged/Main/ScriptRoot source-loss witness is preserved;
genuine compatibility construction remains witness-free.

Focused evidence:

- `mir::builder::raw_invocation_source_transport::lineage_witness_tests` — 2 passed;
- existing transport tests — 13 passed;
- statement-classification tests — 5 passed;
- script claim-transport tests — 4 passed;
- `cargo check --profile quick -p nyash-rust` — passed with baseline warnings;
- `tools/checks/script_static_source_lineage_witness_guard.sh` — passed;
- `tools/checks/current_state_pointer_guard.sh` and `git diff --check` — passed.

The reusable guard also pins the complete state-table vocabulary
(`Located(Cataloged)`, `Located(non-Cataloged)`, source-backed and genuine
unlocated states, and `Foreign/contradictory witness`) so a future card cannot
silently collapse the neither-selected-nor-rejected state. All touched source
files remain below the 760-line split trigger; the existing transport test file
remains below the 800-line hard stop. The next row owns the typed
`Unavailable | Absent | Selected | Error` ingress; this row emits none of
those semantic/publication effects.
