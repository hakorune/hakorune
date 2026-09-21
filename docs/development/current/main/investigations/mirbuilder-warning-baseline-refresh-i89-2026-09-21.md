---
Status: design_stop__2026-09-21__WarningBaselineRefreshI89__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I89
Date: 2026-09-21
Parent: mirbuilder-warning-parser-body-source-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: select one caller-zero facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I89

## Six-line brief

```text
Decision: refresh warning surfaces after the parser body-source test facade
  and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,723/553, census one
  candidate, and select Delete or NoSafeSlice.
Non-claims: no semantic refactor, suppression, production switch, or broad
  warning cleanup.
```

## Acceptance

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record warning class, file and line, owner, role, grouped-diagnostic
membership, and caller inventory. Select at most one caller-zero warning
facade whose production edge can be removed with a focused guard; otherwise
record `NoSafeSlice`. Keep dead-code and private-interface rows with their
owners. A selected edge must prove caller-zero before physical removal, then
run its focused gate and the stable warning refresh at its parent/current pair.

## I89 handoff evidence

The parser body-source facade closed at `7e7deb4a87`: lib **1,723**, lib-test
**553**, body-source **1/1**, release-source **7/7**, and preserved production
function-carrier **3/3**. This card remeasures that pair before choosing the
next warning row; it does not reopen the parser or lease design.
