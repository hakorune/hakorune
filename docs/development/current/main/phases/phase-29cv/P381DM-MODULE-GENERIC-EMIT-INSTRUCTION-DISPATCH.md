# P381DM Module Generic Emit Instruction Dispatch

Date: 2026-05-05
Scope: collapse module-generic active-body instruction emission into one dispatcher.

## Context

P381DG through P381DL moved the module-generic prepass and emit call-family,
constant, return, and terminator details behind named helper entries. The
active-body emit loop still owned the opcode if-chain itself.

That loop should only own block traversal and PHI emission. Per-instruction
behavior belongs behind one dispatcher before the uniform multi-function MIR
emitter can replace capsule-specific routing.

## Change

Added `module_generic_string_emit_instruction(...)` and moved active-body
opcode dispatch into it.

The active-body loop now:

1. reads each block
2. emits the block label
3. emits PHI records for that block
4. delegates each instruction to the dispatcher

The dispatcher preserves the existing opcode order and helper calls:
`phi`, `const`, `copy`, `newbox`, `compare`, `unop`, `select`, `binop`,
`branch`, `jump`, `ret`, `mir_call`/`call`/`boxcall`, `externcall`,
`keepalive`, and `release_strong`.

No opcode acceptance, fallback behavior, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dm_emit_instruction_dispatch.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dm_emit_instruction_dispatch.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dm_emit_instruction_dispatch.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dm_emit_program.out \
  /tmp/hakorune_p381dm_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dm_emit_program.out \
  /tmp/hakorune_p381dm_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dm_emit_instruction_dispatch.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dm_emit_mir.out \
  /tmp/hakorune_p381dm_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dm_emit_mir.out \
  /tmp/hakorune_p381dm_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic active-body emitter is now a thin block/PHI traversal loop
with one instruction dispatcher entry.
