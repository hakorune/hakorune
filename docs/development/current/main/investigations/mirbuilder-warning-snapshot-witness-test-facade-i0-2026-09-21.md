---
Status: closed__2026-09-21__WarningSnapshotWitnessTestFacade
Task: MIRBUILDER-WARNING-SNAPSHOT-WITNESS-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i12-2026-09-21.md
Implementation permission: true for one cfg(test) re-export gate only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I13
---

# Warning cleanup: snapshot-witness test facade

## Six-line brief

```text
Decision: gate the snapshot-witness re-export with cfg(test); existing
  analysis and strict-JSON tests keep using the same helper.
Source authority + canonical issuer: Rust cfg/name resolution in
  bounded_body_snapshot_v0/mod.rs, the witness helper, and the I12 warning
  inventory; production snapshot ownership remains unchanged.
Non-authority: cargo-fix, wildcard imports, snapshot semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) to the selected re-export, then run both
  fixed checks and fmt/diff/pointer/lifecycle guards.
Non-claims: no snapshot redesign, test behavior change, suppression, broad
  warning cleanup, or production route change.
```

## Precondition and acceptance

The I12 inventory records one lib-only unused-import diagnostic at
`src/analysis/bounded_body_snapshot_v0/mod.rs:23`. The helper is consumed only
by analysis and strict-JSON snapshot tests; production keeps its existing
snapshot owners. Acceptance requires lib warnings to drop from 1,819 to 1,818,
lib-test to remain 561, both fixed quick-profile commands to exit 0, and
fmt/diff/pointer/lifecycle guards to remain green.

## Execution evidence

The selected re-export is now gated with `#[cfg(test)]`. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at 1,818, and `cargo
test --profile quick --lib --no-run -j4` exited 0 with lib-test warnings at
561 and produced the test executable. Production snapshot owners remain
unchanged.
