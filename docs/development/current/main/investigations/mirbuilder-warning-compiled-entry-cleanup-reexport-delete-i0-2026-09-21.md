---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-COMPILED-ENTRY-CLEANUP-REEXPORT-DELETE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i84-2026-09-21.md
Implementation permission: true for removal of the unused test-only parent re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I85
---

# MirBuilder compiled-entry cleanup re-export delete I0

## Six-line brief

```text
Decision: remove the unused test-only parent re-export of
  CompiledEntryCleanupKindV1; retain the canonical enum and nested tests.
Source authority + canonical issuer: published_backend_view/compiled_entry_contract.rs.
Non-authority: parent facade reachability, warning guesses, or cleanup semantics.
Fail-fast boundary: any parent caller, unresolved nested test, changed cleanup
  behavior, or warning classification drift stops the slice.
Smallest next slice: remove one parent export and run lib check, test build, and
  the compiled-entry focused suite.
Non-claims: no cleanup ABI change, no test deletion, no suppression, and no
  production route change.
```

## Census boundary and acceptance

`CompiledEntryCleanupKindV1` is defined and consumed inside
`published_backend_view/compiled_entry_contract.rs` and its nested
`compiled_entry_contract_tests` module. The parent
`#[cfg(test)] pub(crate) use` in `published_backend_view.rs` has no caller.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib -j4 compiled_entry_contract -- --nocapture
```

The focused suite must run at least one test and pass, lib-test warnings must
decrease from **555**, and fmt/diff/pointer guards must be green before closeout.

## Closeout evidence

The unused `#[cfg(test)]` parent re-export of `CompiledEntryCleanupKindV1` was
removed. The enum definition and nested test module remain unchanged.
Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warnings **1,741**.
- `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **554**.
- `cargo test --profile quick --lib -j4 compiled_entry_contract -- --nocapture`: **3/3**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

No cleanup behavior, ABI, production route, or test body changed. One unused
test-only parent export was physically deleted. The next action is the I86
warning baseline refresh.
