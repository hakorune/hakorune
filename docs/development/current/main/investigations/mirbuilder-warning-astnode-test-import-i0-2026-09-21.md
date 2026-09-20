---
Status: closed__2026-09-21__WarningAstNodeTestImport
Task: MIRBUILDER-WARNING-ASTNODE-TEST-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i4-2026-09-21.md
Implementation permission: true for one cfg(test) import-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I5
---

# Warning cleanup: test-only `ASTNode` import

## Six-line brief

```text
Decision: gate the `ASTNode` import in logical_shortcircuit.rs with cfg(test);
  its only uses are the test-only reference helper and test fixture.
Source authority + canonical issuer: Rust cfg/name resolution in the selected
  module and the I4 two-surface warning inventory.
Non-authority: cargo-fix, wildcard imports, warning counts, dead-code ownership,
  visibility changes, or logical short-circuit semantics.
Fail-fast boundary: any non-test ASTNode reference, compile error, new warning,
  or changed failure name rejects the move.
Smallest next slice: split the ast import into production BinaryOperator and
  cfg(test) ASTNode imports, then run both fixed checks and guards.
Non-claims: no lowering change, test behavior change, suppression, broad
  warning cleanup, or production cutover.
```

## Precondition and acceptance

The I4 inventory records one lib-only unused-import warning at
`src/mir/builder/ops/logical_shortcircuit.rs:33`. The production body uses
`BinaryOperator`; `ASTNode` is used only by `#[cfg(test)]` items. Acceptance
requires lib warnings to drop from 1,842 to 1,841, lib-test to remain 561,
both fixed quick-profile commands to exit 0, and fmt/diff/pointer guards to
remain green.

## Execution evidence

The import was split into a production `BinaryOperator` import and a
`#[cfg(test)]` `ASTNode` import. `cargo check --profile quick --lib -j4`
completed with lib warnings 1,841 (down from 1,842), and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test
warnings 561 and produced the test executable. Both commands exited 0; the
focused source search found no non-test `ASTNode` use. The change is
BoxShape-only and does not alter short-circuit lowering or the red inventory.
