---
Status: closed__2026-09-21__WarningBaselineRefreshI16__SelectedSourceAuthorityTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I16
Date: 2026-09-21
Parent: mirbuilder-warning-parser-module-rows-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) parser source-authority re-export facade only
NextCard: MIRBUILDER-WARNING-SOURCE-AUTHORITY-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I16

## Six-line brief

```text
Decision: refresh both warning surfaces after the parser module-rows
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,816/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I16 inventory and decision

The fixed commands completed successfully with the expected fresh baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,816 | `/tmp/hakorune-warning-i16-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i16-lib-test-20260921.log` |

The selected caller-zero cohort is the parser source-authority test facade:

* `src/parser/source_authority.rs:30` — `ConstructorSourceOriginV1` is
  re-exported for parser tests; production consumers use the defining
  `constructor_source` module.
* `src/parser/source_authority.rs:64` — `SourceBoxPathSegmentV1` is
  re-exported for parser tests; production consumers use `source_path`
  directly.

Both rows are used by the existing `#[cfg(test)]` parser modules and have no
production consumer through this facade. The bounded next slice is to gate
only these two re-exports with `#[cfg(test)]`; no source authority or path
semantics change is authorized.
