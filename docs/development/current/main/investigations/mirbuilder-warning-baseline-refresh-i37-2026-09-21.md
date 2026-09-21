---
Status: closed__2026-09-21__WarningBaselineRefreshI37__LiveLoopFactsImportTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I37
Date: 2026-09-21
Parent: mirbuilder-warning-shared-absent-route-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) live-loop-facts import facade; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I38
---

# MirBuilder warning baseline refresh I37

## Six-line brief

```text
Decision: refresh both warning surfaces after the I36 shared-absent route
  test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,795/561 and select at most one caller-zero import cohort.
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
| lib | 1,795 | `/tmp/hakorune-warning-i37-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i37-lib-test-20260921.log` |

The selected caller-zero cohort is the live-loop-facts import facade:

* `src/mir/builder/control_flow/plan/facts/loop_builder.rs:48` —
  `LiveLoopFactsV1` and `bind_live_loop_facts_v1` are used only by the
  `#[cfg(test)]` `try_build_live_loop_facts` helper. The production facts
  builder uses the existing `LoopFacts` path and does not require these names.

The bounded execution slice gates only this import with `#[cfg(test)]`.
Live-loop fact construction, the production facts path, and route consumers
remain unchanged.

## Closeout

The two-name import group is now test-only. Rustc reports the grouped import
as one warning, so the observed lib count is **1,794** rather than the
preselection estimate of 1,793:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,794 | `/tmp/hakorune-warning-i0-live-loop-facts-import-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-live-loop-facts-import-lib-test-20260921.log` |

Both commands exited 0. The production facts path and route consumers remain
unchanged; I38 is the next design-stop baseline at 1,794/561.
