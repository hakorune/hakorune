# MIRBUILDER-UNIFIED-SELFHOST-RESUME-D0 — lane entry census

Status: open__census__2026-09-25
Date: 2026-09-25
Owner: workstream row H (unified selfhost resume owner),
  docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
Authority: docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md

## Question

Row H orders: Whole MirBuilder gate → language conformance/rejection
matrix → canonical mimalloc gate →
`MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001` — "select
actual Facts authority/caller/deletion, not the retired
MapStore/classifier queue."

Which of the arrow steps is the first actionable bounded slice, and
what is its authority/caller/deletion tuple?

## Census boundary

- Start: the named gate checks (whole MirBuilder gate, conformance
  matrix, mimalloc gate) — verify whether each is already green or has
  a concrete failing owner.
- End: `MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001` —
  the Facts-authority pilot selection this lane exists to reach.
- Includes: locating each gate's check script/manifest, its last
  recorded result, and the Facts authority inventory it feeds.
- Excludes: reopening rows A..G (closed or parked), re-running the R7
  caller-zero census (exhausted per
  `mir-call-r7-caller-zero-d4-2026-09-25.md`), and the retired
  MapStore/classifier queue (explicitly out of scope per row H).

## Evidence

(recorded during census)

## Six-line Decision

(recorded during census)

## Exit

- [ ] First bounded slice of row H selected with authority/caller/
      deletion named, or `NoSafeSlice` with observable reopen trigger.
- [ ] Guard/pointer/workstream synced.
