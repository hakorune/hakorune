# P381CV Module Generic Origin Publish Helper

Date: 2026-05-05
Scope: share result-origin plus scan-origin publication in the Stage0 module-generic emitter.

## Context

The module-generic body emitter repeatedly published the same origin to both
`origin` and `scan_origin` with paired calls:

- string results
- array/map births
- MIR JSON field-derived results
- extern Stage1 string-return routes

Those pairs represent one semantic action: publish a durable result origin.
Keeping the paired calls open-coded made future edits easy to partially apply.

## Change

Added `module_generic_string_publish_origin(reg, origin_kind)` and rewired
paired `set_origin` / `set_scan_origin` sites through it.

Single-owner updates remain open-coded where only one channel is intentionally
updated, such as `ORG_MAP_GET` origin-only publication or copied scan-origin
propagation.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cv_origin_publish.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cv_origin_publish.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cv_origin_publish.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cv_emit_program.out \
  /tmp/hakorune_p381cv_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cv_emit_program.out \
  /tmp/hakorune_p381cv_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cv_origin_publish.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cv_emit_mir.out \
  /tmp/hakorune_p381cv_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cv_emit_mir.out \
  /tmp/hakorune_p381cv_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Dual-channel origin publication now has one local helper in the module-generic
emitter. Intentional origin-only updates remain visible.
