---
Status: design_stop__2026-09-21__WarningBaselineRefreshI86__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I86
Date: 2026-09-21
Parent: mirbuilder-warning-completed-result-context-test-import-delete-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: select one caller-zero facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I86

## Six-line brief

```text
Decision: refresh warning surfaces after the completed-result-context import
  deletion and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,741/553, census one
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

## I86 evidence

The completed-result-context slice removed one unused test-only glob. Lib remains
**1,741**, lib-test is **553**, and the exact focused tests passed **2/2**.
