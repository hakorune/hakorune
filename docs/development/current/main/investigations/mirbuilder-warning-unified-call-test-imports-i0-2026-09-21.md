---
Status: closed__2026-09-21__WarningUnifiedCallTestImports
Task: MIRBUILDER-WARNING-UNIFIED-CALL-TEST-IMPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i19-2026-09-21.md
Implementation permission: true for two cfg(test) unified-call post-success imports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I20
---

# Warning cleanup: unified-call post-success test imports

## Six-line brief

```text
Decision: gate the two unified-call post-success imports used only by local
  tests; keep Callee and production post-success ownership unchanged.
Source authority + canonical issuer: unified_emitter/post_success.rs;
  cfg(test) name resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, call semantics, visibility
  redesign, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: gate CalleeBoxKind and TypeCertainty with cfg(test), then
  run the fixed gates once each.
Non-claims: no call-lowering redesign, semantic change, suppression, or
  production route change.
```

## Precondition and acceptance

I19 records two lib-only unused-import diagnostics at
`src/mir/builder/calls/unified_emitter/post_success.rs:9`. The local test
module consumes both names through `super::*`; production code does not.
Acceptance requires lib warnings to drop from 1,812 to 1,810, lib-test warnings
to remain 561, both fixed quick-profile commands to exit 0, and fmt/diff/pointer/
lifecycle guards to remain green.

Only the import declaration may change. If either consumer classification or
count differs, stop and return to design stop.

## Execution evidence

The two selected imports are now gated with `#[cfg(test)]`; `Callee` and the
production post-success path are unchanged. `cargo check --profile quick
--lib -j4` exited 0 with lib warnings at **1,810**. The sequential `cargo test
--profile quick --lib --no-run -j4` exited 0 with lib-test warnings at **561**
and produced the test executable. No call semantics or production route
changed.
