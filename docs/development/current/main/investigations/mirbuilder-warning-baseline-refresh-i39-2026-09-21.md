---
Status: closed__2026-09-21__WarningBaselineRefreshI39__GenericLoopExtractObserveTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I39
Date: 2026-09-21
Parent: mirbuilder-warning-generic-loop-carrier-facts-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-GENERIC-LOOP-EXTRACT-OBSERVE-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I40
---

# MirBuilder warning baseline refresh I39

## Six-line brief

```text
Decision: refresh both warning surfaces after the I38 generic-loop carrier
  facts test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,792/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Selection evidence

The fixed commands completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,793 | `/tmp/hakorune-warning-i39-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i39-lib-test-20260921.log` |

The selected caller-zero cohort is the generic-loop extract observation
facade:

* `src/mir/builder/control_flow/plan/generic_loop/facts/extract/mod.rs:13` —
  the parent `extract::observe_generic_loop_carrier_observation` re-export has
  no caller. Production v1 imports `collection::...` directly, and test code
  uses the higher `plan::facts` facade already covered by I38.

The bounded execution slice gates only this parent re-export with
`#[cfg(test)]`. Generic-loop collection, v1 extraction, and production route
consumers remain unchanged.

## Closeout evidence

The delegated execution slice completed with the fixed gates, sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,792 | `/tmp/hakorune-warning-i0-generic-loop-extract-observe-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-generic-loop-extract-observe-lib-test-20260921.log` |

Both commands exited 0. The one parent re-export was gated with `#[cfg(test)]`;
production generic-loop collection and extraction owners were unchanged. I39 is
closed, and I40 is the next design-stop baseline refresh.
