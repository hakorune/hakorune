---
Status: closed__2026-09-21__WarningBaselineRefreshI30__RootCatalogLifecycleStageTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I30
Date: 2026-09-21
Parent: mirbuilder-warning-function-owner-id-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) lifecycle-stage re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I31
---

# MirBuilder warning baseline refresh I30

## Six-line brief

```text
Decision: refresh both warning surfaces after the I29 FunctionOwnerId facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,801/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Selection evidence

The fixed commands completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,801 | `/tmp/hakorune-warning-i30-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i30-lib-test-20260921.log` |

The selected caller-zero cohort is the root-catalog lifecycle stage test
facade:

* `src/mir/builder.rs:342` —
  `NormalDefaultRootCatalogLifecycleStageV1` is re-exported only for
  `normal_default_root_catalog_lifecycle_tests.rs`. The lifecycle owner uses
  its type internally; no production caller uses this builder re-export.

The bounded execution slice gates only this re-export with `#[cfg(test)]`.
Lifecycle state transitions, the owner module, and production lowering remain
unchanged. Any production consumer, compile failure, changed test warning, or
count mismatch returns the row to design stop.

## Closeout evidence

The lifecycle-stage re-export in `builder.rs` is now `#[cfg(test)]`; the owner
module and production lifecycle state remain unchanged. The fixed commands were
run sequentially and both exited 0:

| command | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib warnings **1,800** | `/tmp/hakorune-warning-i0-root-catalog-lifecycle-stage-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test warnings **561**, test executable built | `/tmp/hakorune-warning-i0-root-catalog-lifecycle-stage-lib-test-20260921.log` |

The only source change is the builder re-export attribute; lifecycle state
transitions, production lowering, and test behavior are unchanged.
