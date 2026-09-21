---
Status: closed__2026-09-21__WarningInvocationIdentityTestFacade
Task: MIRBUILDER-WARNING-INVOCATION-IDENTITY-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i11-2026-09-21.md
Implementation permission: true for one cfg(test) import gate only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I12
---

# Warning cleanup: invocation-identity test facade

## Six-line brief

```text
Decision: gate the NonZeroU64 import with cfg(test); the existing test-only
  preflight factory keeps using the same numeric type.
Source authority + canonical issuer: Rust cfg/name resolution in
  module_invocation_identity.rs, its test factory, and the I11 warning
  inventory; production token issuance remains the existing owner.
Non-authority: cargo-fix, wildcard imports, identity semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) to the selected std import, then run both
  fixed checks and fmt/diff/pointer/lifecycle guards.
Non-claims: no identity redesign, test behavior change, suppression, broad
  warning cleanup, or production route change.
```

## Precondition and acceptance

The I11 inventory records one lib-only unused-import diagnostic at
`src/mir/builder/module_invocation_identity.rs:8`. The import is consumed only
by `#[cfg(test)] TestInvocationPreflightFactoryV1`; production retains the
existing `ModuleInvocationTokenV1` owner. Acceptance requires lib warnings to
drop from 1,820 to 1,819, lib-test to remain 561, both fixed quick-profile
commands to exit 0, and fmt/diff/pointer/lifecycle guards to remain green.

## Execution evidence

The selected import is now gated with `#[cfg(test)]`. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at 1,819, and `cargo
test --profile quick --lib --no-run -j4` exited 0 with lib-test warnings at
561 and produced the test executable. The production invocation identity
types and issuer remain unchanged.
