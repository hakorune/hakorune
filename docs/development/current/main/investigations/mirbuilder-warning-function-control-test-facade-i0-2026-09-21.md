---
Status: closed__2026-09-21__WarningFunctionControlTestFacade__TwoLayerFacadeDeletion
Task: MIRBUILDER-WARNING-FUNCTION-CONTROL-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i66-2026-09-21.md
Implementation permission: true for one cfg(test) parent re-export deletion only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I67
---

# Warning cleanup: function-control test facade

## Six-line brief

```text
Decision: remove the two unused cfg(test) re-export layers for
  verify_function_completion_with_new_homes_v1; the helper definition remains
  owned by function_control_new_homes.rs.
Source authority + canonical issuer: function_control.rs owns the helper; the
  I66 two-surface inventory proves the resolved_control_flow facade is unused.
Non-authority: warning counts alone, cargo-fix, wildcard imports, visibility
  widening, completion semantics, or production control-flow interpretation.
Fail-fast boundary: any parent caller, compile error, new warning/failure
  name, or changed test behavior rejects the deletion.
Smallest next slice: delete the two cfg(test) re-export lines from
  resolved_control_flow/mod.rs and function_control.rs.
Non-claims: no completion verifier change, semantic control-flow change,
  suppression, dead-code cleanup, or production route change.
```

## Preconditions and acceptance

The current fixed baseline is lib **1,754** and lib-test **559**. The source
census finds only the helper definition and the two unused re-export layers;
the parent-only attempt exposed the owner-local warning, so both layers form
one bounded chain. Then remove only those two lines and run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo fmt --all -- --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Expected counts are lib **1,754** and lib-test **558**. Any extra reference,
compile failure, new warning/red name, or behavior change rejects the deletion
and returns to the next baseline refresh. No `#[allow]`, cargo-fix, or
neighboring import cleanup is allowed.

## Execution evidence

The initial parent-only edit was rejected as incomplete because the warning
moved to the owner-local re-export. The bounded same-helper chain was then
completed by removing exactly both cfg(test) re-export lines, while leaving
the helper definition and all production completion logic unchanged.

The fixed gates ran sequentially and exited 0:

| gate | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib **1,754** warnings | `/tmp/hakorune-warning-function-control-chain-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test **558** warnings | `/tmp/hakorune-warning-function-control-chain-lib-test-20260921.log` |
| `cargo fmt --all -- --check` | PASS | local command |
| `git diff --check` | PASS | local command |
| current-state pointer guard | PASS | local command |

The post-edit census contains only the helper definition in
`function_control_new_homes.rs`; no facade caller or re-export remains. No
new failure name, warning family, or control-flow behavior appeared.
