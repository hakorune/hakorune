---
Status: design_stop__2026-09-21__WarningBaselineRefreshI81__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I81
Date: 2026-09-21
Parent: mirbuilder-warning-loop-true-reject-mapper-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: select one caller-zero facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I81

## Six-line brief

```text
Decision: refresh warning surfaces after the LoopTrue mapper edge closeout and
  select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,741/557, census one
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

## I80/I81 evidence

I80 refreshed the stable baseline at lib **1,742** and lib-test **557**. I81
scoped only the test-only LoopTrue reject mapper edge after rejecting an
over-broad scope that would have removed production route-policy types. Its
lib check produced **1,741** warnings and the focused LoopTrue suite passed
**9/9**.

## Refresh result and selected bounded edge

The sequential refresh completed with lib **1,741** warnings and lib-test
**557**, with no new red. The next finite warning edge is the unused
`use super::*` in `published_backend_view/physical_abi_tests.rs`; the test
imports `MirCompiler` and `NormalCompileRequestV1` explicitly and no symbol from
the parent glob. Removing that one test-only import is the selected I82 slice.

## Closeout result

The selected physical ABI test import deletion passed its lib check, test build,
and named **1/1** focused test. Lib stayed at **1,741** and lib-test decreased
from **557** to **556**.
