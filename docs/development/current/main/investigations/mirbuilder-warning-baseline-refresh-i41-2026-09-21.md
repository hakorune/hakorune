---
Status: closed__2026-09-21__WarningBaselineRefreshI41__MergedSourceSegmentTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I41
Date: 2026-09-21
Parent: mirbuilder-warning-valueid-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-MERGED-SOURCE-SEGMENT-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I42
---

# MirBuilder warning baseline refresh I41

## Six-line brief

```text
Decision: refresh both warning surfaces after the I40 ValueId test facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,791/561 and select at most one caller-zero import cohort.
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
| lib | 1,791 | `/tmp/hakorune-warning-i41-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i41-lib-test-20260921.log` |

The selected caller-zero import cohort is the `MergedSourceSegmentV1` test
facade:

* `src/runner/modes/common_util/resolve/strip/mod.rs:7` re-exports
  `MergedSourceLineageV1` and `MergedSourceSegmentV1` together. Production
  merge code imports the segment type from `import_lineage` directly.
* All observed `strip::MergedSourceSegmentV1` consumers are under test
  modules: parser source-admission tests, normal-root execution tests, and
  host-provider test fixtures. The production module retains the lineage
  re-export.

The bounded execution slice gates only this segment re-export with
`#[cfg(test)]`; lineage ownership and merge behavior remain unchanged.

## Closeout evidence

The delegated import-facade slice completed with the fixed gates, sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,790 | `/tmp/hakorune-warning-i0-merged-source-segment-test-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-merged-source-segment-test-facade-lib-test-20260921.log` |

Both commands exited 0. `MergedSourceSegmentV1` is now re-exported from
`strip` only under `#[cfg(test)]`; production lineage and merge owners remain
unchanged. I41 is closed, and I42 is the next design-stop baseline refresh.
