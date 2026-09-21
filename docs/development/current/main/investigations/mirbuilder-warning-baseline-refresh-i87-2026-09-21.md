---
Status: design_stop__2026-09-21__WarningBaselineRefreshI87__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I87
Date: 2026-09-21
Parent: mirbuilder-warning-program-v0-snapshot-witness-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: select one caller-zero facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I87

## Six-line brief

```text
Decision: refresh warning surfaces after the ProgramV0 witness test-scope
  closeout and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,737/553, census one
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

## I87 evidence

The ProgramV0 snapshot witness module is now test-only. Lib warnings are
**1,737**, lib-test is **553**, and the snapshot focused suite passed **38/38**.

## Refresh result and selected bounded edge

The sequential refresh completed with lib **1,737** warnings and lib-test
**553**, with no new red. The strict JSON tree warning family is excluded
because runtime-direct extern-provider code consumes it. The next finite test-only
edge is the `entry`, `branch_count`, and `row` methods on
`AdmittedTextScanRowV1`/`AdmittedTextScanRegistryV1`; repository census finds
callers only in the module's own tests. I88 will scope those three methods to
`cfg(test)`.

## Closeout result

The admitted-registry method slice first rejected production use of
`branch_count`, restored it, and then passed lib check, test build, and the
focused **2/2** suite with only `entry` and `row` scoped to tests. Lib warnings
decreased from **1,737** to **1,736**; lib-test remained **553**.
