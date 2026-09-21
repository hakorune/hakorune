---
Status: closed__2026-09-21__WarningFunctionOwnerIdTestFacade
Task: MIRBUILDER-WARNING-FUNCTION-OWNER-ID-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i29-2026-09-21.md
Implementation permission: true for one cfg(test) FunctionOwnerIdV1 import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I30
---

# Warning cleanup: scalar recipe FunctionOwnerId test facade

## Six-line brief

```text
Decision: resolve FunctionOwnerIdV1 only for the scalar recipe test helper;
  keep production fields and accessors on their existing qualified path.
Source authority + canonical issuer: scalar_operand_recipe.rs and the existing
  resolved-semantics owner; cfg(test) import scope is the boundary.
Non-authority: cargo-fix, wildcard imports, type-path redesign, recipe
  semantics, visibility changes, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: move FunctionOwnerIdV1 to a cfg(test) import and run the
  fixed quick-profile gates once each.
Non-claims: no scalar-recipe redesign, suppression, or route change.
```

## Preconditions and acceptance

I29 records one lib-only unused-import diagnostic at
`src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs:15`.
The unqualified type is used only by the `#[cfg(test)]` constructor helper;
production uses qualified paths.

Acceptance requires lib warnings to drop from **1,802 to 1,801**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the import grouping may change. If the consumer classification or warning
count differs, stop and return to design stop.

## Execution evidence

`FunctionOwnerIdV1` is now in a test-only import. The fixed sequential gates
exited 0:

* `cargo check --profile quick --lib -j4`: lib warnings **1,801**.
* `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **561**;
  the test executable was produced.

No production scalar recipe, physical input, or lowering route changed.
