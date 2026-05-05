# P381CW Module Generic I64 Origin Publish Helper

Date: 2026-05-05
Scope: share i64 type plus result-origin publication in the Stage0 module-generic emitter.

## Context

P381CV centralized paired `origin` and `scan_origin` publication. The emitter
still had repeated adjacent pairs where a value was first marked as `T_I64` and
then published with the same durable origin.

Those sites represent one local contract: this result is an i64-compatible
handle/scalar value with a known origin.

## Change

Added `module_generic_string_publish_i64_origin(reg, origin_kind)` and rewired
the repeated `set_type(..., T_I64)` plus origin-publication sites.

Plain `set_type(..., T_I64)` remains explicit for scalar-only results, and
`module_generic_string_publish_origin(...)` remains explicit where the type was
already established by a call helper.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cw_i64_origin.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cw_i64_origin.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cw_i64_origin.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cw_emit_program.out \
  /tmp/hakorune_p381cw_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cw_emit_program.out \
  /tmp/hakorune_p381cw_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cw_i64_origin.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cw_emit_mir.out \
  /tmp/hakorune_p381cw_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cw_emit_mir.out \
  /tmp/hakorune_p381cw_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic emitter now has separate helpers for origin-only publication
and i64-plus-origin publication, making scalar-only cases easier to distinguish.
