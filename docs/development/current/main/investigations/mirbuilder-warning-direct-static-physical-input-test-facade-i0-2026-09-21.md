---
Status: closed__2026-09-21__WarningDirectStaticPhysicalInputTestFacade
Task: MIRBUILDER-WARNING-DIRECT-STATIC-PHYSICAL-INPUT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i25-2026-09-21.md
Implementation permission: true for one cfg(test) direct-static physical-input import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I26
---

# Warning cleanup: direct-static physical-input test facade

## Six-line brief

```text
Decision: gate the direct-static physical-input SourceExprSite import consumed
  only by local tests; keep physical_input.rs as the owner.
Source authority + canonical issuer: physical_input.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, static-join semantics, visibility
  redesign, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the SourceExprSite import, then run the
  fixed gates once each.
Non-claims: no static-join redesign, suppression, or production route change.
```

## Precondition and acceptance

I25 records one lib-only unused-import diagnostic at
`src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs:8`.
The local tests consume `SourceExprSiteV1` through `super::*`; production code
does not. Acceptance requires lib warnings to drop from 1,806 to 1,805,
lib-test warnings to remain 561, both fixed quick-profile commands to exit 0,
and fmt/diff/pointer/lifecycle guards to remain green.

Only the import declaration may change. If the consumer classification or count
differs, stop and return to design stop.

## Execution evidence

The `SourceExprSiteV1` import is now gated with `#[cfg(test)]`; static-join
physical-input validation and production lowering are unchanged. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at **1,805**. The clean
sequential `cargo test --profile quick --lib --no-run -j4` exited 0 with
lib-test warnings at **561** and produced the test executable. Fmt, diff,
pointer, and lifecycle guards remain required at closeout.
