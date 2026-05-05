# P381CM Module Generic MIR JSON Map Field View Helper

Date: 2026-05-05
Scope: reduce repeated MIR JSON map-field generic-method view predicates in the module-generic Stage0 emitter.

## Context

P381CL centralized the repeated MIR JSON array-item `generic_method.get`
predicates in `hako_llvmc_ffi_module_generic_string_method_views.inc`.

The neighboring MIR JSON map-field predicates still repeated the same
route/core/symbol/publication/tier checks, with only proof, return contract,
value demand, and key policy differences.

## Change

Added a shared helper:

- `module_generic_string_method_view_is_direct_mir_json_map_get_with`

Then rewired the existing named predicates for:

- block field get
- const value field get
- numeric value field get
- callee field get
- instruction field get
- function field get
- module field get
- flags record map get

No public predicate names changed. Each named predicate keeps its local key
policy, while the route contract is checked in one helper.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cm_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cm_stage1_cli_env.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cm_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cm_emit_program.out \
  /tmp/hakorune_p381cm_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cm_emit_program.out \
  /tmp/hakorune_p381cm_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cm_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cm_emit_mir.out \
  /tmp/hakorune_p381cm_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cm_emit_mir.out \
  /tmp/hakorune_p381cm_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The second repeated method-view family is centralized behind a table-like
helper. MIR JSON map-field view contracts are now expressed once, with the
remaining named predicates acting as narrow key-policy wrappers.
