---
Status: closed__2026-09-21__WarningInvocationCollectionTestFacade
Task: MIRBUILDER-WARNING-INVOCATION-COLLECTION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i22-2026-09-21.md
Implementation permission: true for one cfg(test) invocation-collection family import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I23
---

# Warning cleanup: invocation-collection test facade

## Six-line brief

```text
Decision: gate the invocation-collection header-family import consumed only by
  local test helpers; keep capability.rs as the family owner.
Source authority + canonical issuer: compiler/capability.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, collection semantics, visibility
  redesign, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to ResolvedOwnerHeaderFamilyV1's import,
  then run the fixed gates once each.
Non-claims: no invocation-collection redesign, semantic change, suppression,
  or production route change.
```

## Precondition and acceptance

I22 records one lib-only unused-import diagnostic at
`src/mir/builder/module_invocation_collection.rs:21`. The local test helpers
consume the family enum; production code does not. Acceptance requires lib
warnings to drop from 1,809 to 1,808, lib-test warnings to remain 561, both
fixed quick-profile commands to exit 0, and fmt/diff/pointer/lifecycle guards
to remain green.

Only the import declaration may change. If the consumer classification or
count differs, stop and return to design stop.

## Execution evidence

The selected family import is now gated with `#[cfg(test)]`; verified header
transport and production collection remain unchanged. `cargo check --profile
quick --lib -j4` exited 0 with lib warnings at **1,808**. The sequential
`cargo test --profile quick --lib --no-run -j4` exited 0 with lib-test warnings
at **561** and produced the test executable. No invocation collection or
header-family semantics changed.
