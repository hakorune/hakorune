---
Status: closed__2026-09-21__WarningNormalSourcePlanTestImport__Execution
Task: MIRBUILDER-WARNING-NORMAL-SOURCE-PLAN-TEST-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i58-2026-09-21.md
Implementation permission: true for the selected `NormalUnsupportedTopLevelKindV1` re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I59
---

# Warning cleanup: normal source-plan test re-export

## Six-line brief

```text
Decision: gate the unused NormalUnsupportedTopLevelKindV1 re-export for
  source-plan test fixtures.
Source authority + canonical issuer: rejection owns the type; production
  source-plan modules import it from their owning submodule.
Non-authority: cargo-fix, wildcard imports, visibility changes, policy or
  rejection logic, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the single re-export item, then run the
  fixed gates sequentially.
Non-claims: no source-plan policy, rejection mapping, admission, or test-body
  change.
```

## Preconditions and acceptance

I58 selected the unused re-export at
`src/mir/compiler/normal_source_plan/mod.rs:135`.
`NormalUnsupportedTopLevelKindV1` is referenced through this parent only by
`#[cfg(test)]` fixtures. Production source-plan modules import the type directly
from `rejection` or use the mapped module-local type.

Split the re-export so only `NormalUnsupportedTopLevelKindV1` receives
`#[cfg(test)]`. Do not edit source-plan policy, rejection mapping, or test bodies.

Expected result: one warning diagnostic is removed, so lib warnings
**1,764 → 1,763** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`NormalUnsupportedTopLevelKindV1` now receives `#[cfg(test)]` on the parent
re-export; source-plan policy, rejection mapping, and test bodies were unchanged.
The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,763 | `/tmp/hakorune-warning-i0-normal-source-plan-test-import-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-normal-source-plan-test-import-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
