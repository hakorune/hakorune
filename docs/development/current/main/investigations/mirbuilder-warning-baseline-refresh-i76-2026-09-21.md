---
Status: design_stop__2026-09-21__WarningBaselineRefreshI76__NextDeletionSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I76
Date: 2026-09-21
Parent: mirbuilder-warning-loop-family-admission-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded deletion cohort only
NextCard: select one caller-zero warning facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I76

## Six-line brief

```text
Decision: refresh both warning surfaces after the LoopFamilyAdmission
  test-only scope and select one finite caller-zero source deletion if proven.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or test-only
  red interpretation without parent reproduction.
Fail-fast boundary: a new warning, a non-test caller, command drift, or an
  unclassified focused red stops deletion selection.
Smallest next slice: compare lib/lib-test against 1,746/557, census one
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

## I75 closeout evidence

The LoopFamilyAdmission slice passed `family_admission` **6/6** and
`family_selector` **5/5**, reducing lib warnings from **1,747** to **1,746**
while lib-test remained **557**. Canonical admission and selector ownership
remain unchanged.

## Refresh result and bounded deletion selection

The sequential refresh completed with lib **1,746** warnings and lib-test **557**,
matching the LoopFamilyAdmission closeout. The next finite caller-zero cohort
is the parent re-export of `select_canonical_loop_family_v1`,
`CanonicalLoopFamilySelectionOutcomeV1`, and
`CanonicalLoopFamilySelectionReasonV1` in `src/mir/loop_route_policy/mod.rs`.
The definitions are owned by `family_selector.rs`; the repository census shows
only `family_selector_tests.rs` consumes these parent-facade names. Retain the
test facade under `cfg(test)` and remove its production edge in one focused
slice.

Selected successor:
`MIRBUILDER-WARNING-LOOP-FAMILY-SELECTOR-TEST-SCOPE-I0`.

## Closeout selection result

The I76 refresh confirmed lib **1,746** and lib-test **557**, then selected the
caller-zero canonical loop selector parent facade. Its fast slice passed
`family_selector` **5/5**, reducing lib warnings to **1,745** while preserving
the lib-test surface.
