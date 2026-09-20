---
Status: closed__2026-09-21__WarningBaselineRefreshI5__ProgramRootClassifierSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I5
Date: 2026-09-21
Parent: mirbuilder-warning-astnode-test-import-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-PROGRAM-ROOT-CLASSIFIER-IMPORT-I0
---

# MirBuilder warning baseline refresh I5

## Six-line brief

```text
Decision: refresh both warning surfaces after the ASTNode import cohort before
  selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,841/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Inventory and selection

The fixed commands completed with exit 0 and reproduced lib=1,841 and
lib-test=561. The import inventory remains the only selected warning family;
dead-code and private-interface diagnostics retain their module owners. The
selected cohort is the function import
`classify_normal_script_program_item_v1` at
`src/mir/builder/program_root_work_plan.rs:17`. Its only call is inside the
`#[cfg(test)]` preparation helper, while the sibling
`NormalScriptProgramItemAdmissionV1` enum is consumed by production
`classify_statement`. Split the import scope without changing either owner.
