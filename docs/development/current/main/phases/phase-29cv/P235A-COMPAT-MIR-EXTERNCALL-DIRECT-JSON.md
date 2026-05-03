---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P235a, CompatMirEmit externcall direct JSON owner path
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P234A-LOWER-LOOP-SUM-BC-DIRECT-JSON.md
  - lang/src/mir/builder/internal/compat_mir_emit_box.hako
---

# P235a: Compat MIR Externcall Direct JSON

## Problem

P234a advances the source-exe probe to:

```text
target_shape_blocker_symbol=CompatMirEmitBox._inst_externcall/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`_inst_externcall/3` returns a MapBox MIR instruction object:

```text
%{"op"=> "externcall", "func"=> func, "args"=> arg_ids, ...}
```

The active use is `emit_console_log_then_return/2`. Teaching Stage0 to accept
this helper would pull generic MIR-instruction MapBox construction into the
generic string lane.

## Decision

Keep the fix source-owned. `emit_console_log_then_return/2` already owns the
complete MIR(JSON) shape it emits:

```text
const call_int
externcall env.console.log([call_int])
const ret_int
ret ret_int
```

Emit that JSON string directly and remove the private `_inst_externcall/3`
helper from the active path. This keeps Stage0 from learning a MapBox
instruction-constructor body and preserves the current source pattern.

## Non-Goals

- no `CompatMirEmitBox._inst_externcall/3` body shape
- no generic MapBox instruction-constructor support
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to console-log-then-return semantics

## Acceptance

Probe result should move past `CompatMirEmitBox._inst_externcall/3`. Observed
next blocker after the source cleanup:

```text
target_shape_blocker_symbol=LowerIfNestedBox._read_cmp_side_int/4
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

Verification:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p235a_compat_externcall_direct_json.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
