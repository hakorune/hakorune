# P381CZ Module Generic Prepass Get Route View

Date: 2026-05-05
Scope: reuse the module-generic `get` route view in the call prepass.

## Context

P381CY moved the actual `get` emitter to a local route-view helper. The call
prepass still repeated the same MIR JSON `get` route predicate chain only to
publish provisional result type and origin facts.

The prepass has one extra nuance: numeric value fields stay scalar-only there,
while several map-like fields publish `ORG_MAP_GET`.

## Change

Extended `ModuleGenericStringGetRouteView` with
`prepass_publishes_map_get_origin` and rewired the call prepass to consume the
same route view as the emitter.

The prepass now keeps only route-specific result handling:

- all accepted `get` routes publish `T_I64`
- module-function array item publishes `ORG_MAP_BIRTH`
- PHI incoming array item publishes `ORG_ARRAY_BIRTH`
- map-like prepass routes publish `ORG_MAP_GET`
- inst/function/module field-key origins reuse the P381CX key-origin helpers

Numeric value field behavior is unchanged: it publishes only `T_I64` during
prepass.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cz_prepass_get_route_view.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cz_prepass_get_route_view.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cz_prepass_get_route_view.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cz_emit_program.out \
  /tmp/hakorune_p381cz_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cz_emit_program.out \
  /tmp/hakorune_p381cz_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cz_prepass_get_route_view.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cz_emit_mir.out \
  /tmp/hakorune_p381cz_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cz_emit_mir.out \
  /tmp/hakorune_p381cz_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic prepass and actual `get` emitter now share one route view for
MIR JSON `get` routes. The prepass-specific numeric-field origin exception is
explicit on that view.
