---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P245a, string-or-void child return profile passthrough
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P244A-CLI-RUN-SHAPE-SCANNER-VOID.md
  - src/mir/global_call_route_plan/string_return_profile.rs
---

# P245a: String-Or-Void Child Return Profile

## Problem

P244a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerLoopLocalReturnVarBox._read_step_int/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`_read_step_int/4` is a wrapper that returns null on no-match and otherwise
returns the result of `_read_int_field_in_range/4`. The child target is already
classified as `GenericStringOrVoidSentinelBody`.

The return-profile analyzer treats that child as `StringOrVoid`, but for
unknown return-type wrappers it still requires a separate concrete string
return. That blocks wrappers whose only string evidence is an already-proven
string-or-void child.

## Decision

Do not add a new body shape and do not add source workaround.

Treat a proven `StringOrVoid` child return as concrete string-or-void evidence
for the parent return profile. This remains value-level route fact propagation:
the child target must already be classified.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new C emitter
- no generic MapBox/Object semantics
- no by-name source exception
- no change to loop local return-var pattern acceptance

## Acceptance

```bash
cargo test -q global_call_route_plan::tests::void_sentinel --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p245a_string_or_void_child_return_profile.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`LowerLoopLocalReturnVarBox._read_step_int/4`; a later blocker may remain.

## Result

Observed probe:

```text
target_shape=generic_string_or_void_sentinel_body
target_symbol=LowerLoopLocalReturnVarBox._read_step_int/4
return_shape=string_handle_or_null
tier=DirectAbi
```

The next source-exe frontier is:

```text
target_shape_blocker_symbol=DefsScannerBox.find_defs_bounds/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```
