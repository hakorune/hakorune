---
Status: design_stop__2026-09-21__WarningBaselineRefreshI67__NextCohortSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I67
Date: 2026-09-21
Parent: mirbuilder-warning-function-control-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0
---

# MirBuilder warning baseline refresh I67

## Six-line brief

```text
Decision: refresh both warning surfaces after the two-layer function-control
  facade deletion before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,754/558 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run these commands once each, sequentially, and record warning lint, file and
line, owner, and production/test/compat/generated role:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Select one finite caller-zero import cohort or write `NoSafeSlice`. Keep
dead-code and private-interface rows with their owners. No code edit is
permitted until the selection is recorded and the pointer enters its bounded
successor. The current fixed baseline is lib **1,754** and lib-test **558**.
