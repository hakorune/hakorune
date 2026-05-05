# P381DL Module Generic Emit Control Terminator Helpers

Date: 2026-05-05
Scope: move module-generic emit `branch` and `jump` handling behind helpers.

## Context

P381DK moved `const` emission out of the active-body emit loop. The remaining
control-transfer terminators `branch` and `jump` still emitted LLVM text inline
from the loop.

Those direct branches are small, but they are terminator emission contracts and
belong behind named entries before the loop can become a uniform dispatcher.

## Change

Added:

- `module_generic_string_emit_branch_instruction(...)`
- `module_generic_string_emit_jump_instruction(...)`

Both helpers preserve the existing generated text and field reads. No route
acceptance, block selection, or generated C behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dl_emit_control_terminator_helpers.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dl_emit_control_terminator_helpers.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dl_emit_control_terminator_helpers.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dl_emit_program.out \
  /tmp/hakorune_p381dl_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dl_emit_program.out \
  /tmp/hakorune_p381dl_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dl_emit_control_terminator_helpers.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dl_emit_mir.out \
  /tmp/hakorune_p381dl_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dl_emit_mir.out \
  /tmp/hakorune_p381dl_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The active-body emit loop no longer owns `branch`/`jump` terminator emission
inline.
