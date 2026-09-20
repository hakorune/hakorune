---
Status: closed__2026-09-21__WarningBaselineRefreshI4__AstNodeTestImportSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I4
Date: 2026-09-21
Parent: mirbuilder-warning-unused-import-local-receipt-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-ASTNODE-TEST-IMPORT-I0
---

# MirBuilder warning baseline refresh I4

## Six-line brief

```text
Decision: refresh both warning surfaces after the fourth import cohort before
  selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,842/561 and select at most one caller-zero import cohort.
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

The fixed commands completed with exit 0 and reproduced the expected surfaces:
lib=1,842 warnings and lib-test=561 warnings. The import inventory contains
75 lib unused-import diagnostics and 8 lib-test unused-import diagnostics; the
remaining warnings are dead-code/visibility families owned by their modules.
The selected bounded cohort is the `ASTNode` import at
`src/mir/builder/ops/logical_shortcircuit.rs:33`: all references outside the
import are behind `#[cfg(test)]`, so the production module can retain
`BinaryOperator` while making `ASTNode` a test-only import. The test surface
already receives the name through the parent module and has no independent
warning. This is a BoxShape-only import-scope cleanup; no semantic route is
selected.
