# P381DD Module Generic Global Emit Helper

Date: 2026-05-05
Scope: move module-generic global-call emission behind one helper.

## Context

P381DB moved method-family emission behind one helper. The main MIR call emitter
still owned the global-call lowering sequence directly: first try the
LoweringPlan global-call emitter, then fall back to the older single-argument
global surface emitter.

That sequence is the global-call family contract, not a main dispatcher detail.

## Change

Added `module_generic_string_emit_global_call_mir_call(...)` and moved the
global-call emission sequence into it.

The main `mir_call` emitter now delegates the `Global` branch to the helper while
preserving the existing order:

1. try `emit_global_call_lowering_plan_mir_call(...)`
2. if that does not accept, try the legacy one-argument `emit_global_mir_call(...)`

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dd_global_emit_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dd_global_emit_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dd_global_emit_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dd_emit_program.out \
  /tmp/hakorune_p381dd_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dd_emit_program.out \
  /tmp/hakorune_p381dd_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dd_global_emit_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dd_emit_mir.out \
  /tmp/hakorune_p381dd_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dd_emit_mir.out \
  /tmp/hakorune_p381dd_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic main MIR call emitter now delegates both global-call and
generic-method families through local helpers.
