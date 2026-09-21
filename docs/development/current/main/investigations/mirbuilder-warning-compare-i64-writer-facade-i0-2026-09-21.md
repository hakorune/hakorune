---
Status: closed__2026-09-21__WarningCompareI64WriterFacade__Execution
Task: MIRBUILDER-WARNING-COMPARE-I64-WRITER-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i51-2026-09-21.md
Implementation permission: true for the single test-only facade export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I52
---

# Warning cleanup: compare-I64 writer test facade

## Six-line brief

```text
Decision: gate one unused forwarding export that is consumed only by the
  strict compare writer tests.
Source authority + canonical issuer: canonical_compare_writer.rs owns the
  production writer; compare_i64_writer.rs is a test-ledger facade.
Non-authority: cargo-fix, wildcard imports, visibility changes, writer logic,
  trait implementation, guard edits, or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to compare_i64_writer.rs:10 and run the
  fixed check and lib-test no-run gates sequentially.
Non-claims: no compare lowering behavior, physical route, or production cutover.
```

## Preconditions and acceptance

I51 selected the single `unused import` at
`src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_writer.rs:10`.
The facade export is consumed only by
`loop_recipe_physicalizer/compare_i64_writer_tests.rs`; production code imports
`CanonicalLoopCompareI64WriterV1` from the canonical `resolved_lowering`
surface. The implementation and trait imports remain shared by the test-ledger
path and are outside this slice.

Add `#[cfg(test)]` to that one `pub(in crate::mir::builder::resolved_lowering)`
`use` line. Do not edit the canonical writer, trait implementation, production
emitter, or guard scripts.

Expected result: lib warnings **1,773 → 1,772** and lib-test warnings remain
**561**. Both fixed gates must exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```


## Closeout evidence

The single facade export received `#[cfg(test)]`; the canonical writer, trait
implementation, production emitter, and guard scripts were unchanged. The
fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,772 | `/tmp/hakorune-warning-i0-compare-i64-writer-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-compare-i64-writer-facade-lib-test-20260921.log` |

The expected one-warning lib reduction was observed; the lib-test baseline was
unchanged.
