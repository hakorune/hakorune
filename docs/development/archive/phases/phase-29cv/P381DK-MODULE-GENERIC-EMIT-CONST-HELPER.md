# P381DK Module Generic Emit Const Helper

Date: 2026-05-05
Scope: move module-generic emit `const` handling behind one helper.

## Context

P381DJ moved `ret` value emission out of the active-body emit loop. The `const`
branch still owned string constant box emission, i64 constant publication, void
sentinel publication, and unsupported-shape diagnostics inline.

Those details are one constant-instruction emission contract.

## Change

Added `module_generic_string_emit_const_instruction(...)` and moved `const`
emission into it.

The helper preserves the existing result order:

1. string constants use `module_generic_string_emit_const_box(...)`
2. i64 constants are recorded with `put_const(...)` and `T_I64`
3. void sentinel constants are recorded as `0` and `T_I64`
4. unknown constants fail the active body path

The existing `module_generic_string_const_missing` diagnostic remains attached
to the same block and instruction index.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dk_emit_const_instruction.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dk_emit_const_instruction.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dk_emit_const_instruction.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dk_emit_program.out \
  /tmp/hakorune_p381dk_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dk_emit_program.out \
  /tmp/hakorune_p381dk_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dk_emit_const_instruction.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dk_emit_mir.out \
  /tmp/hakorune_p381dk_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dk_emit_mir.out \
  /tmp/hakorune_p381dk_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The active-body emit loop no longer owns constant classification, publication,
or unsupported-shape reporting inline.
