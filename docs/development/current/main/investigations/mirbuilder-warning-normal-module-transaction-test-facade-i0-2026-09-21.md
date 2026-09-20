---
Status: closed__2026-09-21__WarningNormalModuleTransactionTestFacade
Task: MIRBUILDER-WARNING-NORMAL-MODULE-TRANSACTION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i8-2026-09-21.md
Implementation permission: true for one cfg(test) normal-module-transaction facade-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I9
---

# Warning cleanup: normal-module-transaction test facade

## Six-line brief

```text
Decision: gate five unused normal-module-transaction re-exports with cfg(test);
  the existing transaction tests keep using the same types.
Source authority + canonical issuer: Rust cfg/name resolution in
  normal_module_transaction/mod.rs, its defining submodules and #[cfg(test)]
  consumers, and the I8 two-surface warning inventory.
Non-authority: cargo-fix, wildcard imports, transaction semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) only to the five selected re-export names,
  then run both fixed checks and fmt/diff/pointer guards.
Non-claims: no transaction lowering change, test behavior change, suppression,
  broad warning cleanup, or production route change.
```

## Precondition and acceptance

The I8 inventory records five lib-only unused-import diagnostics in
`src/mir/builder/normal_module_transaction/mod.rs`. Repository-wide search
shows the five re-export names are consumed only by the module's
`#[cfg(test)]` tests; the production submodules use their defining paths
directly. Acceptance requires lib warnings to drop from 1,831 to 1,826,
lib-test to remain 561, both fixed quick-profile commands to exit 0, and
fmt/diff/pointer guards to remain green.

## Execution evidence

The five selected re-export names are now individually gated with
`#[cfg(test)]`. `cargo check --profile quick --lib -j4` completed with lib
warnings 1,826 (down from 1,831), and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test
warnings 561 and produced the test executable. Both commands exited 0;
production transaction submodules and their test consumers retain their
existing owners and behavior.
