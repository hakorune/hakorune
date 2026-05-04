---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380Y, generic i64 body canonical print dead-dst acceptance
Related:
  - docs/development/current/main/phases/phase-29cv/P380X-STAGE1-EMIT-PROGRAM-JSON-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P162-GENERIC-I64-STRING-INDEXOF-METHOD.md
  - docs/development/current/main/phases/phase-29cv/P326A-GLOBAL-PRINT-STRING-HANDLE-MARSHAL.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
---

# P380Y: Generic I64 Print Dead-Dst

## Problem

P380X moved Program(JSON) source emission to an explicit stage1 extern route.
The phase29cg replay now reaches the Program(JSON) result validator:

```text
target_shape_blocker_symbol=Stage1ProgramResultValidationBox.finalize_emit_result/1
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

`Stage1ProgramResultValidationBox.finalize_emit_result/1` is not a string
body. It validates Program(JSON) text, prints diagnostics or the payload, and
returns an i64 exit code:

```text
print(...)
return 96
...
print(prog_text)
return 0
```

P162 already made this validator shape a `generic_i64_body`, but the current
Canonical MIR print surface emits a dead result value for `print`:

```json
{"dst":31,"op":"mir_call","callee":{"type":"Global","name":"print"},"args":[2]}
```

The generic i64 classifier only accepted backend-global `print` when `dst` was
absent. That makes the validator fall through to generic string classification,
which correctly rejects its i64 returns as `generic_string_return_not_string`.

## Decision

Accept backend-global `print` in `generic_i64_body` when:

- callee is exactly the supported backend global `print`
- arity is exactly 1
- `dst` is absent, or the `dst` value is dead inside the function

If a `print` result value is used, keep rejecting the body. The current module
generic emitter consumes `print` as a side-effect surface and does not define a
usable `%r<dst>` result, so accepting a used print result would be unsound.

## Non-Goals

- no new `GlobalCallTargetShape`
- no body-specific validator capsule
- no generic string acceptance for i64 validators
- no C shim result-value publication for `print`
- no VM fallback

## Acceptance

```bash
cargo test --release print_dead_dst -- --nocapture
cargo test --release generic_i64 -- --nocapture

rm -rf /tmp/p380y_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p380y_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: `Stage1ProgramResultValidationBox.finalize_emit_result/1` is no
longer the blocker, and any next failure moves to the next actual unsupported
function or emitter boundary.

## Result

Accepted.

Verified:

```bash
cargo test --release print_dead_dst -- --nocapture
cargo test --release generic_i64 -- --nocapture
cargo build --release --bin hakorune
```

The phase29cg replay now consumes the validator as a direct generic i64 target:

```text
consumer=mir_call_global_generic_i64_emit
site=b6.i1
symbol=Stage1ProgramResultValidationBox.finalize_emit_result/1
```

The replay still fails, but the blocker moved beyond the Program(JSON) result
validator:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1InputContractBox._debug_len_inline/1
```

This keeps the fix in the generic MIR print surface: dead print dsts are
accepted, used print dsts remain a negative canary.
