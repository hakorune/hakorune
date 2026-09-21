---
Status: closed__2026-09-21__WarningBaselineRefreshI65__SelectedGenericG0TestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I65
Date: 2026-09-21
Parent: mirbuilder-warning-raw-profile-facade-import-i0-2026-09-21.md
Implementation permission: false; selection recorded, successor must enter fast mode before code
NextCard: MIRBUILDER-WARNING-GENERIC-G0-DEMAND-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I65

## Six-line brief

```text
Decision: refresh both warning surfaces after the Raw profile facade-import
  deletion before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,754/560 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run these commands once each, sequentially, and record warning lint, file and
line, owner, and production/test/compat/generated role:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Select one finite caller-zero import cohort or write `NoSafeSlice`. Keep
dead-code and private-interface rows with their owners. No code edit is
permitted until the selection is recorded and the pointer enters its bounded
successor. The current fixed baseline is lib **1,754** and lib-test **560**.

## Selection evidence

The fixed gates completed sequentially with exit 0 at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,754 | `/tmp/hakorune-warning-i65-lib-20260921.log` |
| lib test | 560 | `/tmp/hakorune-warning-i65-lib-test-20260921.log` |

The selected bounded cohort is the cfg(test)-only parent re-export
`generic_g0::generic_operation_demand_parts_for_test` at
`src/mir/loop_recipe_contract/mod.rs:151`. Its definition and canonical test
re-export live under `loop_recipe_contract/generic_g0`; both current test
callers import that owner path directly. No production caller or parent-facade
caller exists. The successor may remove only the one parent re-export, with
expected lib unchanged at **1,754** and lib-test **560 → 559**.

The other first warning groups remain owned rows: dynamic invocation and
operator facades have local test consumers, published backend/function and If
recipe imports have mixed external consumers, and dead-code/private-interface
warnings remain informational owner rows. No broad import cleanup or
visibility change is selected.
