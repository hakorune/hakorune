---
Status: design_stop__2026-09-21__WarningBaselineRefreshI70__NextCohortSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I70
Date: 2026-09-21
Parent: mirbuilder-warning-dynamic-operator-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0
---

# MirBuilder warning baseline refresh I70

## Six-line brief

```text
Decision: refresh both warning surfaces after the dynamic-operator facade
  deletion and focused tests.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: diagnostic guesses, cargo-fix, blanket allow, or dead-code
  interpretation without a caller census.
Fail-fast boundary: a new warning, owner-local move, command drift, or an
  unclassified test-only reference stops selection.
Smallest next slice: compare lib/lib-test against 1,751/557 and select at most
  one caller-zero cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or route
  change.
```

## Acceptance

Run these commands once each, sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record lint, file and line, owner, role, and grouped-diagnostic membership.
Select one finite caller-zero import cohort or write `NoSafeSlice`. Keep
dead-code and private-interface rows with their owners.
