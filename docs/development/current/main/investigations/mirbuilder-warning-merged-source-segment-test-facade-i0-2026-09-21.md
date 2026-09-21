---
Status: closed__2026-09-21__WarningMergedSourceSegmentTestFacade
Task: MIRBUILDER-WARNING-MERGED-SOURCE-SEGMENT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i41-2026-09-21.md
Implementation permission: true for one cfg(test) MergedSourceSegmentV1 re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I42
---

# Warning cleanup: MergedSourceSegmentV1 test facade

## Six-line brief

```text
Decision: split the strip re-export and gate MergedSourceSegmentV1 under
  cfg(test); keep MergedSourceLineageV1 unconditional.
Source authority + canonical issuer: strip/import_lineage.rs; production merge
  owns the segment type and imports it directly.
Non-authority: cargo-fix, wildcard imports, lineage behavior, merge semantics,
  or warning guesses.
Fail-fast boundary: any production strip-facade consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: split the re-export, run the fixed quick-profile gates once.
Non-claims: no lineage redesign, import behavior change, suppression, or cutover.
```

## Preconditions and acceptance

I41 records one lib-only unused import at
`src/runner/modes/common_util/resolve/strip/mod.rs:7`.
Production merge imports `MergedSourceSegmentV1` from `import_lineage`; observed
strip-facade consumers are test-only.

Acceptance requires lib warnings to drop from **1,791 to 1,790**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export split and cfg annotation may change.

## Closeout evidence

The sole permitted source edit split the `strip` re-export: production keeps
`MergedSourceLineageV1`, while `MergedSourceSegmentV1` is gated by `#[cfg(test)]`.
The fixed gates completed sequentially with exit 0:

* lib: 1,790 warnings — `/tmp/hakorune-warning-i0-merged-source-segment-test-facade-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-merged-source-segment-test-facade-lib-test-20260921.log`

No import-lineage or merge semantics changed.
