# P381CO Module Generic Receiver Method View Helper

Date: 2026-05-05
Scope: centralize receiver-origin generic-method view contracts in the module-generic Stage0 emitter.

## Context

P381CL through P381CN removed repeated predicate bodies from the largest
method-view families. The remaining receiver-origin predicates still repeated
the same route/core/symbol/proof/receiver/return/publication/tier checks across
ArrayBox, MapBox, and StringBox methods.

## Change

Added shared helpers:

- `module_generic_string_method_view_str_eq`
- `module_generic_string_method_view_optional_str_eq`
- `module_generic_string_method_view_is_direct_receiver_with`

Then rewired the existing named predicates for:

- `ArrayBox.len`
- `ArrayBox.push`
- `ArrayBox.get`
- `MapBox.get`
- `MapBox.set`
- `StringBox.substring`
- the P381CN StringBox scalar wrapper

No public predicate names changed. Optional return/publication contracts are
still exact: passing `NULL` means the field must be absent.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381co_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381co_stage1_cli_env.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381co_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381co_emit_program.out \
  /tmp/hakorune_p381co_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381co_emit_program.out \
  /tmp/hakorune_p381co_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381co_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381co_emit_mir.out \
  /tmp/hakorune_p381co_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381co_emit_mir.out \
  /tmp/hakorune_p381co_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Receiver-origin method-view contracts now have one exact-match helper. The
named predicates remain as the stable emitter surface, but the contract truth is
thinner and less branch-shaped.
