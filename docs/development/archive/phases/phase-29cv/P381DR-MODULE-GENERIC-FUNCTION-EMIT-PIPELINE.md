# P381DR Module Generic Function Emit Pipeline

Date: 2026-05-05
Scope: move module-generic function prepass/body emission into one pipeline helper.

## Context

P381DP and P381DQ made module-generic function context save, activate, and
restore explicit. The definition shell still owned the active-context pipeline:
prepass, const globals, signature, active body emission, close brace, and
symbol registration.

That sequence is the uniform emitter's replaceable body contract.

## Change

Added `module_generic_string_emit_function_pipeline(...)` and moved the active
function emission sequence into it.

The helper preserves existing behavior:

- prepass failure reports `module_generic_prepass_failed` and returns `-1`
- const globals are emitted before the function signature
- signature emission uses `module_generic_string_emit_function_signature(...)`
- body failure reports `module_generic_body_emit_failed`, emits `unreachable`,
  closes the function, and returns `-1`
- success closes the function and registers the emitted symbol

No route acceptance, emitted text, or failure rc changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dr_function_emit_pipeline.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dr_function_emit_pipeline.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dr_function_emit_pipeline.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dr_emit_program.out \
  /tmp/hakorune_p381dr_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dr_emit_program.out \
  /tmp/hakorune_p381dr_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dr_function_emit_pipeline.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dr_emit_mir.out \
  /tmp/hakorune_p381dr_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dr_emit_mir.out \
  /tmp/hakorune_p381dr_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic function definition shell now delegates active-context
prepass/body emission through one pipeline helper.
