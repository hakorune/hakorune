# P381DI Module Generic Emit Externcall Helper

Date: 2026-05-05
Scope: move module-generic emit `externcall` handling behind one helper.

## Context

P381DH aligned call-instruction helper vocabulary between prepass and emit.
The active-body emit loop still owned the `externcall` lowering-plan dispatch
inline.

That left one call-family branch with local payload reads and rc handling in
the loop.

## Change

Added `module_generic_string_emit_externcall_instruction(...)` and moved the
`externcall` emit handling into it.

The helper still delegates to `emit_extern_call_lowering_plan_mir_call(...)`
and only accepts `rc == 1`. No route acceptance, fallback behavior, or
generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381di_emit_externcall_instruction.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381di_emit_externcall_instruction.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381di_emit_externcall_instruction.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381di_emit_program.out \
  /tmp/hakorune_p381di_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381di_emit_program.out \
  /tmp/hakorune_p381di_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381di_emit_externcall_instruction.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381di_emit_mir.out \
  /tmp/hakorune_p381di_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381di_emit_mir.out \
  /tmp/hakorune_p381di_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The active-body emit loop no longer owns `externcall` lowering-plan dispatch
inline.
