# P381CN Module Generic String Scalar View Helper

Date: 2026-05-05
Scope: reduce repeated StringBox scalar generic-method view predicates in the module-generic Stage0 emitter.

## Context

P381CL and P381CM centralized the repeated MIR JSON array-item and map-field
view predicates in `hako_llvmc_ffi_module_generic_string_method_views.inc`.

The StringBox scalar predicates for `indexOf`, `lastIndexOf`, and `contains`
still repeated the same receiver, return, value-demand, publication, and tier
checks. Only the route vocabulary differed.

## Change

Added a shared helper:

- `module_generic_string_method_view_is_direct_string_scalar_with`

Then rewired the existing named predicates for:

- `indexOf`
- `lastIndexOf`
- `contains`

No public predicate names changed. Emit/prepass callers still use the same
surface.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cn_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cn_stage1_cli_env.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cn_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cn_emit_program.out \
  /tmp/hakorune_p381cn_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cn_emit_program.out \
  /tmp/hakorune_p381cn_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cn_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cn_emit_mir.out \
  /tmp/hakorune_p381cn_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cn_emit_mir.out \
  /tmp/hakorune_p381cn_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The StringBox scalar view contracts are now expressed once, with the named
predicates acting as route-vocabulary wrappers. This keeps the method-view
surface table-like while reducing another repeated C branch family.
