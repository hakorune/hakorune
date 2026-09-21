---
Status: design_stop__2026-09-21__WarningBaselineRefreshI94__SelectNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I94
Date: 2026-09-21
Parent: mirbuilder-warning-loop-phi-index-cfg-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: one explicitly justified caller-zero deletion or warning row
---

# MirBuilder warning baseline refresh I94

## Six-line brief

```text
Decision: refresh the post-cfg-cleanup warning surface and select one bounded
row; keep the old-edge lane parked until a live compatibility successor exists.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the LoopBreak physical owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
source caller for the compatibility route.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,710/552, census one candidate, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad warning cleanup, LoopBreak route deletion, source
publication, VM repair, or LegacyCallV0 retirement.
```

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record each selected diagnostic's class, owner, cfg/production role, and
caller inventory. A future old-edge row must first name a successor for the
live compatibility registry entry; otherwise keep `NoSafeSlice`. Select at
most one next implementation row and update this card before code.

## Handoff

I93 established that the first source deletion can be a small warning edge,
but the proposed LoopBreak compatibility deletion is not yet safe. The
current measured baseline is lib **1,710** and lib-test **552** after the
LoopPhi mixed-cfg cleanup. The next selection must preserve this distinction:
warning cleanup may proceed only as a bounded owner row, while the old-edge
lane remains blocked by the live compatibility caller.
