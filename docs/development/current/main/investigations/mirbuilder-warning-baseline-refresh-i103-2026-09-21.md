---
Status: closed__2026-09-21__WarningBaselineRefreshI103__SelectedPostpassCompatibilityPayload
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I103
Date: 2026-09-21
Parent: mirbuilder-warning-postpass-test-projection-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-POSTPASS-COMPATIBILITY-COHORT-PAYLOAD-I0
---

# MirBuilder warning baseline refresh I103

## Six-line brief

```text
Decision: refresh the postpass projection-facade warning surface and select
one bounded row; keep the LoopBreak old-edge lane parked without a successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,704/552, census one candidate, and choose
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

I103 follows the coupled postpass test-projection facade. The measured baseline
is lib **1,704** and lib-test **552**. The old-edge deletion lane remains
blocked because `route_loop_break_recipe` is still the live registry
compatibility handler; a future deletion row requires a named successor and
caller-zero proof.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,704** warnings and lib-test
**552** warnings. The old-edge census remains `NoSafeSlice`: the live
compatibility registry entry still has no named successor. The next warning
group is `ParserBoxPostpassRowV1::AstOnlyCompatibility::cohort`.

The exact caller census is finite: the field is constructed only by
`postpass_envelope/source_rows.rs`, read only by one postpass test pattern, and
has no production read. `ParserCompatibilityCohortV1` and its conversion
method exist only to carry that unread row payload; `program_cohort` and the
named `SourceSealForCompatibility` error remain the production authorities.
The selected next slice deletes that payload, its helper enum/conversion, and
the now-unneeded source-row arguments while preserving row presence and the
program-cohort assertion.

The next bounded cohort is
`MIRBUILDER-WARNING-POSTPASS-COMPATIBILITY-COHORT-PAYLOAD-I0`.
