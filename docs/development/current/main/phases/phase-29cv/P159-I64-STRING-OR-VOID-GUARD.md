---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P159, i64 wrapper string-or-void guard state
Related:
  - docs/development/current/main/phases/phase-29cv/P158-STRING-OR-VOID-PHI-SENTINEL.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P159: I64 String-Or-Void Guard State

## Problem

P158 fixed the resolver's own `string|void` PHI shape, but the source-execution
probe then stopped at the emit-program i64 wrapper:

```text
target_symbol=Main._run_emit_program_mode/0
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

The wrapper's shape is narrow and already present in Stage-1 code:

```text
local source_text = resolve_emit_program_source_text()
if source_text == null { return 96 }
...
if program_json == null { return 96 }
return finalize_emit_result(program_json)
```

The missing state was not new control flow. It was forwarding a
`string_handle_or_null` DirectABI child result through a one-input PHI and
checking it against `null` inside an i64-returning wrapper.

## Decision

Add a `StringOrVoid` value class to the generic i64 body classifier and allow
only Eq/Ne comparisons with a void sentinel. Also allow generic string bodies
to forward an existing `StringOrVoid` value through a PHI.

This accepts:

```text
DirectABI string_handle_or_null child -> PHI -> == null / != null -> i64 branch
```

It does not make object returns lowerable and does not add MapBox/ArrayBox
support.

## Evidence

The source-execution probe now moves the top-level stop from the emit-program
wrapper sentinel to the real child owner:

```text
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1UsingResolverBox._collect_using_entries/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

That is the next true boundary: `_collect_using_entries/1` returns an object
(`ArrayBox`) shape, which remains outside the current generic string/i64
DirectABI capsule.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_string_or_void_child_null_guard_in_generic_i64_body --lib
cargo test -q refresh_module_global_call_routes_accepts_string_or_void_child_forward_phi_body --lib
cargo test -q generic_i64 --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p159_i64_string_or_void_guard2.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p159_stage1_cli_env2.exe lang/src/runner/stage1_cli_env.hako
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
