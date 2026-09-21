---
Status: closed__2026-09-22__WarningBaselineRefreshI134__SelectedI135
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I134
Date: 2026-09-22
Parent: mirbuilder-warning-forest-binding-accessors-i133-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-VARIABLE-ACCUM-ROLE-ORDINAL-I135
---

# MirBuilder warning baseline refresh I134

## Six-line brief

```text
Decision: refresh the warning baseline after I133 and select one bounded
  production-zero cleanup row; keep the old-edge lane parked unless its
  caller-zero or same-series switch proof changes.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
  warning classification policy, and each selected owner’s caller census.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  or semantic receipts without a named source owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
  or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: one caller-zero old edge, or one finite production-zero
  warning item after exact census.
Non-claims: no broad warning cleanup, parser semantic change, VM repair,
  fallback, or LegacyCallV0 retirement beyond one selected slice.
```

## Baseline and retirement boundary

I133 restricted the two forest-binding convenience accessors to test builds;
the forest, parent indices, owner field, and Recipe conversion remain live.
The focused forest suite passed **28/28**. The new baseline is lib **1,687**
warnings and lib-test **545** warnings. The LoopBreak old-edge census remains
`NoSafeSlice` with live registry, compatibility, parser-composite, test, and
guard callers.

## Acceptance before selection

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record both warning counts and a complete owner/caller inventory before
selecting the next fast card. Do not delete parser source-admission rows or
shared compatibility helpers without a fresh census and successor proof.
