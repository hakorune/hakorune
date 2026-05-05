# P381DA Module Generic Prepass Method Helper

Date: 2026-05-05
Scope: move module-generic method prepass routing behind one helper.

## Context

P381CZ made the call prepass reuse the module-generic `get` route view, but the
main prepass loop still owned the remaining generic-method route chain directly.
That made the loop responsible for method-route acceptance, provisional type
publication, array-push origin promotion, and the malformed-argument fail-fast
case.

Those responsibilities belong to one local prepass helper.

## Change

Added `module_generic_string_prepass_generic_method_view(...)` and moved the
generic-method prepass route handling into it.

The helper returns:

- `1` when a method route was accepted and prepass facts were published
- `0` when the generic-method view does not belong to this helper
- `-1` when an accepted route is malformed and the caller must fail fast

The main prepass loop now keeps only method birth handling, then delegates
generic-method facts to this helper before trying extern/global routes.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381da_prepass_method_helper.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381da_prepass_method_helper.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381da_prepass_method_helper.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381da_emit_program.out \
  /tmp/hakorune_p381da_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381da_emit_program.out \
  /tmp/hakorune_p381da_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381da_prepass_method_helper.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381da_emit_mir.out \
  /tmp/hakorune_p381da_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381da_emit_mir.out \
  /tmp/hakorune_p381da_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic prepass loop now has one helper for generic-method route
facts, keeping the main loop focused on dispatching between method, extern, and
global route families.
