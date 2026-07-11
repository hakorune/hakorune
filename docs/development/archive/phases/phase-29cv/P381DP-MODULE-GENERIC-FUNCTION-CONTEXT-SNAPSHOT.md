# P381DP Module Generic Function Context Snapshot

Date: 2026-05-05
Scope: centralize module-generic function context save and restore.

## Context

P381DO moved function signature emission behind one helper. The function
definition shell still saved and restored active function state through many
local variables and assignment statements.

That made the temporal context contract implicit: save all active function
fields before activating the module-generic function, then restore all of them
after success or failure.

## Change

Added `ModuleGenericStringFunctionContextSnapshot` plus:

- `module_generic_string_save_function_context(...)`
- `module_generic_string_restore_function_context(...)`

The restore helper preserves the existing cleanup order:

1. free owned string constants
2. reset string concat tracking
3. restore lowering state
4. restore active function JSON/view pointers
5. restore block count and rune selection

No activation behavior, cleanup behavior, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dp_function_context_snapshot.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dp_function_context_snapshot.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dp_function_context_snapshot.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dp_emit_program.out \
  /tmp/hakorune_p381dp_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dp_emit_program.out \
  /tmp/hakorune_p381dp_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dp_function_context_snapshot.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dp_emit_mir.out \
  /tmp/hakorune_p381dp_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dp_emit_mir.out \
  /tmp/hakorune_p381dp_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Module-generic function context save/restore is now one named contract instead
of a scattered local variable list.
