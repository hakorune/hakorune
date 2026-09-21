---
Status: design_stop__2026-09-21__WarningBaselineRefreshI78__NextDeletionSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I78
Date: 2026-09-21
Parent: mirbuilder-warning-loop-cond-observation-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded deletion cohort only
NextCard: select one caller-zero warning facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I78

## Six-line brief

```text
Decision: refresh both warning surfaces after the LoopCond observation
  test-only scope and select one finite caller-zero source deletion if proven.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or test-only
  red interpretation without parent reproduction.
Fail-fast boundary: a new warning, a non-test caller, command drift, or an
  unclassified focused red stops deletion selection.
Smallest next slice: compare lib/lib-test against 1,744/557, census one
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

## I77 closeout evidence

The LoopCond observation slice passed focused LoopCond **9/9** and
`family_admission` **6/6**, reducing lib warnings from **1,745** to **1,744**
while lib-test remained **557**. Canonical observation ownership remains
unchanged.

## Refresh result and bounded deletion selection

The sequential refresh completed with lib **1,744** warnings and lib-test **557**,
matching the LoopCond observation closeout. The next finite caller-zero cohort is
the parent re-export of `issue_loop_true_family_observation_v1`,
`LoopTrueObservationContextV1`, `LoopTrueObservationDeclineV1`,
`LoopTrueObservationRejectV1`, and `LoopTrueObservationUnresolvedV1` in
`src/mir/loop_route_policy/mod.rs`. Their canonical owner is
`loop_true_break_continue_observation.rs`; parent-facade consumers are test
modules only. Retain the test facade under `cfg(test)` and remove its
production edge in one focused slice.

Selected successor:
`MIRBUILDER-WARNING-LOOP-TRUE-OBSERVATION-TEST-SCOPE-I0`.

## Closeout selection result

The I78 refresh confirmed lib **1,744** and lib-test **557**, then selected the
caller-zero LoopTrue observation parent facade. Its fast slice passed the
LoopTrue observation suite **9/9** and `family_admission` **6/6**, reducing lib
warnings to **1,743** while preserving the lib-test surface.
