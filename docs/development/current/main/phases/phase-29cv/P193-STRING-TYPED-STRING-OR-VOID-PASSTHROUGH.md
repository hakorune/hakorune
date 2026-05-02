---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P193, string-typed string-or-void wrapper passthrough
Related:
  - docs/development/current/main/phases/phase-29cv/P191-UNKNOWN-RETURN-STRING-OR-VOID-WRAPPER.md
  - docs/development/current/main/phases/phase-29cv/P192-HOSTBRIDGE-EXTERN-INVOKE-ROUTE-FACT.md
  - src/mir/global_call_route_plan/string_return_profile.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P193: String-Typed String-Or-Void Passthrough

## Problem

P192 moved the source-execution probe to:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox._methodize_if_enabled_checked/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The source shape is a checked wrapper:

```hako
if BuilderConfigBox.methodize_on() != 1 { return m }
local result = FuncLoweringBox.methodize_calls_in_mir(m)
if result != null { return result }
me.log_fail("methodize_null")
return null
```

This is not a new collection/body language. It is the same string-or-void
sentinel wrapper family as P191, but with a string-typed return boundary and an
unknown input passthrough branch.

## Decision

Keep the body shape as `generic_string_or_void_sentinel_body`.

The return-profile classifier may treat an unknown parameter passthrough as a
string-like return when the function return ABI is string-compatible, but only
as passthrough evidence. If the string-compatible return relies on unknown
passthrough, a concrete string return must also exist in the same body.

This preserves P191's guard:

```text
unknown/string-compatible passthrough alone + void sentinel is not enough
concrete string child/constant/substr evidence + void sentinel is required
```

## Forbidden

- adding a by-name rule for `BuilderFinalizeChainBox`
- treating every unknown param-or-null wrapper as a concrete string body
- moving normalizer/collection semantics back into `generic_string_body`
- backend-local reclassification in C shims

## Acceptance

```bash
cargo test -q string_typed_string_or_void --lib
cargo test -q void_sentinel --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p193_string_typed_string_or_void_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Probe Result

Observed on 2026-05-02:

```text
BuilderFinalizeChainBox._methodize_if_enabled_checked/1
  target_shape=generic_string_or_void_sentinel_body
  proof=typed_global_call_generic_string_or_void_sentinel

target_shape_blocker_symbol=BuilderRegistryAuthorityBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

This confirms that the methodize wrapper blocker from P192 is closed. The next
card should inspect the registry authority body without adding by-name
acceptance.
