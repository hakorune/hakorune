---
Status: closed__2026-09-22__WarningBaselineRefreshI144__SelectedI145
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I144
Date: 2026-09-22
Parent: mirbuilder-warning-initial-source-missing-slot-retire-i143-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one finite next cohort
NextCard: MIRBUILDER-WARNING-PARSER-TEST-ONLY-PARSE-HELPER-I145
---

# MirBuilder warning baseline refresh I144

## Six-line brief

```text
Decision: refresh the warning baseline after I143 and select at most one
  finite production-zero cleanup candidate from the measured diagnostics.
Source authority + canonical issuer: quick-profile Cargo diagnostics,
  warning classification policy, and each selected owner's source references.
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

I143 measured lib **1,675 warnings** and lib-test **543 warnings**. I144 must
rerun the quick baseline and classify proposed warnings as
`current-change failure`, `known baseline debt`, or `informational census`.
The retired I139, I141, and I143 symbols are not candidates again. A new
candidate needs a complete owner/caller/delete-set tuple before selection.

Boundary: quick-profile `src/lib.rs` diagnostics -> one selected owner and
its focused guard; includes Rust production and test references for the
candidate, excludes semantic expansion, feature-only consumers, historical
snapshots, and unrelated grouped diagnostics.

Until that census is complete, no new warning cohort is selected and the
work mode remains `design_stop`.

## Baseline refresh and bounded selection

The fresh HEAD `c532a3df1c` quick check completed successfully with the same
**1,675 lib warnings** as I143. The LoopBreak old-edge recheck is unchanged:
the compatibility registry and raw legacy caller still keep
`MIR-RETIRE-FIRST-OLD-EDGE-R0` at `NoSafeSlice`.

The next finite production-zero cohort is the parser's test-only helper pair:

* `NyashParser::parse_normal_callable_program_with_build_config` in
  `src/parser/normal_callable_program_source/mod.rs`;
* its sole production-body callee
  `string_postpass_entry::parse_normal_callable_program`.

Repository census found no non-test caller of either symbol. All observed
callers of the `NyashParser` method are in `#[cfg(test)]` modules or test
files; the normal production path uses
`parse_with_callable_parameter_source` directly. The helper pair therefore
has one owner, no production caller, and a finite delete set consisting only
of adding `#[cfg(test)]` to both declarations. This preserves every existing
test call while removing the library-only dead-code warnings; no parser
behavior, source authority, or route changes are involved.

`MIRBUILDER-WARNING-PARSER-TEST-ONLY-PARSE-HELPER-I145` is selected as the
next fast slice. Its closeout must record the focused parser tests, quick
library and test-binary warning counts, formatter, pointer, and diff guards.
