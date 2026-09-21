---
Status: closed__2026-09-21__WarningBaselineRefreshI36__SharedAbsentRouteTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I36
Date: 2026-09-21
Parent: mirbuilder-warning-raw-legacy-expression-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) shared-absent route re-export; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I37
---

# MirBuilder warning baseline refresh I36

## Six-line brief

```text
Decision: refresh both warning surfaces after the I35 raw legacy expression
  test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,796/561 and select at most one caller-zero import cohort.
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
| lib | 1,796 | `/tmp/hakorune-warning-i36-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i36-lib-test-20260921.log` |

The selected caller-zero cohort is the route registry's parent re-export:

* `src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs:53` —
  `SharedAbsentContractDeclineRouteV1` has its production users importing
  `types::SharedAbsentContractDeclineRouteV1` directly. No production caller
  uses this parent registry re-export.

The bounded execution slice gates only the parent re-export with
`#[cfg(test)]`. The route type, handlers, execution witness, and route policy
remain unchanged.

## Closeout

The parent route re-export is now test-only while the direct owner and route
consumers remain unchanged. The fixed gates completed sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,795 | `/tmp/hakorune-warning-i0-shared-absent-route-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-shared-absent-route-lib-test-20260921.log` |

Both commands exited 0. Route handlers, execution witness, and route policy
are unchanged; I37 is the next design-stop baseline at 1,795/561.
