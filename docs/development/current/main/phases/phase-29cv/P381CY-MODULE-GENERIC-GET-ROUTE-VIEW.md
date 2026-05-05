# P381CY Module Generic Get Route View

Date: 2026-05-05
Scope: collect module-generic `get` route flags behind one local route view.

## Context

P381CX shared MIR JSON key-origin publication between the prepass and the actual
`get` emitter. The emitter body still had a long list of local boolean flags
that served three separate decisions:

- whether the method route is accepted by this emitter
- whether the helper call should use array slot load or map get
- which result-origin updates apply after the call

Those decisions are one route view over the cached LoweringPlan generic-method
metadata.

## Change

Added `ModuleGenericStringGetRouteView` plus
`module_generic_string_read_get_route_view(...)`.

The helper now reads the existing route predicates once and exposes only the
facts the emitter needs:

- `is_array_load`
- module-function array item origin
- PHI incoming array item origin
- broad map-get origin publication
- inst/function/module field-key override flags

The actual `get` emitter now consumes that view instead of maintaining the full
predicate list inline.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cy_get_route_view.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cy_get_route_view.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cy_get_route_view.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cy_emit_program.out \
  /tmp/hakorune_p381cy_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cy_emit_program.out \
  /tmp/hakorune_p381cy_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cy_get_route_view.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cy_emit_mir.out \
  /tmp/hakorune_p381cy_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cy_emit_mir.out \
  /tmp/hakorune_p381cy_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The module-generic `get` emitter now has one local route-view helper for
accepted route shape, helper selection, and result-origin follow-up. No route
acceptance or origin behavior changed.
