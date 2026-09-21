---
Status: closed__2026-09-21__WarningBaselineRefreshI105__SelectedModuleIdSame
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I105
Date: 2026-09-21
Parent: mirbuilder-warning-provider-impossible-reject-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-MODULE-ID-SAME-I0
---

# MirBuilder warning baseline refresh I105

## Six-line brief

```text
Decision: refresh the provider-reject deletion surface and select one bounded
row; keep the LoopBreak old-edge lane parked without a successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,702/551, census one candidate, and choose
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

I105 follows deletion of an impossible provider admission rejection. The
measured baseline is lib **1,702** and lib-test **551**. The old-edge deletion
lane remains `NoSafeSlice`: the live compatibility registry entry still has no
named successor; any future deletion row requires a named successor and
caller-zero proof.

## Refresh result and selected cohort

The sequential refresh reproduced lib **1,702** warnings and lib-test **551**
warnings. The old-edge census remains `NoSafeSlice`. The selected warning is
`ModuleInvocationIdV1::same` in `module_invocation_identity.rs`.

The finite caller census is production-zero: the method is declared once and
has no source, tool, or tracked-doc caller. One test assertion uses the method
to compare two IDs; the bounded implementation row replaces that assertion
with the existing derived `PartialEq` identity comparison, then removes the
method. `ModuleInvocationBrandV1::same` remains the canonical brand comparison
used by the identity kernel and is outside this row.

The next bounded cohort is `MIRBUILDER-WARNING-MODULE-ID-SAME-I0`.
