---
Status: closed__2026-09-21__WarningBaselineRefreshI25__SelectedDirectStaticPhysicalInputTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I25
Date: 2026-09-21
Parent: mirbuilder-warning-loop-physical-input-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) direct-static physical-input import facade only
NextCard: MIRBUILDER-WARNING-DIRECT-STATIC-PHYSICAL-INPUT-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I25

## Six-line brief

```text
Decision: refresh both warning surfaces after the loop physical-input
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,806/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I25 inventory and decision

The fixed commands completed successfully with the current warning baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,806 | `/tmp/hakorune-warning-i25-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i25-lib-test-20260921.log` |

The selected caller-zero cohort is the direct-static physical-input test
facade:

* `src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs:8`
  — `SourceExprSiteV1` is consumed only by the local `#[cfg(test)]` input
  projection tests; production physical-input code uses the owner identity and
  direct static recipe types without this direct import.

The bounded next slice is to gate only this import with `#[cfg(test)]`;
static-join physical input validation remains unchanged.
