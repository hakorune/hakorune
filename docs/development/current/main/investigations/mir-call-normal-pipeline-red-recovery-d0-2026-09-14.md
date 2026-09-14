---
Status: closed__design__2026-09-14
Task: MIR-CALL-NORMAL-PIPELINE-RED-RECOVERY-D0
Date: 2026-09-14
Priority: classify and recover the selected normal-pipeline reds without weakening a route or hiding a terminal
Parent: mir-call-static-compatibility-i0-a3-package-admission-boundary-2026-09-14.md
NextCard: mir-call-normal-pipeline-lifecycle-route-recovery-d0-2026-09-14.md
Implementation permission: false until each red has a reproduced baseline and one existing owner
---

# Normal-pipeline red recovery design stop

## Six-line brief

```text
Decision: keep the four observed normal_default_pipeline reds explicit; separate the two documented baseline failures from the two route/view failures whose parent result must be reproduced before any fix.
Source authority + canonical issuer: normal default root finalization and PublishedMirBackendView are the owners; the tests are observers only and cannot define route meaning.
Non-authority: expected test route alone, generic compatibility view, test filtering, a baseline number, VM parity, AST/name inference, and changing assertions to match UnsupportedBeforeObject.
Fail-fast boundary: artifact-root completion failure and UnsupportedBeforeObject remain named terminals; no compatibility retry, fallback, or baseline rewrite may make a red disappear.
Smallest next slice: reproduce the finite parent result at b61aef93ec, classify each red, then choose one owner-local fix with positive/negative guards and the exact selected old edge.
Non-claims: whole-lib green, publication cutover, VM promotion, resolver If support, Windows evidence, or retirement of the existing selected C gate.
```

## Finite red inventory

| Test | Current observation | Initial classification | Owner / next evidence |
| --- | --- | --- | --- |
| `published_consumer_runs_once_and_propagates_failure_without_retry` | callback count was `0` instead of `1` | known baseline candidate; documented at parent `b61aef93ec` | normal default finalization / callback admission; reproduce parent and current before editing |
| `published_consumer_does_not_consume_explicit_compatibility` | `[artifact-root-completion-unavailable]` before explicit compatibility result | known baseline candidate; documented at parent `b61aef93ec` | normal default compatibility finalization; preserve pre-effect stop and reproduce parent/current |
| `normal_ingress_preserves_app_main_free_static_definition_after_finish` | view route `UnsupportedBeforeObject`, expected `CanonicalTyped` | current-change candidate until parent replay | `PublishedMirBackendView` selected static/free route and the source call inventory; inspect unsupported callee before changing expectation |
| `normal_ingress_preserves_top_level_free_function_after_finish` | view route `UnsupportedBeforeObject`, expected `CanonicalTyped` | current-change candidate until parent replay | same view/physical route owner; compare the exact module call rows and selected-C admission |

The documented baseline at `docs/development/current/main/workstreams/`
records two pipeline reds at `b61aef93ec`; it does not authorize treating the
two route failures above as the same baseline. Parent replay is required to
close that classification gap.

## Parent/current comparison receipt

Parent replay at `b61aef93ec` used one quick-profile Cargo process against the
normal-pipeline test module. It ran 23 tests: 21 passed and the two documented
`published_consumer_*` tests failed. Both
`normal_ingress_preserves_app_main_free_static_definition_after_finish` and
`normal_ingress_preserves_top_level_free_function_after_finish` passed there.

The same four exact test paths at current `2894a77b82` reproduced four reds:
the two known baseline terminals above, plus
`UnsupportedBeforeObject` for both `normal_ingress_preserves_*` tests. The
latter pair is therefore `ParentPassCurrentFail`, a current-change blocker;
the former pair is `ParentFailCurrentFail`, immutable known baseline debt.
The next card owns only the latter pair.

## Ordered bounded tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Parent baseline replay | Run the exact four test paths at `b61aef93ec` with one quick Cargo process and record pass/fail, profile, and SHA. |
| 2 | Current comparison | Run the same four paths at the selected HEAD; classify each as `known baseline`, `current-change failure`, or `informational` with the first named terminal. |
| 3 | Owner diagnosis | For each current-change failure, inspect source call rows, `PublishedMirBackendView` route state, lifecycle/artifact terminal, and selected-C admission; name one existing owner and delete/retry edge. |
| 4 | Smallest recovery slice | Fix one owner-local cause per bounded series, retaining fail-fast behavior and adding positive/negative/terminal guards. Do not rewrite expected route merely to turn red green. |
| 5 | Baseline reconciliation | Update the existing workstream baseline only with parent/current evidence; do not add an ignored test or a larger numeric waiver. |
| 6 | Closeout | Re-run the exact inventory, classify all four, update the owner README/card, then commit/push and return to the resolver expressivity D0 only if verification health is Green or StableKnownRed. |

## Explicit states

| State | Meaning | Action |
| --- | --- | --- |
| `ParentPassCurrentFail` | regression introduced after the pinned parent | keep `CutoverBlockerOpen`; fix the named owner |
| `ParentFailCurrentFail` | reproduced known baseline debt | record immutable baseline; do not waive new failures |
| `ParentPassCurrentPass` | no red at current HEAD | close the row as informational correction |
| `TerminalChanged` | same test reaches a different named terminal | reopen owner diagnosis; no assertion-only fix |
| `Unclassified` | parent evidence or owner boundary is missing | remain at design stop |

No red may be reclassified from `Unclassified` to baseline solely because a
test name appears in an old document. The exact command, SHA and first
terminal are part of the receipt.
