# P381DH Module Generic Emit Call Instruction Helper

Date: 2026-05-05
Scope: align module-generic emit call-instruction helper naming with prepass.

## Context

P381DG made the prepass side use
`module_generic_string_prepass_call_instruction(...)` for
`mir_call`/`call`/`boxcall` handling. The emit side already had the same
single-entry shape, but the helper was still named
`module_generic_string_emit_mir_call(...)`.

That name was narrower than the contract: the helper accepts all call
instruction spellings, not just `mir_call`.

## Change

Renamed the emit-side helper to
`module_generic_string_emit_call_instruction(...)` and updated the active body
call site.

No route acceptance, helper ordering, or generated C behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381dh_emit_call_instruction.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381dh_emit_call_instruction.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dh_emit_call_instruction.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381dh_emit_program.out \
  /tmp/hakorune_p381dh_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381dh_emit_program.out \
  /tmp/hakorune_p381dh_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381dh_emit_call_instruction.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381dh_emit_mir.out \
  /tmp/hakorune_p381dh_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381dh_emit_mir.out \
  /tmp/hakorune_p381dh_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Prepass and emit now expose matching call-instruction helper vocabulary for the
module-generic path.
