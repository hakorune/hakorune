# P381DC Module Generic Method Birth Helper

Date: 2026-05-05
Scope: share ArrayBox/MapBox method-birth detection in the module-generic emitter.

## Context

The module-generic prepass and actual method emitter both checked the same
`Method` + `birth` + `ArrayBox|MapBox` route shape. The resulting actions differ
between prepass and emit time, but the route acceptance itself is one contract.

## Change

Added `module_generic_string_call_is_array_or_map_birth(...)` and reused it in
both the prepass and the actual `Method` branch.

Prepass behavior still publishes the provisional `T_I64` type, and emit behavior
still emits the zero handle constant and `T_I64` type.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dc_birth_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dc_birth_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dc_birth_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dc_emit_program.out \
  /tmp/hakorune_p381dc_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dc_emit_program.out \
  /tmp/hakorune_p381dc_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dc_birth_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dc_emit_mir.out \
  /tmp/hakorune_p381dc_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dc_emit_mir.out \
  /tmp/hakorune_p381dc_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

ArrayBox/MapBox method-birth route recognition now has one helper shared by
module-generic prepass and emit-time handling.
