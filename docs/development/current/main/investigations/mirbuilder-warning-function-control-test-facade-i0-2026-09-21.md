---
Status: design_stop__2026-09-21__WarningFunctionControlTestFacade__FastTransitionRequired
Task: MIRBUILDER-WARNING-FUNCTION-CONTROL-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i66-2026-09-21.md
Implementation permission: false; one cfg(test) parent re-export deletion after fast transition
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I67
---

# Warning cleanup: function-control test facade

## Six-line brief

```text
Decision: remove only the unused cfg(test) parent re-export of
  verify_function_completion_with_new_homes_v1; the function_control owner
  and its canonical test re-export remain unchanged.
Source authority + canonical issuer: function_control.rs owns the helper; the
  I66 two-surface inventory proves the resolved_control_flow facade is unused.
Non-authority: warning counts alone, cargo-fix, wildcard imports, visibility
  widening, completion semantics, or production control-flow interpretation.
Fail-fast boundary: any parent caller, compile error, new warning/failure
  name, or changed test behavior rejects the deletion.
Smallest next slice: delete exactly one cfg(test) use line from
  src/mir/resolved_control_flow/mod.rs.
Non-claims: no completion verifier change, semantic control-flow change,
  suppression, dead-code cleanup, or production route change.
```

## Preconditions and acceptance

The current fixed baseline is lib **1,754** and lib-test **559**. The source
census finds the helper definition and its owner-local cfg(test) re-export,
but no parent facade caller. Implementation is forbidden until the pointer
enters fast mode. Then remove only the parent re-export and run sequentially:

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
