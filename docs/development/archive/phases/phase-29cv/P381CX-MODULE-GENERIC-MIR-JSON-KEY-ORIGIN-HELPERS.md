# P381CX Module Generic MIR JSON Key Origin Helpers

Date: 2026-05-05
Scope: share MIR JSON field-key result-origin publication in the Stage0 module-generic emitter.

## Context

P381CU made method emitters consume one cached generic-method view per call, and
P381CV/P381CW split result publication into small local helpers. The remaining
MIR JSON `get` path still duplicated the same `key_const_text` checks in both
the call prepass and the actual `get` emitter.

Those checks are one local contract: specific MIR JSON field keys publish string,
array, or map origins for the resulting handle.

## Change

Added shared module-generic helpers for:

- `inst` field keys: `op`, `operation`, `op_kind`, `cmp`, and `value` publish
  `ORG_STRING`; `args` and `effects` publish `ORG_ARRAY_BIRTH`
- `function` field keys: `name` publishes `ORG_STRING`, `params`/`blocks`
  publish `ORG_ARRAY_BIRTH`, and `flags` publishes `ORG_MAP_BIRTH`
- `module` field keys: `functions` publishes `ORG_ARRAY_BIRTH`, and
  `functions_0` publishes `ORG_MAP_BIRTH`

The prepass keeps its existing fallback for unknown instruction fields by
publishing only `ORG_MAP_GET`. The actual emitter still publishes its broad
map-get origin before applying key-specific overrides.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cx_key_origin.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cx_key_origin.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cx_key_origin.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cx_emit_program.out \
  /tmp/hakorune_p381cx_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cx_emit_program.out \
  /tmp/hakorune_p381cx_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cx_key_origin.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cx_emit_mir.out \
  /tmp/hakorune_p381cx_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cx_emit_mir.out \
  /tmp/hakorune_p381cx_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0
- `current_state_pointer_guard.sh` passed
- `stage0_shape_inventory_guard.sh` passed

## Result

MIR JSON key-origin publication now has one module-generic helper surface shared
by prepass and emit-time origin updates. Route acceptance and numeric field
origin behavior are unchanged.
