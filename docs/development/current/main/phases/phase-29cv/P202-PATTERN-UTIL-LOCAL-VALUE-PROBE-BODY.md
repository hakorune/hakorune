---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P202, PatternUtil local numeric/bool value probe body shape
Related:
  - docs/development/current/main/phases/phase-29cv/P201-BOX-TYPE-INSPECTOR-DESCRIBE-BODY.md
  - docs/development/current/main/phases/phase-29cv/P183-GENERIC-I64-SCALAR-NULL-SENTINEL.md
  - lang/src/mir/builder/internal/pattern_util_box.hako
  - src/mir/global_call_route_plan/model.rs
---

# P202: PatternUtil Local Value Probe Body

## Problem

P201 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=PatternUtilBox.find_local_bool_before/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

`find_local_bool_before/3` and its child `find_local_int_before/3` are not
string bodies. They scan MIR JSON for the last local value before a position and
return a nullable runtime value:

```text
read_int_after/read_bool_after result -> string handle or scalar i64
arithmetic/compare result            -> scalar i64
miss                                 -> null/zero sentinel
```

Forcing this into `generic_string_body.rs` would extend the string classifier
with PatternUtil-specific body understanding. It is also not a pure
`GenericI64Body`, because `JsonFragBox.read_int_after/2` returns digit text.

## Decision

Introduce a dedicated shape:

```text
target_shape=pattern_util_local_value_probe_body
proof=typed_global_call_pattern_util_local_value_probe
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
```

The existing generic function emitter still emits the body as an `i64` ABI
function. The new shape only records the route contract so MIR owns the
eligibility and C shims do not rediscover body meaning.

## Boundary

The classifier may observe:

- three ABI-compatible parameters
- Local JSON scan markers such as `"type":"Local"` and `"name":"`
- int/bool expression markers such as `"expr":{"type":"Int"` or
  `"expr":{"type":"Bool"`
- `JsonFragBox.index_of_from/3` and `JsonFragBox.read_string_after/2`
- `JsonFragBox.read_int_after/2` and optionally `JsonFragBox.read_bool_after/2`
- null/void sentinel returns
- recursive or child local-value probe calls
- scalar coercion through an already-classified i64 helper
- scalar arithmetic/compare flow

The shape must not:

- extend `generic_string_body.rs`
- treat arbitrary mixed string/scalar helpers as PatternUtil probes
- require the callee function name to be `PatternUtilBox.*`
- expose the result as a pure string handle or pure scalar fact

## Implementation

- Added `GlobalCallTargetShape::PatternUtilLocalValueProbeBody`.
- Added `src/mir/global_call_route_plan/pattern_util_local_value_probe_body.rs`.
- The classifier recognizes the Local scan surface plus either:
  - int probe flow with self-recursive local-value lookup and scalar coercion
  - bool probe flow with `read_bool_after`, child local-value probes, and scalar
    compare/coercion
- The route metadata exposes `mixed_runtime_i64_or_handle` so callers do not
  treat the result as a pure string handle or pure scalar.
- The C shim reuses the existing same-module generic function emitter and only
  adds LoweringPlan validation/registration for the new direct shape.

## Probe Result

P202 removes the previous blocker:

```text
target_shape_blocker_symbol=PatternUtilBox.find_local_bool_before/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

The source-execution probe now reaches:

```text
target_shape_blocker_symbol=MirSchemaBox.module/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q pattern_util_local_value_probe --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p202_pattern_util_local_value_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
