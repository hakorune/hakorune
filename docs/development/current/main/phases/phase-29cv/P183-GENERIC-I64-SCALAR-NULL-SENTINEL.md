---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P183, generic i64 scalar/null sentinel flow
Related:
  - docs/development/current/main/phases/phase-29cv/P180-GENERIC-I64-BOOL-SCALAR-FLOW.md
  - docs/development/current/main/phases/phase-29cv/P182C-JSONFRAG-NORMALIZER-COLLECTION-ROUTE-FACTS.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - src/mir/global_call_route_plan/tests/generic_i64.rs
  - src/mir/global_call_route_plan/tests/void_sentinel.rs
---

# P183: Generic I64 Scalar/Null Sentinel

## Problem

After P182c, `JsonFragNormalizerBox._normalize_instructions_array/1` still had
unsupported child calls through the integer JSON helper path:

```text
JsonFragBox.get_int/2
  -> JsonFragBox.read_int_after/2
  -> JsonFragBox.read_int_from/2
```

There are two different shapes in that path:

- `read_int_from/2` returns digit text or null, but its function type can be
  encoded as `i64` at the ABI layer.
- `get_int/2` returns scalar i64 or null, and callers compare the result with
  null.

Treating both as generic string bodies is wrong. Treating a null-only body as a
scalar body is also wrong.

## Decision

Keep the split structural:

- `string_or_void_sentinel_return_type_candidate()` includes `i64` ABI return
  types, but the existing return-value profile must still prove string-or-null.
- `generic_i64_body` accepts `void`/`null` returns only when the body also has a
  real scalar return. The null arm is the ABI zero sentinel.
- Null-only bodies remain unsupported and keep the existing
  `generic_string_unsupported_void_sentinel_const` contract.

This keeps `read_int_from/2` in the string-or-void lane and `get_int/2` in the
generic i64 lane.

## Result

The real `stage1_cli_env.hako` lowering plan now shows:

```text
JsonFragBox.read_int_after/2 -> generic_string_or_void_sentinel_body
JsonFragBox.get_int/2       -> generic_i64_body
JsonFragNormalizerBox._const_value_sig/1 -> generic_pure_string_body
```

Inside `_normalize_instructions_array/1`, the previous unsupported child calls
are now direct:

```text
b8392.i4 JsonFragBox.get_int/2 DirectAbi generic_i64_body ScalarI64
b8392.i8 JsonFragNormalizerBox._const_value_sig/1 DirectAbi generic_pure_string_body string_handle
```

The remaining blocker is the planned shape itself:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

That is the P184 C emitter/direct-shape wiring task.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_integer_typed_string_or_void_sentinel_body --lib
cargo test -q refresh_module_global_call_routes_accepts_i64_or_null_return_as_zero_sentinel --lib
cargo test -q refresh_module_global_call_routes_accepts_void_typed_i64_or_null_return_as_zero_sentinel --lib
cargo test -q global_call_routes --lib
cargo test -q generic_i64 --lib
cargo test -q void_sentinel --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p183_scalar_null_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
