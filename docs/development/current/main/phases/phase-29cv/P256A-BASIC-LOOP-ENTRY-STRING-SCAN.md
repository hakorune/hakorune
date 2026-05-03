---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P256a, FuncBodyBasic loop entry string scan
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P255A-FUNC-LOWERING-CALL-RESOLVE-TEXT-TABLE.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P256a: Basic Loop Entry String Scan

## Problem

After P255a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_loop/4
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The delegated loop lowerers already route as DirectAbi:

```text
LowerLoopSumBcBox.try_lower/1             generic_string_or_void_sentinel_body
LowerLoopMultiCarrierBox.try_lower/2      generic_string_or_void_sentinel_body
LowerLoopCountParamBox.try_lower_text/1   generic_pure_string_body
LowerLoopSimpleBox.try_lower/1            generic_string_or_void_sentinel_body
FuncBodyBasicLowerBox._rebind/5           generic_string_or_void_sentinel_body
```

The remaining unsupported method call is the loop-entry guard:

```text
body_json.contains("\"type\":\"Loop\"")
```

This guard only needs a textual Program(JSON) marker scan.

## Decision

Do not widen generic string method acceptance.

Coerce the input once to `body_str` and use `JsonFragBox.index_of_from` for the
loop marker guard. Pass the same `body_str` to the delegated loop lowerers.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic method acceptance change
- no change to loop lowering priority
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p256a_basic_loop_entry_string_scan.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the
`FuncBodyBasicLowerBox._try_lower_loop/4` method-call blocker; a later blocker
may remain.

## Result

`FuncBodyBasicLowerBox._try_lower_loop/4` now routes as:

```text
FuncBodyBasicLowerBox._try_lower_loop/4  generic_string_or_void_sentinel_body  DirectAbi
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_binary/5
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
