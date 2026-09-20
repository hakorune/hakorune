---
Status: closed__2026-09-21__WarningLoopPhysicalizerTestFacade
Task: MIRBUILDER-WARNING-LOOP-PHYSICALIZER-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i6-2026-09-21.md
Implementation permission: true for one cfg(test) facade-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I7
---

# Warning cleanup: loop physicalizer test facade

## Six-line brief

```text
Decision: gate five unused loop physicalizer re-exports with cfg(test); the
  test-only Generic G0 session keeps using the same names.
Source authority + canonical issuer: Rust cfg/name resolution in the facade,
  its generic-G0 test consumer, and the I6 two-surface inventory.
Non-authority: cargo-fix, wildcard imports, lowerer semantics, dead-code
  ownership, visibility changes, or warning counts.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) to only the five selected re-export lines,
  then run both fixed checks and fmt/diff/pointer guards.
Non-claims: no loop lowering change, test behavior change, suppression, broad
  warning cleanup, or production cutover.
```

## Precondition and acceptance

The I6 inventory records five lib-only unused-import warnings at
`loop_recipe_physicalizer/mod.rs:40-44`. The only consumer is the cfg(test)
`generic_g0_physical_emitter_session`; the production lowerers import the
defining modules directly. Acceptance requires lib warnings to drop from
1,840 to 1,835, lib-test to remain 561, both fixed quick-profile commands to
exit 0, and fmt/diff/pointer guards to remain green.

## Execution evidence

The five selected re-exports are now individually gated with `#[cfg(test)]`.
`cargo check --profile quick --lib -j4` completed with lib warnings 1,835
(down from 1,840), and `cargo test --profile quick --lib --no-run -j4`
completed with lib-test warnings 561 and produced the test executable. Both
commands exited 0; production lowerers and the Generic G0 test consumer retain
their existing owners and behavior.
