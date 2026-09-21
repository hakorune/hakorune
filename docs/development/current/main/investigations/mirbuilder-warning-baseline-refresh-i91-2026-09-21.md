---
Status: design_stop__2026-09-21__WarningBaselineRefreshI91__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I91
Date: 2026-09-21
Parent: mirbuilder-warning-array-state-identity-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: MIRBUILDER-WARNING-PARSER-SOURCE-CATALOG-SAME-PARSER-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I91

## Six-line brief

```text
Decision: refresh warning surfaces after the ArrayStateIdentity test facade
  and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,713/553, census one
  candidate, and select Delete or NoSafeSlice.
Non-claims: no semantic refactor, suppression, production switch, or broad
  warning cleanup.
```

## Acceptance

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record warning class, file and line, owner, role, grouped-diagnostic
membership, and caller inventory. Select at most one caller-zero warning
facade whose production edge can be removed with a focused guard; otherwise
record `NoSafeSlice`. Keep dead-code and private-interface rows with their
owners. A selected edge must prove caller-zero before physical removal, then
run its focused gate and the stable warning refresh at its parent/current pair.

## I91 handoff evidence

The ArrayStateIdentity facade closed with lib **1,713**, lib-test **553**, and
focused identity/runtime-contract evidence **1/1** and **4/4**. This card
remeasures that pair before choosing the next warning row; it does not reopen
array storage or identity semantics.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,713** warnings and lib-test
**553** warnings. No new red or private-interface diagnostic appeared.

The candidate inventory was checked against the current requirements and
caller boundaries:

* `admitted_registry::branch_count` remains excluded because
  `aot_admission.rs:153` is a production caller.
* `loop_phi_materializer.rs:467` (`index`) is a mixed-cfg local used by the
  test failure-injection branch; it is not a removable facade and remains
  parked for a separate cfg-aware cleanup row.
* `src/parser/callable_parameter_source/catalog.rs:36`
  `ParserCallableParameterSourceCatalogV1::same_parser_source` has exactly
  the two assertions in `callable_parameter_source/tests.rs:225-226` and no
  production caller. It is the selected one-warning test facade.

The next bounded cohort is
`MIRBUILDER-WARNING-PARSER-SOURCE-CATALOG-SAME-PARSER-TEST-FACADE-I0`.
