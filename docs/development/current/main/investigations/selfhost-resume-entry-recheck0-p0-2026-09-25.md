# SELFHOST-RESUME-ENTRY-RECHECK0-P0 — selfhost resume entry recheck

Status: open
Date: 2026-09-25
Parent: REPO-FINAL-CONVERGENCE-AUDIT0-G0 (closed 2026-09-25)
Authority: docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  (unified resume order)
Implementation permission: false; this is a census/reconciliation row.
No code, route, fixture, or fallback change.

## Scope

G0 closed the repo-structure cleanup lane only. MirBuilder overall is
not complete. This row reconciles the named residuals against the
selfhost resume entry conditions and selects the next bounded owner;
it does not resume selfhost and does not open the staged Call/R6-R7
queue.

## Six-line brief

```text
Decision: reconcile Call/R7 and B3 residuals against unified resume
  order gates 1-4; select exactly one next bounded owner.
Source authority + canonical issuer:
  selfhost-parser-mirbuilder-migration-order-ssot.md unified resume
  order owns gates 1-6; mirbuilder-inplace-replacement-current.md owns
  the Call/R7 and B3 residual dispositions.
Non-authority: this card selects no production switch, deletes no
  legacy edge, and grants no implementation permission.
Fail-fast boundary: a residual with no bounded tuple stays
  frontier-paused; a missing gate owner is a named blocker, never a
  default fallback.
Smallest next slice: record the gate-by-gate verdicts and name the
  next row/card.
Non-claims: no whole-MirBuilder completion claim; no language or
  mimalloc gate result is invented.
```

## Census boundary

`このcensusが覆う境界: Call/R7 residual + B3 residual -> unified
resume order gates 1-4; includes the staged R6-S0..R7 queue and the
11-entry EXE owner; excludes language-row implementation, mimalloc
evidence, and any new authority selection beyond naming the next owner.`

## Named residuals (input)

- Call/R7 `MIR-CALL-COMPATIBILITY-RETIRE-R7`: aggregate
  writer/reader/reissuer/re-entry deletion open; frontier pause; staged
  R6-S0..R7 queue unopened pending exact boundary selection.
- B3 D2: `NoSafeSlice` — production substring route/codepoint outcome
  and ArrayPush provider/failure/commit authorities missing.
- 11-entry EXE suite (`real-apps-exe-boundary.txt`): 2 pass / 9 fail on
  2026-09-25; all failures typed fail-fast terminals in the selfhost
  emit lane; owned by `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE`.

## Exit

Gate-by-gate verdict table plus the selected next execution row.
Any unowned failure names a blocker with reopen trigger.
