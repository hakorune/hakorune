---
Status: closed__2026-09-21__WarningBaselineRefreshI47__BuilderFacadeTestExports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I47
Date: 2026-09-21
Parent: mirbuilder-warning-resolved-lowering-facade-test-exports-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-BUILDER-FACADE-TEST-EXPORTS-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I48
---

# MirBuilder warning baseline refresh I47

## Six-line brief

```text
Decision: refresh both warning surfaces after the I46 resolved-lowering facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,778/561 and select at most one caller-zero import cohort.
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
| lib | 1,778 | `/tmp/hakorune-warning-i47-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i47-lib-test-20260921.log` |

The selected caller-zero cohort is the two test-only facade re-exports in
`src/mir/builder.rs:663,666`:

* `issue_selected_dynamic_v2_emission_plan` is consumed through the
  `crate::mir::builder` facade only by
  `normal_callable_semantic_package/tests.rs`; the production emitter uses
  `resolved_lowering` directly.
* `with_common_v2_canonical_session` is consumed through the builder facade only
  by `compiler/common_v2_session_admission_tests.rs` and
  `resolved_lowering/common_v2_initial_index_seed_tests.rs`; production session
  code uses the branded child entry directly.

The bounded execution slice gates these two parent re-exports and the now-unused
child re-export at `resolved_lowering/mod.rs:69` with `#[cfg(test)]`; the
resolved-lowering issuers, physical emitter, canonical session owner, and all
production lowering paths remain unchanged.

## Closeout evidence

The selected builder facade cohort and its dependent child re-export were
gated under `#[cfg(test)]`. Both fixed gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,776 | `/tmp/hakorune-warning-i0-builder-facade-test-exports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-builder-facade-test-exports-lib-test-20260921.log` |

I47 closes with no production lowering or session behavior change.
