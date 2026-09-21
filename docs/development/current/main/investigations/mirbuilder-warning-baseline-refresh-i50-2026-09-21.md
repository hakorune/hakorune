---
Status: closed__2026-09-21__WarningBaselineRefreshI50__APrimeFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I50
Date: 2026-09-21
Parent: mirbuilder-warning-resolved-control-flow-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-A-PRIME-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I51
---

# MirBuilder warning baseline refresh I50

## Six-line brief

```text
Decision: refresh both warning surfaces after the I49 resolved-control-flow
  facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,775/561 and select at most one caller-zero import cohort.
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
| lib | 1,775 | `/tmp/hakorune-warning-i50-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i50-lib-test-20260921.log` |

The selected caller-zero cohort is the test-only forwarding surface in
`src/mir/compiler/a_prime_i64_physical_capability/mod.rs:10,13`:

* `issue_selected_a_prime_i64_physical_demand` is used through this facade by
  `normal_callable_semantic_package/tests.rs`; the production physical emitter
  uses the `_from_parts` issuer directly.
* `APrimeI64PhysicalDemandRejectV1` is asserted by the same test-side package
  fixtures; the issuer and model modules use their child paths directly.

The bounded execution slice gates only these two facade exports under
`#[cfg(test)]`; the A-prime issuer, model, physical demand, and production
emitter remain unchanged.

## Closeout evidence

The selected A-prime facade cohort was gated under `#[cfg(test)]`. Both fixed
gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,773 | `/tmp/hakorune-warning-i0-a-prime-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-a-prime-facade-lib-test-20260921.log` |

I50 closes with no production A-prime behavior change.
