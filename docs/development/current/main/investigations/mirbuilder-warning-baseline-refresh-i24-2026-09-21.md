---
Status: closed__2026-09-21__WarningBaselineRefreshI24__SelectedLoopPhysicalInputTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I24
Date: 2026-09-21
Parent: mirbuilder-warning-bounded-body-snapshot-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) loop-physical-input import facade only
NextCard: MIRBUILDER-WARNING-LOOP-PHYSICAL-INPUT-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I24

## Six-line brief

```text
Decision: refresh both warning surfaces after the bounded-body snapshot
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,807/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I24 inventory and decision

The fixed commands completed successfully with the current warning baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,807 | `/tmp/hakorune-warning-i24-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i24-lib-test-20260921.log` |

The selected caller-zero cohort is the loop-physical-input test facade:

* `src/mir/builder/control_flow/plan/loop_physical_input.rs:10` —
  `BindingId` is consumed only by the local `#[cfg(test)]` projection tests;
  the production physical-input types use `BasicBlockId` and binding
  references without this direct import.

The bounded next slice is to gate only this import with `#[cfg(test)]`;
physical-input validation and role planning remain unchanged.
