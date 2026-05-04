---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381X, FuncLowering CLI guard bool contract
Related:
  - docs/development/current/main/phases/phase-29cv/P381W-RETURN-METHOD-ARG-PRESENCE-CONTRACT.md
  - lang/src/mir/builder/func_lowering.hako
---

# P381X: Func Lowering CLI Guard Bool Contract

## Problem

After P381W, the direct Stage1 env EXE route reaches:

```text
FuncLoweringBox._lower_func_body/5
%r156 = icmp eq i64 %r145, 1
%r145 defined with type i1 but expected i64
```

`is_cli` is only a guard for the specialized `HakoCli.run` observation path.
It is not an arithmetic value and should not be maintained as a 0/1 numeric
carrier.

## Decision

Remove the local numeric flag and guard the `CliRunLowerBox.lower_run/5` call
directly with nested `box_name == "HakoCli"` and `func_name == "run"` checks.

## Boundary

Allowed:

- remove the `is_cli` local flag
- keep `CliRunLowerBox` ownership unchanged
- keep fallback order unchanged after the CLI observation attempt

Not allowed:

- add backend Bool/i64 rescue
- change CLI run lowering semantics
- add new dispatch owners

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381x_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `_lower_func_body/5` no longer lowers the CLI guard as an i1 value compared
  through an i64 equality operation

## Result

Implemented. `FuncLoweringBox._lower_func_body/5` keeps the CLI guard as a
direct Bool branch instead of a numeric `0/1` local.
