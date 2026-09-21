---
Status: closed__2026-09-21__WarningBaselineRefreshI17__SelectedCallableSourceRetentionTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I17
Date: 2026-09-21
Parent: mirbuilder-warning-source-authority-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) parser callable-source re-export facade only
NextCard: MIRBUILDER-WARNING-CALLABLE-SOURCE-RETENTION-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I17

## Six-line brief

```text
Decision: refresh both warning surfaces after the source-authority test-facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,814/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I17 inventory and decision

The fixed commands completed successfully with the expected baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,814 | `/tmp/hakorune-warning-i17-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i17-lib-test-20260921.log` |

The selected caller-zero cohort is the parser callable-source retention test
facade:

* `src/parser/callable_parameter_source/mod.rs:69` —
  `ParserCallableSourceRetentionErrorV1` is re-exported for
  `retained_tests.rs`; production code uses the defining `product` module
  directly.

The row has no production consumer through this facade. The bounded next
slice is to gate only this re-export with `#[cfg(test)]`; parser retention
semantics and the error owner remain unchanged.
