# P381DJ Module Generic Emit Ret Helper

Date: 2026-05-05
Scope: move module-generic emit `ret` handling behind one helper.

## Context

P381DI removed `externcall` lowering-plan dispatch from the active-body emit
loop. The `ret` branch still carried copy-source resolution, constant folding,
i1-to-i64 zext emission, and value-reference formatting inline.

Those details are one terminator emission contract.

## Change

Added `module_generic_string_emit_ret_instruction(...)` and moved `ret`
emission into it.

The helper preserves the existing result order:

1. return a known constant directly
2. zext i1 return values to i64 using the existing block/reg suffix
3. otherwise format the i64 value reference and emit it

No route acceptance or generated C behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dj_emit_ret_instruction.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dj_emit_ret_instruction.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dj_emit_ret_instruction.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dj_emit_program.out \
  /tmp/hakorune_p381dj_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dj_emit_program.out \
  /tmp/hakorune_p381dj_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dj_emit_ret_instruction.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dj_emit_mir.out \
  /tmp/hakorune_p381dj_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dj_emit_mir.out \
  /tmp/hakorune_p381dj_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The active-body emit loop no longer owns `ret` value-reference and zext
emission inline.
