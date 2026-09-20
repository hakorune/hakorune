---
Status: closed__2026-09-21__WarningRawRootTestFacade
Task: MIRBUILDER-WARNING-RAW-ROOT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i7-2026-09-21.md
Implementation permission: true for one cfg(test) raw-root facade-scope move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I8
---

# Warning cleanup: raw-root test facade

## Six-line brief

```text
Decision: gate four unused raw-root completion imports with cfg(test); the
  existing raw completion tests keep using the same types.
Source authority + canonical issuer: Rust cfg/name resolution in
  raw_root_completion.rs, its #[cfg(test)] tests module, and the I7 two-surface
  warning inventory.
Non-authority: cargo-fix, wildcard imports, raw-root semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) only to the four selected import groups,
  then run both fixed checks and fmt/diff/pointer guards.
Non-claims: no raw-root lowering change, test behavior change, suppression,
  broad warning cleanup, or production route change.
```

## Precondition and acceptance

The I7 inventory records four lib-only unused-import diagnostics at the top of
`src/mir/builder/raw_root_completion.rs`. All four names are consumed only by
the file's `#[cfg(test)] mod tests`; the production completion path retains its
sealed ledger and compatibility disposition imports. Acceptance requires the
lib warning count to drop from 1,835 to 1,831, lib-test to remain 561, both
fixed quick-profile commands to exit 0, and fmt/diff/pointer guards to remain
green.

## Execution evidence

The four selected import groups are now individually gated with `#[cfg(test)]`.
`cargo check --profile quick --lib -j4` completed with lib warnings 1,831
(down from 1,835), and `cargo test --profile quick --lib --no-run -j4`
completed with lib-test warnings 561 and produced the test executable. Both
commands exited 0; `cargo fmt --all -- --check`, `git diff --check`, and the
current-state pointer guard are green. Production raw-root completion and its
test consumers retain their existing owners and behavior.
