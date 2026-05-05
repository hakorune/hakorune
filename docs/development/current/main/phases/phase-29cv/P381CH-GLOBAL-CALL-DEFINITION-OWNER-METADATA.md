# P381CH Global Call Definition Owner Metadata

Date: 2026-05-05
Scope: publish direct global-call definition ownership and emit trace consumer facts from Rust LoweringPlan JSON.

## Context

P381CG moved result-origin truth out of C proof-name reconstruction and into
Rust LoweringPlan metadata. The next C-side list was definition ownership:
Stage0 still decided whether a global call belonged to the leaf, generic-i64,
module-generic, or future uniform-MIR definition path by rechecking proof names.

That kept retired capsules visible in selected-set planning and route trace
selection even though the route plan already knew the answer.

## Change

Rust now publishes two explicit global-call route fields:

- `definition_owner`
- `emit_trace_consumer`

The owner vocabulary is:

- `leaf_i64`: numeric leaf direct ABI body
- `generic_i64_or_leaf`: generic-i64 direct ABI body, with the existing leaf
  fallback if the target function is a numeric leaf
- `module_generic`: module-generic direct ABI body
- `uniform_mir`: unsupported route whose blocker is the missing uniform MIR
  multi-function emitter
- `none`: unknown or unsupported route with no body definition owner

Stage0 now reads these fields from the LoweringPlan global-call view. The
shared selected-set helper uses `definition_owner` instead of a capsule proof
list, and route tracing uses `emit_trace_consumer` directly.

## Tests

Runner MIR JSON tests now assert owner/consumer metadata for:

- unsupported routes: `none` / `mir_call_global_unknown_emit`
- missing uniform emitter routes: `uniform_mir` /
  `mir_call_global_uniform_mir_emit`
- numeric leaf routes: `leaf_i64` / `mir_call_global_leaf_emit`
- generic-i64 routes: `generic_i64_or_leaf` /
  `mir_call_global_generic_i64_emit`
- module-generic routes: `module_generic` /
  `mir_call_global_module_generic_emit`

## Verification

Commands:

```bash
cargo test --release global_call_routes --lib -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
cargo build --release --bin hakorune
target/release/hakorune \
  --emit-mir-json /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --backend mir \
  lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_p381ch_stage1_cli_env.ll \
  target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381ch_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381ch_stage1_cli_env.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ch_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381ch_emit_program.out \
  /tmp/hakorune_p381ch_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381ch_emit_program.out \
  /tmp/hakorune_p381ch_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ch_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381ch_emit_mir.out \
  /tmp/hakorune_p381ch_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381ch_emit_mir.out \
  /tmp/hakorune_p381ch_emit_mir.err
```

Observed:

- Rust route tests passed: 124 passed, 0 failed
- C shim build passed
- `hakorune` release build passed
- fresh Stage1 MIR JSON contains `definition_owner` and
  `emit_trace_consumer`
- OBJ generation from the fresh MIR JSON passed
- EXE generation from the fresh MIR JSON passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Stage0 no longer uses a capsule proof-name list to decide the module-generic
definition set or emit trace consumer. The remaining proof predicates in
`hako_llvmc_ffi_lowering_plan_metadata.inc` are narrow route-contract
validators, not selected-set ownership truth.

P381CI follows this by deleting the retired-capsule C direct-view predicates
that became unused after this metadata move.
