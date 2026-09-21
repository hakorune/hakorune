---
Status: closed__2026-09-21__WarningBaselineRefreshI61__SelectedVmReferenceTestImport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I61
Date: 2026-09-21
Parent: mirbuilder-warning-nested-predicate-test-import-i0-2026-09-21.md
Implementation permission: true for the selected test-only VM-reference import only
NextCard: MIRBUILDER-WARNING-VM-REFERENCE-TEST-IMPORT-I0
---

# MirBuilder warning baseline refresh I61

## Six-line brief

```text
Decision: refresh both warning surfaces after the nested-predicate test-import
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,760/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,760** and lib-test **561** after the
selected nested-predicate test-import cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the nested-predicate
cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,760 | `/tmp/hakorune-warning-i61-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i61-lib-test-20260921.log` |

The first warning group is the unused `CanonicalPublishedFamilyKindV1` import at
`src/mir/compiler/source_entry_vm_reference.rs:8`. A source census shows it is
referenced only by the `#[cfg(test)]` route classifier; the published owner type
remains a production import for the VM-reference result owner. The bounded slice
is therefore a test-only import gate, with expected lib **1,760 → 1,759** and
lib-test unchanged at **561**. No VM route, publication owner, or test body is
selected.
