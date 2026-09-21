---
Status: closed__2026-09-21__WarningBaselineRefreshI108__SelectedRawPhysicalBrand
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I108
Date: 2026-09-21
Parent: mirbuilder-warning-module-token-id-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-RAW-PHYSICAL-BRAND-I0
---

# MirBuilder warning baseline refresh I108

## Six-line brief

```text
Decision: refresh the module-token identity warning surface and select one
bounded row; keep the LoopBreak old-edge lane parked without a successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,700/551, census one candidate, and choose
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

I108 follows deletion of the unused module-token identity accessors. The
measured baseline is lib **1,700** and lib-test **551**. The old-edge deletion
lane remains `NoSafeSlice`: the live compatibility registry entry still has no
named successor; any future deletion row requires a named successor and
caller-zero proof.

## Refresh result and selected cohort

The sequential refresh reproduced lib **1,700** warnings and lib-test **551**
warnings. The selected warning is the unused
`CompletedRawRootBatchPhysicalV1::brand` accessor in
`raw_root_physical/root_batch_terminal.rs`.

The production caller census is empty. One raw-root environment test reads the
accessor; its bounded assertion now exercises the existing
`prepare_raw_drain` handoff, so the test still verifies the physical route
without retaining a convenience projection. No source, tool, or tracked-doc
caller reads the accessor. Invocation brand checks performed during physical
sealing remain intact.

The next bounded cohort is `MIRBUILDER-WARNING-RAW-PHYSICAL-BRAND-I0`.
