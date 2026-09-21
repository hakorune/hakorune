---
Status: closed__2026-09-22__WarningBaselineRefreshI146__SelectedI147
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I146
Date: 2026-09-22
Parent: mirbuilder-warning-parser-test-only-parse-helper-i145-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one finite next cohort
NextCard: MIRBUILDER-WARNING-PARSER-ADMISSION-ROWS-TEST-ACCESSOR-I147
---

# MirBuilder warning baseline refresh I146

## Six-line brief

```text
Decision: refresh the warning baseline after I145 and select at most one
  production-zero declaration or caller-zero old edge.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
  warning classification policy, and the selected owner's source references.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  inferred route identity, or new semantic receipts.
Fail-fast boundary: unclassified red, a non-test caller, changed owner/route
  inventory, or a missing focused guard returns to design_stop.
Smallest next slice: one declaration-only production-zero item or one
  caller-zero old edge with an explicit successor proof.
Non-claims: no broad warning sweep, parser semantic expansion, VM repair,
  fallback, backend promotion, or LegacyCallV0 retirement.
```

## Selection gate

I145 closed with **1,673 lib warnings**, **543 lib-test warnings**, and
focused parser evidence 44/44. I146 must rerun the quick baseline and classify
the proposed diagnostic as `current-change failure`, `known baseline debt`,
or `informational census`. Retired I139, I141, I143, and I145 symbols are not
candidates again. A new candidate requires a complete owner/caller/delete-set
tuple and a focused guard before implementation.

The LoopBreak old-edge lane remains a priority check, not an automatic
permission: `route_loop_break_recipe` must first prove caller-zero and a
successor for the live compatibility caller. If that census remains live,
I146 may select one safe production-zero warning row instead.

## Baseline refresh and bounded selection

The fresh HEAD `aa12995682` quick check completed successfully with **1,673
lib warnings**. The old-edge census remains the I120/I130 result: the live
compatibility registry and raw legacy caller prevent caller-zero proof, so
`MIR-RETIRE-FIRST-OLD-EDGE-R0` stays `NoSafeSlice`.

The selected production-zero warning is the test-only accessor
`ParserSourceAdmissionWitnessV1::rows` at
`src/parser/normal_callable_program_source/source_admission.rs:96`.
Repository search found its only uses in the three `#[cfg(test)]` assertions
in the same module; production code consumes the witness as an opaque
admission proof and reads no rows. The finite delete set is one declaration:
add `#[cfg(test)]` to `rows`, preserving the existing tests and all admission
behavior. This is the exact owner/caller/delete tuple required by I146.

`MIRBUILDER-WARNING-PARSER-ADMISSION-ROWS-TEST-ACCESSOR-I147` is selected as
the next fast slice.
