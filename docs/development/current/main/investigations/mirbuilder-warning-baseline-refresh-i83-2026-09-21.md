---
Status: design_stop__2026-09-21__WarningBaselineRefreshI83__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I83
Date: 2026-09-21
Parent: mirbuilder-warning-physical-abi-test-import-delete-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: select one caller-zero facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I83

## Six-line brief

```text
Decision: refresh warning surfaces after the physical ABI test import deletion
  and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,741/556, census one
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

## I82 evidence

The physical ABI test import slice deleted one unused `use super::*` line. Its
lib check stayed at **1,741**, lib-test is now **556**, and the named physical
ABI test passed **1/1**. No ABI or production route changed.

## Refresh result and selected bounded edge

The sequential refresh completed with lib **1,741** warnings and lib-test
**556**, with no new red. The finite candidate is the `#[cfg(test)]` parent
re-export in `src/mir/callable_semantic_batch/mod.rs`:
`issue_resolved_callable_semantic_batch_with_policy_v1` and
`DirectCallObservationBatchPolicyV1` have no repository caller through that
facade; the issuer's internal references remain canonical. I84 will remove only
those two unused export edges.

## Closeout result

The callable semantic batch re-export deletion passed lib check, test build, and
its named **13/13** focused suite. Lib stayed at **1,741** and lib-test
decreased from **556** to **555**.
