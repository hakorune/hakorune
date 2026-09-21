---
Status: closed__2026-09-21__WarningBaselineRefreshI38__GenericLoopCarrierFactsTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I38
Date: 2026-09-21
Parent: mirbuilder-warning-live-loop-facts-import-test-facade-i0-2026-09-21.md
Implementation permission: true for two cfg(test) generic-loop carrier facts re-exports; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I39
---

# MirBuilder warning baseline refresh I38

## Six-line brief

```text
Decision: refresh both warning surfaces after the I37 live-loop-facts import
  test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,794/561 and select at most one caller-zero import cohort.
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
| lib | 1,794 | `/tmp/hakorune-warning-i38-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i38-lib-test-20260921.log` |

The selected caller-zero cohort is the generic-loop carrier facts facade:

* `src/mir/builder/control_flow/plan/facts/mod.rs:46-47` —
  `observe_generic_loop_carrier_observation` and
  `GenericLoopCarrierObservationV1` are consumed through the parent `facts`
  path only by test fixtures. Production code uses the generic-loop owner
  modules directly.

The bounded execution slice gates only these two parent re-exports with
`#[cfg(test)]`. Generic-loop extraction, facts types, and production route
consumers remain unchanged.

## Closeout

The two parent facts re-exports are now test-only. The fixed gates completed
sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,793 | `/tmp/hakorune-warning-i0-generic-loop-carrier-facts-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-generic-loop-carrier-facts-lib-test-20260921.log` |

Both commands exited 0. Generic-loop extraction, facts types, and production
route consumers are unchanged; I39 is the next design-stop baseline at
1,793/561.
