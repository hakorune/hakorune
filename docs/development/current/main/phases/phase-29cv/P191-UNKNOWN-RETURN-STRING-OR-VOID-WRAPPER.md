---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P191, unknown-return string-or-void wrapper return profile
Related:
  - docs/development/current/main/phases/phase-29cv/P190-STATIC-STRING-ARRAY-BODY-SHAPE.md
  - docs/development/current/main/phases/phase-29cv/P158-STRING-OR-VOID-PHI-SENTINEL.md
  - docs/development/current/main/phases/phase-29cv/P184-JSONFRAG-NORMALIZER-DIRECT-SHAPE-EMIT-WIRING.md
  - src/mir/global_call_route_plan/string_return_profile.rs
  - lang/src/mir/builder/internal/finalize_chain_box.hako
---

# P191: Unknown Return String-Or-Void Wrapper

## Problem

P190 moved the source-execution blocker to the methodize finalize wrapper:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox._methodize_if_enabled_checked/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The body is a small wrapper:

```hako
if BuilderConfigBox.methodize_on() != 1 { return m }
local result = FuncLoweringBox.methodize_calls_in_mir(m)
if result != null { return result }
me.log_fail("methodize_null")
return null
```

The accepted shape is not a new operation. It is a return-profile fact:
unknown-return wrappers may return the original unknown parameter only when the
same body also has concrete string-return evidence and a void/null sentinel.

## Decision

Extend the string-or-void sentinel return-profile candidate narrowly:

```text
return_type=?
returns unknown param passthrough
returns concrete string/string-or-void evidence
returns void/null sentinel
```

This keeps the final target shape as the existing:

```text
target_shape=generic_string_or_void_sentinel_body
proof=typed_global_call_generic_string_or_void_sentinel
return_shape=string_handle_or_null
```

## Forbidden

- accepting unknown-return unknown-param-or-null wrappers with no concrete
  string-return evidence
- adding by-name `BuilderFinalizeChainBox` logic
- treating arbitrary unknown object params as strings
- adding backend-local body classification

## Expected Evidence

After this card, `_methodize_if_enabled_checked/1` should be direct:

```text
target_shape=generic_string_or_void_sentinel_body
proof=typed_global_call_generic_string_or_void_sentinel
return_shape=string_handle_or_null
```

The source-execution probe advances past
`BuilderFinalizeChainBox._methodize_if_enabled_checked/1` and currently stops at
the next blocker:

```text
target_shape_blocker_symbol=hostbridge.extern_invoke/3
target_shape_blocker_reason=generic_string_global_target_missing
```

## Acceptance

```bash
cargo test -q unknown_return_string_or_void --lib
cargo test -q void_sentinel --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p191_unknown_return_string_or_void_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Probe Result

Observed on 2026-05-02:

```text
target_shape_blocker_symbol=hostbridge.extern_invoke/3
target_shape_blocker_reason=generic_string_global_target_missing
```
