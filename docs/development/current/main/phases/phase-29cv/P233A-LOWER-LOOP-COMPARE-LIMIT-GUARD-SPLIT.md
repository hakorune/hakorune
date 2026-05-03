---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P233a, LowerLoopLocalReturnVar compare-limit guard split
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207D-LOWER-LOOP-LOCAL-RETURN-VAR-GUARD-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P232A-FOLD-VARINT-RESOLVE-SIDE-TEXT-ABI.md
  - lang/src/mir/builder/internal/lower_loop_local_return_var_box.hako
---

# P233a: Lower Loop Compare-Limit Guard Split

## Problem

P232a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerLoopLocalReturnVarBox._read_compare_limit/4
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`_read_compare_limit/4` is a string-or-null schema scanner. It still contains
combined null/value guards:

```hako
if op == null || op != "<" { return null }
if lhs_type == null || lhs_type != "Var" { return null }
if lhs_name == null || lhs_name != varname { return null }
if rhs_type == null || rhs_type != "Int" { return null }
```

Those guards keep null sentinel facts live through boolean joins before the
successful string path is proven.

## Decision

Mirror P207d inside `_read_compare_limit/4` only:

```hako
if value == null { return null }
if value != expected { return null }
```

This is source-owner cleanup. It preserves behavior and avoids teaching the
generic string classifier another void-sentinel control-flow shape.

## Non-Goals

- no `generic_string_body.rs` widening
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to `_read_step_int/4`
- no change to emitted loop MIR JSON

## Acceptance

Probe result should move past `_read_compare_limit/4`; a later blocker may
remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p233a_loop_compare_limit.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=LowerLoopSumBcBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```
