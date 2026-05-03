---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P268a, return-call lowerer select-free text assembly
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P262A-RETURN-CALL-LOWERER-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P265A-RETURN-CALL-LOWERER-PURE-TEXT-MAINLINE.md
  - lang/src/mir/builder/func_lowering/return_call_lower_box.hako
---

# P268a: Return-Call Lowerer Select-Free Text Assembly

## Problem

After P267a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=ReturnCallLowerBox.lower/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

The lowerer no longer uses methodized/debug side paths, but the generated MIR
still contains `select` instructions from branch-local string assignment
patterns, especially comma insertion while building instruction and argument
lists.

## Decision

Do not add `select` support to the generic string classifier for this owner
helper.

Keep `ReturnCallLowerBox.lower/6` as orchestration and move two local policies
into tiny string helpers:

```text
_extract_call_name(body, call_idx) -> string|null
_append_csv(text, item) -> string
```

The lowerer should build MIR text by assigning helper results, not by mutating
the same string variable in conditional branches.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic classifier acceptance change
- no C shim/body-specific emitter change
- no methodized Return(Call) path restoration
- no call-target resolution policy change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p268a_return_call_lowerer_select_free_text.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`ReturnCallLowerBox.lower/6` as `generic_string_unsupported_instruction`.
A later explicit blocker may remain.

## Result

Accepted.

The source-exe probe no longer stops at `ReturnCallLowerBox.lower/6`:

```text
ReturnCallLowerBox.lower/6              generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
ReturnCallLowerBox._extract_call_name/2 generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
ReturnCallLowerBox._append_csv/2        generic_pure_string_body              string_handle          DirectAbi
```

The next explicit blocker is now the count-param loop lowerer text helper:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
