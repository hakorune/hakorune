# P381DO Module Generic Function Signature Helper

Date: 2026-05-05
Scope: move module-generic function signature emission behind one helper.

## Context

P381DM and P381DN made emit and prepass active loops delegate
per-instruction behavior through dispatchers. The function definition shell
still owned LLVM function signature emission inline.

The uniform multi-function emitter needs the definition shell to expose named
steps instead of mixing setup, prepass, header emission, body emission, and
cleanup in one function.

## Change

Added `module_generic_string_emit_function_signature(...)` and moved
`define i64 @"name"(...` emission into it.

The helper preserves existing parameter behavior:

- reads `params` from the function JSON
- treats missing or non-array params as arity 0
- prints each parameter as `i64 %rN`
- opens the function body with `{`

No signature text, route acceptance, or generated C behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381do_function_signature_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381do_function_signature_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381do_function_signature_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381do_emit_program.out \
  /tmp/hakorune_p381do_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381do_emit_program.out \
  /tmp/hakorune_p381do_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381do_function_signature_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381do_emit_mir.out \
  /tmp/hakorune_p381do_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381do_emit_mir.out \
  /tmp/hakorune_p381do_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic function definition shell no longer owns signature emission
inline.
