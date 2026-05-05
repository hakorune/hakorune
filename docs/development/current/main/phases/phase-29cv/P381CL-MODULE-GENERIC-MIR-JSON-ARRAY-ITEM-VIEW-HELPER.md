# P381CL Module Generic MIR JSON Array Item View Helper

Date: 2026-05-05
Scope: reduce repeated MIR JSON array-item generic-method view predicates in the module-generic Stage0 emitter.

## Context

P381CJ fixed the durable boundary: the `module_generic_string_*` C files are
historical names, while the active responsibility is module-generic uniform ABI
function emission.

The next cleanup target is `hako_llvmc_ffi_module_generic_string_method_views.inc`.
Its MIR JSON array-item `generic_method.get` predicates repeated the same
route/core/symbol/publication/tier checks with only proof and result contract
differences.

## Change

Added a shared helper:

- `module_generic_string_method_view_is_direct_mir_json_array_item_get_with`

Then rewired the existing named predicates for:

- block instruction array item
- effects array item
- function block array item
- module function array item
- params array item
- phi incoming array item
- phi incoming pair scalar
- vid array item
- flags record array get

No public predicate names changed. Emit/prepass callers still use the same
surface.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cl_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cl_stage1_cli_env.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cl_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cl_emit_program.out \
  /tmp/hakorune_p381cl_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cl_emit_program.out \
  /tmp/hakorune_p381cl_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cl_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cl_emit_mir.out \
  /tmp/hakorune_p381cl_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cl_emit_mir.out \
  /tmp/hakorune_p381cl_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The first repeated method-view family is now centralized behind a table-like
helper. This reduces the method-view surface without changing the route
contract or body emitter behavior.
