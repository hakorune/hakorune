---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-PHYSICAL-ABI-TEST-IMPORT-DELETE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i81-2026-09-21.md
Implementation permission: true for removal of the unused test-only `use super::*` line only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I82
---

# MirBuilder physical ABI test import delete I0

## Six-line brief

```text
Decision: delete one unused test-only parent glob import from the physical ABI
  tests; retain explicit imports and all test behavior.
Source authority + canonical issuer: physical_abi_tests.rs explicit imports.
Non-authority: parent module exports, warning guesses, or production ABI code.
Fail-fast boundary: any unresolved symbol, changed test behavior, or warning
  classification drift stops the slice.
Smallest next slice: remove the glob import and run lib check, test build, and
  the physical ABI focused suite.
Non-claims: no ABI change, no test deletion, no suppression, no production
  route change, and no semantic refactor.
```

## Census boundary and acceptance

`src/mir/compiler/normal_default_pipeline/published_backend_view/physical_abi_tests.rs`
contains `use super::*` followed by explicit `MirCompiler` and
`NormalCompileRequestV1` imports. A file-local symbol census shows no use of a
parent-only name, so the glob is a caller-zero test facade.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib -j4 physical_abi_tests -- --nocapture
```

The focused suite must pass, lib-test warnings must decrease from **557**, and
fmt/diff/pointer guards must be green before closeout.

## Closeout evidence

The unused `use super::*` line was removed from the physical ABI test module;
its explicit imports remain unchanged. Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warnings **1,741**.
- `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **556**.
- `cargo test --profile quick --lib -j4 numeric_capability_rejects_drift_in_referenced_physical_layout -- --nocapture`: **1/1**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

No ABI code, production route, test body, or suppression changed. One unused
test-only import was physically deleted. The next action is the I83 warning
baseline refresh.
