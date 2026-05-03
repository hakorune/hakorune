---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P232a, LowerIfCompareFoldVarInt resolve-side text ABI
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207H-LOWER-IF-COMPARE-FOLD-VARINT-RESOLVE-I64.md
  - docs/development/current/main/phases/phase-29cv/P220A-FOLD-BIN-INTS-RESOLVE-SIDE-TEXT-ABI.md
  - lang/src/mir/builder/internal/lower_if_compare_fold_varint_box.hako
---

# P232a: Fold VarInt Resolve-Side Text ABI

## Problem

P231a advances the source-exe probe back to the residual noted by P207h:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._resolve_side/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`LowerIfCompareFoldVarIntBox.try_lower/1` is a string-producing MIR JSON
lowering path. Its side resolver currently returns scalar i64-or-null:

```text
_resolve_side/3    -> i64 or null
_fold_bin_varint/3 -> i64 or null
```

That is valid semantically, but it forces a scalar-return helper through the
generic string caller lane. Adding a new body shape or widening the generic
string ABI would grow Stage0 for a source-owned representation mismatch.

## Decision

Mirror P220a and keep the resolver source-owned:

```text
successful side resolution -> text handle
failure -> null sentinel
```

`IfMirEmitBox.emit_compare_ret2/5` stores the resolved side values as MIR const
payloads, so text-compatible numeric payloads preserve the existing JSON output
contract while keeping the generic string call boundary handle-compatible.

## Non-Goals

- no `generic_string_body.rs` ABI expansion
- no `generic_i64_body.rs` expansion
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to compare-fold semantics
- no change to `PatternUtilBox.find_local_int_before/3`

## Acceptance

Probe result should move past `_resolve_side/3`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p232a_varint_resolve_side.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=LowerLoopLocalReturnVarBox._read_compare_limit/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
