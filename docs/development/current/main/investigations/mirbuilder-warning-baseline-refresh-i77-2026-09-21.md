---
Status: design_stop__2026-09-21__WarningBaselineRefreshI77__NextDeletionSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I77
Date: 2026-09-21
Parent: mirbuilder-warning-loop-family-selector-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded deletion cohort only
NextCard: select one caller-zero warning facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I77

## Six-line brief

```text
Decision: refresh both warning surfaces after the canonical loop selector
  test-only scope and select one finite caller-zero source deletion if proven.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or test-only
  red interpretation without parent reproduction.
Fail-fast boundary: a new warning, a non-test caller, command drift, or an
  unclassified focused red stops deletion selection.
Smallest next slice: compare lib/lib-test against 1,745/557, census one
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

## I76 closeout evidence

The canonical loop selector slice passed `family_selector` **5/5**, reducing
lib warnings from **1,746** to **1,745** while lib-test remained **557**.
Canonical selector ownership remains unchanged.
