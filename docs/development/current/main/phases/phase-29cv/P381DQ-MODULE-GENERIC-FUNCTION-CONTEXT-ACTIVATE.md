# P381DQ Module Generic Function Context Activate

Date: 2026-05-05
Scope: centralize module-generic function context activation.

## Context

P381DP added a snapshot contract for saving and restoring module-generic
function context. The activation step still assigned active function pointers,
block count, and lowering state inline in the function definition shell.

That was the remaining temporal context transition next to save/restore.

## Change

Added `module_generic_string_activate_function_context(...)` and moved active
function activation into it.

The helper preserves the existing activation order:

1. set active function and JSON/view pointers
2. set block count
3. reset generic pure function lowering state
4. reset string concat tracking

No prepass behavior, cleanup behavior, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dq_function_context_activate.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dq_function_context_activate.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dq_function_context_activate.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dq_emit_program.out \
  /tmp/hakorune_p381dq_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dq_emit_program.out \
  /tmp/hakorune_p381dq_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dq_function_context_activate.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dq_emit_mir.out \
  /tmp/hakorune_p381dq_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dq_emit_mir.out \
  /tmp/hakorune_p381dq_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Module-generic function context save, activate, and restore now have named
contracts.
