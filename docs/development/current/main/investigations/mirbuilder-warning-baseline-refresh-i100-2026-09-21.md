---
Status: closed__2026-09-21__WarningBaselineRefreshI100__SelectedProgramCohortFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I100
Date: 2026-09-21
Parent: mirbuilder-warning-source-active-ordinal-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-POSTPASS-PROGRAM-COHORT-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I100

## Six-line brief

```text
Decision: refresh the post-active-ordinal-facade warning surface and select
one bounded row; keep the LoopBreak old-edge lane parked without a successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,706/552, census one candidate, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, LoopBreak deletion, publication, VM repair, or
LegacyCallV0 retirement.
```

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record the warning class, owner, cfg/production role, and caller inventory.
Keep similarly named production counters and the live compatibility route in
their existing owner rows. Select at most one implementation row, or record
`NoSafeSlice`, before changing code.

## Handoff

I100 follows the source active-ordinal test facade. The measured baseline is
lib **1,706** and lib-test **552**. The old-edge deletion lane remains blocked
because `route_loop_break_recipe` is still the live registry compatibility
handler; a future deletion row requires a named successor and caller-zero
proof.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,706** warnings and lib-test
**552** warnings. The old-edge census remains `NoSafeSlice`: the live
compatibility registry entry still has no named successor. The next warning is
`ParserBoxPostpassCoverageV1::program_cohort`; its three observed callers are
inside the postpass envelope test module, with no production caller. The
coverage rows and source-backed conversion methods remain unchanged.

The next bounded cohort is
`MIRBUILDER-WARNING-POSTPASS-PROGRAM-COHORT-TEST-FACADE-I0`.
