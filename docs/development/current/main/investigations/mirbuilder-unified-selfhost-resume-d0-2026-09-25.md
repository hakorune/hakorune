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

## Evidence (worker 9a80426c, spot-checked)

- Gate identity: row H's arrow = unified resume order gates 1-4 in
  `design/selfhost-parser-mirbuilder-migration-order-ssot.md:44-60`.
- Gate 1 (whole MirBuilder gate) is RED: last recorded
  `real-apps-exe-boundary` receipt is 2 pass / 9 fail
  (`investigations/repo-final-convergence-audit0-2026-09-25.md:117`,
  verdict `selfhost-resume-entry-recheck0-p0-2026-09-25.md:69,77-81`).
  The 9 reds are typed fail-fast terminals with owner classes already
  mapped (`mirbuilder-final-pipeline-ssot.md:1509-1521`).
- Gate 2 (language-v1-convergence-current.md) is a parked taskboard —
  grammar slice landed (106/106 + differential green) but macro rows
  (type guarantees, coercion-equality, ownership-identity,
  capability-effect, closeout) remain open; requires explicit
  CURRENT_STATE selection.
- Gate 3 (MIRBUILDER-HAKO-MIMALLOC-PROMOTION-GATE0) is parked and
  unexecuted; its milestone `MIRBUILDER-FIRST-PRODUCTION-CUTOVER`
  closed at `6e88441c0b` but gates 1-2 still hold it.
- Gate 4 token `MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001`
  has no card file; defined at
  `design/mirbuilder-authority-based-hako-migration-ssot.md:577-582`.
  15-17 `.hako` parity owners are adopted but none switched a
  production caller — the real pilot tail is caller-switch + Rust
  deletion.
- Retired queue confirmed: MapStore 3454/3455/3456 chain and
  vocabulary rerun 111 are not resume targets.

## Six-line Decision (accepted 2026-09-25)

```text
Decision: row H's first bounded slice is
          MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0 — package one
          of the nine typed EXE fail-fast classes into a (caller,
          terminal, delete) tuple; this is gate-1 work, and jumping
          to the pilot selection would skip the accepted order.
Source authority + canonical issuer: the nine terminal classes are
          already owner-mapped in the acceptance table
          (mirbuilder-final-pipeline-ssot.md:1509-1521); the D0 picks
          one owner and its issuer.
Non-authority: the retired MapStore/classifier queue; fresh
          disconnected pilots.
Fail-fast boundary: each selected class already terminates at a named
          typed stop — the D0's job is to select one whose full
          authority/caller/delete tuple closes.
Smallest next slice: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0 —
          rerun the suite receipt at HEAD, then select one class
          (candidates ranked: exact-usize parameter contract 3/9,
          callable-loop-handoff 2/9, untyped-storage 2/9, selfhost
          emit residuals).
Non-claims: does not assert gate-1 green; gates 2-4 stay queued;
          does not select the Facts parity pilot.
```

## Exit

- [x] First bounded slice of row H selected —
      `MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0`.
- [x] Guard/pointer/workstream synced.
