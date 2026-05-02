---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P216a, MirJsonEmitBox array assert split
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P209A-MIR-JSON-MAP-ASSERT-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P215A-MIR-JSON-CALLEE-FIELD-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P216a: MIR JSON Array Assert Split

## Problem

P215a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._expect_array/2
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

`_expect_array/2` returns the original object, but its active callsites in
`MirJsonEmitBox._emit_inst/1` use it only as a dev assertion:

```hako
me._expect_array(args_box, "mir_call.args")
me._expect_array(effects_box, "mir_call.effects")
```

Teaching Stage0 an object-return body for an ignored assertion would violate the
Stage0 size guard.

## Decision

Mirror P209a's map assertion split:

```text
_assert_array(value, context) -> i64 0
```

Use `_assert_array/2` at ignored assertion sites and retire the local
`_expect_array/2` helper from `MirJsonEmitBox`.

## Non-Goals

- no object-return `GlobalCallTargetShape`
- no generic array object return support
- no C body-specific emitter
- no change to MIR JSON `args` / `effects` schema

## Acceptance

Probe result should move past `_expect_array/2`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p216a_array_assert.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
