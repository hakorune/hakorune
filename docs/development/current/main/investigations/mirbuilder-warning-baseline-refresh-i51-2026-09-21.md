---
Status: closed__2026-09-21__WarningBaselineRefreshI51__CompareI64WriterFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I51
Date: 2026-09-21
Parent: mirbuilder-warning-a-prime-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-COMPARE-I64-WRITER-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I52
---

# MirBuilder warning baseline refresh I51

## Six-line brief

```text
Decision: refresh both warning surfaces after the I50 A-prime facade cohort
  before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,773/561 and select at most one caller-zero import cohort.
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
| lib | 1,773 | `/tmp/hakorune-warning-i51-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i51-lib-test-20260921.log` |

The selected caller-zero cohort is the single test-only forwarding export in
`src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_writer.rs:10`.
`CanonicalLoopCompareI64WriterV1` is consumed through this facade only by
`compare_i64_writer_tests.rs`; production emitters import the canonical writer
from the `resolved_lowering` surface directly. The implementation and trait
imports remain shared by the existing test-ledger path, so only this facade
export is gated.

The bounded execution slice adds `#[cfg(test)]` to that one re-export. No
canonical writer, trait implementation, emitter, guard, or production route
changes. The earlier A-prime cohort remains closed in its own card.

## Closeout evidence

The selected forwarding export was gated under `#[cfg(test)]`. Both fixed
gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,772 | `/tmp/hakorune-warning-i0-compare-i64-writer-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-compare-i64-writer-facade-lib-test-20260921.log` |

I51 closes with no canonical writer, trait implementation, emitter, guard, or
production route change.
