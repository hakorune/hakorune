---
Status: closed__2026-09-21__WarningBaselineRefreshI20__SelectedEnumMatchScopeboxTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I20
Date: 2026-09-21
Parent: mirbuilder-warning-unified-call-test-imports-i0-2026-09-21.md
Implementation permission: true for one cfg(test) enum-match ScopeBox route import facade only
NextCard: MIRBUILDER-WARNING-ENUM-MATCH-SCOPEBOX-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I20

## Six-line brief

```text
Decision: refresh both warning surfaces after the unified-call test-import
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,810/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I20 inventory and decision

The fixed commands completed successfully with the expected baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,810 | `/tmp/hakorune-warning-i20-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i20-lib-test-20260921.log` |

The selected caller-zero cohort is the enum-match ScopeBox test facade:

* `src/mir/builder/exprs_enum_match.rs:12` —
  `PreparedRawScopeBoxRouteV1` is consumed only by the local `#[cfg(test)]`
  route-shape test; production ScopeBox code uses the defining
  `enum_match_scopebox` module directly.

The bounded next slice is to gate only this route import with `#[cfg(test)]`;
enum-match lowering and ScopeBox route semantics remain unchanged.
