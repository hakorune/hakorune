---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P149, global-call return-profile blocker propagation
Related:
  - docs/development/current/main/phases/phase-29cv/P148-GLOBAL-CALL-STRING-RETURN-PARAM-PASSTHROUGH.md
  - src/mir/global_call_route_plan/string_return_profile.rs
  - src/mir/global_call_route_plan/tests/void_sentinel.rs
  - lang/src/runner/stage1_cli_env.hako
---

# P149: Global-Call Void-Typed Blocker Propagation

## Problem

After P148, pure-first source execution advanced past
`Stage1ModeContractBox.resolve_mode/0`, but the next top-level stop was
misleading:

```text
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox.emit_program_from_source/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

`emit_program_from_source/2` was not blocked by a standalone ABI policy
decision. Its returned value came from a nested unresolved global call, but the
value also carried `void` type metadata from the null-sentinel path. The
return-profile blocker scan treated the value as void-like first and therefore
failed to surface the nested blocker.

## Decision

When a returned value has a propagated global-call blocker, report that blocker
even if the same value is seeded as `void` by value-type metadata. The value may
still contribute to the void-sentinel observation, but the blocker is no longer
hidden by the sentinel metadata.

This is diagnostic/contract cleanup only:

- no new direct-call acceptance is added
- no return ABI is widened
- no backend fallback path is introduced
- the lowerer still fails fast on the unresolved child shape

## Evidence

After rebuilding and emitting `lang/src/runner/stage1_cli_env.hako`, the
top-level emit-program route now points at the real child blocker:

```text
Main._run_emit_program_mode/0 -> Stage1SourceProgramAuthorityBox.emit_program_from_source/2
  target_shape_reason=generic_string_global_target_shape_unknown
  target_shape_blocker_symbol=BuildBox._resolve_parse_src/1
  target_shape_blocker_reason=generic_string_return_not_string
```

The owner-local nested routes also remain explicit:

```text
Stage1SourceProgramAuthorityBox._merge_using_prefix/1
  target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1

Stage1SourceProgramAuthorityBox._emit_program_json_from_source_checked/2
  target_shape_blocker_symbol=BuildBox._resolve_parse_src/1
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_void_typed_call_result_child_blocker
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p149_blocker_propagation.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p149_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```
