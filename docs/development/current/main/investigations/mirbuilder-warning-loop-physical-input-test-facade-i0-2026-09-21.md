---
Status: closed__2026-09-21__WarningLoopPhysicalInputTestFacade
Task: MIRBUILDER-WARNING-LOOP-PHYSICAL-INPUT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i24-2026-09-21.md
Implementation permission: true for one cfg(test) loop-physical-input import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I25
---

# Warning cleanup: loop physical-input test facade

## Six-line brief

```text
Decision: gate the loop physical-input BindingId import consumed only by tests;
  keep loop_physical_input.rs as the physical-input owner.
Source authority + canonical issuer: loop_physical_input.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, physical validation semantics,
  role planning, visibility redesign, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the BindingId import, then run the fixed
  gates once each.
Non-claims: no physical-input redesign, suppression, or production route change.
```

## Precondition and acceptance

I24 records one lib-only unused-import diagnostic at
`src/mir/builder/control_flow/plan/loop_physical_input.rs:10`. The local tests
consume `BindingId` through `super::*`; production code does not.
Acceptance requires lib warnings to drop from 1,807 to 1,806, lib-test warnings
to remain 561, both fixed quick-profile commands to exit 0, and fmt/diff/
pointer/lifecycle guards to remain green.

Only the import declaration may change. If the consumer classification or count
differs, stop and return to design stop.

## Execution evidence

The `BindingId` import is now gated with `#[cfg(test)]`; physical-input
validation, role planning, and production lowering are unchanged. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at **1,806**. The clean
sequential `cargo test --profile quick --lib --no-run -j4` exited 0 with
lib-test warnings at **561** and produced the test executable. Fmt, diff,
pointer, and lifecycle guards remain required at closeout.
