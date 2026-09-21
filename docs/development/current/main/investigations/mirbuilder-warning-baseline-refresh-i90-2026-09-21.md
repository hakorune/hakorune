---
Status: design_stop__2026-09-21__WarningBaselineRefreshI90__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I90
Date: 2026-09-21
Parent: mirbuilder-warning-source-handoff-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: MIRBUILDER-WARNING-ARRAY-STATE-IDENTITY-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I90

## Six-line brief

```text
Decision: refresh warning surfaces after the source-resolver handoff facade
  and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,715/553, census one
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

## I90 handoff evidence

The source-resolver handoff facade closed at `2c216e96d0`: lib **1,715**,
lib-test **553**, source-handoff **3/3**, and preserved production consumer
**8/8**. This card remeasures that pair before choosing the next warning row;
it does not reopen source identity or resolver ownership.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,715** warnings and lib-test
**553** warnings. Both required commands exited successfully and the test
binary was produced; no new red or private-interface diagnostic appeared.

The warning inventory was checked against the current requirements, pointer,
and this card before selection:

* `src/box_callable/provider_admission/admitted_registry.rs:59`
  `branch_count` remains **production-owned** because
  `src/box_callable/provider_admission/aot_admission.rs:153` calls it. It is
  retained and is not a test facade candidate.
* `src/boxes/array/mod.rs:47` (`ArrayStateCell::identity`) and `:104`
  (`ArrayBox::state_identity`) are the selected two-warning cohort. Exact
  callers are the `#[cfg(test)]` array runtime-contract assertions and
  `src/boxes/array/tests/state_identity.rs`; no production caller exists.
  The planned change is limited to a test-only identity observation surface
  (type/field/initializer/method gating), preserving the runtime storage and
  share/clone behavior tested by those callers.
* `src/mir/builder/control_flow/plan/loop_phi_materializer.rs:467` (`index`)
  is used only by the test failure injection branch but is a mixed cfg local,
  not a standalone facade with a removable production edge. It is left for a
  later bounded row.

The next bounded cohort is therefore
`MIRBUILDER-WARNING-ARRAY-STATE-IDENTITY-TEST-FACADE-I0`. Its implementation
card records the caller census and the focused guard; no code change is
claimed by this refresh.
