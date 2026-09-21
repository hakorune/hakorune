---
Status: fast__2026-09-21__SelectedBoundedWarningCohort__ExecuteTestFacadeDeletion
Task: MIRBUILDER-WARNING-DYNAMIC-INVOCATION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i68-2026-09-21.md
Implementation permission: true for the two catalog test-facade imports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I69
---

# MirBuilder dynamic invocation test facade I0

## Six-line brief

```text
Decision: move two catalog test imports to the owner test module and delete the
  unused parent facade re-export.
Source authority + canonical issuer: dynamic_invocation_contract/catalog.rs.
Non-authority: the parent mod.rs facade, warning-count guesses, or production
  route code.
Fail-fast boundary: a production reference, unresolved test import, or changed
  catalog behavior stops the slice.
Smallest next slice: import the two catalog items directly in tests.rs, remove
  only the parent use, and run the two fixed quick gates.
Non-claims: no dynamic operator change, no semantic route change, no suppressions,
  and no broad warning cleanup.
```

## Evidence and acceptance

I68 refreshed the fixed baseline at lib **1,754** / lib-test **558**. The source
census is finite: the catalog types are defined in `catalog.rs`, used by the
catalog implementation and its tests, and have no production consumer outside
that owner. The fast slice must preserve all catalog tests and run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Success requires the parent facade items to disappear, the test module to import
the canonical owner directly, no warning to move to another parent layer, and
both commands to pass. The fixed counts may change only according to the
observed diagnostic grouping and must be recorded explicitly.
