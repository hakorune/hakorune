---
Status: closed__2026-09-21__WarningScalarOperandRecipeArgumentTestFacade
Task: MIRBUILDER-WARNING-SCALAR-OPERAND-RECIPE-ARGUMENT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i27-2026-09-21.md
Implementation permission: true for one cfg(test) scalar-operand recipe argument re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I28
---

# Warning cleanup: scalar operand recipe argument test facade

## Six-line brief

```text
Decision: gate the scalar operand recipe argument re-export consumed only by
  the script_physical_exit test module; keep scalar_operand_recipe.rs as owner.
Source authority + canonical issuer: scalar_operand_recipe.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, recipe semantics, visibility
  redesign, production route changes, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: split the grouped re-export, gate only the argument type,
  then run the fixed quick-profile gates once each.
Non-claims: no scalar-recipe redesign, suppression, or direct-static cutover.
```

## Preconditions and acceptance

I27 records one lib-only unused-import diagnostic at
`src/mir/builder/normal_script_direct_static_join_handoff.rs:302`.
`ScalarOperandRecipeArgumentV1` is imported through this facade only by the
`script_physical_exit` test module; production code reaches the owner module
directly.

Acceptance requires lib warnings to drop from **1,804 to 1,803**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export declaration may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Execution evidence

The argument-type re-export is now gated with `#[cfg(test)]`; the scalar
operator/node exports and owner module remain unchanged. The fixed sequential
gates exited 0:

* `cargo check --profile quick --lib -j4`: lib warnings **1,803**.
* `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **561**;
  the test executable was produced.

No production caller, recipe construction, or direct-static route changed.
