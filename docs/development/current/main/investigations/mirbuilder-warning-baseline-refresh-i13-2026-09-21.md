---
Status: closed__2026-09-21__WarningBaselineRefreshI13__ParserSourceCatalogTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I13
Date: 2026-09-21
Parent: mirbuilder-warning-snapshot-witness-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) parser-source catalog import move only
NextCard: MIRBUILDER-WARNING-PARSER-SOURCE-CATALOG-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I13

## Six-line brief

```text
Decision: refresh both warning surfaces after the snapshot-witness
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,818/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I13 inventory and decision

The fresh commands both exited 0: lib produced 1,818 warnings and lib-test
produced 561 warnings. The `unused_imports` diagnostic at
`src/parser/callable_parameter_source/product.rs:3` is one caller-zero
test-facade row for `ParserCallableParameterSourceCatalogV1`. Its references
inside this owner are limited to `#[cfg(test)]` terminal helpers; production
uses the sibling disposition type and canonical catalog owner directly. The
selected next slice is one import split with a `#[cfg(test)]` gate, with
expected lib 1,817 and lib-test 561. Execution completed with those counts;
the implementation card records the fixed command evidence.
