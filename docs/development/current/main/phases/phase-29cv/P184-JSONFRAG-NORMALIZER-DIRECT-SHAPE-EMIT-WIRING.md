---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P184, JsonFrag normalizer DirectAbi shape and module emit wiring
Related:
  - docs/development/current/main/phases/phase-29cv/P182A-GENERIC-STRING-CLASSIFIER-BOUNDARY.md
  - docs/development/current/main/phases/phase-29cv/P182B-JSONFRAG-INSTRUCTION-ARRAY-NORMALIZER-SHAPE.md
  - docs/development/current/main/phases/phase-29cv/P182C-JSONFRAG-NORMALIZER-COLLECTION-ROUTE-FACTS.md
  - docs/development/current/main/phases/phase-29cv/P183-GENERIC-I64-SCALAR-NULL-SENTINEL.md
  - src/mir/global_call_route_plan/model.rs
  - src/mir/global_call_route_plan/jsonfrag_normalizer_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P184: JsonFrag Normalizer Direct Shape Emit Wiring

## Problem

P183 removed the scalar/null child blockers inside
`JsonFragNormalizerBox._normalize_instructions_array/1`, but the planned
normalizer shape still stopped at the backend boundary:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

The important constraint was to avoid putting normalizer collection semantics
back into `generic_string_body.rs`.

## Decision

Promote the dedicated shape as a DirectAbi global-call target:

```text
target_shape=jsonfrag_instruction_array_normalizer_body
proof=typed_global_call_jsonfrag_instruction_array_normalizer
return_shape=string_handle
value_demand=runtime_i64_or_handle
```

The C shim consumes this through LoweringPlan metadata only. It does not
rediscover the normalizer body semantics.

Two adjacent route facts were needed to reach the next blocker without widening
the normalizer classifier:

- Treat `JsonFragInstructionArrayNormalizerBody` as a string-returning direct
  global target for caller dataflow.
- Accept the existing string surface form `indexOf(needle, start)` by routing
  it as `generic_method.indexOf` and emitting it as
  `substring(start, len).indexOf(needle) + start`.
- Accept void-typed unknown-param-or-void wrappers as
  `generic_string_or_void_sentinel_body` only for `void` return signatures.

## Result

The real `stage1_cli_env.hako` probe moved past:

```text
JsonFragNormalizerBox._normalize_instructions_array/1
JsonFragNormalizerBox._normalize_all_blocks/2
BuilderFinalizeChainBox._normalize_jsonfrag_if_enabled_checked/1
```

The remaining blocker is now a separate reverse string scan helper:

```text
target_shape_blocker_symbol=JsonFragBox.last_index_of_from/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

That is intentionally not added to `generic_string_body` as normalizer
semantics. It should be handled as the next string-scan surface blocker.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_jsonfrag_instruction_array_normalizer_shape --lib
cargo test -q refresh_module_semantic_metadata_accepts_string_indexof_in_generic_pure_string_body --lib
cargo test -q refresh_module_global_call_routes_accepts_void_typed_unknown_param_or_void_sentinel_body --lib
cargo test -q global_call_routes --lib
cargo test -q void_sentinel --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p184_jsonfrag_direct_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
