---
Status: design_stop__2026-09-21__WarningBaselineRefreshI71__NextCohortSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I71
Date: 2026-09-21
Parent: mirbuilder-warning-dynamic-operator-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0
---

# MirBuilder warning baseline refresh I71

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

## Refresh result and bounded selection

The sequential refresh completed on 2026-09-21 with no new failure: lib
generated **1,751** warnings and lib-test generated **557** warnings. The next
caller-zero item is `PublishedStaticMethodCFrameV1` at
`src/mir/function.rs:59`. A complete census finds its consumers only in
`#[cfg(test)]` modules (`mir_json_emit/io.rs`, the published-backend-view test
modules, and the compiler map-query test); no non-test caller exists. The
canonical definition remains `mir/compiler/normal_default_pipeline` and the
historical function facade is only a test bridge. Scoping the C-frame re-export
to `cfg(test)` in both function facade layers removes it from the production
warning surface while preserving those test paths.

Selected successor: `MIRBUILDER-WARNING-STATIC-METHOD-CFRAME-TEST-SCOPE-I0`.
The C-row owner and C-frame ABI are outside the slice.
