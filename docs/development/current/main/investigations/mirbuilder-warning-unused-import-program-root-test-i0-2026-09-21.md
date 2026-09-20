---
Status: closed__2026-09-21__WarningUnusedImportProgramRootTest
Task: MIRBUILDER-WARNING-UNUSED-IMPORT-PROGRAM-ROOT-TEST-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i2-2026-09-21.md
Implementation permission: true for one cfg(test) import deletion only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I3
---

# Warning cleanup: unused program-root test import

## Six-line brief

```text
Decision: remove the unused cfg(test) import of
  NormalScriptRuntimeStatementAdmissionV1 from the program-root work-plan
  module.
Source authority + canonical issuer: Rust test compilation and the parent
  two-surface warning inventory; no production route owns this row.
Non-authority: cargo-fix, production imports, warning counts alone, dead-code
  ownership, or semantic script-runtime interpretation.
Fail-fast boundary: any test reference, compile error, new warning, or changed
  failure name rejects the deletion.
Smallest next slice: delete only the cfg(test) import and rerun both fixed
  quick-profile checks plus fmt/pointer guards.
Non-claims: no production behavior, script admission change, suppression,
  broad test cleanup, or route retirement.
```

## Precondition and acceptance

The I2 inventory reports this import only on the lib-test surface. A source
search finds no use in the parent module or its test siblings. Acceptance
requires lib warnings to remain 1,843, lib-test warnings to drop by one from
563 to 562, both fixed commands to exit 0, and fmt/diff/pointer guards to stay
green.

## Execution evidence

The cfg(test)-only import was removed. `cargo check --profile quick --lib -j4`
completed with lib warnings 1,843, and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test warnings
562 (down from 563). Both exit 0; fmt, diff, and pointer guards are green. No
production code path or failure inventory changed.
