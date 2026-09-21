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

## Refresh result and bounded selection

The sequential refresh completed on 2026-09-21 with the fixed commands and no
new failure: lib **1,754** warnings and lib-test **558** warnings. The first
caller-zero import cohort is the parent facade re-export
`src/mir/function.rs:59::PublishedStaticMethodCallCRowV1`. A repository-wide
source census finds the definition and its real consumers under
`mir/compiler/normal_default_pipeline/published_backend_view`; the only
occurrence in `mir/function.rs` is the unused parent re-export, while the
historical child module imports the canonical compiler owner directly. Removing
this one parent import therefore does not alter the function-view API used by
the C-frame consumers and cannot move the warning into an owner-local import.

Selected successor: `MIRBUILDER-WARNING-STATIC-METHOD-CROW-FACADE-I0`.
The successor may remove only this parent facade item, then rerun both fixed
gates and classify any changed warning count. No other warning family, dead-code
row, visibility change, or production route is in scope.
