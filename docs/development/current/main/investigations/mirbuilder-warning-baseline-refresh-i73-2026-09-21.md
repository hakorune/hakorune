---
Status: design_stop__2026-09-21__WarningBaselineRefreshI73__NextDeletionSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I73
Date: 2026-09-21
Parent: mirbuilder-warning-if-join-reexport-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded deletion cohort only
NextCard: select one caller-zero warning facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I73

## Six-line brief

```text
Decision: refresh both warning surfaces after the If Join test-only scope and
  select one finite caller-zero source deletion if its owner is proven.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or test-only
  red interpretation without parent reproduction.
Fail-fast boundary: a new warning, a non-test caller, command drift, or an
  unclassified focused red stops deletion selection.
Smallest next slice: compare lib/lib-test against 1,749/557, census one
  caller-zero facade, and choose Delete or NoSafeSlice.
Non-claims: no semantic refactor, suppression, production switch, or broad
  warning cleanup.
```

## Acceptance

Run these commands once each, sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record lint, file and line, owner, role, and grouped-diagnostic membership.
Select at most one caller-zero warning facade whose production edge can be
removed with a focused guard; otherwise record `NoSafeSlice`. Keep dead-code
and private-interface rows with their owners. The next selected deletion must
prove caller-zero before physical removal, then run its focused gate and the
stable warning refresh at its parent/current pair.

## I72/If-Join closeout evidence

I72 refreshed the baseline at lib **1,750** and lib-test **557**, then selected
the three caller-zero If Join parent re-exports. I0 landed the test-only scope
and passed its focused `if_recipe` suite **24/24**. The current lib count is
**1,749**; the next refresh must confirm the lib-test surface remains **557**.
The canonical JoinSig and physical owners remain unchanged.
