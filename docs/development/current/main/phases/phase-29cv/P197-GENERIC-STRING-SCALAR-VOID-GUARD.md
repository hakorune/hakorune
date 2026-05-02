---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P197, generic string body scalar/null guard flow
Related:
  - docs/development/current/main/phases/phase-29cv/P183-GENERIC-I64-SCALAR-NULL-SENTINEL.md
  - docs/development/current/main/phases/phase-29cv/P196-MIR-SCHEMA-MAP-CONSTRUCTOR-BODY.md
  - lang/src/mir/builder/internal/lower_return_bool_box.hako
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/generic_string_facts.rs
---

# P197: Generic String Scalar/Void Guard

## Problem

P196 moved the stage1 source-execution probe to:

```text
target_shape_blocker_symbol=LowerReturnBoolBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`LowerReturnBoolBox.try_lower/1` is a string-or-null body. Its final success
path returns MIR JSON text, while miss paths return `null`.

The unsupported part is not the final string return. It is the internal scalar
parser guard:

```hako
local is_true = JsonFragBox.read_bool_after(s, k_val+8)
if is_true == null { return null }
local v = is_true == 1 ? 1 : 0
```

`JsonFragBox.read_bool_after/2` is already accepted by the generic i64 lane as a
scalar/null helper. The string body classifier still treats its result as plain
`I64`, so comparing that result with a void sentinel is rejected.

## Decision

Keep this in the existing generic string-or-void body because the outer body is
still a string-or-null lowerer. Do not introduce a `LowerReturnBoolBox` by-name
shape.

Add a structural value class:

```text
ScalarOrVoid
```

The generic string body may use it only for:

- direct `GenericI64Body` child calls whose target return type is `Unknown` or
  `Void`
- `== null` / `!= null` guard compares
- PHI values on the non-null branch, where the value becomes scalar `I64`

The shape must not:

- treat arbitrary scalar values as nullable
- treat scalar/null as string/null
- allow scalar/null returns from a string body
- add by-name matching for `LowerReturnBoolBox`

## Acceptance

```bash
cargo test -q scalar_void_guard --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p197_scalar_void_guard_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Implementation

- Added `GenericPureValueClass::ScalarOrVoid`.
- Classified `GenericI64Body` child calls with `Unknown`/`Void` target return
  types as scalar/null values inside generic string bodies.
- Allowed `ScalarOrVoid == null` and `ScalarOrVoid != null` guards.
- Reused the existing non-null guard PHI analysis so guarded PHI values become
  scalar `I64` before later scalar compares.

## Probe Result

P197 removes the previous blocker:

```text
target_shape_blocker_symbol=LowerReturnBoolBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The stage1 `--emit-exe` probe now fails later at:

```text
target_shape_blocker_symbol=BuilderDelegateProviderBox.try_emit/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```
