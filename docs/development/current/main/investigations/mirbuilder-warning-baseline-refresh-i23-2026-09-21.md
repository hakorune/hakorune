---
Status: closed__2026-09-21__WarningBaselineRefreshI23__SelectedBoundedBodySnapshotTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I23
Date: 2026-09-21
Parent: mirbuilder-warning-invocation-collection-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) bounded-body snapshot import facade only
NextCard: MIRBUILDER-WARNING-BOUNDED-BODY-SNAPSHOT-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I23

## Six-line brief

```text
Decision: refresh both warning surfaces after the invocation-collection
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,808/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I23 inventory and decision

The fixed commands completed successfully with the current warning baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,808 | `/tmp/hakorune-warning-i23-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i23-lib-test-20260921.log` |

The selected caller-zero cohort is the bounded-body snapshot test facade:

* `src/analysis/bounded_body_snapshot_v0/mod.rs:32` —
  `StrictJsonArenaV0`, `StrictJsonKindV0`, and `StrictJsonNodeIdV0` are
  consumed only by the local `#[cfg(test)]` strict-JSON tree tests; no
  production module consumes this re-export.

The bounded next slice is to gate only this grouped import with `#[cfg(test)]`;
the strict-JSON parser and snapshot schema remain unchanged.
