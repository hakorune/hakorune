# P381DE Module Generic Method Emit Helper

Date: 2026-05-05
Scope: move the module-generic Method branch behind one helper.

## Context

P381DB split the generic-method emitter dispatch chain, and P381DC centralized
ArrayBox/MapBox method-birth recognition. The main MIR call emitter still owned
the Method branch details: receiver lookup, method birth handling, LoweringPlan
view lookup, and generic-method dispatch.

Those are all Method-family concerns.

## Change

Added `module_generic_string_emit_method_call_mir_call(...)` and moved the
Method branch internals into it.

The main `mir_call` emitter now keeps the Method branch as a single delegation,
while the helper preserves the existing order:

1. handle ArrayBox/MapBox method birth
2. read the LoweringPlan generic-method view
3. delegate to the generic-method emitter dispatch helper

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381de_method_emit_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381de_method_emit_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381de_method_emit_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381de_emit_program.out \
  /tmp/hakorune_p381de_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381de_emit_program.out \
  /tmp/hakorune_p381de_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381de_method_emit_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381de_emit_mir.out \
  /tmp/hakorune_p381de_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381de_emit_mir.out \
  /tmp/hakorune_p381de_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic main MIR call emitter now dispatches call families through
thin Extern, Global, and Method branches. Method-family internals live behind
one helper.
