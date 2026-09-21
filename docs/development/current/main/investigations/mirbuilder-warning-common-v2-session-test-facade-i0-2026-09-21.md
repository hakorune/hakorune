---
Status: closed__2026-09-21__WarningCommonV2SessionTestFacade
Task: MIRBUILDER-WARNING-COMMON-V2-SESSION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i9-2026-09-21.md
Implementation permission: true for one cfg(test) common-v2-session facade-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I10
---

# Warning cleanup: common V2 session test facade

## Six-line brief

```text
Decision: gate five unused common-V2-session reject re-exports with cfg(test);
  existing focused tests keep using the same types.
Source authority + canonical issuer: Rust cfg/name resolution in
  common_v2_session/mod.rs, defining modules, test consumers, and the I9
  two-surface warning inventory.
Non-authority: cargo-fix, wildcard imports, session semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) only to the five selected re-export groups,
  then run both fixed checks and fmt/diff/pointer guards.
Non-claims: no session lowering change, test behavior change, suppression,
  broad warning cleanup, or production route change.
```

## Precondition and acceptance

The I9 inventory records five lib-only unused-import diagnostics in
`src/mir/builder/resolved_lowering/common_v2_session/mod.rs`. The parent
re-exports are test-only; production uses the defining modules or direct
qualified paths. Acceptance requires lib warnings to drop from 1,826 to 1,821,
lib-test to remain 561, both fixed quick-profile commands to exit 0, and
fmt/diff/pointer guards to remain green.

## Execution evidence

The five selected re-export groups are now individually gated with
`#[cfg(test)]`. `cargo check --profile quick --lib -j4` completed with lib
warnings 1,821 (down from 1,826), and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test
warnings 561 and produced the test executable. Both commands exited 0;
production session code and test consumers retain their defining owners and
direct paths.
