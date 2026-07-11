# P381CP MIR Call Extern View Helper

Date: 2026-05-05
Scope: centralize extern-call LoweringPlan view validation in the Stage0 MIR call shell.

## Context

The current Stage1 env probe no longer reproduces the older
`missing_multi_function_emitter` or module-generic body-emission blockers:

```bash
target/release/hakorune \
  --emit-mir-json /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cp_stage1_cli_env_probe.o
```

The next safe cleanup target is therefore `.inc` consolidation, not widening a
body-specific emitter.

`hako_llvmc_ffi_mir_call_shell.inc` still repeated the same extern-call
LoweringPlan checks for env, hostbridge, and Stage1 extern routes.

## Change

Added a shared helper:

- `lowering_plan_extern_call_view_is_valid_with`

Then rewired the existing named validators for:

- `extern.env.get`
- `extern.env.set`
- `extern.hostbridge.extern_invoke`
- `extern.stage1.emit_program_json_v0`
- `extern.stage1.emit_mir_from_source_v0`
- `extern.stage1.emit_mir_from_program_json_v0`

No public validator names changed. The helper fixes the common contract once:
`emit_kind=runtime_call`, `proof=extern_registry`, `route_proof=extern_registry`,
and `tier=ColdRuntime`.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cp_extern_view_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cp_extern_view_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cp_extern_view_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cp_extern_emit_program.out \
  /tmp/hakorune_p381cp_extern_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cp_extern_emit_program.out \
  /tmp/hakorune_p381cp_extern_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cp_extern_view_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cp_extern_emit_mir.out \
  /tmp/hakorune_p381cp_extern_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cp_extern_emit_mir.out \
  /tmp/hakorune_p381cp_extern_emit_mir.err
```

Observed:

- current Stage1 env MIR generation passed
- current Stage1 env OBJ generation passed before the cleanup probe
- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Extern-call route validation in the MIR call shell is now table-like. The
route-specific validators remain as stable call sites while the shared
LoweringPlan contract lives in one helper.
