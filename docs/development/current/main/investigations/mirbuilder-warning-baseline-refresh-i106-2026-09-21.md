---
Status: closed__2026-09-21__WarningBaselineRefreshI106__SelectedModuleTokenId
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I106
Date: 2026-09-21
Parent: mirbuilder-warning-module-id-same-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-MODULE-TOKEN-ID-I0
---

# MirBuilder warning baseline refresh I106

## Six-line brief

```text
Decision: refresh the module-identity warning surface and select one bounded
row; keep the LoopBreak old-edge lane parked without a successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,701/551, census one candidate, and choose
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

I106 follows deletion of the unused module identity comparison. The measured
baseline is lib **1,701** and lib-test **551**. The old-edge deletion lane
remains `NoSafeSlice`: the live compatibility registry entry still has no
named successor; any future deletion row requires a named successor and
caller-zero proof.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,701** warnings and lib-test
**551** warnings. The selected warning cohort is the unused
`ModuleInvocationTokenV1::id` accessor and its now-unreachable test-only
`ModuleInvocationIdV1::ordinal` helper in `module_invocation_identity.rs`.

The finite caller census is production-zero: no source, tool, or tracked-doc
caller exists. Four identity fixtures use the accessor only to inspect a brand
or ordinal; they can use the existing `brand()` accessor and token
`PartialEq` directly. The bounded row leaves
`ModuleInvocationBrandV1::same` unchanged.

The next bounded cohort is `MIRBUILDER-WARNING-MODULE-TOKEN-ID-I0`.
