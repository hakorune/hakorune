---
Status: closed__2026-09-21__WarningProgramRootClassifierImport
Task: MIRBUILDER-WARNING-PROGRAM-ROOT-CLASSIFIER-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i5-2026-09-21.md
Implementation permission: true for one cfg(test) import-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I6
---

# Warning cleanup: program-root classifier import

## Six-line brief

```text
Decision: keep NormalScriptProgramItemAdmissionV1 in the production import and
  gate classify_normal_script_program_item_v1 behind cfg(test).
Source authority + canonical issuer: Rust cfg/name resolution in
  program_root_work_plan.rs and the I5 two-surface warning inventory.
Non-authority: cargo-fix, wildcard imports, warning counts, dead-code ownership,
  visibility changes, or root scheduling semantics.
Fail-fast boundary: any non-test function reference, compile error, new warning,
  or changed failure name rejects the move.
Smallest next slice: split only the two import items and run both fixed checks
  plus fmt/diff/pointer guards.
Non-claims: no root admission change, test behavior change, suppression, broad
  warning cleanup, or production cutover.
```

## Precondition and acceptance

The I5 inventory records a lib-only unused-import warning at line 17. The enum
type is used by production classification, but the classifier function is
called only from the cfg(test) preparation helper. Acceptance requires lib
warnings to drop from 1,841 to 1,840, lib-test to remain 561, both fixed
quick-profile commands to exit 0, and fmt/diff/pointer guards to remain green.

## Execution evidence

The import was split so the production module retains only
`NormalScriptProgramItemAdmissionV1`; the classifier function is now imported
under `#[cfg(test)]`. `cargo check --profile quick --lib -j4` completed with
lib warnings 1,840 (down from 1,841), and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test
warnings 561 and produced the test executable. Both commands exited 0; root
admission and the red inventory are unchanged.
