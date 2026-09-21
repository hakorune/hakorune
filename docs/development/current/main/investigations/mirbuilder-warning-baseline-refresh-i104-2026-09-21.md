---
Status: closed__2026-09-21__WarningBaselineRefreshI104__SelectedProviderImpossibleReject
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I104
Date: 2026-09-21
Parent: mirbuilder-warning-postpass-compatibility-cohort-payload-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-PROVIDER-IMPOSSIBLE-REJECT-I0
---

# MirBuilder warning baseline refresh I104

## Six-line brief

```text
Decision: refresh the postpass compatibility-payload warning surface and
select one bounded row; keep the LoopBreak old-edge lane parked without a
successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,703/552, census one candidate, and choose
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

I104 follows the deletion of an unread AST-only compatibility payload. The
measured baseline is lib **1,703** and lib-test **552**. The old-edge deletion
lane remains `NoSafeSlice`: the live compatibility registry entry still has no
named successor; any future deletion row requires a named successor and
caller-zero proof.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,703** warnings and lib-test
**552** warnings. The old-edge census remains `NoSafeSlice`. The next warning
candidate is `ProviderAdmissionRejectV1::MissingCoreRow`.

The finite caller census is empty: the variant is declared in
`provider_admission/seal.rs`, but no constructor, match arm, test, or external
consumer names it. `consume_text_scan` receives both core rows by reference and
reports `CoreRowMismatch` for invalid rows, so the missing-row state cannot be
issued through the current owner. The next slice removes only this impossible
enum variant; all reachable rejection states and the provider admission owner
remain unchanged.

The next bounded cohort is
`MIRBUILDER-WARNING-PROVIDER-IMPOSSIBLE-REJECT-I0`.
