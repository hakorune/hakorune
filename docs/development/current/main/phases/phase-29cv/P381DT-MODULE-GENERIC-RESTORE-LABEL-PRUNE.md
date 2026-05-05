# P381DT Module Generic Restore Label Prune

Date: 2026-05-06
Scope: remove the stale module-generic function restore label.

## Context

P381DR moved active-context prepass/body emission into
`module_generic_string_emit_function_pipeline(...)`. After that split,
`emit_generic_pure_string_function_definition(...)` no longer had any `goto`
edges, but still kept the old `MODULE_GENERIC_STRING_RESTORE` label before the
restore helper call.

The label was now only historical structure.

## Change

Removed `MODULE_GENERIC_STRING_RESTORE` and left the function shell with a
direct `module_generic_string_restore_function_context(...)` call before
returning `rc`.

No restore order, cleanup behavior, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dt_restore_label_prune.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dt_restore_label_prune.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dt_restore_label_prune.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dt_emit_program.out \
  /tmp/hakorune_p381dt_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dt_emit_program.out \
  /tmp/hakorune_p381dt_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dt_restore_label_prune.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dt_emit_mir.out \
  /tmp/hakorune_p381dt_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dt_emit_mir.out \
  /tmp/hakorune_p381dt_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic function definition shell no longer carries a stale restore
label after the pipeline split.
