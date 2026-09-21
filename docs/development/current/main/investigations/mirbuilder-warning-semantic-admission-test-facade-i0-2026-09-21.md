---
Status: closed__2026-09-21__WarningSemanticAdmissionTestFacade
Task: MIRBUILDER-WARNING-SEMANTIC-ADMISSION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i18-2026-09-21.md
Implementation permission: true for one cfg(test) MirBuilder semantic-admission re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I19
---

# Warning cleanup: MirBuilder semantic-admission test facade

## Six-line brief

```text
Decision: gate the MirBuilder semantic-admission enum re-export consumed only
  by tests; keep normal_callable_semantic_source.rs as the owner.
Source authority + canonical issuer: normal_callable_semantic_source.rs;
  cfg(test) name resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, visibility redesign, semantic
  admission behavior, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to NormalCallableSemanticAdmissionV1's
  re-export, then run the fixed gates once each.
Non-claims: no resolver redesign, semantic change, suppression, or production
  route change.
```

## Precondition and acceptance

I18 records one lib-only unused-import diagnostic at
`src/mir/builder.rs:83`. Production consumers use the defining module, while
the parent facade is consumed by existing test modules. Acceptance requires
lib warnings to drop from 1,813 to 1,812, lib-test warnings to remain 561,
both fixed quick-profile commands to exit 0, and fmt/diff/pointer/lifecycle
guards to remain green.

Only the re-export declaration may change. If the count or consumer
classification differs, stop and return to design stop.

## Execution evidence

The selected re-export is now gated with `#[cfg(test)]`; the defining module
and production semantic-admission paths are unchanged. `cargo check --profile
quick --lib -j4` exited 0 with lib warnings at **1,812**. The sequential
`cargo test --profile quick --lib --no-run -j4` exited 0 with lib-test warnings
at **561** and produced the test executable. No resolver or production-route
behavior changed.
