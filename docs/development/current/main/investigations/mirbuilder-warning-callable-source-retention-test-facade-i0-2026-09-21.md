---
Status: closed__2026-09-21__WarningCallableSourceRetentionTestFacade
Task: MIRBUILDER-WARNING-CALLABLE-SOURCE-RETENTION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i17-2026-09-21.md
Implementation permission: true for one cfg(test) parser callable-source re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I18
---

# Warning cleanup: parser callable-source retention test facade

## Six-line brief

```text
Decision: gate the parser callable-source retention error re-export that is
  consumed only by retained parser tests; keep product.rs as the owner.
Source authority + canonical issuer: callable_parameter_source/product.rs;
  cfg(test) name resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, visibility redesign, retention
  semantics, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to ParserCallableSourceRetentionErrorV1's
  re-export, then run the fixed gates once each.
Non-claims: no parser redesign, semantic change, suppression, or production
  route change.
```

## Precondition and acceptance

I17 records one lib-only unused-import diagnostic at
`src/parser/callable_parameter_source/mod.rs:69`. The defining product module
uses the error internally, while the facade is consumed only by the existing
`#[cfg(test)]` retained test module. Acceptance requires lib warnings to drop
from 1,814 to 1,813, lib-test warnings to remain 561, both fixed quick-profile
commands to exit 0, and fmt/diff/pointer/lifecycle guards to remain green.

Only the re-export declaration may change. If the count or consumer
classification differs, stop and return to design stop.

## Execution evidence

The selected re-export is now gated with `#[cfg(test)]`; `product.rs` remains
the defining owner and production retention paths are unchanged. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at **1,813**. The
sequential `cargo test --profile quick --lib --no-run -j4` exited 0 with
lib-test warnings at **561** and produced the test executable. No parser
semantics or production route changed.
