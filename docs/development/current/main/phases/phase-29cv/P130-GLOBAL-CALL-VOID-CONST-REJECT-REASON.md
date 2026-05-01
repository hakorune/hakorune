---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P130, MIR global-call void const reject reason split
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P129-GLOBAL-CALL-VOID-SENTINEL-BODY-BLOCKER.md
  - src/mir/global_call_route_plan.rs
---

# P130: Global Call Void Const Reject Reason

## Problem

After P129, the first nested blocker is `_stage1_debug_on/0`, which delegates to
`_env_flag_enabled/1`. That helper is not a string body; it is an i64 flag probe
using `env.get/1`, a `void` sentinel presence check, string equality, and i64
returns.

Before this card, MIR reported the target as:

```text
Stage1InputContractBox._env_flag_enabled/1
target_shape_reason=generic_string_unsupported_instruction
```

That reason is too broad for choosing the next accepted shape.

## Decision

Split strict generic-string rejection for `null`/`void` constants:

```text
generic_string_unsupported_void_sentinel_const
```

This reason is diagnostic evidence only. It does not make env flag helpers
lowerable and does not widen `generic_pure_string_body`.

## Rules

Allowed:

- report `null`/`void` constants as a specific generic-string reject reason
- keep `null`/`void` constants allowed only in the string-or-void sentinel body
  scan from P129
- use the reason to identify presence-probe helpers as the next slice

Forbidden:

- accepting `null`/`void` constants in strict generic string bodies
- treating i64 flag helpers as string bodies
- adding backend-local detection for `_env_flag_enabled/1`
- changing runtime/env semantics

## Expected Evidence

After this card, `stage1_cli_env.hako` still fails fast, but the nested blocker
is specific:

```text
callee=Stage1InputContractBox._stage1_debug_on/0
target_shape_blocker_symbol=Stage1InputContractBox._env_flag_enabled/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The next BoxCount can decide whether to add a narrow env-flag i64 helper shape or
split debug-only branches before accepting more global-call targets.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `generic_string_unsupported_void_sentinel_const` for
  `Stage1InputContractBox._env_flag_enabled/1`.
