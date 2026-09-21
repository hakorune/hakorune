---
Status: closed__2026-09-21__WarningDynamicPhysicalInputTestImport__Execution
Task: MIRBUILDER-WARNING-DYNAMIC-PHYSICAL-INPUT-TEST-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i55-2026-09-21.md
Implementation permission: true for the selected `LoopJoinNextItemV1` import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I56
---

# Warning cleanup: dynamic physical-input test import

## Six-line brief

```text
Decision: gate the unused LoopJoinNextItemV1 import for the physical-input
  module's test-only continuation fixture.
Source authority + canonical issuer: loop_recipe_contract owns the type;
  physical_input.rs production code does not consume it.
Non-authority: cargo-fix, wildcard imports, visibility changes, loop semantics,
  Recipe/physical layout changes, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the single import item, then run the
  fixed gates sequentially.
Non-claims: no loop admission, source handoff, physical consumer, or test-body
  change.
```

## Preconditions and acceptance

I55 selected the unused import at
`src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs:18`.
`LoopJoinNextItemV1` is referenced only by the module's `#[cfg(test)] mod tests`
at line 459, where it builds a fallthrough continuation. The production physical
input view and validators do not reference it.

Split the import so only `LoopJoinNextItemV1` receives `#[cfg(test)]`. Do not
edit the loop contract, physical-input logic, or test body.

Expected result: one warning diagnostic is removed, so lib warnings
**1,768 → 1,767** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`LoopJoinNextItemV1` now receives `#[cfg(test)]`; the loop contract, physical
input validators, source handoff, Recipe, and test body were unchanged. The
fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,767 | `/tmp/hakorune-warning-i0-dynamic-physical-input-test-import-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-dynamic-physical-input-test-import-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
