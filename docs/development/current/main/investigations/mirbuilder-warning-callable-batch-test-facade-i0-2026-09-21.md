---
Status: closed__2026-09-21__WarningCallableBatchTestFacade
Task: MIRBUILDER-WARNING-CALLABLE-BATCH-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i10-2026-09-21.md
Implementation permission: true for one cfg(test) callable-batch facade-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I11
---

# Warning cleanup: callable-batch test facade

## Six-line brief

```text
Decision: gate two unused callable-module plan imports with cfg(test); the
  existing source_from_test helper keeps using the same types.
Source authority + canonical issuer: Rust cfg/name resolution in
  module_invocation_callable_batch.rs, its test helper and production source
  proof, and the I10 two-surface warning inventory.
Non-authority: cargo-fix, wildcard imports, callable-batch semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) only to the selected two-name import group,
  then run both fixed checks and fmt/diff/pointer guards.
Non-claims: no callable-batch lowering change, test behavior change, suppression,
  broad warning cleanup, or production route change.
```

## Precondition and acceptance

The I10 inventory records two lib-only unused-import diagnostics at
`src/mir/builder/module_invocation_callable_batch.rs:17-18`. Both names are
consumed only by `#[cfg(test)] source_from_test`; production retains the
source proof and capability owners. Acceptance requires lib warnings to drop
from 1,821 to 1,820, lib-test to remain 561, both fixed quick-profile
commands to exit 0, and fmt/diff/pointer guards to remain green.

## Execution evidence

The selected two-name import group is now gated with `#[cfg(test)]`.
`cargo check --profile quick --lib -j4` completed with lib warnings 1,820
(down from 1,821), and `cargo test --profile quick --lib --no-run -j4`
completed with lib-test warnings 561 and produced the test executable. Both
commands exited 0; the callable-batch source helper retains its existing test
consumer and production source/capability owners.
