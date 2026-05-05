# P381CF Global Call Module Generic SSOT

Date: 2026-05-05
Scope: consolidate Stage0 direct global-call module-generic planning, origin propagation, and emit tracing behind LoweringPlan view helpers.

## Context

P381CD and P381CE removed parser Program(JSON) and static array body-specific
emitter paths. The remaining backend risk was duplication: selected-set
planning, prepass origin propagation, and MIR call emission each still repeated
the same proof-specific direct-call list.

That made every retired capsule look alive at each call site even though the
real truth was already the stored LoweringPlan proof and return contract.

## Change

Added shared LoweringPlan view helpers in
`hako_llvmc_ffi_lowering_plan_metadata.inc`:

- `lowering_plan_global_call_view_uses_module_generic_definition`
- `lowering_plan_global_call_view_requires_module_generic_definition`
- `lowering_plan_global_call_view_result_origin_kind`
- `lowering_plan_global_call_view_emit_trace_consumer`

Then rewired the Stage0 call sites:

- selected-set planning now asks the shared module-generic helper instead of
  listing parser/static/schema/box/pattern proof names locally
- module-generic prepass sets `T_I64` and result origin through the shared
  result-origin helper
- MIR call emission keeps only the numeric-leaf and generic-i64 owner exception,
  then uses the shared module-generic helper for every other direct ABI module
  function
- route tracing now reports the common
  `consumer=mir_call_global_module_generic_emit` for module-generic direct ABI
  bodies

The result-origin behavior is unchanged:

- `string_handle` and `string_handle_or_null`: `ORG_STRING`
- static-array `array_handle`: `ORG_ARRAY_STRING_BIRTH`
- schema/describe `map_handle`: `ORG_MAP_BIRTH`
- mixed/scalar returns: no origin side effect

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_p381cf_stage1_cli_env.ll \
  target/release/ny-llvmc \
  --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cf_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cf_stage1_cli_env.exe
```

Runtime sanity used the Stage1 env runner directly because the produced EXE is
not a Rust CLI artifact:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cf_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cf_emit_program.out \
  /tmp/hakorune_p381cf_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cf_emit_program.out \
  /tmp/hakorune_p381cf_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cf_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cf_emit_mir.out \
  /tmp/hakorune_p381cf_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cf_emit_mir.out \
  /tmp/hakorune_p381cf_emit_mir.err
```

Observed:

- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0
- route trace contains `consumer=mir_call_global_module_generic_emit` for the
  direct ABI module-generic calls

## Result

The call-site truth for retired direct global-call capsules is now centralized
behind LoweringPlan view helpers. Capsule-specific proof predicates still exist
as metadata readers, but selected-set planning, prepass origin propagation, and
MIR call emission no longer each carry their own capsule list.

