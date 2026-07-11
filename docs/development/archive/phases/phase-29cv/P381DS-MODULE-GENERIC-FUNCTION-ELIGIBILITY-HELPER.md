# P381DS Module Generic Function Eligibility Helper

Date: 2026-05-05
Scope: move module-generic function definition eligibility behind one helper.

## Context

P381DR moved active-context prepass/body emission into a pipeline helper. The
function definition shell still owned eligibility checks inline before reading
the generic pure function view.

Those checks are the definition-entry acceptance contract.

## Change

Added `module_generic_string_function_definition_is_eligible(...)` and moved the
existing definition-entry checks into it.

The helper preserves existing rejection behavior:

- missing/unsafe symbol names are skipped with rc `0`
- symbols not planned for module-generic emission are skipped with rc `0`
- already-emitted symbols are skipped with rc `0`
- numeric leaf functions owned by the leaf emitter are skipped with rc `0`

No view parsing, route acceptance, or generated C changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381ds_function_eligibility_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381ds_function_eligibility_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ds_function_eligibility_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381ds_emit_program.out \
  /tmp/hakorune_p381ds_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381ds_emit_program.out \
  /tmp/hakorune_p381ds_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ds_function_eligibility_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381ds_emit_mir.out \
  /tmp/hakorune_p381ds_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381ds_emit_mir.out \
  /tmp/hakorune_p381ds_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic function definition shell no longer owns eligibility checks
inline.
