---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P158, generic string-or-void sentinel PHI state
Related:
  - docs/development/current/main/phases/phase-29cv/P157-ENV-SET-EXTERN-ROUTE.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/tests/void_sentinel.rs
---

# P158: String-Or-Void PHI Sentinel State

## Problem

P157 moved the source-execution route past `env.set/2` and exposed the next
over-strict classifier boundary:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The concrete shape is not a new language feature. It is the existing
string-or-null sentinel contract moving through SSA:

```text
local fail_msg = null
...
fail_msg = "[freeze:contract]..."
...
if fail_msg != null { ... }
return null
```

Before P158, `generic_string_body` rejected any PHI that carried a void/null
sentinel, even when the function was already classified through the
`generic_string_or_void_sentinel_body` lane.

## Decision

Add a `StringOrVoid` value class inside the generic string body classifier.
This class is intentionally narrow:

- `void`-only PHI remains a void sentinel.
- `string|void` PHI becomes `StringOrVoid`.
- `StringOrVoid == null` and `StringOrVoid != null` are accepted.
- `StringOrVoid` is not treated as a definite string receiver for methods.
- Scalar/string/void mixed PHI remains rejected.

This keeps the sentinel contract local to the existing DirectABI
`string_handle_or_null` shape and does not add generic object lowering or
MapBox/ArrayBox support.

## Evidence

The new fixture locks the resolver-like pattern:

```text
void-only PHI -> string|void PHI -> null guard -> string_handle_or_null return
```

The source-execution probe now moves past the resolver's previous sentinel
blocker. The next observed owner is the emit-program wrapper/child route:

```text
target_shape_blocker_symbol=-
target_shape_blocker_reason=-
target_shape_reason=generic_string_unsupported_void_sentinel_const
target_symbol=Main._run_emit_program_mode/0
```

Inspecting the emitted MIR shows `Stage1UsingResolverBox.resolve_for_source/1`
now advances to its child route blocker:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox._collect_using_entries/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_string_or_void_phi_guard_body --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p158_string_or_void_phi.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p158_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
