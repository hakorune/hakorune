---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-PROGRAM-V0-SNAPSHOT-WITNESS-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i86-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the verification-only module declaration only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I87
---

# MirBuilder ProgramV0 snapshot witness test scope I0

## Six-line brief

```text
Decision: compile the verification-only ProgramV0 snapshot witness module only
  for tests; retain its canonical test re-export and snapshot tests.
Source authority + canonical issuer: bounded_body_snapshot_v0/program_v0_snapshot_witness.rs.
Non-authority: production schema/reader modules, warning guesses, or snapshot
  semantics.
Fail-fast boundary: any non-test caller, missing test module, changed snapshot
  behavior, or warning classification drift stops the slice.
Smallest next slice: add cfg(test) to one module declaration and run lib check,
  test build, and the snapshot witness focused suite.
Non-claims: no wire-schema change, no test deletion, no suppression, and no
  production route change.
```

## Census boundary and acceptance

`program_v0_snapshot_witness.rs` is declared unconditionally in
`analysis/bounded_body_snapshot_v0/mod.rs`, but its only parent re-export is
`#[cfg(test)]` and repository callers are test modules. Scope the declaration
to `#[cfg(test)]`; keep the witness implementation and test re-export intact.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib -j4 bounded_body_snapshot_v0 -- --nocapture
```

The focused suite must run at least one test and pass, lib warnings must drop
from **1,741**, lib-test behavior must remain green, and fmt/diff/pointer guards
must pass before closeout.

## Closeout evidence

The verification-only `program_v0_snapshot_witness` module declaration is now
scoped to `cfg(test)`. Its test re-export and implementation are unchanged.
Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warnings **1,737** (down from **1,741**).
- `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **553**.
- `cargo test --profile quick --lib -j4 bounded_body_snapshot_v0 -- --nocapture`: **38/38**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

No schema, snapshot behavior, production route, test body, or suppression changed.
One verification-only production module edge was physically removed. The next
action is the I87 warning baseline refresh.
