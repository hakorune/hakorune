---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P219b, MirJsonEmitBox instruction null guard cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P219A-MIR-JSON-INST-FIELD-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P219b: MIR JSON Instruction Null Guard Cleanup

## Problem

P219a clears the `_emit_inst/1` method-call blocker and exposes:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_inst/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

The remaining issue is direct `null` sentinel flow inside `_emit_inst/1`, such
as:

```hako
if payload == null { return "{\"op\":\"mir_call\"}" }
if func_id == null { func_id = inst.get("name") }
```

This is source-level guard shape, not a request for Stage0 to learn broader
void/object union semantics.

## Decision

Normalize `_emit_inst/1` guards to the existing owner-local helpers:

```text
me._is_present_compat(value) == 0
me._is_map_missing_sentinel(value) == 1
```

Do not add a new classifier value class or a new route shape. Keep the cleanup
local to the MIR JSON emitter source.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new generic string value class
- no generic object-or-void support
- no MIR instruction schema change

## Acceptance

Probe result should move past the `_emit_inst/1` void-sentinel blocker; a later
blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p219b_inst_null.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
