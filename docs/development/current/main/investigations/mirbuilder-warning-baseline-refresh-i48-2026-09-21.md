---
Status: closed__2026-09-21__WarningBaselineRefreshI48__ResolvedControlFlowFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I48
Date: 2026-09-21
Parent: mirbuilder-warning-builder-facade-test-exports-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-RESOLVED-CONTROL-FLOW-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I49
---

# MirBuilder warning baseline refresh I48

## Six-line brief

```text
Decision: refresh both warning surfaces after the I47 builder-facade cohort
  before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,776/561 and select at most one caller-zero import cohort.
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

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,776 | `/tmp/hakorune-warning-i48-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i48-lib-test-20260921.log` |

The selected caller-zero cohort is the two forwarding names in
`src/mir/resolved_control_flow/mod.rs:23,25`:

* `verify_function_completion_with_new_homes_v1` is used by the test-side
  Main function-plan contract helper; production completion uses the
  argument-observation variant through the child owner.
* `SealedFunctionExitContractV1` is read by the test-side plan contract helper;
  production control modules use the child `function_control` path directly.

The bounded execution slice gates these two parent re-exports and the now-unused
child re-export at `function_control.rs:423` under `#[cfg(test)]`; the completion
issuer, exit contract owner, and production control flow remain unchanged.

## Closeout evidence

The selected resolved-control-flow facade cohort and dependent child re-export
were gated under `#[cfg(test)]`. Both fixed gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,775 | `/tmp/hakorune-warning-i0-resolved-control-flow-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-resolved-control-flow-facade-lib-test-20260921.log` |

I48 closes with no production control-flow behavior change.
