---
Status: closed__2026-09-21__WarningUnusedImportArrayTextCallee
Task: MIRBUILDER-WARNING-UNUSED-IMPORT-ARRAY-TEXT-CALLEE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i0-2026-09-21.md
Implementation permission: true for one private import deletion only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I1
---

# Warning cleanup: unused `Callee` import

## Six-line brief

```text
Decision: move the test-only `Callee` dependency out of the production
  array/text edit analyzer and into its explicit test module.
Source authority + canonical issuer: Rust name resolution in
  src/mir/array_text_edit_plan.rs and the parent fixed warning inventories.
Non-authority: cargo-fix, wildcard test imports, warning counts, dead-code
  ownership, visibility edits, or semantic route inference.
Fail-fast boundary: any source/test reference, compile error, new warning, or
  changed failure name rejects the deletion.
Smallest next slice: remove only the production import, add the explicit test
  import, and rerun the two fixed quick-profile checks plus fmt/pointer guards.
Non-claims: no broad import cleanup, dead-code cleanup, suppression, semantic
  change, red-baseline update, or production cutover.
```

## Precondition and acceptance

The parent lib inventory contains the production `Callee` warning while the
lib-test inventory does not, because the test module currently receives that
name through the parent's wildcard import. The test file uses `Callee` once.
Acceptance requires both fixed commands to finish successfully, lib warnings
to drop by one with no lib-test increase, and no new failure name, plus
`cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
guard.

## Execution evidence

The production import was removed and the test-only dependency was made
explicit in `array_text_edit_plan/tests.rs`. Both fixed commands completed with
exit 0: lib warnings changed from 1,845 to 1,844, and lib-test remained 563.
`cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
guard are green. No test failure inventory or semantic route changed.

If either surface changes for another reason, restore the production import
and classify this row as `NoSafeSlice`; do not replace it with a qualified path
or an allow.
