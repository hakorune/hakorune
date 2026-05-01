---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P121, MIR global call canonical target symbols
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P120-GENERIC-PURE-STRING-FUNCTION-EMITTER.md
  - src/mir/naming.rs
  - src/mir/global_call_route_plan.rs
---

# P121: Global Call Canonical Target Symbol

## Problem

P120 moved the selected entry through `Stage1ModeContractBox.resolve_mode/0`,
but the next `Main.main` callsites stopped as unknown globals:

```text
callee_name=main._run_emit_program_mode/0
reason=unknown_global_callee
```

The target functions exist in the module as `Main._run_emit_program_mode/0`,
`Main._run_emit_mir_mode/1`, and
`Main._run_emit_mir_program_json_compat_mode/1`. The mismatch is MIR static-box
naming, not a missing helper body.

## Decision

Use the existing MIR NamingBox policy for same-module global target lookup:

```text
callee_name    = observed diagnostic call name
target_symbol  = canonical MIR function symbol
```

Exact target names still win first. Only when exact lookup misses does the route
planner try `naming::normalize_static_global_name`, which currently preserves the
narrow `main.* -> Main.*` static entry alias policy.

## Rules

Allowed:

- keep `callee_name` unchanged for diagnostics and resolver evidence
- resolve `target_symbol` through MIR-owned naming normalization
- let ny-llvmc continue to consume only `target_symbol`

Forbidden:

- hardcoding `_run_emit_*` helpers
- rewriting `.hako` source to avoid the mismatch
- using backend-local raw name classification
- changing VM/source-execution behavior

## Evidence

Focused metadata check:

```text
b14948.i0 callee_name=main._run_emit_program_mode/0
          target_symbol=Main._run_emit_program_mode/0
          reason=missing_multi_function_emitter
b14961.i2 callee_name=main._run_emit_mir_mode/1
          target_symbol=Main._run_emit_mir_mode/1
          reason=missing_multi_function_emitter
b14962.i1 callee_name=main._run_emit_mir_program_json_compat_mode/1
          target_symbol=Main._run_emit_mir_program_json_compat_mode/1
          reason=missing_multi_function_emitter
```

Full `lang/src/runner/stage1_cli_env.hako` now moves from
`unknown_global_callee` to the expected module-emitter stop:

```text
first_block=14948 first_inst=0 first_op=mir_call
owner_hint=backend_lowering reason=missing_multi_function_emitter
```

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- generated `stage1_cli_env.hako` MIR carries canonical `target_symbol` for
  the `main._...` callsites.
