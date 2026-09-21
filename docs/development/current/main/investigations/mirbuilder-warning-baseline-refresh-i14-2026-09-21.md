---
Status: closed__2026-09-21__WarningBaselineRefreshI14__ParserModuleRowsTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I14
Date: 2026-09-21
Parent: mirbuilder-warning-parser-source-catalog-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) parser-module-rows re-export move only
NextCard: MIRBUILDER-WARNING-PARSER-MODULE-ROWS-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I14

## Six-line brief

```text
Decision: refresh both warning surfaces after the parser-source catalog
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,817/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I14 inventory and decision

The fresh commands both exited 0: lib produced 1,817 warnings and lib-test
produced 561 warnings. The `unused_imports` diagnostic at
`src/parser/callable_parameter_source/script_source_authority/mod.rs:24` is
one caller-zero test-facade row for
`ParserNormalModuleSourceRowsDispositionV1`. Repository-wide reference search
finds this re-export only in the `#[cfg(test)]` normal-root test terminal;
production model code uses the defining module directly. The selected next
slice was one `#[cfg(test)]` re-export gate, with expected lib 1,816 and
lib-test 561. Execution completed with those counts; the implementation card
records the fixed command evidence.
