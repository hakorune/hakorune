# P381CG Global Call Result Origin Metadata

Date: 2026-05-05
Scope: publish direct global-call result-origin facts from Rust LoweringPlan JSON and consume them in Stage0 metadata views.

## Context

P381CF centralized Stage0 result-origin propagation behind a shared
LoweringPlan view helper, but that helper still reconstructed the origin from
proof names. That kept a small capsule-specific proof list alive in C even
after the retired target-shape variants had moved to Rust route facts.

The next cleanup step is to make the origin an explicit MIR-owned route fact.

## Change

Rust now publishes `result_origin` for every global-call route in both
LoweringPlan JSON surfaces:

- `"string"` for pure string, string-or-void sentinel, and parser Program(JSON)
  direct ABI calls
- `"array_string_birth"` for static string array direct ABI calls
- `"map_birth"` for MIR schema map constructors and BoxTypeInspector describe
  direct ABI calls
- `"none"` for scalar, void, mixed, or unsupported routes

Stage0 now reads `result_origin` from the LoweringPlan global-call view and
maps only that field to the existing origin enum:

- `"string"` -> `ORG_STRING`
- `"array_string_birth"` -> `ORG_ARRAY_STRING_BIRTH`
- `"map_birth"` -> `ORG_MAP_BIRTH`
- `"none"` or missing -> `ORG_NONE`

The C helper no longer rediscovers result-origin behavior from proof names.

## Tests

Added runner MIR JSON assertions for origin-carrying and non-origin routes:

- parser Program(JSON): `result_origin=string`
- static string array: `result_origin=array_string_birth`
- string-or-void sentinel: `result_origin=string`
- generic pure string: `result_origin=string`
- unsupported and generic-i64 scalar routes: `result_origin=none`

## Verification

Commands:

```bash
cargo test --release global_call_routes --lib -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
cargo build --release --bin hakorune
target/release/hakorune \
  --emit-mir-json /tmp/hakorune_p381cg_stage1_cli_env_rust.mir.json \
  --backend mir \
  lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_p381cg_stage1_cli_env.ll \
  target/release/ny-llvmc \
  --in /tmp/hakorune_p381cg_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cg_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cg_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cg_stage1_cli_env.exe
```

Runtime sanity used the produced Stage0 EXE directly:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cg_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cg_emit_program.out \
  /tmp/hakorune_p381cg_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cg_emit_program.out \
  /tmp/hakorune_p381cg_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cg_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cg_emit_mir.out \
  /tmp/hakorune_p381cg_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cg_emit_mir.out \
  /tmp/hakorune_p381cg_emit_mir.err
```

Observed:

- Rust route tests passed: 124 passed, 0 failed
- C shim build passed
- `hakorune` release build passed
- fresh Stage1 MIR JSON contains `result_origin`
- OBJ generation from the fresh MIR JSON passed
- EXE generation from the fresh MIR JSON passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Result-origin truth now lives in the Rust global-call route plan and is emitted
as explicit LoweringPlan metadata. Stage0 consumes that metadata as a typed
view fact instead of keeping proof-name origin branches.

The remaining shared module-generic helper still contains the direct ABI
definition ownership list. That is the next metadata candidate before deleting
more body-emitter surface.
