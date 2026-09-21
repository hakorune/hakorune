---
Status: closed__2026-09-21__WarningModuleDeclarationShellPrepareErrorTestFacade
Task: MIRBUILDER-WARNING-MODULE-DECLARATION-SHELL-PREPARE-ERROR-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i32-2026-09-21.md
Implementation permission: true for one cfg(test) declaration-shell error re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I33
---

# Warning cleanup: module declaration-shell prepare-error test facade

## Six-line brief

```text
Decision: gate the declaration-shell prepare-error re-export consumed only by
  the P0 test fixture; keep declaration_fact_commit.rs as its owner.
Source authority + canonical issuer: module_lowering_shell/declaration_fact_commit.rs;
  cfg(test) export scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, shell semantics, declaration facts,
  production lowering, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the parent re-export and run the fixed
  quick-profile gates once each.
Non-claims: no shell redesign, suppression, or production cutover.
```

## Preconditions and acceptance

I32 records one lib-only unused-import diagnostic at
`src/mir/builder/module_lowering_shell.rs:17`. The re-export is consumed by
the `#[cfg(test)]` `module_declaration_fact_shell_commit_p0` fixture; production
code does not use it.

Acceptance requires lib warnings to drop from **1,799 to 1,798**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export declaration may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Closeout evidence

`#[cfg(test)]` was added to the parent re-export at
`src/mir/builder/module_lowering_shell.rs:17`. The fixed gates completed
sequentially and exited 0:

* `cargo check --profile quick --lib -j4`: 1,798 lib warnings,
  `/tmp/hakorune-warning-i0-module-declaration-shell-prepare-error-lib-20260921.log`
* `cargo test --profile quick --lib --no-run -j4`: 561 lib-test warnings,
  `/tmp/hakorune-warning-i0-module-declaration-shell-prepare-error-lib-test-20260921.log`

No production consumer, shell owner, or declaration-fact semantics changed.
