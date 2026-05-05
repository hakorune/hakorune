# P381CS Module Generic Ownerless View Helper

Date: 2026-05-05
Scope: share ownerless MIR JSON generic-method view matching in the Stage0 module-generic emitter.

## Context

`hako_llvmc_ffi_module_generic_string_method_views.inc` had two local patterns
for generic-method view checks:

- receiver-owned direct routes
- ownerless MIR JSON array/map/key routes

Both repeated the same route/core/kind/tier tuple checks, and the ownerless MIR
JSON routes also duplicated the no-receiver, shape, demand, and publication
checks across array item, map field, and keys helpers.

## Change

`module_generic_string_method_view_is_direct_receiver_with` now delegates the
shared route/core/kind/tier comparison to
`lowering_plan_generic_method_view_matches_route_tuple`.

Added `module_generic_string_method_view_is_ownerless_with` and rewired:

- MIR JSON array-item view matching
- MIR JSON map-field view matching
- MIR JSON flags keys view matching

The route-specific helpers still own their proof names and key allowlists.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cs_ownerless_view.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cs_ownerless_view.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cs_ownerless_view.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cs_emit_program.out \
  /tmp/hakorune_p381cs_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cs_emit_program.out \
  /tmp/hakorune_p381cs_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cs_ownerless_view.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cs_emit_mir.out \
  /tmp/hakorune_p381cs_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cs_emit_mir.out \
  /tmp/hakorune_p381cs_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Ownerless MIR JSON generic-method view matching now has one local helper. The
direct receiver helper also shares the route tuple SSOT introduced in P381CR.
