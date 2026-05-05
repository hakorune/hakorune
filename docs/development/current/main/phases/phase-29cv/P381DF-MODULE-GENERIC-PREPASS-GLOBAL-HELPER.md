# P381DF Module Generic Prepass Global Helper

Date: 2026-05-05
Scope: move module-generic global-call prepass facts behind one helper.

## Context

P381DD moved global-call emission behind one helper. The prepass still kept the
global-call fact publication chain inline: module-generic definition routes,
leaf-i64 definition routes, and the legacy print fallback.

That chain is the global-call prepass contract.

## Change

Added `module_generic_string_prepass_global_call_view(...)` and moved global-call
prepass handling into it.

The helper preserves the existing order:

1. module-generic definition routes publish `T_I64` and result origin
2. leaf-i64 definition routes publish `T_I64`
3. legacy `print` global calls with one argument remain accepted

The main prepass loop now tries method, extern, and global helper families in
sequence and fails fast when none accepts.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381df_prepass_global_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381df_prepass_global_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381df_prepass_global_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381df_emit_program.out \
  /tmp/hakorune_p381df_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381df_emit_program.out \
  /tmp/hakorune_p381df_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381df_prepass_global_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381df_emit_mir.out \
  /tmp/hakorune_p381df_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381df_emit_mir.out \
  /tmp/hakorune_p381df_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Module-generic prepass call-family handling now delegates method, extern, and
global facts through local helpers.
