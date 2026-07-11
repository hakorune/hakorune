# P381CI Global Call Retired C View Prune

Date: 2026-05-05
Scope: remove unused Stage0 direct global-call view predicates for retired capsule bodies.

## Context

P381CG moved result-origin truth into Rust LoweringPlan metadata, and P381CH
moved definition ownership plus emit trace consumer truth into the same route
metadata surface.

After that, several C-side direct global-call predicates no longer had any
consumer. They only repeated retired capsule proof/return checks and made the
metadata file look as if Stage0 still selected those capsules directly.

## Change

Removed unused `LoweringPlanGlobalCallView` predicates for:

- generic string module bodies
- parser Program(JSON)
- static string arrays
- MIR schema map constructors
- BoxTypeInspector describe bodies
- PatternUtil local-value probes

Kept the predicates still used by active call emission and selected-set
planning:

- numeric leaf
- generic-i64
- void-sentinel i64-zero call sites

## Verification

Commands:

```bash
rg -n \
  "lowering_plan_global_call_view_is_direct_(generic_string|parser_program_json|static_string_array|mir_schema_map_constructor|box_type_inspector_describe|pattern_util_local_value_probe)" \
  lang/c-abi/shims
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381ci_stage1_cli_env.o
```

Observed:

- the `rg` query found no remaining retired C view predicates
- C shim build passed
- OBJ generation from the P381CH fresh Stage1 MIR JSON passed

## Result

Stage0 no longer carries dead per-capsule C view predicates for the retired
direct global-call bodies. The remaining C global-call predicates are the
active ABI primitives that still have specialized emission behavior.
