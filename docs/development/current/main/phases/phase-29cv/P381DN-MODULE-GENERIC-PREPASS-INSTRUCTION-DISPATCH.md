# P381DN Module Generic Prepass Instruction Dispatch

Date: 2026-05-05
Scope: collapse module-generic active-function prepass handling into one dispatcher.

## Context

P381DM made emit-side active-body traversal delegate per-instruction behavior
through one dispatcher. The prepass side still owned its opcode if-chain inside
`module_generic_string_prepass_active_function(...)`.

The prepass loop should own parameter setup, block traversal, final PHI
refinement, and capacity checks. Per-instruction fact collection belongs behind
one dispatcher.

## Change

Added `module_generic_string_prepass_instruction(...)` and moved active-function
opcode dispatch into it.

The active-function prepass now:

1. publishes parameter origins
2. walks blocks and instructions
3. delegates each instruction to the dispatcher
4. refines PHI types
5. checks generic capacity

The dispatcher preserves the existing opcode behavior and order for `const`,
`copy`, `newbox`, `compare`, `unop`, `phi`, `binop`,
`mir_call`/`call`/`boxcall`, `externcall`, `select`, `branch`, `jump`, `ret`,
`keepalive`, and `release_strong`.

No prepass acceptance, origin publication, type publication, or generated C
behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dn_prepass_instruction_dispatch.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dn_prepass_instruction_dispatch.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dn_prepass_instruction_dispatch.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dn_emit_program.out \
  /tmp/hakorune_p381dn_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dn_emit_program.out \
  /tmp/hakorune_p381dn_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dn_prepass_instruction_dispatch.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dn_emit_mir.out \
  /tmp/hakorune_p381dn_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dn_emit_mir.out \
  /tmp/hakorune_p381dn_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Module-generic prepass and emit now both use instruction dispatcher entries
from their active function/body traversal loops.
