---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P259a, return-binary text sentinels
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P258A-RETURN-CALL-DIRECT-ARG-BUILDER.md
  - lang/src/mir/builder/func_lowering.hako
---

# P259a: Return Binary Text Sentinels

## Problem

After P258a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_binary/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`FuncLoweringBox._lower_return_binary/5` uses `null` as internal extraction
state for `lhs_type`, `lhs_val`, `rhs_type`, and `rhs_val`.

Those locals are later used as string/scalar values while the helper returns a
string-or-null MIR fragment. This mixes local extraction state with the
function return sentinel and makes the generic string route reject the body.

## Decision

Do not add a new body shape and do not widen generic void-sentinel handling.

Use owner-local text sentinels for internal extraction state:

```text
lhs_type = ""
lhs_val  = ""
rhs_type = ""
rhs_val  = ""
```

`null` remains only as the helper's unsupported-result return value.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic void-sentinel acceptance change
- no change to `Return(Binary(...))` accepted shapes
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p259a_return_binary_text_sentinels.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the
`FuncLoweringBox._lower_return_binary/5` void-sentinel blocker; a later blocker
may remain.

## Result

`FuncLoweringBox._lower_return_binary/5` now routes as:

```text
FuncLoweringBox._lower_return_binary/5  generic_string_or_void_sentinel_body  DirectAbi
```

The nullable read wrappers route as:

```text
FuncLoweringBox._read_string_after_text/2  generic_pure_string_body  DirectAbi
FuncLoweringBox._read_int_after_text/2     generic_pure_string_body  DirectAbi
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
