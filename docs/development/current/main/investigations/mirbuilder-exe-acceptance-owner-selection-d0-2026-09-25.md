# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D0 — pick one EXE fail class

Status: open__census__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-UNIFIED-SELFHOST-RESUME-D0 (accepted 2026-09-25)
Owner: workstream row H / unified resume order gate 1
  (docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md:49)
Authority: docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
             acceptance table (:1509-1521)

## Question

The `real-apps-exe-boundary` suite last recorded 2 pass / 9 fail, each
red a typed fail-fast terminal with an owner class already mapped.
Select exactly ONE fail class whose (source authority, canonical
issuer, live production caller, fail-fast terminal, exclusive
delete-set, acceptance) tuple can close — i.e. the class's fix lands
the missing authority path AND retires the old edge, so the suite
entry flips green on a rerun.

## Census boundary

- Start: fresh `real-apps-exe-boundary` rerun receipt at current HEAD
  (`tools/smokes/v2/run.sh --profile integration --owner-profile
  integration --suite real-apps-exe-boundary`).
- End: one selected class with a complete tuple + one bounded
  execution card (I0/S0).
- Includes: the four named owner classes (exact-usize parameter
  contract, callable-loop-handoff, untyped-storage ordinary-new
  commit, selfhost emit lane residuals).
- Excludes: gates 2-4 (parked/queued), the retired MapStore queue,
  baseline cargo-lib debt (manifests).

## Evidence

(recorded during census)

## Six-line Decision

(recorded during census)

## Exit

- [ ] One EXE fail class selected with complete tuple, or
      `NoSafeSlice` with observable reopen trigger.
- [ ] Guard/pointer/workstream synced.
