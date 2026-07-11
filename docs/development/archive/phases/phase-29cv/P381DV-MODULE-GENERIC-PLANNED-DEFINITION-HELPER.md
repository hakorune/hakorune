# P381DV Module Generic Planned Definition Helper

Date: 2026-05-06
Scope: move module-generic planned-symbol definition handling behind one helper.

## Context

P381DU moved generic pure function view parsing out of the function definition
shell. The outer planned-symbol driver still owned per-symbol lookup, entry
skip, function definition emission, and emitted-count updates inline.

That loop is the outer entry that the uniform multi-function emitter will
replace or reuse.

## Change

Added `module_generic_string_emit_planned_function_definition(...)` and moved
per-symbol handling into it.

The helper preserves existing behavior:

- missing planned symbol lookup is skipped
- the program entry function is skipped
- definition emission failures return `-1`
- successful definitions increment the emitted count

No planned-symbol ordering, skip behavior, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dv_planned_definition_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dv_planned_definition_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dv_planned_definition_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dv_emit_program.out \
  /tmp/hakorune_p381dv_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dv_emit_program.out \
  /tmp/hakorune_p381dv_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dv_planned_definition_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dv_emit_mir.out \
  /tmp/hakorune_p381dv_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dv_emit_mir.out \
  /tmp/hakorune_p381dv_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic planned-symbol driver no longer owns per-symbol lookup,
entry skip, definition emission, or emitted-count updates inline.
