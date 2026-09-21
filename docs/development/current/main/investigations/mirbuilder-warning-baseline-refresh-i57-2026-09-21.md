---
Status: closed__2026-09-21__WarningBaselineRefreshI57__SelectedPublishedBackendTestImports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I57
Date: 2026-09-21
Parent: mirbuilder-warning-generic-g0-admission-test-import-i0-2026-09-21.md
Implementation permission: true for the selected test-only published-backend re-exports only
NextCard: MIRBUILDER-WARNING-PUBLISHED-BACKEND-TEST-IMPORTS-I0
---

# MirBuilder warning baseline refresh I57

## Six-line brief

```text
Decision: refresh both warning surfaces after the Generic G0 admission
  test-import cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,766/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,766** and lib-test **561** after the
selected Generic G0 admission test re-export cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the Generic G0 admission
cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,766 | `/tmp/hakorune-warning-i57-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i57-lib-test-20260921.log` |

The first two warning groups are the unused `PublishedCallKindV1` and
`CompiledEntryCleanupKindV1` re-exports at
`src/mir/compiler/normal_default_pipeline/published_backend_view.rs:39-42`. A
source census shows both are consumed only by `#[cfg(test)]` fixtures: the call
kind by the historical function-view tests and the cleanup kind by the compiled
entry contract fixture. Production code consumes the canonical transport and
compiled-entry contract types directly. The bounded same-owner slice gates both
test-only re-exports together, with expected lib **1,766 → 1,764** and lib-test
unchanged at **561**. No backend view, transport, cleanup, or test body is selected.
