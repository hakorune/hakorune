# P381DG Module Generic Prepass Call Instruction Helper

Date: 2026-05-05
Scope: move module-generic prepass call-instruction handling behind one helper.

## Context

P381DA and P381DF split method, extern, and global prepass families into local
helpers. The prepass loop still owned the `mir_call`/`call`/`boxcall` payload
reading, LoweringPlan view reads, method birth handling, and family helper
ordering inline.

Those details are one call-instruction prepass contract.

## Change

Added `module_generic_string_prepass_call_instruction(...)` and moved
call-instruction prepass handling into it.

The helper preserves the existing order:

1. read call payload and LoweringPlan views
2. handle ArrayBox/MapBox method birth
3. try generic-method prepass facts
4. try extern-call prepass facts
5. try global-call prepass facts

The main prepass loop now delegates call instructions with one helper call.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dg_prepass_call_instruction.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dg_prepass_call_instruction.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dg_prepass_call_instruction.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dg_emit_program.out \
  /tmp/hakorune_p381dg_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dg_emit_program.out \
  /tmp/hakorune_p381dg_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dg_prepass_call_instruction.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dg_emit_mir.out \
  /tmp/hakorune_p381dg_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dg_emit_mir.out \
  /tmp/hakorune_p381dg_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic prepass loop no longer owns call-family payload/view routing
inline. Call-instruction prepass behavior now has one helper entry.
