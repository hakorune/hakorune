---
Status: closed__2026-09-21__WarningEnumMatchScopeboxTestFacade
Task: MIRBUILDER-WARNING-ENUM-MATCH-SCOPEBOX-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i20-2026-09-21.md
Implementation permission: true for one cfg(test) enum-match ScopeBox route import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I21
---

# Warning cleanup: enum-match ScopeBox test facade

## Six-line brief

```text
Decision: gate the enum-match ScopeBox route import consumed only by the
  local route-shape test; keep enum_match_scopebox.rs as the owner.
Source authority + canonical issuer: enum_match_scopebox.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, route semantics, visibility
  redesign, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to PreparedRawScopeBoxRouteV1's import,
  then run the fixed gates once each.
Non-claims: no enum-match lowering redesign, semantic change, suppression, or
  production route change.
```

## Precondition and acceptance

I20 records one lib-only unused-import diagnostic at
`src/mir/builder/exprs_enum_match.rs:12`. The local test module consumes the
route type through `super::*`; production code uses the defining module.
Acceptance requires lib warnings to drop from 1,810 to 1,809, lib-test warnings
to remain 561, both fixed quick-profile commands to exit 0, and fmt/diff/pointer/
lifecycle guards to remain green.

Only the import declaration may change. If the consumer classification or
count differs, stop and return to design stop.

## Execution evidence

The selected route import is now gated with `#[cfg(test)]`; the defining
ScopeBox module and enum-match lowering are unchanged. `cargo check --profile
quick --lib -j4` exited 0 with lib warnings at **1,809**. The sequential
`cargo test --profile quick --lib --no-run -j4` exited 0 with lib-test warnings
at **561** and produced the test executable. No production route changed.
