# P381DB Module Generic Method Emit Dispatch Helper

Date: 2026-05-05
Scope: move module-generic method emitter dispatch behind one helper.

## Context

P381DA split generic-method prepass facts out of the main prepass loop. The
runtime emitter still had the full method-family dispatch chain inside
`module_generic_string_emit_mir_call`.

That made the main emitter own two responsibilities: choosing the call family
and ordering the individual method emitters.

## Change

Added `module_generic_string_emit_generic_method_mir_call(...)` and moved the
method emitter chain into it.

The main `mir_call` emitter now keeps the method-family branch focused on:

- method birth handling
- loading the cached LoweringPlan generic-method view
- delegating method emission to one helper

The individual method emitters and their ordering are unchanged.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381db_method_emit_dispatch.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381db_method_emit_dispatch.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381db_method_emit_dispatch.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381db_emit_program.out \
  /tmp/hakorune_p381db_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381db_emit_program.out \
  /tmp/hakorune_p381db_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381db_method_emit_dispatch.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381db_emit_mir.out \
  /tmp/hakorune_p381db_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381db_emit_mir.out \
  /tmp/hakorune_p381db_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic main MIR call emitter now delegates generic-method emission
through one helper, matching the prepass split from P381DA.
