# P381DU Module Generic Function View Helper

Date: 2026-05-06
Scope: move module-generic function view read behind one helper.

## Context

P381DT removed the stale restore label from the module-generic function
definition shell. The shell still directly called
`hako_llvmc_read_generic_pure_function_view(...)` and checked the metadata
field inline.

That direct read is the definition-entry view contract.

## Change

Added `module_generic_string_read_function_view(...)` and moved the existing
generic pure function view read plus metadata check into it.

No eligibility behavior, activation behavior, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381du_function_view_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381du_function_view_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381du_function_view_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381du_emit_program.out \
  /tmp/hakorune_p381du_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381du_emit_program.out \
  /tmp/hakorune_p381du_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381du_function_view_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381du_emit_mir.out \
  /tmp/hakorune_p381du_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381du_emit_mir.out \
  /tmp/hakorune_p381du_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic function definition shell no longer owns generic pure
function view parsing inline.
